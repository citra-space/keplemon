// SGP4 Batch Propagation Kernel
// Main propagation kernel based on Vallado's SGP4 algorithm

#include "sgp4_types.cuh"
#include "sgp4_constants.cuh"

// Device function for single satellite propagation
__device__ void sgp4_propagate_single(
    const Sgp4Params& p,
    double tsince,  // minutes since TLE epoch
    Sgp4State& state
) {
    state.error_code = 0;
    
    // ═════════════════════════════════════════════════════════════
    // UPDATE FOR SECULAR GRAVITY AND ATMOSPHERIC DRAG
    // ═════════════════════════════════════════════════════════════
    
    double xmdf = p.mo + p.mdot * tsince;
    double argpdf = p.argpo + p.argpdot * tsince;
    double nodedf = p.nodeo + p.nodedot * tsince;
    double argpm = argpdf;
    double mm = xmdf;
    
    double t2 = tsince * tsince;
    double xnode = nodedf + p.xnodcf * t2;
    double tempa = 1.0 - p.cc1 * tsince;
    double tempe = p.bstar * p.cc4 * tsince;
    double templ = p.t2cof * t2;
    
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
    }
    
    double nm = p.no_unkozai;
    double em = p.ecco;
    double inclm = p.inclo;
    
    // Update for secular effects
    double am = pow(XKE / nm, X2O3) * tempa * tempa;
    nm = XKE / pow(am, 1.5);
    em = em - tempe;
    
    // Error checks
    if (em < 1.0e-6) em = 1.0e-6;
    if (em > 0.9999 || am < 0.95) {
        state.error_code = 1;  // Satellite has decayed
        return;
    }
    
    mm = mm + nm * templ;
    
    // Normalize angles
    xnode = fmod(xnode, TWOPI);
    argpm = fmod(argpm, TWOPI);
    mm = fmod(mm, TWOPI);
    
    // ═════════════════════════════════════════════════════════════
    // LONG PERIOD PERIODICS
    // ═════════════════════════════════════════════════════════════
    
    double sinip = sin(inclm);
    double cosip = cos(inclm);
    
    double axnl = em * cos(argpm);
    double temp = 1.0 / (am * (1.0 - em * em));
    double aynl = em * sin(argpm) + temp * p.aycof;
    double xl = mm + argpm + xnode + temp * p.xlcof * axnl;
    
    // ═════════════════════════════════════════════════════════════
    // SOLVE KEPLER'S EQUATION
    // ═════════════════════════════════════════════════════════════
    
    double u = fmod(xl - xnode, TWOPI);
    double eo1 = u;
    double tem5 = 1.0;
    int ktr = 1;
    
    // Newton-Raphson iteration for eccentric anomaly
    while (fabs(tem5) >= 1.0e-12 && ktr <= 10) {
        double sineo1 = sin(eo1);
        double coseo1 = cos(eo1);
        tem5 = 1.0 - coseo1 * axnl - sineo1 * aynl;
        tem5 = (u - aynl * coseo1 + axnl * sineo1 - eo1) / tem5;
        
        if (fabs(tem5) >= 0.95) {
            tem5 = tem5 > 0.0 ? 0.95 : -0.95;
        }
        eo1 = eo1 + tem5;
        ktr++;
    }
    
    // ═════════════════════════════════════════════════════════════
    // SHORT PERIOD PERIODICS
    // ═════════════════════════════════════════════════════════════
    
    double sineo1 = sin(eo1);
    double coseo1 = cos(eo1);
    
    double ecose = axnl * coseo1 + aynl * sineo1;
    double esine = axnl * sineo1 - aynl * coseo1;
    double el2 = axnl * axnl + aynl * aynl;
    double pl = am * (1.0 - el2);
    
    if (pl < 0.0) {
        state.error_code = 2;  // Semi-latus rectum < 0
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
    
    // Update short period periodics
    double mrt = rl * (1.0 - 1.5 * temp2 * betal * p.con41) + 
                 0.5 * temp1 * p.x1mth2 * cos2u;
    su = su - 0.25 * temp2 * p.x7thm1 * sin2u;
    double xnode_new = xnode + 1.5 * temp2 * cosip * sin2u;
    double xinc = inclm + 1.5 * temp2 * cosip * sinip * cos2u;
    double mvt = rdotl - nm * temp1 * p.x1mth2 * sin2u / XKE;
    double rvdot = rvdotl + nm * temp1 * (p.x1mth2 * cos2u + 1.5 * p.con41) / XKE;
    
    // ═════════════════════════════════════════════════════════════
    // ORIENTATION VECTORS
    // ═════════════════════════════════════════════════════════════
    
    double sinsu = sin(su);
    double cossu = cos(su);
    double snod = sin(xnode_new);
    double cnod = cos(xnode_new);
    double sini = sin(xinc);
    double cosi = cos(xinc);
    
    double xmx = -snod * cosi;
    double xmy = cnod * cosi;
    
    double ux = xmx * sinsu + cnod * cossu;
    double uy = xmy * sinsu + snod * cossu;
    double uz = sini * sinsu;
    
    double vx = xmx * cossu - cnod * sinsu;
    double vy = xmy * cossu - snod * sinsu;
    double vz = sini * cossu;
    
    // ═════════════════════════════════════════════════════════════
    // POSITION AND VELOCITY (km and km/s in TEME frame)
    // ═════════════════════════════════════════════════════════════
    
    state.x = mrt * ux * RE;
    state.y = mrt * uy * RE;
    state.z = mrt * uz * RE;
    state.vx = (mvt * ux + rvdot * vx) * VKMPERSEC;
    state.vy = (mvt * uy + rvdot * vy) * VKMPERSEC;
    state.vz = (mvt * uz + rvdot * vz) * VKMPERSEC;
}

// Main batch propagation kernel
__global__ void sgp4_propagate_batch(
    const Sgp4Params* __restrict__ params,  // [n_sats]
    const double* __restrict__ times_tsince, // [n_times] minutes since epoch
    Sgp4State* __restrict__ states,          // [n_sats * n_times]
    int n_sats,
    int n_times
) {
    int sat_idx = blockIdx.x * blockDim.x + threadIdx.x;
    int time_idx = blockIdx.y * blockDim.y + threadIdx.y;
    
    if (sat_idx >= n_sats || time_idx >= n_times) return;
    
    const Sgp4Params& p = params[sat_idx];
    double tsince = times_tsince[time_idx];
    Sgp4State& state = states[sat_idx * n_times + time_idx];
    
    sgp4_propagate_single(p, tsince, state);
}
