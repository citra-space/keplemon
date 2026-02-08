// ═══════════════════════════════════════════════════════════════════════════════════
// GEO Numerical Propagation Kernel
// ═══════════════════════════════════════════════════════════════════════════════════
//
// CUDA kernel for GPU-accelerated GEO satellite propagation using numerical
// integration (RK4, Cowell's method).
//
// STATUS: Experimental - suitable for short-term propagation only (<7 days).
// Slower than GPU SDP4; use only when propagating from ECI states (no TLE).
// Accuracy: ~12 km @ 7d, ~689 km @ 30d vs CPU SDP4 reference.
//
// ═══════════════════════════════════════════════════════════════════════════════════

#include "geo_numerical.cuh"
#include <stdio.h>

// Maximum number of times that can be cached in shared memory per block
#define GEO_MAX_TIMES_SHARED 256

// ═══════════════════════════════════════════════════════════════════════════════════
// INITIALIZATION KERNEL
// ═══════════════════════════════════════════════════════════════════════════════════
// Converts ECI states to GeoNumericalParams for efficient batch propagation

extern "C" __global__ void geo_init_kernel(
    const GeoStateGpu* __restrict__ states,      // [n_sats] Input ECI states
    GeoNumericalParams* __restrict__ params,    // [n_sats] Output parameters
    int n_sats
) {
    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (sat_idx >= n_sats) return;

    const GeoStateGpu& state = states[sat_idx];
    GeoNumericalParams& param = params[sat_idx];

    // Initialize parameters from ECI state
    geo_init_from_eci(&state, &param);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BATCH PROPAGATION KERNEL (AoS Output)
// ═══════════════════════════════════════════════════════════════════════════════════

extern "C" __global__ void geo_propagate_kernel(
    const GeoNumericalParams* __restrict__ params,  // [n_sats] satellite parameters
    const double* __restrict__ jd_times,              // [n_times] Julian Dates
    Sgp4State* __restrict__ states,                   // [n_sats * n_times] output (reuses Sgp4State)
    int n_sats,
    int n_times
) {
    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;

    if (sat_idx >= n_sats || time_idx >= n_times) return;

    const GeoNumericalParams& p = params[sat_idx];

    // Compute time since epoch (minutes)
    double jd = jd_times[time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;

    // Propagate
    Sgp4State& state = states[sat_idx * n_times + time_idx];
    geo_propagate_single(&p, tsince, &state.x, &state.y, &state.z,
                         &state.vx, &state.vy, &state.vz, &state.error_code);
    state._padding = 0;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SoA PROPAGATION KERNEL (Optimized Memory Layout)
// ═══════════════════════════════════════════════════════════════════════════════════

extern "C" __global__ void geo_propagate_soa_kernel(
    const GeoNumericalParams* __restrict__ params,  // [n_sats] satellite parameters
    const double* __restrict__ jd_times,              // [n_times] Julian Dates
    double* __restrict__ state_x,                     // [n_sats * n_times] output X positions
    double* __restrict__ state_y,                     // [n_sats * n_times] output Y positions
    double* __restrict__ state_z,                     // [n_sats * n_times] output Z positions
    double* __restrict__ state_vx,                    // [n_sats * n_times] output X velocities
    double* __restrict__ state_vy,                    // [n_sats * n_times] output Y velocities
    double* __restrict__ state_vz,                    // [n_sats * n_times] output Z velocities
    int* __restrict__ state_error,                    // [n_sats * n_times] error codes
    int n_sats,
    int n_times
) {
    // Shared memory for time values
    __shared__ double shared_times[GEO_MAX_TIMES_SHARED];

    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;

    int thread_id = threadIdx.y * blockDim.x + threadIdx.x;
    int block_size = blockDim.x * blockDim.y;

    // Cooperatively load time values into shared memory
    int time_block_start = blockIdx.y * blockDim.y;
    int time_block_end = min(time_block_start + (int)blockDim.y, n_times);
    int times_to_load = time_block_end - time_block_start;

    for (int i = thread_id; i < times_to_load && i < GEO_MAX_TIMES_SHARED; i += block_size) {
        int global_time_idx = time_block_start + i;
        if (global_time_idx < n_times) {
            shared_times[i] = jd_times[global_time_idx];
        }
    }
    __syncthreads();

    if (sat_idx >= n_sats || time_idx >= n_times) return;

    const GeoNumericalParams& p = params[sat_idx];

    // Get time from shared memory
    int local_time_idx = time_idx - time_block_start;
    double jd = shared_times[local_time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;

    // Propagate
    double x, y, z, vx, vy, vz;
    int error_code;
    geo_propagate_single(&p, tsince, &x, &y, &z, &vx, &vy, &vz, &error_code);

    // Write output with time-major ordering for coalesced access
    int out_idx = time_idx * n_sats + sat_idx;
    state_x[out_idx] = x;
    state_y[out_idx] = y;
    state_z[out_idx] = z;
    state_vx[out_idx] = vx;
    state_vy[out_idx] = vy;
    state_vz[out_idx] = vz;
    state_error[out_idx] = error_code;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INDEXED SoA KERNEL - For partitioned propagation with GPU-side scatter
// ═══════════════════════════════════════════════════════════════════════════════════

extern "C" __global__ void geo_propagate_soa_indexed_kernel(
    const GeoNumericalParams* __restrict__ params,   // [n_partition_sats] partition params
    const double* __restrict__ jd_times,               // [n_times] Julian Dates
    const int* __restrict__ original_indices,          // [n_partition_sats] index mapping
    double* __restrict__ state_x,                      // [n_total_sats * n_times] shared output
    double* __restrict__ state_y,
    double* __restrict__ state_z,
    double* __restrict__ state_vx,
    double* __restrict__ state_vy,
    double* __restrict__ state_vz,
    int* __restrict__ state_error,
    long long packed_dims,                             // high 32-bits: n_partition_sats, low: n_total_sats
    int n_times
) {
    int n_partition_sats = (int)(packed_dims >> 32);
    int n_total_sats = (int)(packed_dims & 0xFFFFFFFF);

    __shared__ double shared_times[GEO_MAX_TIMES_SHARED];

    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;

    int thread_id = threadIdx.y * blockDim.x + threadIdx.x;
    int block_size = blockDim.x * blockDim.y;

    // Cooperatively load time values
    int time_block_start = blockIdx.y * blockDim.y;
    int time_block_end = min(time_block_start + (int)blockDim.y, n_times);
    int times_to_load = time_block_end - time_block_start;

    for (int i = thread_id; i < times_to_load && i < GEO_MAX_TIMES_SHARED; i += block_size) {
        int global_time_idx = time_block_start + i;
        if (global_time_idx < n_times) {
            shared_times[i] = jd_times[global_time_idx];
        }
    }
    __syncthreads();

    if (sat_idx >= n_partition_sats || time_idx >= n_times) return;

    const GeoNumericalParams& p = params[sat_idx];

    int local_time_idx = time_idx - time_block_start;
    double jd = shared_times[local_time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;

    // Propagate
    double x, y, z, vx, vy, vz;
    int error_code;
    geo_propagate_single(&p, tsince, &x, &y, &z, &vx, &vy, &vz, &error_code);

    // Write to indexed position using original satellite index
    int original_sat_idx = original_indices[sat_idx];
    int out_idx = time_idx * n_total_sats + original_sat_idx;

    state_x[out_idx] = x;
    state_y[out_idx] = y;
    state_z[out_idx] = z;
    state_vx[out_idx] = vx;
    state_vy[out_idx] = vy;
    state_vz[out_idx] = vz;
    state_error[out_idx] = error_code;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DIRECT ECI PROPAGATION KERNEL (No initialization required)
// ═══════════════════════════════════════════════════════════════════════════════════
// This kernel takes raw ECI states and propagates them directly.
// Useful when you want to propagate without pre-initializing parameters.
// Less efficient for repeated propagations of the same satellites.

extern "C" __global__ void geo_propagate_eci_kernel(
    const GeoStateGpu* __restrict__ eci_states,  // [n_sats] Input ECI states at epoch
    const double* __restrict__ jd_times,          // [n_times] Julian Dates
    double* __restrict__ state_x,                 // [n_sats * n_times] output
    double* __restrict__ state_y,
    double* __restrict__ state_z,
    double* __restrict__ state_vx,
    double* __restrict__ state_vy,
    double* __restrict__ state_vz,
    int* __restrict__ state_error,
    int n_sats,
    int n_times
) {
    __shared__ double shared_times[GEO_MAX_TIMES_SHARED];

    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;

    int thread_id = threadIdx.y * blockDim.x + threadIdx.x;
    int block_size = blockDim.x * blockDim.y;

    // Load times cooperatively
    int time_block_start = blockIdx.y * blockDim.y;
    int time_block_end = min(time_block_start + (int)blockDim.y, n_times);
    int times_to_load = time_block_end - time_block_start;

    for (int i = thread_id; i < times_to_load && i < GEO_MAX_TIMES_SHARED; i += block_size) {
        int global_time_idx = time_block_start + i;
        if (global_time_idx < n_times) {
            shared_times[i] = jd_times[global_time_idx];
        }
    }
    __syncthreads();

    if (sat_idx >= n_sats || time_idx >= n_times) return;

    // Initialize parameters on-the-fly (less efficient but simpler API)
    const GeoStateGpu& eci = eci_states[sat_idx];
    GeoNumericalParams params;
    geo_init_from_eci(&eci, &params);

    // Get time from shared memory
    int local_time_idx = time_idx - time_block_start;
    double jd = shared_times[local_time_idx];
    double tsince = (jd - params.epoch_jd) * MINUTES_PER_DAY;

    // Propagate
    double x, y, z, vx, vy, vz;
    int error_code;
    geo_propagate_single(&params, tsince, &x, &y, &z, &vx, &vy, &vz, &error_code);

    // Write output
    int out_idx = time_idx * n_sats + sat_idx;
    state_x[out_idx] = x;
    state_y[out_idx] = y;
    state_z[out_idx] = z;
    state_vx[out_idx] = vx;
    state_vy[out_idx] = vy;
    state_vz[out_idx] = vz;
    state_error[out_idx] = error_code;
}
