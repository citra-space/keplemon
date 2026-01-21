// SGP4 Batch Propagation Kernel
// Main propagation kernel based on Vallado's SGP4 algorithm

#include "sgp4_types.cuh"
#include "sgp4_constants.cuh"
#include "sgp4_deepspace.cuh"
#include <stdio.h>

// Debug output disabled for production performance
// Set to 1 to enable detailed propagation debug output
#define DEBUG_PRINT 0

// Helper macro for fused sincos (CUDA provides sincos for double precision)
#define SINCOS(angle, sinvar, cosvar) sincos((angle), &(sinvar), &(cosvar))

// Device function for single satellite propagation
__device__ void sgp4_propagate_single(
    Sgp4Params& p,  // Non-const since deep space updates atime/xli/xni
    double tsince,  // minutes since TLE epoch
    Sgp4State& state,
    int sat_idx,    // for debug output
    int time_idx    // for debug output
) {
    state.error_code = 0;
    
    // Only print debug for first satellite at first time
    bool debug = DEBUG_PRINT && (sat_idx == 0) && (time_idx == 4);  // ISS at t=24h
    
    if (debug) {
        printf("\n=== GPU SGP4 DEBUG OUTPUT ===\n");
        printf("Input tsince: %.10f minutes\n", tsince);
        printf("\n--- Initialized Parameters ---\n");
        printf("epoch_jd:    %.10f\n", p.epoch_jd);
        printf("inclo:       %.10f rad (%.6f deg)\n", p.inclo, p.inclo * RAD2DEG);
        printf("nodeo:       %.10f rad\n", p.nodeo);
        printf("ecco:        %.10f\n", p.ecco);
        printf("argpo:       %.10f rad\n", p.argpo);
        printf("mo:          %.10f rad\n", p.mo);
        printf("no_kozai:    %.10f rad/min\n", p.no_kozai);
        printf("no_unkozai:  %.10f rad/min\n", p.no_unkozai);
        printf("a:           %.10f ER\n", p.a);
        printf("bstar:       %.10e\n", p.bstar);
        printf("is_deep:     %d\n", p.is_deep_space);
        printf("irez:        %d\n", p.irez);
        printf("eta:         %.10f\n", p.eta);
        printf("mdot:        %.10f rad/min\n", p.mdot);
        printf("argpdot:     %.10e rad/min\n", p.argpdot);
        printf("nodedot:     %.10e rad/min\n", p.nodedot);
        printf("cc1:         %.10e\n", p.cc1);
        printf("cc4:         %.10e\n", p.cc4);
        printf("cc5:         %.10e\n", p.cc5);
        printf("t2cof:       %.10e\n", p.t2cof);
        printf("con41:       %.10f\n", p.con41);
        printf("x1mth2:      %.10f\n", p.x1mth2);
        printf("x7thm1:      %.10f\n", p.x7thm1);
        printf("xlcof:       %.10e\n", p.xlcof);
        printf("aycof:       %.10e\n", p.aycof);
        printf("delmo:       %.10f\n", p.delmo);
        printf("sinmao:      %.10f\n", p.sinmao);
        printf("\n--- Drag Coefficients ---\n");
        printf("d2:          %.16e\n", p.d2);
        printf("d3:          %.16e\n", p.d3);
        printf("d4:          %.16e\n", p.d4);
        printf("t3cof:       %.16e\n", p.t3cof);
        printf("t4cof:       %.16e\n", p.t4cof);
        printf("t5cof:       %.16e\n", p.t5cof);
        printf("xnodcf:      %.16e\n", p.xnodcf);
        printf("omgcof:      %.16e\n", p.omgcof);
        printf("xmcof:       %.16e\n", p.xmcof);
    }
    
    // Handle tsince = 0 case (propagation at epoch)
    if (fabs(tsince) < 1e-12) {
        tsince = 0.0;
    }
    
    // ═════════════════════════════════════════════════════════════
    // UPDATE FOR SECULAR GRAVITY AND ATMOSPHERIC DRAG
    // ═════════════════════════════════════════════════════════════
    
    double xmdf = p.mo + p.mdot * tsince;
    double argpdf = p.argpo + p.argpdot * tsince;
    double nodedf = p.nodeo + p.nodedot * tsince;
    double argpm = argpdf;
    double mm = xmdf;
    
    double t2 = tsince * tsince;
    double tempa = 1.0 - p.cc1 * tsince;
    double tempe = p.bstar * p.cc4 * tsince;
    double templ = p.t2cof * t2;
    
    if (debug) {
        printf("\n--- Secular Updates (t=%.2f min) ---\n", tsince);
        printf("xmdf:    %.10f rad\n", xmdf);
        printf("argpdf:  %.10f rad\n", argpdf);
        printf("nodedf:  %.10f rad\n", nodedf);
        printf("tempa:   %.10f\n", tempa);
        printf("tempe:   %.10e\n", tempe);
        printf("templ:   %.10e\n", templ);
    }
    
    double nm = p.no_unkozai;
    double em = p.ecco;
    double inclm = p.inclo;
    double nodem = nodedf + p.xnodcf * t2;

    if (!p.is_deep_space || p.force_near_earth) {
        // Near-earth satellite (or forced to use SGP4)
        double delomg = p.omgcof * tsince;
        double delm = p.xmcof * (pow(1.0 + p.eta * cos(xmdf), 3.0) - p.delmo);
        double temp = delomg + delm;
        mm = xmdf + temp;
        argpm = argpdf - temp;
        
        double t3 = t2 * tsince;
        double t4 = t3 * tsince;

        if (debug) {
            printf("\n--- Before drag updates ---\n");
            printf("tempa(before): %.16f\n", tempa);
            printf("tempe(before): %.16e\n", tempe);
            printf("templ(before): %.16e\n", templ);
            printf("sin(mm):       %.16f\n", sin(mm));
            printf("mm:            %.16f rad\n", mm);
        }

        tempa = tempa - p.d2 * t2 - p.d3 * t3 - p.d4 * t4;
        tempe = tempe + p.bstar * p.cc5 * (sin(mm) - p.sinmao);
        templ = templ + p.t3cof * t3 + t4 * (p.t4cof + tsince * p.t5cof);
        
        if (debug) {
            printf("\n--- Near-Earth Secular Terms ---\n");
            printf("delomg:  %.16e\n", delomg);
            printf("delm:    %.16e\n", delm);
            printf("temp:    %.16e\n", temp);
            printf("mm:      %.16f rad\n", mm);
            printf("argpm:   %.16f rad\n", argpm);
            printf("nodem:   %.16f rad\n", nodem);
            printf("t3:      %.16e\n", t3);
            printf("t4:      %.16e\n", t4);
            printf("tempa(final): %.16f\n", tempa);
            printf("tempe(final): %.16e\n", tempe);
            printf("templ(final): %.16e\n", templ);
        }
    } else {
        // ═══════════════════════════════════════════════════════════════
        // DEEP SPACE SECULAR EFFECTS
        // ═══════════════════════════════════════════════════════════════
        
        // Initialize working variables
        mm = xmdf;
        argpm = argpdf;
        nodem = nodedf;
        double tc = tsince;
        
        // Apply deep space secular contributions (DSPACE)
        dspace(tsince, tc, em, argpm, inclm, mm, nm, nodem, p);
        
        if (debug) {
            printf("\n--- Deep Space Secular ---\n");
            printf("nm (ds):    %.10f rad/min\n", nm);
            printf("em (ds):    %.10f\n", em);
            printf("inclm (ds): %.10f rad\n", inclm);
            printf("mm (ds):    %.10f rad\n", mm);
            printf("argpm (ds): %.10f rad\n", argpm);
            printf("nodem (ds): %.10f rad\n", nodem);
        }
    }
    
    // Update for secular effects
    double am = pow(XKE / nm, X2O3) * tempa * tempa;
    nm = XKE / pow(am, 1.5);
    em = em - tempe;
    
    if (debug) {
        printf("\n--- Semi-major axis ---\n");
        printf("am:      %.10f ER (%.6f km)\n", am, am * RE);
        printf("nm:      %.10f rad/min\n", nm);
        printf("em:      %.10f\n", em);
    }
    
    // Error checks
    if (em < 1.0e-6) em = 1.0e-6;
    if (em > 0.9999 || am < 0.95) {
        state.error_code = 1;  // Satellite has decayed
        if (debug) printf("ERROR: Satellite decayed (am=%.6f, em=%.6f)\n", am, em);
        return;
    }
    
    mm = mm + p.no_unkozai * templ;

    // For deep space, apply lunar-solar periodics (DPPER)
    double xnode = nodem;
    if (p.is_deep_space && !p.force_near_earth) {
        dpper(p.inclo, false, tsince, em, inclm, nodem, argpm, mm, p, debug);
        xnode = nodem;
        
        if (debug) {
            printf("\n--- Deep Space Periodics ---\n");
            printf("em (pp):    %.10f\n", em);
            printf("inclm (pp): %.10f rad\n", inclm);
            printf("nodem (pp): %.10f rad\n", nodem);
            printf("argpm (pp): %.10f rad\n", argpm);
            printf("mm (pp):    %.10f rad\n", mm);
        }
    }
    
    // Normalize angles (following python-sgp4 exactly)
    double xlm = mm + argpm + xnode;
    xnode = fmod(xnode, TWOPI);
    argpm = fmod(argpm, TWOPI);
    xlm = fmod(xlm, TWOPI);
    mm = fmod(xlm - argpm - xnode, TWOPI);
    if (xnode < 0.0) xnode += TWOPI;
    if (mm < 0.0) mm += TWOPI;
    
    if (debug) {
        printf("mm (final): %.10f rad\n", mm);
        printf("xnode:      %.10f rad\n", xnode);
        printf("argpm:      %.10f rad\n", argpm);
    }
    
    // ═════════════════════════════════════════════════════════════
    // LONG PERIOD PERIODICS
    // ═════════════════════════════════════════════════════════════
    
    double sinip, cosip;
    SINCOS(inclm, sinip, cosip);
    
    // For deep space satellites, recalculate aycof and xlcof using
    // the dpper-modified inclination (matching python-sgp4 behavior)
    double aycof_eff = p.aycof;
    double xlcof_eff = p.xlcof;
    if (p.is_deep_space && !p.force_near_earth) {
        aycof_eff = -0.5 * J3OJ2 * sinip;
        if (fabs(cosip + 1.0) > 1.5e-12) {
            xlcof_eff = -0.25 * J3OJ2 * sinip * (3.0 + 5.0 * cosip) / (1.0 + cosip);
        } else {
            xlcof_eff = -0.25 * J3OJ2 * sinip * (3.0 + 5.0 * cosip) / 1.5e-12;
        }
    }
    
    double sinargpm, cosargpm;
    SINCOS(argpm, sinargpm, cosargpm);

    double axnl = em * cosargpm;
    double temp = 1.0 / (am * (1.0 - em * em));
    double aynl = em * sinargpm + temp * aycof_eff;
    double xl = mm + argpm + xnode + temp * xlcof_eff * axnl;

    if (debug) {
        printf("\n=== GPU LONG-PERIOD (t=1440min) ===\n");
        printf("inclm:   %.16f rad\n", inclm);
        printf("em:      %.16f\n", em);
        printf("am:      %.16f ER\n", am);
        printf("argpm:   %.16f rad\n", argpm);
        printf("ep:      %.16f\n", em);  // ep is same as em at this point
        printf("axnl:    %.16f\n", axnl);
        printf("aynl:    %.16f\n", aynl);
        printf("xl:      %.16f rad\n", xl);
        printf("aycof:   %.16e\n", aycof_eff);
        printf("xlcof:   %.16e\n", xlcof_eff);
    }
    
    // ═════════════════════════════════════════════════════════════
    // SOLVE KEPLER'S EQUATION
    // ═════════════════════════════════════════════════════════════
    
    double u = fmod(xl - xnode, TWOPI);
    double eo1 = u;
    double tem5 = 1.0;
    int ktr = 1;
    
    // Newton-Raphson iteration for eccentric anomaly
    while (fabs(tem5) >= 1.0e-12 && ktr <= 10) {
        double sineo1, coseo1;
        SINCOS(eo1, sineo1, coseo1);
        tem5 = 1.0 - coseo1 * axnl - sineo1 * aynl;
        tem5 = (u - aynl * coseo1 + axnl * sineo1 - eo1) / tem5;
        
        if (fabs(tem5) >= 0.95) {
            tem5 = tem5 > 0.0 ? 0.95 : -0.95;
        }
        eo1 = eo1 + tem5;
        ktr++;
    }
    
    if (debug) {
        printf("\n--- Kepler Solution ---\n");
        printf("u:       %.10f rad\n", u);
        printf("eo1:     %.10f rad (iterations: %d)\n", eo1, ktr-1);
    }
    
    // ═════════════════════════════════════════════════════════════
    // SHORT PERIOD PERIODICS
    // ═════════════════════════════════════════════════════════════
    
    double sineo1, coseo1;
    SINCOS(eo1, sineo1, coseo1);
    
    double ecose = axnl * coseo1 + aynl * sineo1;
    double esine = axnl * sineo1 - aynl * coseo1;
    double el2 = axnl * axnl + aynl * aynl;
    double pl = am * (1.0 - el2);
    
    if (pl < 0.0) {
        state.error_code = 2;  // Semi-latus rectum < 0
        if (debug) printf("ERROR: pl < 0\n");
        return;
    }
    
    double rl = am * (1.0 - ecose);
    double rdotl = sqrt(am) * esine / rl;
    double rvdotl = sqrt(pl) / rl;
    double betal = sqrt(1.0 - el2);
    temp = esine / (1.0 + betal);
    
    double sinu = am / rl * (sineo1 - aynl - axnl * temp);
    double cosu = am / rl * (coseo1 - axnl + aynl * temp);
    double su = atan2(sinu, cosu);
    
    if (debug) {
        printf("\n--- Argument of Latitude ---\n");
        printf("sineo1:  %.10f\n", sineo1);
        printf("coseo1:  %.10f\n", coseo1);
        printf("ecose:   %.10f\n", ecose);
        printf("esine:   %.10f\n", esine);
        printf("betal:   %.10f\n", betal);
        printf("temp(esine/1+b): %.10f\n", temp);
        printf("sinu:    %.10f\n", sinu);
        printf("cosu:    %.10f\n", cosu);
        printf("su:      %.10f rad\n", su);
    }
    
    double sin2u = (cosu + cosu) * sinu;
    double cos2u = 1.0 - 2.0 * sinu * sinu;
    
    temp = 1.0 / pl;
    double temp1 = 0.5 * J2 * temp;
    double temp2 = temp1 * temp;
    
    // For deep space satellites, recalculate con41, x1mth2, x7thm1
    // using the dpper-modified inclination (matching python-sgp4 behavior)
    double con41_eff = p.con41;
    double x1mth2_eff = p.x1mth2;
    double x7thm1_eff = p.x7thm1;
    if (p.is_deep_space && !p.force_near_earth) {
        double cosisq = cosip * cosip;
        con41_eff = 3.0 * cosisq - 1.0;
        x1mth2_eff = 1.0 - cosisq;
        x7thm1_eff = 7.0 * cosisq - 1.0;
    }
    
    if (debug) {
        printf("\n--- Short Period Periodics (BEFORE updates) ---\n");
        printf("rl:      %.16f ER\n", rl);
        printf("rdotl:   %.16f\n", rdotl);
        printf("rvdotl:  %.16f\n", rvdotl);
        printf("su:      %.16f rad\n", su);
        printf("sin2u:   %.16f\n", sin2u);
        printf("cos2u:   %.16f\n", cos2u);
        printf("pl:      %.16f ER\n", pl);
        printf("temp:    %.16f\n", temp);
        printf("temp1:   %.16e\n", temp1);
        printf("temp2:   %.16e\n", temp2);
        printf("betal:   %.16f\n", betal);
        printf("cosip:   %.16f\n", cosip);
        printf("sinip:   %.16f\n", sinip);
        printf("con41_eff:  %.16f\n", con41_eff);
        printf("x1mth2_eff: %.16f\n", x1mth2_eff);
        printf("x7thm1_eff: %.16f\n", x7thm1_eff);
    }
    
    // Update short period periodics
    double mrt = rl * (1.0 - 1.5 * temp2 * betal * con41_eff) + 
                 0.5 * temp1 * x1mth2_eff * cos2u;
    su = su - 0.25 * temp2 * x7thm1_eff * sin2u;
    double xnode_new = xnode + 1.5 * temp2 * cosip * sin2u;
    double xinc = inclm + 1.5 * temp2 * cosip * sinip * cos2u;
    double mvt = rdotl - nm * temp1 * x1mth2_eff * sin2u / XKE;
    double rvdot = rvdotl + nm * temp1 * (x1mth2_eff * cos2u + 1.5 * con41_eff) / XKE;
    
    if (debug) {
        printf("\n--- Short Period Periodics (AFTER updates) ---\n");
        printf("mrt:     %.16f ER (%.10f km)\n", mrt, mrt * RE);
        printf("su_new:  %.16f rad\n", su);
        printf("xnode:   %.16f rad\n", xnode_new);
        printf("xinc:    %.16f rad\n", xinc);
        printf("mvt:     %.16f\n", mvt);
        printf("rvdot:   %.10f\n", rvdot);
        printf("xinc:    %.10f rad\n", xinc);
    }
    
    // ═════════════════════════════════════════════════════════════
    // ORIENTATION VECTORS
    // ═════════════════════════════════════════════════════════════
    
    double sinsu, cossu;
    SINCOS(su, sinsu, cossu);
    double snod, cnod;
    SINCOS(xnode_new, snod, cnod);
    double sini, cosi;
    SINCOS(xinc, sini, cosi);
    
    double xmx = -snod * cosi;
    double xmy = cnod * cosi;
    
    double ux = xmx * sinsu + cnod * cossu;
    double uy = xmy * sinsu + snod * cossu;
    double uz = sini * sinsu;
    
    double vx = xmx * cossu - cnod * sinsu;
    double vy = xmy * cossu - snod * sinsu;
    double vz = sini * cossu;
    
    if (debug) {
        printf("\n--- Orientation Vectors ---\n");
        printf("sinsu:   %.16f\n", sinsu);
        printf("cossu:   %.16f\n", cossu);
        printf("snod:    %.16f\n", snod);
        printf("cnod:    %.16f\n", cnod);
        printf("sini:    %.16f\n", sini);
        printf("cosi:    %.16f\n", cosi);
        printf("U: [%.16f, %.16f, %.16f]\n", ux, uy, uz);
        printf("V: [%.16f, %.16f, %.16f]\n", vx, vy, vz);
    }
    
    // ═════════════════════════════════════════════════════════════
    // POSITION AND VELOCITY (km and km/s in TEME frame)
    // ═════════════════════════════════════════════════════════════
    
    double mrt_RE = mrt * RE;
    state.x = mrt_RE * ux;
    state.y = mrt_RE * uy;
    state.z = mrt_RE * uz;
    state.vx = (mvt * ux + rvdot * vx) * VKMPERSEC;
    state.vy = (mvt * uy + rvdot * vy) * VKMPERSEC;
    state.vz = (mvt * uz + rvdot * vz) * VKMPERSEC;
    
    if (debug) {
        printf("\n--- Final Output ---\n");
        printf("Position: [%.6f, %.6f, %.6f] km\n", state.x, state.y, state.z);
        printf("Velocity: [%.6f, %.6f, %.6f] km/s\n", state.vx, state.vy, state.vz);
        printf("|r| = %.6f km\n", sqrt(state.x*state.x + state.y*state.y + state.z*state.z));
        printf("|v| = %.6f km/s\n", sqrt(state.vx*state.vx + state.vy*state.vy + state.vz*state.vz));
        printf("=== END GPU DEBUG ===\n\n");
    }
}

