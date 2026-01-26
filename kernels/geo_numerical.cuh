// ═══════════════════════════════════════════════════════════════════════════════════
// GEO Numerical Propagator - Perturbation Functions (v2.0)
// ═══════════════════════════════════════════════════════════════════════════════════
//
// This file implements numerical integration of perturbation forces for GEO satellites.
// Uses RK4 integration rather than analytical closed-form solutions.
// Designed for maximum GPU parallelization:
//   - No iteration loops (unlike SDP4 resonance)
//   - Purely formulaic force evaluation
//   - All threads execute identical instructions
//
// Perturbations included:
//   - J2-J4 geopotential (secular and long-period)
//   - J22 tesseral harmonic (longitude-dependent)
//   - Lunar/solar third-body gravity (VSOP87/Brown theory)
//   - Solar radiation pressure (SRP) with Earth shadow
//
// Integration method:
//   - RK4 integrator with Encke's method for numerical stability
//   - Propagates perturbation deviations from Keplerian reference orbit
//
// ⚠️ WARNING: This propagator is currently BROKEN (32,000+ km errors @ 7 days)
//
// ═══════════════════════════════════════════════════════════════════════════════════

#ifndef GEO_NUMERICAL_CUH
#define GEO_NUMERICAL_CUH

#include "geo_constants.cuh"
#include "geo_types.cuh"
#include "tle_propagator_types.cuh"  // For Sgp4State (shared output type)


// ═══════════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

// Normalize angle to [0, 2π)
__device__ __forceinline__ double normalize_angle(double angle) {
    double result = fmod(angle, TWOPI);
    if (result < 0.0) {
        result += TWOPI;
    }
    return result;
}

// Compute eccentric anomaly from mean anomaly using Newton-Raphson
// For near-circular orbits (GEO), converges quickly
__device__ __forceinline__ double mean_to_eccentric_anomaly(double M, double e) {
    double E = M;  // Initial guess

    // Newton-Raphson iteration (max 10 iterations for safety)
    for (int i = 0; i < 10; i++) {
        double sinE = sin(E);
        double cosE = cos(E);
        double f = E - e * sinE - M;
        double f_prime = 1.0 - e * cosE;
        double delta = f / f_prime;
        E = E - delta;
        if (fabs(delta) < 1e-12) break;
    }
    return E;
}

