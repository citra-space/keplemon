// SGP4 Near-Earth Propagation (CUDA)
// Handles secular effects for satellites with orbital periods < 225 minutes
// Based on Vallado's sgp4() - near-earth branch

#ifndef SGP4_PROPAGATE_CUH
#define SGP4_PROPAGATE_CUH

#include "tle_propagator_types.cuh"
#include "tle_propagator_constants.cuh"

// Apply near-earth secular effects
// Updates mean motion components based on drag and higher-order terms
__device__ void sgp4_secular_effects(
    const Sgp4Params& p,
    double tsince,       // minutes since epoch
    double xmdf,         // mean anomaly + mdot*t
    double t2,           // tsince^2
    // Outputs (modified in place)
    double& mm,          // mean anomaly (modified)
    double& argpm,       // argument of perigee (modified)
    double& tempa,       // semi-major axis factor
    double& tempe,       // eccentricity factor
    double& templ        // mean longitude factor
) {
    double delomg = p.omgcof * tsince;
    double delm = p.xmcof * (pow(1.0 + p.eta * cos(xmdf), 3.0) - p.delmo);
    double temp = delomg + delm;
    mm = xmdf + temp;
    argpm = argpm - temp;  // argpm was initialized to argpdf

    double t3 = t2 * tsince;
    double t4 = t3 * tsince;

    tempa = tempa - p.d2 * t2 - p.d3 * t3 - p.d4 * t4;
    tempe = tempe + p.bstar * p.cc5 * (sin(mm) - p.sinmao);
    templ = templ + p.t3cof * t3 + t4 * (p.t4cof + tsince * p.t5cof);
}

// Get effective coefficients for near-earth propagation
// For SGP4, these are the pre-computed values from initialization
__device__ void sgp4_get_coefficients(
    const Sgp4Params& p,
    double& aycof_eff,
    double& xlcof_eff,
    double& con41_eff,
    double& x1mth2_eff,
    double& x7thm1_eff
) {
    aycof_eff = p.aycof;
    xlcof_eff = p.xlcof;
    con41_eff = p.con41;
    x1mth2_eff = p.x1mth2;
    x7thm1_eff = p.x7thm1;
}

#endif // SGP4_PROPAGATE_CUH