// Main batch propagation kernel
// Takes Julian Dates and computes tsince per satellite
extern "C" __global__ void sgp4_propagate_kernel(
    const Sgp4Params* __restrict__ params,  // [n_sats]
    const double* __restrict__ jd_times,     // [n_times] Julian Dates
    Sgp4State* __restrict__ states,          // [n_sats * n_times]
    int n_sats,
    int n_times
) {
    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;
    
    if (sat_idx >= n_sats || time_idx >= n_times) return;
    
    // Create a local copy of params for this thread
    // Deep space propagation modifies atime/xli/xni, so each thread needs its own copy
    Sgp4Params p = params[sat_idx];
    
    // Compute tsince (minutes since this satellite's TLE epoch)
    double jd = jd_times[time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;
    
    Sgp4State& state = states[sat_idx * n_times + time_idx];
    
    sgp4_propagate_single(p, tsince, state, sat_idx, time_idx);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SoA (STRUCT OF ARRAYS) PROPAGATION KERNEL
// ═══════════════════════════════════════════════════════════════════════════════
//
// Optimizations vs the AoS kernel:
// 1. Shared memory caching for jd_times - all threads in a block share time values
// 2. SoA output layout - coalesced writes with time-major ordering
// 3. Reduced global memory transactions per warp
//
// Memory layout: state_x[time_idx * n_sats + sat_idx] (time-major)
// This ensures adjacent threads (adjacent sat_idx) write to adjacent addresses

// Maximum number of times that can be cached in shared memory per block
// 256 doubles * 8 bytes = 2KB shared memory for times
#define MAX_TIMES_SHARED 256

extern "C" __global__ void sgp4_propagate_soa_kernel(
    const Sgp4Params* __restrict__ params,      // [n_sats] satellite parameters
    const double* __restrict__ jd_times,         // [n_times] Julian Dates
    double* __restrict__ state_x,                // [n_sats * n_times] output X positions
    double* __restrict__ state_y,                // [n_sats * n_times] output Y positions
    double* __restrict__ state_z,                // [n_sats * n_times] output Z positions
    double* __restrict__ state_vx,               // [n_sats * n_times] output X velocities
    double* __restrict__ state_vy,               // [n_sats * n_times] output Y velocities
    double* __restrict__ state_vz,               // [n_sats * n_times] output Z velocities
    int* __restrict__ state_error,               // [n_sats * n_times] error codes
    int n_sats,
    int n_times
) {
    // Shared memory for time values - reduces global memory reads
    // Each block caches up to MAX_TIMES_SHARED time values
    __shared__ double shared_times[MAX_TIMES_SHARED];
    
    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;
    
    // Linear thread index within block for shared memory loading
    int thread_id = threadIdx.y * blockDim.x + threadIdx.x;
    int block_size = blockDim.x * blockDim.y;
    
    // Cooperatively load time values into shared memory
    // Only load times that this block's y-range will access
    int time_block_start = blockIdx.y * blockDim.y;
    int time_block_end = min(time_block_start + (int)blockDim.y, n_times);
    int times_to_load = time_block_end - time_block_start;
    
    // Each thread helps load time values
    for (int i = thread_id; i < times_to_load && i < MAX_TIMES_SHARED; i += block_size) {
        int global_time_idx = time_block_start + i;
        if (global_time_idx < n_times) {
            shared_times[i] = jd_times[global_time_idx];
        }
    }
    __syncthreads();
    
    // Bounds check after shared memory load
    if (sat_idx >= n_sats || time_idx >= n_times) return;
    
    // Create a local copy of params for this thread
    // Deep space propagation modifies atime/xli/xni, so each thread needs its own copy
    Sgp4Params p = params[sat_idx];
    
    // Get time from shared memory (local index within block's time range)
    int local_time_idx = time_idx - time_block_start;
    double jd = shared_times[local_time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;
    
    // Use the same propagation function as the AoS kernel for correctness
    Sgp4State state;
    sgp4_propagate_single(p, tsince, state, sat_idx, time_idx);
    
    // Write output with time-major ordering for coalesced access
    // Adjacent satellites (sat_idx, sat_idx+1) write to adjacent memory addresses
    // This is optimal because blockDim.x threads have adjacent sat_idx values
    int out_idx = time_idx * n_sats + sat_idx;

    state_x[out_idx] = state.x;
    state_y[out_idx] = state.y;
    state_z[out_idx] = state.z;
    state_vx[out_idx] = state.vx;
    state_vy[out_idx] = state.vy;
    state_vz[out_idx] = state.vz;
    state_error[out_idx] = state.error_code;
}

// ═══════════════════════════════════════════════════════════════════════════════
// INDEXED SoA KERNEL - Supports GPU-side scatter for two-kernel optimization
// ═══════════════════════════════════════════════════════════════════════════════
//
// This kernel writes to indexed positions, allowing partitioned satellites
// (SGP4 and SDP4) to write directly to their correct positions in a shared
// output buffer without CPU-side scatter.
//
// Memory layout: state_x[time_idx * n_total_sats + original_indices[sat_idx]]

extern "C" __global__ void sgp4_propagate_soa_indexed_kernel(
    const Sgp4Params* __restrict__ params,       // [n_partition_sats] partition params
    const double* __restrict__ jd_times,          // [n_times] Julian Dates
    const int* __restrict__ original_indices,     // [n_partition_sats] index mapping
    double* __restrict__ state_x,                 // [n_total_sats * n_times] shared output
    double* __restrict__ state_y,
    double* __restrict__ state_z,
    double* __restrict__ state_vx,
    double* __restrict__ state_vy,
    double* __restrict__ state_vz,
    int* __restrict__ state_error,
    long long packed_dims,                        // high 32-bits: n_partition_sats, low 32-bits: n_total_sats
    int n_times
) {
    // Unpack dimensions
    int n_partition_sats = (int)(packed_dims >> 32);
    int n_total_sats = (int)(packed_dims & 0xFFFFFFFF);
    __shared__ double shared_times[MAX_TIMES_SHARED];

    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;

    int thread_id = threadIdx.y * blockDim.x + threadIdx.x;
    int block_size = blockDim.x * blockDim.y;

    // Cooperatively load time values into shared memory
    int time_block_start = blockIdx.y * blockDim.y;
    int time_block_end = min(time_block_start + (int)blockDim.y, n_times);
    int times_to_load = time_block_end - time_block_start;

    for (int i = thread_id; i < times_to_load && i < MAX_TIMES_SHARED; i += block_size) {
        int global_time_idx = time_block_start + i;
        if (global_time_idx < n_times) {
            shared_times[i] = jd_times[global_time_idx];
        }
    }
    __syncthreads();

    if (sat_idx >= n_partition_sats || time_idx >= n_times) return;

    Sgp4Params p = params[sat_idx];

    int local_time_idx = time_idx - time_block_start;
    double jd = shared_times[local_time_idx];
    double tsince = (jd - p.epoch_jd) * MINUTES_PER_DAY;

    Sgp4State state;
    sgp4_propagate_single(p, tsince, state, sat_idx, time_idx);

    // Write to indexed position in shared output buffer
    // Uses original_indices to map partition satellite to original position
    int original_sat_idx = original_indices[sat_idx];
    int out_idx = time_idx * n_total_sats + original_sat_idx;

    state_x[out_idx] = state.x;
    state_y[out_idx] = state.y;
    state_z[out_idx] = state.z;
    state_vx[out_idx] = state.vx;
    state_vy[out_idx] = state.vy;
    state_vz[out_idx] = state.vz;
    state_error[out_idx] = state.error_code;
}
