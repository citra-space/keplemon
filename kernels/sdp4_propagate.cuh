// SDP4 Deep-Space Propagation (CUDA)
// Handles secular effects for satellites with orbital periods >= 225 minutes
// Based on Vallado's sgp4() - deep space branch with DSPACE/DPPER

#ifndef SDP4_PROPAGATE_CUH
#define SDP4_PROPAGATE_CUH

#include "tle_propagator_types.cuh"
#include "tle_propagator_constants.cuh"
#include "sdp4_deepspace.cuh"

// Apply deep-space secular effects
// Updates orbital elements based on lunar-solar perturbations and resonance
__device__ void sdp4_secular_effects(
    Sgp4Params& p,       // non-const: dspace updates atime/xli/xni
    double tsince,       // minutes since epoch
    double xmdf,         // mean anomaly + mdot*t
    double argpdf,       // argument of perigee + argpdot*t
    double nodedf,       // RAAN + nodedot*t
    // Outputs (modified in place)
    double& mm,          // mean anomaly
    double& argpm,       // argument of perigee
    double& nodem,       // RAAN
    double& nm,          // mean motion
    double& em,          // eccentricity
    double& inclm        // inclination
) {
    // Initialize working variables for deep space
    mm = xmdf;
    argpm = argpdf;
    nodem = nodedf;
    double tc = tsince;

    // Apply deep space secular contributions (DSPACE)
    // This handles resonance effects and updates mean motion
    dspace(tsince, tc, em, argpm, inclm, mm, nm, nodem, p);
}

// Apply deep-space periodic perturbations (lunar-solar)
// Must be called after secular effects and before short-period terms
__device__ void sdp4_periodic_effects(
    Sgp4Params& p,
    double tsince,
    double& em,
    double& inclm,
    double& nodem,
    double& argpm,
    double& mm,
    bool debug = false
) {
    // Apply lunar-solar periodics (DPPER)
    dpper(p.inclo, false, tsince, em, inclm, nodem, argpm, mm, p, debug);
}

// Get effective coefficients for deep-space propagation
// For SDP4, these must be recalculated using the dpper-modified inclination
__device__ void sdp4_get_coefficients(
    double sinip,        // sin(inclm) after dpper
    double cosip,        // cos(inclm) after dpper
    double& aycof_eff,
    double& xlcof_eff,
    double& con41_eff,
    double& x1mth2_eff,
    double& x7thm1_eff
) {
    aycof_eff = -0.5 * J3OJ2 * sinip;
    if (fabs(cosip + 1.0) > 1.5e-12) {
        xlcof_eff = -0.25 * J3OJ2 * sinip * (3.0 + 5.0 * cosip) / (1.0 + cosip);
    } else {
        xlcof_eff = -0.25 * J3OJ2 * sinip * (3.0 + 5.0 * cosip) / 1.5e-12;
    }

    double cosisq = cosip * cosip;
    con41_eff = 3.0 * cosisq - 1.0;
    x1mth2_eff = 1.0 - cosisq;
    x7thm1_eff = 7.0 * cosisq - 1.0;
}

#endif // SDP4_PROPAGATE_CUH
