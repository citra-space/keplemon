# GPU Performance Analysis and Optimization

This document details the CUDA GPU implementation for SGP4/SDP4 satellite propagation, including benchmark results, optimization techniques applied, and analysis of performance characteristics.

## Executive Summary

| Metric | Value |
|--------|-------|
| Peak GPU Throughput | **14.6M propagations/sec** |
| Kernel-only Speedup vs Sequential CPU | **3.4x** |
| Speedup with GPU-to-CPU Transfer | **2.6-2.8x** |
| Transfer Overhead | ~23% of total time |

The GPU implementation provides meaningful acceleration for batch propagation workloads, with the primary value being:
- Freeing CPU for other tasks during propagation
- GPU-resident mode for chained GPU operations (collision detection, etc.)
- Consistent throughput that doesn't degrade with satellite count

## Benchmark Results

### Test Configuration
- **Satellite mix**: 60% LEO (SGP4), 40% GEO (SDP4)
- **Time points**: 10,080 (7 days at 1-minute intervals)
- **Test satellites**: Mix of ISS, Starlink, GPS, TDRS, and other operational satellites

### Performance by Satellite Count

| Satellites | Total Props | CPU Time | GPU Kernel | GPU + Transfer | Kernel Speedup | Transfer Overhead |
|------------|-------------|----------|------------|----------------|----------------|-------------------|
| 10 | 100,800 | 24ms | 28ms | 29ms | 0.86x | 2.5% |
| 20 | 201,600 | 48ms | 28ms | 30ms | 1.69x | 4.5% |
| 40 | 403,200 | 96ms | 29ms | 37ms | **3.33x** | 21.8% |
| 80 | 806,400 | 191ms | 57ms | 74ms | **3.36x** | 23.0% |
| 160 | 1,612,800 | 378ms | 110ms | 136ms | **3.43x** | 18.8% |
| 1000 | 10,080,000 | 2,362ms | 687ms | 894ms | **3.44x** | 23.1% |

### Throughput Comparison

| Implementation | Throughput (props/sec) |
|----------------|------------------------|
| CPU Sequential | ~4.2M |
| GPU Kernel Only | ~14.6M |
| GPU + Transfer | ~11.3M |

### Key Observations

1. **Crossover point**: GPU becomes faster than CPU at ~30-40 satellites
2. **Throughput plateau**: GPU throughput stabilizes at ~14.6M props/sec regardless of satellite count
3. **Transfer overhead**: Consistently ~23% for large batches (560MB for 1000 sats × 10k times)

## Optimizations Implemented

### 1. Two-Kernel Launch (Warp Divergence Elimination)

**Problem**: Mixed LEO/GEO constellations caused severe warp divergence. Threads in the same warp would execute different code paths (SGP4 for near-earth vs SDP4 for deep-space), serializing execution.

**Solution**: Partition satellites by orbital period at initialization:
- SGP4 partition: Mean motion > 6.4 rev/day (period < 225 min)
- SDP4 partition: Mean motion ≤ 6.4 rev/day (period ≥ 225 min)

Launch separate kernels for each partition, eliminating cross-partition warp divergence.

**Impact**: ~1.9x improvement (from 1.8x to 3.4x speedup)

### 2. GPU-Side Scatter (Indexed Kernel)

**Problem**: After two-kernel partitioning, results needed to be scattered back to original satellite order. CPU-side scatter was inefficient.

**Solution**: New `sgp4_propagate_soa_indexed_kernel` that accepts an index mapping array. Each partition kernel writes directly to the correct position in the shared output buffer.

```cuda
// Write to indexed position in shared output buffer
int original_sat_idx = original_indices[sat_idx];
int out_idx = time_idx * n_total_sats + original_sat_idx;
state_x[out_idx] = state.x;
```

**Impact**: ~20% improvement in total throughput

### 3. GPU-Resident API

**Problem**: For GPU pipeline use cases (e.g., propagate then run collision detection), downloading results to CPU and re-uploading is wasteful.

**Solution**: `propagate_resident()` returns `CudaSlice<f64>` buffers that remain on GPU memory.

```rust
// GPU-resident mode - no transfer
let gpu_results = propagator.propagate_resident(&times)?;
// Use directly in next GPU operation
collision_kernel.launch(cfg, (&gpu_results.x, &gpu_results.y, ...))?;
```

**Impact**: Eliminates 23% transfer overhead for chained GPU operations

### 4. SoA Memory Layout

**Problem**: Array-of-Structures (AoS) layout causes non-coalesced memory access on GPU.

**Solution**: Structure-of-Arrays (SoA) layout with time-major ordering:
```
buffer[time_idx * n_sats + sat_idx]
```

Adjacent threads access adjacent memory addresses, enabling coalesced memory transactions.

### 5. Shared Memory Time Caching

**Problem**: Each thread loading its own time value from global memory is inefficient.

