// ═══════════════════════════════════════════════════════════════════════════════════
// GEO Analytical Propagator Constants Header
// ═══════════════════════════════════════════════════════════════════════════════════
//
// This file contains physical constants used by the CUDA GEO analytical propagator.
// Uses EGM-96 geopotential model for improved accuracy at GEO altitudes.
//
// References:
//   - Lemoine et al., "The Development of the Joint NASA GSFC and NIMA Geopotential
//     Model EGM96" NASA/TP-1998-206861
//   - Montenbruck & Gill, "Satellite Orbits" (2000)
//
// ═══════════════════════════════════════════════════════════════════════════════════

#ifndef GEO_CONSTANTS_CUH
#define GEO_CONSTANTS_CUH

// Include shared math constants from TLE propagator
#include "tle_propagator_constants.cuh"

// ═══════════════════════════════════════════════════════════════════════════════════
// EGM-96 GEOPOTENTIAL CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════
// These constants are from the Earth Gravitational Model 1996 (EGM-96)
// Used by the GEO analytical propagator for improved accuracy

// Earth equatorial radius for EGM-96 (km)
#define RE_EGM96 6378.1363

// Gravitational parameter (km³/s²) - EGM-96
#define MU_EGM96 398600.4415

// Zonal harmonic coefficients (unnormalized) - EGM-96
#define J2_EGM96  1.0826267e-3
#define J3_EGM96 -2.5327e-6
#define J4_EGM96 -1.6196e-6
#define J5_EGM96 -2.2730e-7         // Fifth zonal harmonic
#define J6_EGM96  5.4068e-7         // Sixth zonal harmonic

// ═══════════════════════════════════════════════════════════════════════════════════
// SOLAR RADIATION PRESSURE CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

// Solar flux constant at 1 AU (W/m²)
#define SOLAR_FLUX 1361.0

// Speed of light (km/s)
#define SPEED_OF_LIGHT 299792.458

// Solar radiation pressure at 1 AU (N/m² = kg/(m·s²))
// P_sun = SOLAR_FLUX / SPEED_OF_LIGHT = 4.54e-6 N/m²
// In km units: 4.54e-6 * 1e-6 = 4.54e-12 km·kg/(m²·s²)
// Simplified: We compute a_srp = Cr * (A/m) * P_sun * (AU/r)²
// where P_sun in acceleration units: ~4.54e-6 m/s² = 4.54e-9 km/s² per m²/kg
#define P_SUN_KM 4.54e-9            // Solar pressure (km/s² per m²/kg)

// Astronomical Unit (km)
#define AU_KM 149597870.7

// ═══════════════════════════════════════════════════════════════════════════════════
// GEO PROPAGATOR DEFAULT PARAMETERS
// ═══════════════════════════════════════════════════════════════════════════════════

// Default reflectivity coefficient for SRP (1.0 = perfect absorber, 2.0 = perfect reflector)
// Common orbit determination default is ~1.3 (partial reflector with some absorption)
#define DEFAULT_CR 1.3

// Default area-to-mass ratio for typical GEO satellites (m²/kg)
// Range is typically 0.01-0.05 m²/kg for operational GEO satellites
#define DEFAULT_AREA_MASS 0.02

// GEO altitude threshold (km) - satellites above this are considered GEO-class
#define GEO_ALTITUDE_THRESHOLD 30000.0

// ═══════════════════════════════════════════════════════════════════════════════════
// TIME CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

// Seconds per minute
#define SEC_PER_MIN 60.0

// Julian date conversion constants
#define JD_J2000 2451545.0          // J2000 epoch (Jan 1, 2000 12:00 TT)
#define DAYS_PER_CENTURY 36525.0

// ═══════════════════════════════════════════════════════════════════════════════════
// LUNAR-SOLAR GRAVITATIONAL CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

// Solar gravitational parameter (km³/s²)
#define MU_SUN 1.32712440018e11

// Lunar gravitational parameter (km³/s²)
#define MU_MOON 4.9028e3

// Mean distance from Earth to Sun (km)
#define MEAN_EARTH_SUN_DIST 149597870.7

// Mean distance from Earth to Moon (km)
#define MEAN_EARTH_MOON_DIST 384400.0

// Lunar mean motion (rad/day) ≈ 2π / 27.321661 days
#define LUNAR_MEAN_MOTION 0.22997150

// Solar mean motion (rad/day) ≈ 2π / 365.25 days
#define SOLAR_MEAN_MOTION 0.01720209895

// Obliquity of the ecliptic (radians) ≈ 23.4393°
#define OBLIQUITY_ECLIPTIC 0.4090926

// ═══════════════════════════════════════════════════════════════════════════════════
// EARTH SHADOW MODEL CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

