// SDP4 Deep-Space Initialization (CUDA)
// Handles satellites with orbital periods >= 225 minutes
// Based on Vallado's sgp4init() - deep space branch with DSCOM/DSINIT

#ifndef SDP4_INIT_CUH
#define SDP4_INIT_CUH

#include "tle_propagator_types.cuh"
#include "tle_propagator_constants.cuh"
#include "sdp4_deepspace.cuh"

// Initialize deep-space specific parameters
// Called after common initialization when is_deep_space == 1
// Calls DSCOM and DSINIT to compute lunar/solar and resonance terms
__device__ void sdp4_init_deepspace(
    const TleData& tle,
    Sgp4Params& p,
    double ao,           // un-Kozai'd semi-major axis
    double con42,        // 1 - 5*cos(inclo)^2
    double cosio,        // cos(inclo)
    double sinio,        // sin(inclo)
    double cosio2,       // cos(inclo)^2
    double eccsq,        // ecco^2
    double omeosq,       // 1 - ecco^2
    double posq,         // (ao * omeosq)^2
    double rp,           // perigee radius (Earth radii)
    double rteosq,       // sqrt(omeosq)
    double s4,           // perigee-dependent parameter
    double xpidot        // argpdot + nodedot
) {
    // Zero out near-earth terms
    p.d2 = p.d3 = p.d4 = 0.0;
    p.t3cof = p.t4cof = p.t5cof = 0.0;
    p.sinmao = 0.0;
    p.delmo = 0.0;
    p.omgcof = p.xmcof = 0.0;

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

#endif // SDP4_INIT_CUH
