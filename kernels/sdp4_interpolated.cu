// ═══════════════════════════════════════════════════════════════════════════════════
// SDP4 Interpolated Propagation Kernel
// ═══════════════════════════════════════════════════════════════════════════════════
//
// CUDA kernels for GPU-accelerated SDP4-compatible propagation using pre-sampled
// resonance interpolation.
//
// Key innovation: Pre-compute resonance at regular intervals during initialization,
// then interpolate during propagation. Eliminates iterative DSPACE loops that
// cause thread divergence in standard GPU SDP4.
//
// Expected performance: ~20-50x speedup vs standard GPU SDP4 (which achieves ~1.6x)
//
// ═══════════════════════════════════════════════════════════════════════════════════

#include "sdp4_interpolated.cuh"
#include <stdio.h>

// Maximum number of times that can be cached in shared memory per block
#define SDP4_MAX_TIMES_SHARED 256

// ═══════════════════════════════════════════════════════════════════════════════════
// INITIALIZATION KERNEL
// ═══════════════════════════════════════════════════════════════════════════════════
// Initializes Sdp4InterpolatedParams from TLE data and pre-samples resonance

extern "C" __global__ void sdp4_interpolated_init_kernel(
    const TleData* __restrict__ tles,          // [n_sats] Input TLE data
    Sdp4InterpolatedParams* __restrict__ params, // [n_sats] Output parameters
    Sdp4ResonanceSample* __restrict__ samples, // [n_sats * SDP4_MAX_RESONANCE_SAMPLES] Resonance samples
    int n_sats
) {
    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (sat_idx >= n_sats) return;

    const TleData& tle = tles[sat_idx];
    Sdp4InterpolatedParams& p = params[sat_idx];
    Sdp4ResonanceSample* sat_samples = &samples[sat_idx * SDP4_MAX_RESONANCE_SAMPLES];

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 1: Store TLE epoch and elements
    // ═══════════════════════════════════════════════════════════════════════════
    p.epoch_jd = tle.epoch_jd;
    p.inclo = tle.inclination * DEG2RAD;
    p.nodeo = tle.raan * DEG2RAD;
    p.ecco = tle.eccentricity;
    p.argpo = tle.arg_perigee * DEG2RAD;
    p.mo = tle.mean_anomaly * DEG2RAD;
    p.bstar = tle.bstar;

    // Convert mean motion from rev/day to rad/min
    double no_kozai = tle.mean_motion * TWOPI / MINUTES_PER_DAY;
    p.no_kozai = no_kozai;

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 2: SGP4-style initialization (un-kozai mean motion)
    // ═══════════════════════════════════════════════════════════════════════════
    double cosio = cos(p.inclo);
    double sinio = sin(p.inclo);
    p.cosio = cosio;
    p.sinio = sinio;
    p.cosio2 = cosio * cosio;
    p.cosio4 = p.cosio2 * p.cosio2;
    p.x1mth2 = 1.0 - p.cosio2;
    p.x7thm1 = 7.0 * p.cosio2 - 1.0;
    p.con41 = 3.0 * p.cosio2 - 1.0;  // Must match SGP4 exactly
    p.con42 = 1.0 - 5.0 * p.cosio2;  // This is different from con41

    double eccsq = p.ecco * p.ecco;
    double omeosq = 1.0 - eccsq;
    double rteosq = sqrt(omeosq);
    double posq = pow(XKE / no_kozai, X2O3);
    double rp = posq * (1.0 - p.ecco);

    // Un-kozai the mean motion
    double d1 = 0.75 * J2 * (3.0 * p.cosio2 - 1.0) / (rteosq * omeosq);
    double a1 = pow(XKE / no_kozai, X2O3);
    double del = d1 / (a1 * a1);
    double adel = a1 * (1.0 - del * del - del * (1.0 / 3.0 + 134.0 * del * del / 81.0));
    del = d1 / (adel * adel);
    double no_unkozai = no_kozai / (1.0 + del);
    p.no_unkozai = no_unkozai;

    double ao = pow(XKE / no_unkozai, X2O3);
    p.a = ao;
    p.alta = ao * (1.0 + p.ecco) - 1.0;
    p.altp = ao * (1.0 - p.ecco) - 1.0;

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 3: Compute SGP4 initialization parameters
    // ═══════════════════════════════════════════════════════════════════════════
    double po = ao * omeosq;
    double con42_temp = 1.0 - 5.0 * p.cosio2;
    double qoms24 = pow((120.0 - 78.0) / RE, 4.0);  // qoms2t from SGP4
    double ss = 78.0 / RE + 1.0;

    // Adjust for perigee height
    if (rp < 156.0 / RE + 1.0) {
        double s = 20.0;
        if (rp < 98.0 / RE + 1.0) {
            s = rp - 1.0;
        } else {
            s = rp - 78.0 / RE;
        }
        qoms24 = pow((120.0 - s * RE) / RE, 4.0);
        ss = s / RE + 1.0;
    }

    double pinvsq = 1.0 / (ao * ao * omeosq * omeosq);
    double tsi = 1.0 / (ao - ss);
    p.eta = ao * p.ecco * tsi;
    double etasq = p.eta * p.eta;
    double eeta = p.ecco * p.eta;
    double psisq = fabs(1.0 - etasq);
    double coef = qoms24 * pow(tsi, 4.0);
    double coef1 = coef / pow(psisq, 3.5);

    double cc2 = coef1 * no_unkozai * (ao * (1.0 + 1.5 * etasq + eeta * (4.0 + etasq)) +
                  0.375 * J2 * tsi / psisq * p.con41 * (8.0 + 3.0 * etasq * (8.0 + etasq)));
    p.cc1 = p.bstar * cc2;
    double cc3 = 0.0;
    if (p.ecco > 1.0e-4) {
        // Must match SGP4: cc3 = -2 * coef * tsi * J3 / (J2 * no_unkozai * sinio)
        cc3 = -2.0 * coef * tsi * J3 / (J2 * no_unkozai * p.sinio);
    }
    p.cc4 = 2.0 * no_unkozai * coef1 * ao * omeosq *
            (p.eta * (2.0 + 0.5 * etasq) + p.ecco * (0.5 + 2.0 * etasq) -
             J2 * tsi / (ao * psisq) *
             (-3.0 * p.con41 * (1.0 - 2.0 * eeta + etasq * (1.5 - 0.5 * eeta)) +
              0.75 * p.x1mth2 * (2.0 * etasq - eeta * (1.0 + etasq)) * cos(2.0 * p.argpo)));
    p.cc5 = 2.0 * coef1 * ao * omeosq * (1.0 + 2.75 * (etasq + eeta) + eeta * etasq);

    double cosio4 = p.cosio2 * p.cosio2;
    double temp1 = 1.5 * J2 * pinvsq * no_unkozai;
    double temp2 = 0.5 * temp1 * J2 * pinvsq;
    double temp3 = -0.46875 * J4 * pinvsq * pinvsq * no_unkozai;

    p.mdot = no_unkozai + 0.5 * temp1 * rteosq * p.con41 +
             0.0625 * temp2 * rteosq * (13.0 - 78.0 * p.cosio2 + 137.0 * cosio4);
    p.argpdot = -0.5 * temp1 * con42_temp +
                0.0625 * temp2 * (7.0 - 114.0 * p.cosio2 + 395.0 * cosio4) +
                temp3 * (3.0 - 36.0 * p.cosio2 + 49.0 * cosio4);
    double xhdot1 = -temp1 * p.cosio;
    p.nodedot = xhdot1 + (0.5 * temp2 * (4.0 - 19.0 * p.cosio2) +
                          2.0 * temp3 * (3.0 - 7.0 * p.cosio2)) * p.cosio;
    p.omgcof = p.bstar * cc3 * cos(p.argpo);
    p.xmcof = 0.0;
    if (p.ecco > 1.0e-4) {
        p.xmcof = -X2O3 * coef * p.bstar / eeta;
    }
    p.xnodcf = 3.5 * omeosq * xhdot1 * p.cc1;
    p.t2cof = 1.5 * p.cc1;

    // Compute remaining coefficients
    double xlcof, aycof;
    if (fabs(cosio + 1.0) > 1.5e-12) {
        xlcof = -0.25 * J3OJ2 * p.sinio * (3.0 + 5.0 * cosio) / (1.0 + cosio);
    } else {
        xlcof = -0.25 * J3OJ2 * p.sinio * (3.0 + 5.0 * cosio) / 1.5e-12;
    }
    aycof = -0.5 * J3OJ2 * p.sinio;
    p.xlcof = xlcof;
    p.aycof = aycof;

    p.delmo = pow(1.0 + p.eta * cos(p.mo), 3.0);
    p.sinmao = sin(p.mo);

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 4: Calculate Greenwich Sidereal Time at epoch (must match SGP4 exactly)
    // ═══════════════════════════════════════════════════════════════════════════
    double epoch = p.epoch_jd - 2433281.5;  // Days since Jan 0, 1950
    // ts70 = total days since Jan 0, 1970 (including fraction)
    double ts70 = epoch - 7305.0;
    // ds70 = integer days since Jan 0, 1970
    double ds70 = floor(ts70 + 1.0e-8);
    // tfrac = fractional day
    double tfrac = ts70 - ds70;
    double c1_gst = 1.72027916940703639e-2;
    double thgr70 = 1.7321343856509374;
    double fk5r = 5.07551419432269442e-15;
    double c1p2p = c1_gst + TWOPI;
    p.gsto = fmod(thgr70 + c1_gst * ds70 + c1p2p * tfrac + ts70 * ts70 * fk5r, TWOPI);
    if (p.gsto < 0.0) p.gsto = p.gsto + TWOPI;

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 5: DSCOM - Compute lunar-solar constants
    // ═══════════════════════════════════════════════════════════════════════════
    double sinim, cosim, emsq, rtemsq;
    double s1, s2, s3, s4, s5, s6, s7;
    double ss1, ss2, ss3, ss4, ss5, ss6, ss7;
    double sz1, sz2, sz3, sz11, sz12, sz13, sz21, sz22, sz23, sz31, sz32, sz33;
    double z1, z2, z3, z11, z12, z13, z21, z22, z23, z31, z32, z33;

    sdp4_dscom(
        epoch, p.ecco, p.argpo, 0.0, p.inclo, p.nodeo, no_unkozai,
        p,
        sinim, cosim, emsq, rtemsq,
        s1, s2, s3, s4, s5, s6, s7,
        ss1, ss2, ss3, ss4, ss5, ss6, ss7,
        sz1, sz2, sz3, sz11, sz12, sz13, sz21, sz22, sz23, sz31, sz32, sz33,
        z1, z2, z3, z11, z12, z13, z21, z22, z23, z31, z32, z33
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 6: DSINIT - Compute secular rates and resonance terms
    // ═══════════════════════════════════════════════════════════════════════════
    double xpidot = p.argpdot + p.nodedot;

    sdp4_dsinit(
        cosim, emsq, p.argpo, s1, s2, s3, s4, s5, sinim, ss1,
        ss2, ss3, ss4, ss5, sz1, sz3, sz11, sz13, sz21, sz23,
        sz31, sz33, p.gsto, p.mo, p.mdot, no_unkozai, p.nodeo, p.nodedot,
        xpidot, z1, z3, z11, z13, z21, z23, z31, z33,
        p.ecco, eccsq, p.ecco, p.inclo, no_unkozai,
        p
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 7: Initialize periodic baselines (standard SGP4 does NOT call DPPER at init)
    // The baseline periodics (peo, pinco, plo, pgho, pho) remain at 0 as set by dscom.
    // This means dpper will not subtract any baseline during propagation.
    // ═══════════════════════════════════════════════════════════════════════════

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 8: Pre-sample resonance
    // ═══════════════════════════════════════════════════════════════════════════
    int n_samples;
    sdp4_presample_resonance(p, sat_samples, n_samples);
    p.n_samples = n_samples;
    p.sample_t0 = -SDP4_MAX_PROPAGATION_SPAN_MIN;
    p.sample_dt = SDP4_RESONANCE_SAMPLE_INTERVAL_MIN;

    // Store original index
    p.original_index = sat_idx;
    p._padding = 0;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SoA PROPAGATION KERNEL
// ═══════════════════════════════════════════════════════════════════════════════════

extern "C" __global__ void sdp4_interpolated_propagate_soa_kernel(
    const Sdp4InterpolatedParams* __restrict__ params,   // [n_sats] satellite parameters
    const Sdp4ResonanceSample* __restrict__ samples,   // [n_sats * SDP4_MAX_RESONANCE_SAMPLES]
    const double* __restrict__ jd_times,                // [n_times] Julian Dates
    double* __restrict__ state_x,                       // [n_sats * n_times] output
    double* __restrict__ state_y,
    double* __restrict__ state_z,
    double* __restrict__ state_vx,
    double* __restrict__ state_vy,
    double* __restrict__ state_vz,
    int* __restrict__ state_error,
    int n_sats,
    int n_times
) {
    // Shared memory for time values
    __shared__ double shared_times[SDP4_MAX_TIMES_SHARED];

    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;

    int thread_id = threadIdx.y * blockDim.x + threadIdx.x;
    int block_size = blockDim.x * blockDim.y;

    // Cooperatively load time values into shared memory
    int time_block_start = blockIdx.y * blockDim.y;
    int time_block_end = min(time_block_start + (int)blockDim.y, n_times);
    int times_to_load = time_block_end - time_block_start;

    for (int i = thread_id; i < times_to_load && i < SDP4_MAX_TIMES_SHARED; i += block_size) {
        int global_time_idx = time_block_start + i;
        if (global_time_idx < n_times) {
            shared_times[i] = jd_times[global_time_idx];
        }
    }
    __syncthreads();

    if (sat_idx >= n_sats || time_idx >= n_times) return;

    const Sdp4InterpolatedParams& p = params[sat_idx];
    const Sdp4ResonanceSample* sat_samples = &samples[sat_idx * SDP4_MAX_RESONANCE_SAMPLES];

    // Get time from shared memory
    int local_time_idx = time_idx - time_block_start;
    double jd = shared_times[local_time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;

    // Propagate
    double x, y, z, vx, vy, vz;
    int error_code;
    sdp4_propagate_single(p, sat_samples, tsince, &x, &y, &z, &vx, &vy, &vz, &error_code);

    // Write output with time-major ordering
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
// AoS PROPAGATION KERNEL
// ═══════════════════════════════════════════════════════════════════════════════════

extern "C" __global__ void sdp4_interpolated_propagate_kernel(
    const Sdp4InterpolatedParams* __restrict__ params,   // [n_sats] satellite parameters
    const Sdp4ResonanceSample* __restrict__ samples,   // [n_sats * SDP4_MAX_RESONANCE_SAMPLES]
    const double* __restrict__ jd_times,                // [n_times] Julian Dates
    Sgp4State* __restrict__ states,                     // [n_sats * n_times] output
    int n_sats,
    int n_times
) {
    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;

    if (sat_idx >= n_sats || time_idx >= n_times) return;

    const Sdp4InterpolatedParams& p = params[sat_idx];
    const Sdp4ResonanceSample* sat_samples = &samples[sat_idx * SDP4_MAX_RESONANCE_SAMPLES];

    double jd = jd_times[time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;

    Sgp4State& state = states[sat_idx * n_times + time_idx];
    sdp4_propagate_single(p, sat_samples, tsince,
                          &state.x, &state.y, &state.z,
                          &state.vx, &state.vy, &state.vz,
                          &state.error_code);
    state._padding = 0;
}
