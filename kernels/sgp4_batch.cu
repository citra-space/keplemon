// SGP4 Batch Propagation Kernel
// Main propagation kernel based on Vallado's SGP4 algorithm

#include "sgp4_types.cuh"
#include "sgp4_constants.cuh"
#include "sgp4_deepspace.cuh"
#include <stdio.h>

// Debug output disabled for production performance
// Set to 1 only for debugging single satellite issues
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
    bool debug = DEBUG_PRINT && (sat_idx == 0) && (time_idx == 0);
    
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
    double nodem = nodedf;
    
    if (!p.is_deep_space) {
        // Near-earth satellite
        double delomg = p.omgcof * tsince;
        double delm = p.xmcof * (pow(1.0 + p.eta * cos(xmdf), 3.0) - p.delmo);
        double temp = delomg + delm;
        mm = xmdf + temp;
        argpm = argpdf - temp;
        
        double t3 = t2 * tsince;
        double t4 = t3 * tsince;
        tempa = tempa - p.d2 * t2 - p.d3 * t3 - p.d4 * t4;
        tempe = tempe + p.bstar * p.cc5 * (sin(mm) - p.sinmao);
        templ = templ + p.t3cof * t3 + t4 * (p.t4cof + tsince * p.t5cof);
        
        if (debug) {
            printf("delomg:  %.10e\n", delomg);
            printf("delm:    %.10e\n", delm);
            printf("mm:      %.10f rad\n", mm);
            printf("argpm:   %.10f rad\n", argpm);
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
    
    mm = mm + nm * templ;
    
    // For deep space, apply lunar-solar periodics (DPPER)
    double xnode = nodem;
    if (p.is_deep_space) {
        dpper(p.inclo, false, tsince, em, inclm, nodem, argpm, mm, p);
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
    
    // Normalize angles
    xnode = fmod(xnode, TWOPI);
    argpm = fmod(argpm, TWOPI);
    mm = fmod(mm, TWOPI);
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
    
    double sinargpm, cosargpm;
    SINCOS(argpm, sinargpm, cosargpm);
    
    double axnl = em * cosargpm;
    double temp = 1.0 / (am * (1.0 - em * em));
    double aynl = em * sinargpm + temp * p.aycof;
    double xl = mm + argpm + xnode + temp * p.xlcof * axnl;
    
    if (debug) {
        printf("\n--- Long Period Periodics ---\n");
        printf("axnl:    %.10f\n", axnl);
        printf("aynl:    %.10f\n", aynl);
        printf("xl:      %.10f rad\n", xl);
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
    
    double sin2u = (cosu + cosu) * sinu;
    double cos2u = 1.0 - 2.0 * sinu * sinu;
    
    temp = 1.0 / pl;
    double temp1 = 0.5 * J2 * temp;
    double temp2 = temp1 * temp;
    
    if (debug) {
        printf("\n--- Short Period Periodics ---\n");
        printf("rl:      %.10f ER\n", rl);
        printf("rdotl:   %.10f\n", rdotl);
        printf("rvdotl:  %.10f\n", rvdotl);
        printf("su:      %.10f rad\n", su);
        printf("temp1:   %.10e\n", temp1);
        printf("temp2:   %.10e\n", temp2);
    }
    
    // Update short period periodics
    double mrt = rl * (1.0 - 1.5 * temp2 * betal * p.con41) + 
                 0.5 * temp1 * p.x1mth2 * cos2u;
    su = su - 0.25 * temp2 * p.x7thm1 * sin2u;
    double xnode_new = xnode + 1.5 * temp2 * cosip * sin2u;
    double xinc = inclm + 1.5 * temp2 * cosip * sinip * cos2u;
    double mvt = rdotl - nm * temp1 * p.x1mth2 * sin2u / XKE;
    double rvdot = rvdotl + nm * temp1 * (p.x1mth2 * cos2u + 1.5 * p.con41) / XKE;
    
    if (debug) {
        printf("mrt:     %.10f ER (%.6f km)\n", mrt, mrt * RE);
        printf("mvt:     %.10f\n", mvt);
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
        printf("U: [%.10f, %.10f, %.10f]\n", ux, uy, uz);
        printf("V: [%.10f, %.10f, %.10f]\n", vx, vy, vz);
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
