// SGP4 Near-Earth Initialization (CUDA)
// Handles satellites with orbital periods < 225 minutes
// Based on Vallado's sgp4init() - near-earth branch

#ifndef SGP4_INIT_CUH
#define SGP4_INIT_CUH

#include "tle_propagator_types.cuh"
#include "tle_propagator_constants.cuh"

// Initialize near-earth specific parameters
// Called after common initialization when is_deep_space == 0
__device__ void sgp4_init_near_earth(
    Sgp4Params& p,
    double ao,           // un-Kozai'd semi-major axis
    double tsi,          // 1 / (ao - s4)
    double s4,           // perigee-dependent parameter
    double cc3,          // C3 coefficient
    double coef,         // qzms24 * tsi^4
    double eeta          // ecco * eta
) {
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

    // Zero out deep space parameters
    p.irez = 0;
    p.gsto = 0.0;
    p.atime = 0.0;

    // Zero out deep space secular rates
    p.dedt = 0.0;
    p.didt = 0.0;
    p.dmdt = 0.0;
    p.dnodt = 0.0;
    p.domdt = 0.0;
}

#endif // SGP4_INIT_CUH