// Compute true anomaly from eccentric anomaly
__device__ __forceinline__ double eccentric_to_true_anomaly(double E, double e) {
    double cosE = cos(E);
    double sinE = sin(E);
    double sqrt_1_e2 = sqrt(1.0 - e * e);
    double cos_nu = (cosE - e) / (1.0 - e * cosE);
    double sin_nu = sqrt_1_e2 * sinE / (1.0 - e * cosE);
    return atan2(sin_nu, cos_nu);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// J2-J4 GEOPOTENTIAL PERTURBATIONS
// ═══════════════════════════════════════════════════════════════════════════════════
// Implements secular and long-period variations from J2, J3, J4 zonal harmonics.
// Based on Brouwer-Lyddane theory with simplifications for near-circular orbits.

// Compute J2 secular rates for orbital elements
// These rates are constant for a given orbit and computed once at initialization
__device__ __forceinline__ void compute_j2_secular_rates(
    double sma,         // Semi-major axis (km)
    double ecc,         // Eccentricity
    double inc,         // Inclination (rad)
    double n,           // Mean motion (rad/s)
    double* raan_dot,   // Output: RAAN rate (rad/s)
    double* argp_dot,   // Output: argument of perigee rate (rad/s)
    double* M_dot       // Output: mean anomaly rate (rad/s) - includes J2 correction
) {
    double cos_i = cos(inc);
    double cos_i_sq = cos_i * cos_i;

    double p = sma * (1.0 - ecc * ecc);  // Semi-latus rectum
    double p_sq = p * p;

    // J2 coefficient factor
    double j2_coeff = 1.5 * J2_EGM96 * RE_EGM96 * RE_EGM96 / p_sq;

    // Secular rate of RAAN (Ω̇)
    *raan_dot = -j2_coeff * n * cos_i;

    // Secular rate of argument of perigee (ω̇)
    *argp_dot = 0.5 * j2_coeff * n * (5.0 * cos_i_sq - 1.0);

    // J2 correction to mean motion
    double sqrt_1_e2 = sqrt(1.0 - ecc * ecc);
    double n_correction = 0.5 * j2_coeff * sqrt_1_e2 * (3.0 * cos_i_sq - 1.0);
    *M_dot = n * (1.0 + n_correction);
}

// Compute J2-J4 long-period perturbations
__device__ __forceinline__ void compute_j2_j4_long_period(
    double sma,
    double ecc,
    double inc,
    double argp,
    double* delta_a,
    double* delta_ecc,
    double* delta_inc
) {
    double cos_i = cos(inc);
    double sin_i = sin(inc);
    double cos_i_sq = cos_i * cos_i;

    double p = sma * (1.0 - ecc * ecc);
    double eta = sqrt(1.0 - ecc * ecc);

    double re_over_p = RE_EGM96 / p;
    double re_over_p_sq = re_over_p * re_over_p;

    double sin_argp = sin(argp);
    double cos_argp = cos(argp);

    // Long-period eccentricity variation from J3
    double j3_ecc = -0.5 * J3_EGM96 * re_over_p_sq * re_over_p * sin_i *
                    (1.0 - 2.5 * sin_i * sin_i) * sin_argp / eta;

    // Long-period inclination variation from J3
    double j3_inc = 0.25 * J3_EGM96 * re_over_p_sq * re_over_p * ecc * cos_i *
                    (1.0 + 4.0 * cos_i_sq) * cos_argp / eta;

    // J4 contribution
    double j4_factor = J4_EGM96 * re_over_p_sq * re_over_p_sq;
    double sin_2argp = sin(2.0 * argp);

    double j4_ecc = 0.0625 * j4_factor * sin_i * sin_i *
                    (35.0 * cos_i_sq - 30.0 * cos_i_sq + 3.0) * sin_2argp / eta;

    *delta_a = 0.0;
    *delta_ecc = j3_ecc + j4_ecc;
    *delta_inc = j3_inc;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// J22 TESSERAL HARMONIC (Longitude-dependent perturbation)
// ═══════════════════════════════════════════════════════════════════════════════════
// J22 causes longitude-dependent east-west acceleration critical for GEO

__device__ __forceinline__ void compute_j22_acceleration(
    double x, double y, double z,       // Satellite position ECI (km)
    double jd,                          // Julian date (for Earth rotation)
    double* ax, double* ay, double* az  // Output: acceleration (km/s²)
) {
    double r = sqrt(x*x + y*y + z*z);
    double r2 = r * r;
    double r5 = r2 * r2 * r;

    // Greenwich Mean Sidereal Time (approximate)
    double T = (jd - JD_J2000) / DAYS_PER_CENTURY;
    double gmst = normalize_angle((280.46061837 + 360.98564736629 * (jd - JD_J2000) +
                                   0.000387933 * T * T) * DEG2RAD);

    // Satellite longitude in Earth-fixed frame
    double sat_lon_eci = atan2(y, x);
    double sat_lon_ecef = sat_lon_eci - gmst;

    // J22 acceleration in local frame (radial and tangential)
    // Uses the formula: a_tangential = -6 * J22 * (Re/r)^2 * (mu/r^2) * sin(2*(lambda - lambda_22))
    double re2 = RE_EGM96 * RE_EGM96;
    double mu_r2 = MU_EGM96 / r2;

    double delta_lon = 2.0 * (sat_lon_ecef - J22_LONGITUDE_RAD);
    double sin_2delta = sin(delta_lon);
    double cos_2delta = cos(delta_lon);

    // J22 perturbation magnitude
    double j22_factor = 6.0 * J22_EGM96 * re2 / r2 * mu_r2;

    // Tangential acceleration (East-West, in equatorial plane)
    double a_tangential = -j22_factor * sin_2delta;

    // Small radial component
    double a_radial = j22_factor * cos_2delta * 0.5;

    // Convert to ECI frame
    // Unit vectors: radial (outward), tangential (East in equatorial plane)
    double xy = sqrt(x*x + y*y);
    if (xy < 1.0) xy = 1.0;  // Avoid division by zero at poles

    double ur_x = x / r;
    double ur_y = y / r;
    double ur_z = z / r;

    // Tangential unit vector (perpendicular to radial, in equatorial plane, eastward)
    double ut_x = -y / xy;
    double ut_y = x / xy;
    double ut_z = 0.0;

    *ax = a_radial * ur_x + a_tangential * ut_x;
    *ay = a_radial * ur_y + a_tangential * ut_y;
    *az = a_radial * ur_z + a_tangential * ut_z;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// IMPROVED SOLAR EPHEMERIS (VSOP87-derived)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ void compute_sun_position_vsop(
    double jd,
    double* sun_x, double* sun_y, double* sun_z
) {
    // Julian centuries from J2000
    double T = (jd - JD_J2000) / DAYS_PER_CENTURY;
    double T2 = T * T;
    double T3 = T2 * T;

    // Mean longitude (degrees)
    double L0 = SUN_L0_0 + SUN_L0_1 * T + SUN_L0_2 * T2 + SUN_L0_3 * T3;

    // Mean anomaly (degrees)
    double M = SUN_M0 + SUN_M1 * T + SUN_M2 * T2;
    double M_rad = M * DEG2RAD;

    // Equation of center (degrees)
    double C = SUN_C1 * sin(M_rad) + SUN_C2 * sin(2.0 * M_rad) + SUN_C3 * sin(3.0 * M_rad);

    // True longitude (degrees)
    double sun_lon = L0 + C;
    double sun_lon_rad = sun_lon * DEG2RAD;

    // Eccentricity
    double e = EARTH_ECC_0 + EARTH_ECC_1 * T;

    // Distance (AU)
    double r_AU = (1.000001018 * (1.0 - e * e)) / (1.0 + e * cos(M_rad + C * DEG2RAD));
    double r_km = r_AU * AU_KM;

    // Obliquity of ecliptic (with nutation approximation)
    double eps = (23.439291 - 0.0130042 * T) * DEG2RAD;

    // Sun position in ECI
    double cos_lon = cos(sun_lon_rad);
    double sin_lon = sin(sun_lon_rad);
    double cos_eps = cos(eps);
    double sin_eps = sin(eps);

    *sun_x = r_km * cos_lon;
    *sun_y = r_km * sin_lon * cos_eps;
    *sun_z = r_km * sin_lon * sin_eps;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// IMPROVED LUNAR EPHEMERIS (Extended Brown theory)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ void compute_moon_position_brown(
    double jd,
    double* moon_x, double* moon_y, double* moon_z
) {
    // Julian centuries from J2000
    double T = (jd - JD_J2000) / DAYS_PER_CENTURY;

    // Fundamental arguments (degrees)
    double L = MOON_L0 + MOON_L1 * T;           // Mean longitude
    double D = MOON_D0 + MOON_D1 * T;           // Mean elongation
    double M = MOON_M0 + MOON_M1 * T;           // Mean anomaly (Moon)
    double F = MOON_F0 + MOON_F1 * T;           // Argument of latitude
    double Omega = MOON_OMEGA0 + MOON_OMEGA1 * T;  // Longitude of ascending node

    // Sun's mean anomaly (for perturbation terms)
    double M_sun = (SUN_M0 + SUN_M1 * T) * DEG2RAD;

    // Convert to radians
    double L_rad = L * DEG2RAD;
    double D_rad = D * DEG2RAD;
    double M_rad = M * DEG2RAD;
    double F_rad = F * DEG2RAD;
    double Omega_rad = Omega * DEG2RAD;

    // Longitude perturbations (degrees)
    double delta_lon = MOON_PERT_L1 * sin(M_rad)
                     + MOON_PERT_L2 * sin(2.0 * D_rad - M_rad)
                     + MOON_PERT_L3 * sin(2.0 * D_rad)
                     + MOON_PERT_L4 * sin(2.0 * M_rad)
                     + MOON_PERT_L5 * sin(M_sun)
                     + MOON_PERT_L6 * sin(2.0 * F_rad)
                     + MOON_PERT_L7 * sin(2.0 * D_rad - 2.0 * M_rad)
                     + MOON_PERT_L8 * sin(2.0 * D_rad - M_sun - M_rad);

    // Ecliptic longitude (radians)
    double lambda = (L + delta_lon) * DEG2RAD;

    // Latitude perturbations (degrees)
    double delta_lat = MOON_PERT_B1 * sin(F_rad)
                     + MOON_PERT_B2 * sin(M_rad + F_rad)
                     + MOON_PERT_B3 * sin(M_rad - F_rad)
                     + MOON_PERT_B4 * sin(2.0 * D_rad - F_rad);

    // Ecliptic latitude (radians)
    double beta = delta_lat * DEG2RAD;

    // Distance perturbations (km from mean)
    double delta_r = MOON_PERT_R1 * cos(M_rad)
                   + MOON_PERT_R2 * cos(2.0 * D_rad - M_rad)
                   + MOON_PERT_R3 * cos(2.0 * D_rad)
                   + MOON_PERT_R4 * cos(2.0 * M_rad);

    // Distance (km)
    double r_moon = MEAN_EARTH_MOON_DIST + delta_r;

    // Obliquity of ecliptic
    double eps = (23.439291 - 0.0130042 * T) * DEG2RAD;

    // Convert to ECI
    double cos_lambda = cos(lambda);
    double sin_lambda = sin(lambda);
    double cos_beta = cos(beta);
    double sin_beta = sin(beta);
    double cos_eps = cos(eps);
    double sin_eps = sin(eps);

    double x_ecl = r_moon * cos_beta * cos_lambda;
    double y_ecl = r_moon * cos_beta * sin_lambda;
    double z_ecl = r_moon * sin_beta;

    *moon_x = x_ecl;
    *moon_y = y_ecl * cos_eps - z_ecl * sin_eps;
    *moon_z = y_ecl * sin_eps + z_ecl * cos_eps;
}

// Legacy function names for backward compatibility
__device__ __forceinline__ void compute_sun_position(
    double jd, double* sun_x, double* sun_y, double* sun_z
) {
    compute_sun_position_vsop(jd, sun_x, sun_y, sun_z);
}

__device__ __forceinline__ void compute_moon_position(
    double jd, double* moon_x, double* moon_y, double* moon_z
) {
    compute_moon_position_brown(jd, moon_x, moon_y, moon_z);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// EARTH SHADOW MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ double compute_shadow_factor(
    double sat_x, double sat_y, double sat_z,
    double sun_x, double sun_y, double sun_z
) {
    double sun_r = sqrt(sun_x*sun_x + sun_y*sun_y + sun_z*sun_z);
    double sun_ux = sun_x / sun_r;
    double sun_uy = sun_y / sun_r;
    double sun_uz = sun_z / sun_r;

    double proj = sat_x*sun_ux + sat_y*sun_uy + sat_z*sun_uz;

    if (proj > 0.0) return 1.0;

    double perp_x = sat_x - proj * sun_ux;
    double perp_y = sat_y - proj * sun_uy;
    double perp_z = sat_z - proj * sun_uz;
    double perp_dist = sqrt(perp_x*perp_x + perp_y*perp_y + perp_z*perp_z);

    double half_width = SHADOW_TRANSITION_WIDTH * 0.5;
    double t = (perp_dist - (EARTH_RADIUS_SHADOW - half_width)) / SHADOW_TRANSITION_WIDTH;
    t = fmax(0.0, fmin(1.0, t));
    return t * t * (3.0 - 2.0 * t);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SOLAR RADIATION PRESSURE
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ void compute_srp_acceleration(
    double sat_x, double sat_y, double sat_z,
    double sun_x, double sun_y, double sun_z,
    double cr, double area_mass,
    double shadow_factor,
    double* ax, double* ay, double* az
) {
    double dx = sun_x - sat_x;
    double dy = sun_y - sat_y;
    double dz = sun_z - sat_z;

    double r_sat_sun = sqrt(dx * dx + dy * dy + dz * dz);

    double ux = dx / r_sat_sun;
    double uy = dy / r_sat_sun;
    double uz = dz / r_sat_sun;

    double au_over_r = AU_KM / r_sat_sun;
    double a_mag = shadow_factor * cr * area_mass * P_SUN_KM * au_over_r * au_over_r;

    *ax = -a_mag * ux;
    *ay = -a_mag * uy;
    *az = -a_mag * uz;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// THIRD-BODY GRAVITY
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ void compute_third_body_acceleration(
    double sat_x, double sat_y, double sat_z,
    double body_x, double body_y, double body_z,
    double mu_body,
    double* ax, double* ay, double* az
) {
    double dx = body_x - sat_x;
    double dy = body_y - sat_y;
    double dz = body_z - sat_z;
    double r_rel = sqrt(dx * dx + dy * dy + dz * dz);
    double r_rel_cubed = r_rel * r_rel * r_rel;

    double r_body = sqrt(body_x * body_x + body_y * body_y + body_z * body_z);
    double r_body_cubed = r_body * r_body * r_body;

    *ax = mu_body * (dx / r_rel_cubed - body_x / r_body_cubed);
    *ay = mu_body * (dy / r_rel_cubed - body_y / r_body_cubed);
    *az = mu_body * (dz / r_rel_cubed - body_z / r_body_cubed);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// J2-J4 ZONAL HARMONIC ACCELERATION
// ═══════════════════════════════════════════════════════════════════════════════════
// These are the dominant perturbations for Earth-orbiting satellites

__device__ __forceinline__ void compute_j2_j4_acceleration(
    double x, double y, double z,
    double* ax, double* ay, double* az
) {
    double r2 = x*x + y*y + z*z;
    double r = sqrt(r2);
    double r3 = r2 * r;
    double r4 = r2 * r2;
    double r5 = r4 * r;
    double r7 = r5 * r2;

    double z2 = z * z;
    double z4 = z2 * z2;

    double Re2 = RE_EGM96 * RE_EGM96;
    double Re3 = Re2 * RE_EGM96;
    double Re4 = Re2 * Re2;

    // J2 acceleration
    // a_J2 = (3/2) * J2 * mu * Re² / r^5 * [x*(5z²/r² - 1), y*(5z²/r² - 1), z*(5z²/r² - 3)]
    double j2_coeff = 1.5 * J2_EGM96 * MU_EGM96 * Re2 / r5;
    double z2_r2 = z2 / r2;
    double j2_xy_term = 5.0 * z2_r2 - 1.0;
    double j2_z_term = 5.0 * z2_r2 - 3.0;

    double j2_ax = j2_coeff * x * j2_xy_term;
    double j2_ay = j2_coeff * y * j2_xy_term;
    double j2_az = j2_coeff * z * j2_z_term;

    // J3 acceleration (causes long-period eccentricity/inclination variations)
    // a_J3 = (5/2) * J3 * mu * Re³ / r^7 * [x*z*(7z²/r² - 3), y*z*(7z²/r² - 3), z²*(7z²/r² - 6) - (3/5)*r²]
    double j3_coeff = 2.5 * J3_EGM96 * MU_EGM96 * Re3 / r7;
    double z2_r2_7 = 7.0 * z2_r2;

    double j3_ax = j3_coeff * x * z * (z2_r2_7 - 3.0);
    double j3_ay = j3_coeff * y * z * (z2_r2_7 - 3.0);
    double j3_az = j3_coeff * (z2 * (z2_r2_7 - 6.0) - 0.6 * r2);

    // J4 acceleration (smaller but included for completeness)
    // a_J4 = (5/8) * J4 * mu * Re^4 / r^7 * [x*(3 - 42z²/r² + 63z^4/r^4), ...]
    double j4_coeff = 0.625 * J4_EGM96 * MU_EGM96 * Re4 / r7;
    double z4_r4 = z4 / r4;
    double j4_xy_term = 3.0 - 42.0 * z2_r2 + 63.0 * z4_r4;
    double j4_z_term = 15.0 - 70.0 * z2_r2 + 63.0 * z4_r4;

    double j4_ax = j4_coeff * x * j4_xy_term;
    double j4_ay = j4_coeff * y * j4_xy_term;
    double j4_az = j4_coeff * z * j4_z_term;

    // Total J2-J4 acceleration
    *ax = j2_ax + j3_ax + j4_ax;
    *ay = j2_ay + j3_ay + j4_ay;
    *az = j2_az + j3_az + j4_az;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TOTAL PERTURBATION ACCELERATION
// ═══════════════════════════════════════════════════════════════════════════════════
// Full perturbation model: J2-J4 + J22 tesseral + lunisolar gravity + SRP with shadow

__device__ __forceinline__ void compute_total_perturbation(
    double x, double y, double z,
    double vx, double vy, double vz,
    double jd,
    double cr, double area_mass,
    double* ax, double* ay, double* az
) {
    // J2-J4 zonal harmonics (dominant perturbation)
    double j2j4_ax, j2j4_ay, j2j4_az;
    compute_j2_j4_acceleration(x, y, z, &j2j4_ax, &j2j4_ay, &j2j4_az);

    // Get Sun and Moon positions
    double sun_x, sun_y, sun_z;
    compute_sun_position_vsop(jd, &sun_x, &sun_y, &sun_z);

    double moon_x, moon_y, moon_z;
    compute_moon_position_brown(jd, &moon_x, &moon_y, &moon_z);

    // SRP acceleration with Earth shadow
    double shadow = compute_shadow_factor(x, y, z, sun_x, sun_y, sun_z);
    double srp_ax, srp_ay, srp_az;
    compute_srp_acceleration(x, y, z, sun_x, sun_y, sun_z, cr, area_mass, shadow,
                             &srp_ax, &srp_ay, &srp_az);

    // Solar gravity
    double sun_ax, sun_ay, sun_az;
    compute_third_body_acceleration(x, y, z, sun_x, sun_y, sun_z, MU_SUN,
                                    &sun_ax, &sun_ay, &sun_az);

    // Lunar gravity
    double moon_ax, moon_ay, moon_az;
    compute_third_body_acceleration(x, y, z, moon_x, moon_y, moon_z, MU_MOON,
                                    &moon_ax, &moon_ay, &moon_az);

    // J22 tesseral harmonic
    double j22_ax, j22_ay, j22_az;
    compute_j22_acceleration(x, y, z, jd, &j22_ax, &j22_ay, &j22_az);

    // Total perturbation (J2-J4 + third-body + SRP + J22)
    *ax = j2j4_ax + srp_ax + sun_ax + moon_ax + j22_ax;
    *ay = j2j4_ay + srp_ay + sun_ay + moon_ay + j22_ay;
    *az = j2j4_az + srp_az + sun_az + moon_az + j22_az;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// KEPLERIAN REFERENCE ORBIT (for Encke's method)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ void propagate_kepler(
    double x0, double y0, double z0,
    double vx0, double vy0, double vz0,
    double dt,
    double* x, double* y, double* z,
    double* vx, double* vy, double* vz
) {
    // Handle zero dt case
    if (fabs(dt) < 1e-10) {
        *x = x0; *y = y0; *z = z0;
        *vx = vx0; *vy = vy0; *vz = vz0;
        return;
    }

    // Universal variable formulation for Kepler propagation
    double r0 = sqrt(x0*x0 + y0*y0 + z0*z0);
    double v0_sq = vx0*vx0 + vy0*vy0 + vz0*vz0;
    double rv_dot = x0*vx0 + y0*vy0 + z0*vz0;
    double sqrt_mu = sqrt(MU_EGM96);

    // Semi-major axis: alpha = 1/a
    double alpha = 2.0/r0 - v0_sq/MU_EGM96;

    // Initial guess for chi (universal anomaly)
    double chi;
    if (fabs(alpha) < 1e-10) {
        // Nearly parabolic - use parabolic approximation
        chi = sqrt_mu * dt / r0;
    } else if (alpha > 0.0) {
        // Elliptic orbit
        chi = sqrt_mu * alpha * dt;
    } else {
        // Hyperbolic orbit
        double a = 1.0 / alpha;
        chi = (dt < 0.0 ? -1.0 : 1.0) * sqrt(-a) *
              log((-2.0 * MU_EGM96 * alpha * dt) /
                  (rv_dot + (dt < 0.0 ? -1.0 : 1.0) * sqrt(-MU_EGM96 * a) * (1.0 - r0 * alpha)));
    }

    // Newton-Raphson iteration
    for (int iter = 0; iter < 30; iter++) {
        double chi2 = chi * chi;
        double chi3 = chi2 * chi;
        double psi = alpha * chi2;

        // Stumpff functions C(psi) and S(psi)
        double c2, c3;
        if (fabs(psi) < 1e-6) {
            c2 = 0.5 - psi/24.0 + psi*psi/720.0;
            c3 = 1.0/6.0 - psi/120.0 + psi*psi/5040.0;
        } else if (psi > 0.0) {
            double sqrt_psi = sqrt(psi);
            c2 = (1.0 - cos(sqrt_psi)) / psi;
            c3 = (sqrt_psi - sin(sqrt_psi)) / (sqrt_psi * psi);
        } else {
            double sqrt_neg_psi = sqrt(-psi);
            c2 = (1.0 - cosh(sqrt_neg_psi)) / psi;
            c3 = (sinh(sqrt_neg_psi) - sqrt_neg_psi) / (sqrt_neg_psi * (-psi));
        }

        double F = r0 * chi * c2 + rv_dot / sqrt_mu * chi2 * c3 + chi3 * c3 - sqrt_mu * dt;
        double r = r0 + rv_dot / sqrt_mu * chi + chi2 * c2 * (1.0 - r0 * alpha);

        if (fabs(r) < 1e-10) break;  // Avoid division by zero

        double delta = F / r;
        chi = chi - delta;

        if (fabs(delta) < 1e-12) break;
    }

    // Compute final Stumpff functions
    double chi2 = chi * chi;
    double chi3 = chi2 * chi;
    double psi = alpha * chi2;

    double c2, c3;
    if (fabs(psi) < 1e-6) {
        c2 = 0.5 - psi/24.0 + psi*psi/720.0;
        c3 = 1.0/6.0 - psi/120.0 + psi*psi/5040.0;
    } else if (psi > 0.0) {
        double sqrt_psi = sqrt(psi);
        c2 = (1.0 - cos(sqrt_psi)) / psi;
        c3 = (sqrt_psi - sin(sqrt_psi)) / (sqrt_psi * psi);
    } else {
        double sqrt_neg_psi = sqrt(-psi);
        c2 = (1.0 - cosh(sqrt_neg_psi)) / psi;
        c3 = (sinh(sqrt_neg_psi) - sqrt_neg_psi) / (sqrt_neg_psi * (-psi));
    }

    // Compute f, g coefficients
    double f = 1.0 - chi2 / r0 * c2;
    double g = dt - chi3 / sqrt_mu * c3;

    // New position
    *x = f * x0 + g * vx0;
    *y = f * y0 + g * vy0;
    *z = f * z0 + g * vz0;

    // New radius
    double r_new = sqrt((*x)*(*x) + (*y)*(*y) + (*z)*(*z));
    if (r_new < 1e-10) r_new = 1e-10;  // Safety check

    // Compute f_dot, g_dot coefficients
    double f_dot = sqrt_mu / (r_new * r0) * chi * (psi * c3 - 1.0);
    double g_dot = 1.0 - chi2 / r_new * c2;

    // New velocity
    *vx = f_dot * x0 + g_dot * vx0;
    *vy = f_dot * y0 + g_dot * vy0;
    *vz = f_dot * z0 + g_dot * vz0;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// RK4 INTEGRATOR WITH ENCKE'S METHOD
// ═══════════════════════════════════════════════════════════════════════════════════

// Compute total acceleration including central gravity and perturbations
__device__ __forceinline__ void compute_total_acceleration(
    double x, double y, double z,
    double vx, double vy, double vz,
    double jd,
    double cr, double area_mass,
    double* ax, double* ay, double* az
) {
    // Central gravity: a = -mu/r³ * r
    double r = sqrt(x*x + y*y + z*z);
    double r3 = r * r * r;
    double mu_r3 = MU_EGM96 / r3;

    double ax_central = -mu_r3 * x;
    double ay_central = -mu_r3 * y;
    double az_central = -mu_r3 * z;

    // Perturbation accelerations
    double ax_pert, ay_pert, az_pert;
    compute_total_perturbation(x, y, z, vx, vy, vz, jd, cr, area_mass,
                               &ax_pert, &ay_pert, &az_pert);

    // Total
    *ax = ax_central + ax_pert;
    *ay = ay_central + ay_pert;
    *az = az_central + az_pert;
}

// RK4 integration of full equations of motion
__device__ void rk4_propagate(
    double x0, double y0, double z0,
    double vx0, double vy0, double vz0,
    double jd0,
    double dt_total,
    double cr, double area_mass,
    double* x_out, double* y_out, double* z_out,
    double* vx_out, double* vy_out, double* vz_out
) {
    // Handle zero dt case
    if (fabs(dt_total) < 1e-10) {
        *x_out = x0; *y_out = y0; *z_out = z0;
        *vx_out = vx0; *vy_out = vy0; *vz_out = vz0;
        return;
    }

    // Determine number of substeps
    int n_steps = (int)(fabs(dt_total) / RK4_NOMINAL_STEP) + 1;
    if (n_steps > RK4_MAX_SUBSTEPS) n_steps = RK4_MAX_SUBSTEPS;
    if (n_steps < 1) n_steps = 1;

    double h = dt_total / (double)n_steps;
    double h_days = h / 86400.0;

    // Current state
    double x = x0, y = y0, z = z0;
    double vx = vx0, vy = vy0, vz = vz0;
    double jd = jd0;

    for (int step = 0; step < n_steps; step++) {
        // k1
        double ax1, ay1, az1;
        compute_total_acceleration(x, y, z, vx, vy, vz, jd, cr, area_mass, &ax1, &ay1, &az1);
        double k1_x = vx;
        double k1_y = vy;
        double k1_z = vz;
        double k1_vx = ax1;
        double k1_vy = ay1;
        double k1_vz = az1;

        // k2
        double x2 = x + 0.5*h*k1_x;
        double y2 = y + 0.5*h*k1_y;
        double z2 = z + 0.5*h*k1_z;
        double vx2 = vx + 0.5*h*k1_vx;
        double vy2 = vy + 0.5*h*k1_vy;
        double vz2 = vz + 0.5*h*k1_vz;

        double ax2, ay2, az2;
        compute_total_acceleration(x2, y2, z2, vx2, vy2, vz2, jd + 0.5*h_days, cr, area_mass, &ax2, &ay2, &az2);
        double k2_x = vx2;
        double k2_y = vy2;
        double k2_z = vz2;
        double k2_vx = ax2;
        double k2_vy = ay2;
        double k2_vz = az2;

        // k3
        double x3 = x + 0.5*h*k2_x;
        double y3 = y + 0.5*h*k2_y;
        double z3 = z + 0.5*h*k2_z;
        double vx3 = vx + 0.5*h*k2_vx;
        double vy3 = vy + 0.5*h*k2_vy;
        double vz3 = vz + 0.5*h*k2_vz;

        double ax3, ay3, az3;
        compute_total_acceleration(x3, y3, z3, vx3, vy3, vz3, jd + 0.5*h_days, cr, area_mass, &ax3, &ay3, &az3);
        double k3_x = vx3;
        double k3_y = vy3;
        double k3_z = vz3;
        double k3_vx = ax3;
        double k3_vy = ay3;
        double k3_vz = az3;

        // k4
        double x4 = x + h*k3_x;
        double y4 = y + h*k3_y;
        double z4 = z + h*k3_z;
        double vx4 = vx + h*k3_vx;
        double vy4 = vy + h*k3_vy;
        double vz4 = vz + h*k3_vz;

        double ax4, ay4, az4;
        compute_total_acceleration(x4, y4, z4, vx4, vy4, vz4, jd + h_days, cr, area_mass, &ax4, &ay4, &az4);
        double k4_x = vx4;
        double k4_y = vy4;
        double k4_z = vz4;
        double k4_vx = ax4;
        double k4_vy = ay4;
        double k4_vz = az4;

        // Update state
        x += h/6.0 * (k1_x + 2.0*k2_x + 2.0*k3_x + k4_x);
        y += h/6.0 * (k1_y + 2.0*k2_y + 2.0*k3_y + k4_y);
        z += h/6.0 * (k1_z + 2.0*k2_z + 2.0*k3_z + k4_z);
        vx += h/6.0 * (k1_vx + 2.0*k2_vx + 2.0*k3_vx + k4_vx);
        vy += h/6.0 * (k1_vy + 2.0*k2_vy + 2.0*k3_vy + k4_vy);
        vz += h/6.0 * (k1_vz + 2.0*k2_vz + 2.0*k3_vz + k4_vz);

        jd += h_days;
    }

    *x_out = x;
    *y_out = y;
    *z_out = z;
    *vx_out = vx;
    *vy_out = vy;
    *vz_out = vz;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ORBITAL ELEMENT PROPAGATION (for J2 secular effects)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ void propagate_mean_elements(
    const GeoNumericalParams* params,
    double dt,
    double* sma_out, double* ecc_out, double* inc_out,
    double* raan_out, double* argp_out, double* mean_anom_out
) {
    double raan_dot, argp_dot, M_dot;
    compute_j2_secular_rates(params->sma, params->ecc, params->inc, params->n,
                             &raan_dot, &argp_dot, &M_dot);

    *sma_out = params->sma;
    *ecc_out = params->ecc;
    *inc_out = params->inc;
    *raan_out = normalize_angle(params->raan + raan_dot * dt);
    *argp_out = normalize_angle(params->argp + argp_dot * dt);
    *mean_anom_out = normalize_angle(params->mean_anom + M_dot * dt);

    double delta_a, delta_ecc, delta_inc;
    compute_j2_j4_long_period(*sma_out, *ecc_out, *inc_out, *argp_out,
                             &delta_a, &delta_ecc, &delta_inc);

    *sma_out += delta_a;
    *ecc_out += delta_ecc;
    *inc_out += delta_inc;

    if (*ecc_out < 1.0e-10) *ecc_out = 1.0e-10;
    if (*ecc_out > 0.999) *ecc_out = 0.999;
}

// Convert orbital elements to ECI
__device__ __forceinline__ void elements_to_eci(
    double sma, double ecc, double inc,
    double raan, double argp, double true_anom,
    double* x, double* y, double* z,
    double* vx, double* vy, double* vz
) {
    double p = sma * (1.0 - ecc * ecc);
    double r = p / (1.0 + ecc * cos(true_anom));
    double sqrt_mu_p = sqrt(MU_EGM96 / p);

    double cos_nu = cos(true_anom);
    double sin_nu = sin(true_anom);

    double r_pqw_x = r * cos_nu;
    double r_pqw_y = r * sin_nu;

    double v_pqw_x = -sqrt_mu_p * sin_nu;
    double v_pqw_y = sqrt_mu_p * (ecc + cos_nu);

    double cos_raan = cos(raan);
    double sin_raan = sin(raan);
    double cos_argp = cos(argp);
    double sin_argp = sin(argp);
    double cos_inc = cos(inc);
    double sin_inc = sin(inc);

    double R11 = cos_raan * cos_argp - sin_raan * sin_argp * cos_inc;
    double R12 = -cos_raan * sin_argp - sin_raan * cos_argp * cos_inc;
    double R21 = sin_raan * cos_argp + cos_raan * sin_argp * cos_inc;
    double R22 = -sin_raan * sin_argp + cos_raan * cos_argp * cos_inc;
    double R31 = sin_argp * sin_inc;
    double R32 = cos_argp * sin_inc;

    *x = R11 * r_pqw_x + R12 * r_pqw_y;
    *y = R21 * r_pqw_x + R22 * r_pqw_y;
    *z = R31 * r_pqw_x + R32 * r_pqw_y;

    *vx = R11 * v_pqw_x + R12 * v_pqw_y;
    *vy = R21 * v_pqw_x + R22 * v_pqw_y;
    *vz = R31 * v_pqw_x + R32 * v_pqw_y;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// MAIN GEO PROPAGATION FUNCTION (v2.0 with RK4/Encke)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void geo_propagate_single(
    const GeoNumericalParams* params,
    double tsince_min,
    double* x, double* y, double* z,
    double* vx, double* vy, double* vz,
    int* error_code
) {
    double dt = tsince_min * SEC_PER_MIN;

    // Get initial state at epoch from orbital elements
    double sma_ep, ecc_ep, inc_ep, raan_ep, argp_ep, mean_anom_ep;
    propagate_mean_elements(params, 0.0, &sma_ep, &ecc_ep, &inc_ep, &raan_ep, &argp_ep, &mean_anom_ep);
    double ecc_anom_ep = mean_to_eccentric_anomaly(mean_anom_ep, ecc_ep);
    double true_anom_ep = eccentric_to_true_anomaly(ecc_anom_ep, ecc_ep);

    double x_epoch, y_epoch, z_epoch, vx_epoch, vy_epoch, vz_epoch;
    elements_to_eci(sma_ep, ecc_ep, inc_ep, raan_ep, argp_ep, true_anom_ep,
                    &x_epoch, &y_epoch, &z_epoch, &vx_epoch, &vy_epoch, &vz_epoch);

    // Integrate from epoch to target time using RK4
    double jd0 = params->epoch_jd;

    rk4_propagate(
        x_epoch, y_epoch, z_epoch,
        vx_epoch, vy_epoch, vz_epoch,
        jd0, dt,
        params->cr, params->area_mass,
        x, y, z, vx, vy, vz
    );

    // Validate output
    double r = sqrt((*x) * (*x) + (*y) * (*y) + (*z) * (*z));
    if (r < 35000.0 || r > 50000.0) {
        *error_code = 2;
    } else {
        *error_code = 0;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INITIALIZATION FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void eci_to_elements(
    double x, double y, double z,
    double vx, double vy, double vz,
    double* sma, double* ecc, double* inc,
    double* raan, double* argp, double* mean_anom
) {
    double r = sqrt(x * x + y * y + z * z);
    double v = sqrt(vx * vx + vy * vy + vz * vz);

    double hx = y * vz - z * vy;
    double hy = z * vx - x * vz;
    double hz = x * vy - y * vx;
    double h = sqrt(hx * hx + hy * hy + hz * hz);

    double nx = -hy;
    double ny = hx;
    double n = sqrt(nx * nx + ny * ny);

    double rv_dot = x * vx + y * vy + z * vz;
    double mu = MU_EGM96;
    double ex = (v * v / mu - 1.0 / r) * x - rv_dot / mu * vx;
    double ey = (v * v / mu - 1.0 / r) * y - rv_dot / mu * vy;
    double ez = (v * v / mu - 1.0 / r) * z - rv_dot / mu * vz;
    *ecc = sqrt(ex * ex + ey * ey + ez * ez);

    double energy = v * v / 2.0 - mu / r;
    *sma = -mu / (2.0 * energy);

    *inc = acos(hz / h);

    if (n > 1.0e-10) {
        *raan = acos(nx / n);
        if (ny < 0.0) *raan = TWOPI - *raan;
    } else {
        *raan = 0.0;
    }

    if (*ecc > 1.0e-10 && n > 1.0e-10) {
        double cos_argp_val = (nx * ex + ny * ey) / (n * (*ecc));
        if (cos_argp_val > 1.0) cos_argp_val = 1.0;
        if (cos_argp_val < -1.0) cos_argp_val = -1.0;
        *argp = acos(cos_argp_val);
        if (ez < 0.0) *argp = TWOPI - *argp;
    } else {
        *argp = 0.0;
    }

    double true_anom;
    if (*ecc > 1.0e-10) {
        double cos_nu_val = (ex * x + ey * y + ez * z) / ((*ecc) * r);
        if (cos_nu_val > 1.0) cos_nu_val = 1.0;
        if (cos_nu_val < -1.0) cos_nu_val = -1.0;
        true_anom = acos(cos_nu_val);
        if (rv_dot < 0.0) true_anom = TWOPI - true_anom;
    } else {
        double cos_u = (nx * x + ny * y) / (n * r);
        if (cos_u > 1.0) cos_u = 1.0;
        if (cos_u < -1.0) cos_u = -1.0;
        true_anom = acos(cos_u);
        if (z < 0.0) true_anom = TWOPI - true_anom;
    }

    double sqrt_1_e2 = sqrt(1.0 - (*ecc) * (*ecc));
    double sin_nu = sin(true_anom);
    double cos_nu = cos(true_anom);
    double ecc_anom = atan2(sqrt_1_e2 * sin_nu, *ecc + cos_nu);
    *mean_anom = ecc_anom - (*ecc) * sin(ecc_anom);
    if (*mean_anom < 0.0) *mean_anom += TWOPI;
}

__device__ void geo_init_from_eci(
    const GeoStateGpu* state,
    GeoNumericalParams* params
) {
    eci_to_elements(
        state->x, state->y, state->z,
        state->vx, state->vy, state->vz,
        &params->sma, &params->ecc, &params->inc,
        &params->raan, &params->argp, &params->mean_anom
    );

    params->epoch_jd = state->epoch_jd;
    params->cr = (state->cr > 0.0) ? state->cr : DEFAULT_CR;
    params->area_mass = (state->area_mass > 0.0) ? state->area_mass : DEFAULT_AREA_MASS;

    params->n = sqrt(MU_EGM96 / (params->sma * params->sma * params->sma));

    params->original_index = state->original_index;

    params->cos_inc = cos(params->inc);
    params->sin_inc = sin(params->inc);
    params->cos_inc_sq = params->cos_inc * params->cos_inc;
    params->sin_inc_sq = params->sin_inc * params->sin_inc;

    double p = params->sma * (1.0 - params->ecc * params->ecc);
    params->j2_factor = J2_EGM96 * RE_EGM96 * RE_EGM96 / (2.0 * p * p);

    compute_j2_secular_rates(params->sma, params->ecc, params->inc, params->n,
                             &params->raan_dot_j2, &params->argp_dot_j2, &params->n_dot_j2);
}

#endif // GEO_NUMERICAL_CUH