// Earth equatorial radius for shadow calculations (km) - EGM96
#define EARTH_RADIUS_SHADOW 6378.1363

// Sun radius (km)
#define SUN_RADIUS 696340.0

// Shadow transition width (km) - smooth transition zone for penumbra approximation
// This avoids GPU warp divergence and is physically reasonable for GEO altitude
#define SHADOW_TRANSITION_WIDTH 200.0

// ═══════════════════════════════════════════════════════════════════════════════════
// TESSERAL HARMONIC CONSTANTS (J22 for GEO longitude drift)
// ═══════════════════════════════════════════════════════════════════════════════════
// J22 causes longitude-dependent acceleration important for GEO stationkeeping

// J22 coefficient (unnormalized) - EGM96
#define J22_EGM96 1.8155e-6

// Longitude of the J22 stable point (radians) - approximately 75°E and 255°E (105°W)
#define J22_LONGITUDE_RAD 1.3090  // ~75 degrees in radians

// Earth rotation rate (rad/s)
#define EARTH_ROTATION_RATE 7.2921150e-5

// ═══════════════════════════════════════════════════════════════════════════════════
// IMPROVED SOLAR EPHEMERIS CONSTANTS (VSOP87-derived)
// ═══════════════════════════════════════════════════════════════════════════════════
// Higher-order terms for more accurate Sun position

// Mean longitude coefficients (degrees)
#define SUN_L0_0 280.4664567
#define SUN_L0_1 360007.6982779
#define SUN_L0_2 0.03032028
#define SUN_L0_3 (-1.0/49931.0)

// Mean anomaly coefficients (degrees)
#define SUN_M0 357.5291092
#define SUN_M1 35999.0502909
#define SUN_M2 (-0.0001536)

// Equation of center coefficients (degrees)
#define SUN_C1 1.9146
#define SUN_C2 0.0200
#define SUN_C3 0.0003

// Eccentricity of Earth's orbit
#define EARTH_ECC_0 0.016708634
#define EARTH_ECC_1 (-0.000042037)

// ═══════════════════════════════════════════════════════════════════════════════════
// IMPROVED LUNAR EPHEMERIS CONSTANTS (Brown theory extended)
// ═══════════════════════════════════════════════════════════════════════════════════
// Additional terms for more accurate Moon position

// Mean longitude (degrees)
#define MOON_L0 218.3164477
#define MOON_L1 481267.88123421

// Mean elongation (degrees)
#define MOON_D0 297.8501921
#define MOON_D1 445267.1114034

// Mean anomaly (degrees)
#define MOON_M0 134.9633964
#define MOON_M1 477198.8675055

// Argument of latitude (degrees)
#define MOON_F0 93.2720950
#define MOON_F1 483202.0175233

// Mean longitude of ascending node (degrees)
#define MOON_OMEGA0 125.0445479
#define MOON_OMEGA1 (-1934.1362891)

// Longitude perturbation amplitudes (degrees)
#define MOON_PERT_L1 6.28875      // sin(M)
#define MOON_PERT_L2 1.27402      // sin(2D - M)
#define MOON_PERT_L3 0.65830      // sin(2D)
#define MOON_PERT_L4 0.21364      // sin(2M)
#define MOON_PERT_L5 (-0.18500)   // sin(M_sun)
#define MOON_PERT_L6 (-0.11434)   // sin(2F)
#define MOON_PERT_L7 0.05883      // sin(2D - 2M)
#define MOON_PERT_L8 0.05726      // sin(2D - M_sun - M)

// Latitude perturbation amplitudes (degrees)
#define MOON_PERT_B1 5.12819      // sin(F)
#define MOON_PERT_B2 0.28060      // sin(M + F)
#define MOON_PERT_B3 0.27769      // sin(M - F)
#define MOON_PERT_B4 0.17320      // sin(2D - F)

// Distance perturbation amplitudes (km from mean)
#define MOON_PERT_R1 (-20905.0)   // cos(M)
#define MOON_PERT_R2 (-3699.0)    // cos(2D - M)
#define MOON_PERT_R3 (-2956.0)    // cos(2D)
#define MOON_PERT_R4 (-570.0)     // cos(2M)

// ═══════════════════════════════════════════════════════════════════════════════════
// RK4 INTEGRATION CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════

// Maximum integration substeps for RK4 (supports ~347 days at 120s steps)
#define RK4_MAX_SUBSTEPS 50000

// Nominal substep size (seconds) - 2 minutes (~0.5° of GEO orbit per step)
#define RK4_NOMINAL_STEP 120.0

// Encke rectification threshold (km) - rectify when deviation exceeds this
#define ENCKE_RECTIFICATION_THRESHOLD 100.0

#endif // GEO_CONSTANTS_CUH
