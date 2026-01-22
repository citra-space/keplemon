// ═══════════════════════════════════════════════════════════════════════════════════
// SGP4 Constants Header - WGS-72 OLD Gravity Model
// ═══════════════════════════════════════════════════════════════════════════════════
//
// This file contains ALL physical constants used by the CUDA SGP4 propagator.
// Constants are from the WGS-72 OLD (original World Geodetic System 1972) gravity model,
// which matches the python-sgp4 library's default configuration.
//
// IMPORTANT: SGP4 uses WGS-72 OLD constants (from the original 1972 specification).
// There are TWO versions of WGS-72:
//   - WGS-72 OLD (1972): Original specification with J2=0.001082616
//   - WGS-72 (1976):     Revised specification with J2=0.00108262998905
//
// python-sgp4 uses WGS-72 OLD to match historical TLE catalogs and the original
// Spacetrack Report #3 implementation.
//
// Constant differences (WGS-72 OLD vs newer standards):
//   Constant     WGS-72 OLD (1972)   WGS-72 (1976)       WGS-84
//   ───────────────────────────────────────────────────────────────────
//   RE (km)      6378.135            6378.135            6378.137
//   J2           0.001082616         0.00108262998905    0.00108263
//   J3          -0.00000253881      -0.00000253215306   -0.00000253215
//   mu (km³/s²)  398600.8            398600.8            398600.5
//
// References:
//   - Hoots & Roehrich, "Spacetrack Report No. 3" (1980)
//   - Vallado et al., "Revisiting Spacetrack Report #3" AIAA 2006-6753
//   - python-sgp4: https://github.com/brandon-rhodes/python-sgp4
//
// ═══════════════════════════════════════════════════════════════════════════════════

#ifndef SGP4_CONSTANTS_CUH
#define SGP4_CONSTANTS_CUH

// ═══════════════════════════════════════════════════════════════════════════════════
// MATHEMATICAL CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

#define PI 3.14159265358979323846
#define TWOPI (2.0 * PI)
#define DEG2RAD (PI / 180.0)
#define RAD2DEG (180.0 / PI)
#define X2O3 (2.0 / 3.0)

// ═══════════════════════════════════════════════════════════════════════════════════
// WGS-72 OLD EARTH GRAVITY MODEL CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════
// These values MUST match python-sgp4's 'wgs72' constants exactly.
// Source: sgp4.earth_gravity.wgs72 (uses WGS-72 OLD from 1972 spec)

// Earth equatorial radius (km)
#define RE 6378.135

// Gravitational parameter mu = GM (km³/s²)
// Note: Not directly used in SGP4 but documented for reference
#define MU 398600.8

// XKE = sqrt(mu) in canonical units: (Earth radii)^(3/2) / minute
// Computed as: 60.0 / sqrt(RE^3 / MU)
// This is THE fundamental constant for mean motion calculations
#define XKE 0.0743669161331734132

// TUMIN = 1.0 / XKE (minutes per time unit)
#define TUMIN 13.44683969695931

// Zonal harmonic coefficients (unnormalized) - WGS-72 OLD (1972) values from python-sgp4
// These MUST match python-sgp4 exactly for agreement with reference implementations
#define J2  0.001082616               // Second zonal harmonic (WGS-72 OLD)
#define J3 -0.00000253881             // Third zonal harmonic (WGS-72 OLD)
#define J4 -0.00000165597             // Fourth zonal harmonic

// Derived ratio for long-period periodics
// J3OJ2 = J3 / J2 ≈ -0.00233889
#define J3OJ2 (J3 / J2)

// ═══════════════════════════════════════════════════════════════════════════════════
// DERIVED CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

// Velocity conversion: internal units to km/s
// VKMPERSEC = XKE * RE / 60
#define VKMPERSEC 7.905365719014155

// Time conversion
#define MINUTES_PER_DAY 1440.0

// ═══════════════════════════════════════════════════════════════════════════════════
// SGP4 ALGORITHM THRESHOLDS
// ═══════════════════════════════════════════════════════════════════════════════════

// Period threshold for deep space vs near-earth propagation
// Satellites with period > 225 minutes use deep space (SDP4)
#define DEEP_SPACE_PERIOD_MIN 225.0

// Small number threshold for divide-by-zero protection
#define SMALL 1.5e-12

// ═══════════════════════════════════════════════════════════════════════════════════
// LUNAR-SOLAR PERTURBATION CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════
// Constants for third-body (Moon and Sun) perturbations in deep space propagation

// Solar constants
#define ZNS 1.19459e-5                // Solar mean motion (rad/min)
#define C1SS 2.9864797e-6             // Solar secular coefficient
#define ZES 0.01675                   // Solar orbital eccentricity

// Lunar constants
#define ZNL 1.5835218e-4              // Lunar mean motion (rad/min)
#define C1L 4.7968065e-7              // Lunar secular coefficient
#define ZEL 0.05490                   // Lunar orbital eccentricity

// Orbital orientation constants (radians)
#define ZSINIS 0.39785416             // sin(obliquity of ecliptic)
#define ZCOSIS 0.91744867             // cos(obliquity of ecliptic)
#define ZCOSGS 0.1945905              // cos(argument of perigee of Sun)
#define ZSINGS -0.98088458            // sin(argument of perigee of Sun)

// ═══════════════════════════════════════════════════════════════════════════════════
// RESONANCE CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════
// Constants for geosynchronous and semi-synchronous resonance terms

// Root terms for resonance calculations
#define ROOT22 1.7891679e-6
#define ROOT32 3.7393792e-7
#define ROOT44 7.3636953e-9
#define ROOT52 1.1428639e-7
#define ROOT54 2.1765803e-9

// G-coefficients for resonance
#define G22 5.7686396
#define G32 0.95240898
#define G44 1.8014998
#define G52 1.0508330
#define G54 4.4108898

// Q-coefficients for resonance
#define Q22 1.7891679e-6
#define Q31 2.1460748e-6
#define Q33 2.2123015e-7

// ═══════════════════════════════════════════════════════════════════════════════════
// RESONANCE INTEGRATION CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

// Earth rotation rate (rad/min) = 2*pi / (sidereal day in minutes)
// Also known as the "theta dot" term
#define RPTIM 4.37526908801129966e-3

// Integration step sizes (minutes)
#define STEP 720.0                    // 12 hours
#define STEPN (-720.0)                // -12 hours (backwards)
#define STEP2 259200.0                // 180 days in minutes

// Resonance thresholds for mean motion
#define RESON_1DAY_LOW  0.0034906585  // ~0.5 rev/day lower bound
#define RESON_1DAY_HIGH 0.0052359877  // ~0.75 rev/day upper bound
#define RESON_HALF_LOW  8.26e-3       // ~1.9 rev/day lower bound
#define RESON_HALF_HIGH 9.24e-3       // ~2.1 rev/day upper bound

// ═══════════════════════════════════════════════════════════════════════════════════
// INCLINATION THRESHOLDS
// ═══════════════════════════════════════════════════════════════════════════════════

// Near-polar inclination limit (~3 degrees)
// Used to avoid singularities in Lyddane modifications
#define INCLM_LIM 5.2359877e-2

#endif // SGP4_CONSTANTS_CUH