**Solution**: Cooperative loading of time values into shared memory:
```cuda
__shared__ double shared_times[256];
// Threads cooperatively load times
for (int i = thread_id; i < times_to_load; i += block_size) {
    shared_times[i] = jd_times[time_block_start + i];
}
__syncthreads();
// All threads read from fast shared memory
double jd = shared_times[local_time_idx];
```

## Performance Limiters

### 1. Register Pressure (Primary Bottleneck)

The SGP4/SDP4 algorithm requires many intermediate variables:

```
Orbital elements: inclination, raan, eccentricity, arg_perigee, mean_anomaly, mean_motion
Perturbation terms: xmdf, argpdf, nodedf, argpm, nodem, mm, nm, em, inclm
Position/velocity: x, y, z, vx, vy, vz, r, rdot, rvdot
Trigonometric: sin/cos of various angles (10+ pairs)
Intermediate: t2, t3, t4, tempa, tempe, templ, am, xlm, etc.
```

**Estimated register usage**: 100-150 registers per thread (800-1200 bytes)

**Impact on occupancy**: With 64KB registers per SM, only 2-3 warps can execute concurrently instead of the optimal 8-12 warps. This limits occupancy to ~25-33%.

### 2. Computational Intensity

SGP4 is compute-heavy with:
- 10+ `SINCOS` operations (~20 cycles each)
- 4+ `fmod` operations (~20-30 cycles each)
- Newton-Raphson iteration (up to 10 iterations with 2 SINCOS each)
- Multiple square roots and divisions

The kernel is **compute-bound**, not memory-bound.

### 3. Algorithm Structure

Even with two-kernel partitioning, internal branches remain:
- Lyddane vs standard coordinate conversion
- Eccentricity-dependent calculations
- Error condition checks

## Potential Future Optimizations

| Optimization | Potential Gain | Implementation Difficulty | Notes |
|--------------|----------------|---------------------------|-------|
| Register spilling to local memory | 1.3-1.5x | High | Trade register pressure for memory latency |
| Precomputed trig lookup tables | 1.1-1.2x | Medium | Accuracy tradeoff |
| Half-precision intermediates | 1.5-2x | Medium | Significant accuracy impact |
| Fused multiply-add (FMA) | 1.05-1.1x | Low | Minor gains |
| Batch time upload caching | 1.05x | Low | Already partially implemented |

**Realistic ceiling with all optimizations**: ~5-6x speedup over sequential CPU

## API Usage

### Standard API (with GPU-to-CPU transfer)

```rust
use keplemon::gpu::{CudaTlePropagator, TleDataGpu};

let mut propagator = CudaTlePropagator::new()?;
propagator.init_satellites(&tle_data)?;

// Returns Vec<f64> arrays on CPU
let results = propagator.propagate_soa_arrays(&julian_dates)?;
println!("Position: ({}, {}, {})", results.x[0], results.y[0], results.z[0]);
```

### GPU-Resident API (for GPU pipelines)

```rust
// Returns CudaSlice<f64> buffers on GPU
let gpu_results = propagator.propagate_resident(&julian_dates)?;

// Use directly with other GPU operations
let device = propagator.cuda_device();
// custom_kernel.launch(cfg, (&gpu_results.x, &gpu_results.y))?;

// Convert to CPU only when needed
let cpu_results = gpu_results.to_soa_arrays(device)?;
```

## Comparison with CPU Parallelism

For comparison, enabling Rayon parallel iteration on CPU typically achieves:
- 4-core CPU: ~3-4x speedup over sequential
- 8-core CPU: ~6-8x speedup over sequential

The GPU's 3.4x kernel speedup is comparable to a 4-core CPU, but:
1. GPU frees CPU cores for other work
2. GPU throughput is consistent regardless of satellite count
3. GPU-resident mode enables zero-copy pipelines

## Recommendations

### When to Use GPU

1. **Large batches**: 100+ satellites with many time points
2. **GPU pipelines**: When results feed into other GPU computations
3. **CPU offloading**: When CPU is needed for other tasks
4. **Consistent latency**: When predictable performance matters

### When to Use CPU

1. **Small batches**: <30 satellites
2. **Single propagations**: One satellite, one time
3. **No GPU available**: Graceful fallback
4. **Memory constrained**: GPU memory is limited

## Benchmark Reproduction

Run the benchmark with:
```bash
cargo test --features cuda test_benchmark_cpu_vs_gpu --release -- --nocapture
```

For quick validation:
```bash
cargo test --features cuda test_quick_benchmark --release -- --nocapture
```

## Hardware Tested

Results in this document were obtained on consumer NVIDIA GPUs. Performance may vary on:
- Data center GPUs (A100, H100): Expect 1.5-2x better due to more SMs
- Older GPUs (Pascal, Volta): Expect 0.7-0.9x due to fewer SMs
- Integrated GPUs: Not recommended for this workload
