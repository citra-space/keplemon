// SGP4 Initialization Kernel
// Based on Vallado's sgp4init() from his C++ implementation
// Uses WGS-72 constants for compatibility with AFSPC/saal

#include "sgp4_types.cuh"
#include "sgp4_constants.cuh"
#include "sgp4_deepspace.cuh"

// Forward declaration
__device__ void sgp4_init_deepspace(const TleData& tle, Sgp4Params& p, 
    double ao, double con42, double cosio, double sinio, double cosio2,
    double eccsq, double omeosq, double posq, double rp, double rteosq,
    double s4, double xpidot);

// Device function for SGP4 initialization
__device__ void sgp4_init_single(const TleData& tle, Sgp4Params& p) {
    // ═══════════════════════════════════════════════════════════════════
    // CONVERT TLE ELEMENTS TO INTERNAL UNITS
    // ═══════════════════════════════════════════════════════════════════
    
    // Store epoch
    p.epoch_jd = tle.epoch_jd;
    
    // Angles: TLE is in degrees, convert to radians
    p.inclo = tle.inclination * DEG2RAD;
    p.nodeo = tle.raan * DEG2RAD;
    p.ecco = tle.eccentricity;
    p.argpo = tle.arg_perigee * DEG2RAD;
    p.mo = tle.mean_anomaly * DEG2RAD;
    p.bstar = tle.bstar;
    p.ndot = tle.ndot;
    p.nddot = tle.nddot;
    
    // Mean motion: TLE is in revs/day, convert to radians/minute
    p.no_kozai = tle.mean_motion * TWOPI / MINUTES_PER_DAY;
    
    // ═══════════════════════════════════════════════════════════════════
    // GEOMETRIC PARAMETERS
    // ═══════════════════════════════════════════════════════════════════
    
    double cosio = cos(p.inclo);
    double sinio = sin(p.inclo);
    double cosio2 = cosio * cosio;
    double cosio4 = cosio2 * cosio2;
    
    p.cosio = cosio;
    p.cosio2 = cosio2;
    p.cosio4 = cosio4;
    
    double eccsq = p.ecco * p.ecco;
    double omeosq = 1.0 - eccsq;
    double rteosq = sqrt(omeosq);
    
    // Common SGP4 expressions
    p.con41 = 3.0 * cosio2 - 1.0;
    p.x1mth2 = 1.0 - cosio2;
    p.x7thm1 = 7.0 * cosio2 - 1.0;
    
    // ═══════════════════════════════════════════════════════════════════
    // UN-KOZAI THE MEAN MOTION
    // ═══════════════════════════════════════════════════════════════════
    
    double a1 = pow(XKE / p.no_kozai, X2O3);
    double d1 = 0.75 * J2 * (3.0 * cosio2 - 1.0) / (rteosq * omeosq);
    double del_1 = d1 / (a1 * a1);
    double adel = a1 * (1.0 - del_1 * del_1 - del_1 * (1.0/3.0 + 134.0 * del_1 * del_1 / 81.0));
    double del_0 = d1 / (adel * adel);
    p.no_unkozai = p.no_kozai / (1.0 + del_0);
    
    double ao = pow(XKE / p.no_unkozai, X2O3);
    p.a = ao;
    
    // ═══════════════════════════════════════════════════════════════════
    // PERIGEE CALCULATIONS  
    // ═══════════════════════════════════════════════════════════════════
    
    double po = ao * omeosq;
    double con42 = 1.0 - 5.0 * cosio2;
    double rp = ao * (1.0 - p.ecco);
    p.altp = (rp - 1.0) * RE;  // Perigee altitude in km
    p.alta = (ao * (1.0 + p.ecco) - 1.0) * RE;  // Apogee altitude in km
    
    // ═══════════════════════════════════════════════════════════════════
    // DEEP SPACE CHECK
    // ═══════════════════════════════════════════════════════════════════

    double period = TWOPI / p.no_unkozai;  // Period in minutes
    p.is_deep_space = (period >= DEEP_SPACE_PERIOD_MIN) ? 1 : 0;
    p.force_near_earth = 0;  // Default: no override
    
    // ═══════════════════════════════════════════════════════════════════
    // ATMOSPHERIC DRAG TERMS (S and QOMS2T)
    // ═══════════════════════════════════════════════════════════════════
    
    // The "s" parameter depends on perigee height
    // For perigee < 156 km: s = perigee - 78
    // For perigee >= 156 km but < 220 km: s = perigee - 78
    // For perigee >= 220 km: s = 78/RE (fixed)
    
    double perige = (rp - 1.0) * RE;  // Perigee altitude in km
    double s4, qzms24;
    
    // s4 is in Earth radii
    if (perige < 156.0) {
        s4 = perige - 78.0;
        if (perige < 98.0) {
            s4 = 20.0;
        }
        qzms24 = pow((120.0 - s4) / RE, 4.0);
        s4 = s4 / RE + 1.0;
    } else {
        s4 = 78.0 / RE + 1.0;
        qzms24 = pow((120.0 - 78.0) / RE, 4.0);
    }
    
    double pinvsq = 1.0 / (po * po);
    double tsi = 1.0 / (ao - s4);
    p.eta = ao * p.ecco * tsi;
    double etasq = p.eta * p.eta;
    double eeta = p.ecco * p.eta;
    double psisq = fabs(1.0 - etasq);
    double coef = qzms24 * pow(tsi, 4.0);
    double coef1 = coef / pow(psisq, 3.5);
    
    // ═══════════════════════════════════════════════════════════════════
    // C COEFFICIENTS
    // ═══════════════════════════════════════════════════════════════════
    
    double cc2 = coef1 * p.no_unkozai * (ao * (1.0 + 1.5 * etasq + eeta * (4.0 + etasq)) +
                 0.375 * J2 * tsi / psisq * p.con41 * (8.0 + 3.0 * etasq * (8.0 + etasq)));
    p.cc1 = p.bstar * cc2;
    
    double cc3 = 0.0;
    if (p.ecco > 1.0e-4) {
        cc3 = -2.0 * coef * tsi * J3 / (J2 * p.no_unkozai * sinio);
    }
    
    p.x1mth2 = 1.0 - cosio2;
    p.cc4 = 2.0 * p.no_unkozai * coef1 * ao * omeosq *
            (p.eta * (2.0 + 0.5 * etasq) + p.ecco * (0.5 + 2.0 * etasq) -
            J2 * tsi / (ao * psisq) *
            (-3.0 * p.con41 * (1.0 - 2.0 * eeta + etasq * (1.5 - 0.5 * eeta)) +
            0.75 * p.x1mth2 * (2.0 * etasq - eeta * (1.0 + etasq)) * cos(2.0 * p.argpo)));
    p.cc5 = 2.0 * coef1 * ao * omeosq * (1.0 + 2.75 * (etasq + eeta) + eeta * etasq);
    
    // ═══════════════════════════════════════════════════════════════════
    // SECULAR RATE TERMS (MDOT, ARGPDOT, NODEDOT)
    // ═══════════════════════════════════════════════════════════════════
    
    double temp1 = 1.5 * J2 * pinvsq * p.no_unkozai;
    double temp2 = 0.5 * temp1 * J2 * pinvsq;
    double temp3 = -0.46875 * J4 * pinvsq * pinvsq * p.no_unkozai;
    
    p.mdot = p.no_unkozai + 0.5 * temp1 * rteosq * p.con41 +
             0.0625 * temp2 * rteosq * (13.0 - 78.0 * cosio2 + 137.0 * cosio4);
    
    p.argpdot = -0.5 * temp1 * con42 +
                0.0625 * temp2 * (7.0 - 114.0 * cosio2 + 395.0 * cosio4) +
                temp3 * (3.0 - 36.0 * cosio2 + 49.0 * cosio4);
    
    double xhdot1 = -temp1 * cosio;
    p.nodedot = xhdot1 + (0.5 * temp2 * (4.0 - 19.0 * cosio2) +
                2.0 * temp3 * (3.0 - 7.0 * cosio2)) * cosio;
    p.xnodcf = 3.5 * omeosq * xhdot1 * p.cc1;
    p.t2cof = 1.5 * p.cc1;
    
    // ═══════════════════════════════════════════════════════════════════
    // LONG PERIOD TERMS
    // ═══════════════════════════════════════════════════════════════════
    
    // Note: Must use J3OJ2 = J3/J2, not just J3!
    if (fabs(cosio + 1.0) > 1.5e-12) {
        p.xlcof = -0.25 * J3OJ2 * sinio * (3.0 + 5.0 * cosio) / (1.0 + cosio);
    } else {
        // Retrograde orbit at 180° - use a safe denominator
        p.xlcof = -0.25 * J3OJ2 * sinio * (3.0 + 5.0 * cosio) / 1.5e-12;
    }
    p.aycof = -0.5 * J3OJ2 * sinio;
    
    // ═══════════════════════════════════════════════════════════════════
    // NEAR-EARTH SPECIFIC TERMS
    // ═══════════════════════════════════════════════════════════════════
    
    // Store parameters needed for deep space init
    double posq = po * po;
    double xpidot = p.argpdot + p.nodedot;
    
    if (!p.is_deep_space) {
        double c1sq = p.cc1 * p.cc1;
        p.d2 = 4.0 * ao * tsi * c1sq;
        double temp = p.d2 * tsi * p.cc1 / 3.0;
        p.d3 = (17.0 * ao + s4) * temp;
        p.d4 = 0.5 * temp * ao * tsi * (221.0 * ao + 31.0 * s4) * p.cc1;
        p.t3cof = p.d2 + 2.0 * c1sq;
        p.t4cof = 0.25 * (3.0 * p.d3 + p.cc1 * (12.0 * p.d2 + 10.0 * c1sq));
        p.t5cof = 0.2 * (3.0 * p.d4 + 12.0 * p.cc1 * p.d3 +
                  6.0 * p.d2 * p.d2 + 15.0 * c1sq * (2.0 * p.d2 + c1sq));
        
        p.sinmao = sin(p.mo);
        p.delmo = pow(1.0 + p.eta * cos(p.mo), 3.0);
        p.omgcof = p.bstar * cc3 * cos(p.argpo);
        p.xmcof = 0.0;
        if (p.ecco > 1.0e-4) {
            p.xmcof = -X2O3 * coef * p.bstar / eeta;
        }

        // Debug: print drag coefficients for ISS
        #ifdef DEBUG_INIT
        if (p.bstar > 2.0e-4) {  // ISS has bstar = 2.2013e-04
            printf("\n=== INIT DEBUG: High-drag satellite ===\n");
            printf("bstar:   %.16e\n", p.bstar);
            printf("cc1:     %.16e\n", p.cc1);
            printf("cc4:     %.16e\n", p.cc4);
            printf("cc5:     %.16e\n", p.cc5);
            printf("d2:      %.16e\n", p.d2);
            printf("d3:      %.16e\n", p.d3);
            printf("d4:      %.16e\n", p.d4);
            printf("t2cof:   %.16e\n", p.t2cof);
            printf("t3cof:   %.16e\n", p.t3cof);
            printf("t4cof:   %.16e\n", p.t4cof);
            printf("t5cof:   %.16e\n", p.t5cof);
            printf("sinmao:  %.16f\n", p.sinmao);
            printf("delmo:   %.16f\n", p.delmo);
            printf("omgcof:  %.16e\n", p.omgcof);
            printf("xmcof:   %.16e\n", p.xmcof);
        }
        #endif
        
        // Initialize deep space parameters to zero for near-earth
        p.irez = 0;
        p.gsto = 0.0;
        p.atime = 0.0;
    } else {
        // ═══════════════════════════════════════════════════════════════════
        // DEEP SPACE INITIALIZATION
        // ═══════════════════════════════════════════════════════════════════
        
        // Zero out near-earth terms
        p.d2 = p.d3 = p.d4 = 0.0;
        p.t3cof = p.t4cof = p.t5cof = 0.0;
        p.sinmao = 0.0;
        p.delmo = 0.0;
        p.omgcof = p.xmcof = 0.0;
        
        // Call deep space initialization
        sgp4_init_deepspace(tle, p, ao, con42, cosio, sinio, cosio2,
                            eccsq, omeosq, posq, rp, rteosq, s4, xpidot);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DEEP SPACE INITIALIZATION
// Calls DSCOM and DSINIT to compute lunar/solar and resonance terms
// ═══════════════════════════════════════════════════════════════════════════
__device__ void sgp4_init_deepspace(const TleData& tle, Sgp4Params& p,
    double ao, double con42, double cosio, double sinio, double cosio2,
    double eccsq, double omeosq, double posq, double rp, double rteosq,
    double s4, double xpidot)
{
    // Compute Greenwich Sidereal Time at epoch (AFSPC method)
    // Julian Date 2433281.5 = Jan 0, 1950 0h UT
    double epoch = p.epoch_jd - 2433281.5;  // Days since Jan 0, 1950
    
    // GST at epoch (radians) - following python-sgp4 _initl exactly
    // ts70 = total days since Jan 0, 1970 (including fraction)
    double ts70 = epoch - 7305.0;
    // ds70 = integer days since Jan 0, 1970
    double ds70 = floor(ts70 + 1.0e-8);
    // tfrac = fractional day
    double tfrac = ts70 - ds70;
    
    double c1 = 1.72027916940703639e-2;
    double thgr70 = 1.7321343856509374;
    double fk5r = 5.07551419432269442e-15;
    double c1p2p = c1 + TWOPI;
    p.gsto = fmod(thgr70 + c1 * ds70 + c1p2p * tfrac + ts70 * ts70 * fk5r, TWOPI);
    if (p.gsto < 0.0) {
        p.gsto = p.gsto + TWOPI;
    }
    
    // Initialize tc (time since epoch in minutes) - set to 0 for init
    double tc = 0.0;
    
    // DSCOM variables
    double snodm, cnodm, sinim, cosim, sinomm, cosomm;
    double day, emsq, gam, rtemsq_ds;
    double s1, s2, s3, s4_ds, s5, s6, s7;
    double ss1, ss2, ss3, ss4, ss5, ss6, ss7;
    double sz1, sz2, sz3, sz11, sz12, sz13, sz21, sz22, sz23, sz31, sz32, sz33;
    double z1, z2, z3, z11, z12, z13, z21, z22, z23, z31, z32, z33;
    double nm, em, inclm, mm, argpm, nodem;
    
    // Call DSCOM to compute lunar/solar common terms
    dscom(
        epoch, p.ecco, p.argpo, tc, p.inclo, p.nodeo, p.no_unkozai,
        snodm, cnodm, sinim, cosim, sinomm, cosomm, day, emsq,
        gam, rtemsq_ds, s1, s2, s3, s4_ds, s5, s6, s7, ss1,
        ss2, ss3, ss4, ss5, ss6, ss7, sz1, sz2, sz3, sz11,
        sz12, sz13, sz21, sz22, sz23, sz31, sz32, sz33, z1, z2,
        z3, z11, z12, z13, z21, z22, z23, z31, z32, z33,
        nm, em, inclm, mm, argpm, nodem, p
    );
    
    // Call DSINIT to compute resonance and secular rates
    dsinit(
        cosim, emsq, p.argpo, s1, s2, s3, s4_ds, s5, sinim, ss1,
        ss2, ss3, ss4, ss5, sz1, sz3, sz11, sz13, sz21, sz23,
        sz31, sz33, 0.0, tc, p.gsto, p.mo, p.mdot, p.no_unkozai,
        p.nodeo, p.nodedot, xpidot, z1, z3, z11, z13, z21, z23,
        z31, z33, p.ecco, eccsq, em, argpm, inclm, mm,
        nm, nodem, p
    );
    
    // Note: python-sgp4 does NOT call dpper during initialization.
    // The baseline periodics (peo, pinco, plo, pgho, pho) remain at 0
    // as set by dscom. This means dpper will not subtract any baseline
    // during propagation, which matches the Vallado algorithm.
    
    // Initialize resonance tracking
    p.atime = 0.0;
}

// Kernel to initialize batch of satellites
extern "C" __global__ void sgp4_init_kernel(
    const TleData* __restrict__ tle_data,  // [n_sats]
    Sgp4Params* __restrict__ params,        // [n_sats] output
    int n_sats
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    
    if (idx >= n_sats) return;
    
    const TleData& tle = tle_data[idx];
    Sgp4Params& p = params[idx];
    
    sgp4_init_single(tle, p);
}
