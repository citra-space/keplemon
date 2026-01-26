// GEO Numerical Propagator Type Definitions for CUDA
// Shared structures between CUDA and Rust for GEO propagation
// IMPORTANT: These must match the Rust structs exactly!

#ifndef GEO_TYPES_CUH
#define GEO_TYPES_CUH

// ECI state input for GEO numerical propagation
// Used when converting TLEs to ECI states for the numerical propagator
// Size: 10 doubles + 2 ints = 88 bytes
struct GeoStateGpu {
    // Position in TEME/ECI frame (km)
    double x, y, z;

    // Velocity in TEME/ECI frame (km/s)
    double vx, vy, vz;

    // Reference epoch (Julian Date)
    double epoch_jd;

    // Solar radiation pressure parameters
    double cr;              // Reflectivity coefficient (default 1.3, range 1.0-2.0)
    double area_mass;       // Area-to-mass ratio (m²/kg, default ~0.02 for GEO)

    // Computed mean orbital elements (set during initialization)
    double sma;             // Semi-major axis (km)

    // Satellite index for output mapping (when partitioned)
    int original_index;

    // Padding for alignment
    int _padding;
};

// Precomputed GEO numerical propagator parameters
// Computed once at initialization, used for all propagation steps
struct GeoNumericalParams {
    // Initial mean orbital elements (at epoch)
    double sma;             // Semi-major axis (km)
    double ecc;             // Eccentricity
    double inc;             // Inclination (rad)
    double raan;            // Right ascension of ascending node (rad)
    double argp;            // Argument of perigee (rad)
    double mean_anom;       // Mean anomaly (rad)

    // Reference epoch
    double epoch_jd;

    // Mean motion (rad/s)
    double n;

    // SRP parameters
    double cr;              // Reflectivity coefficient
    double area_mass;       // Area-to-mass ratio (m²/kg)

    // J2 secular rate coefficients (precomputed)
    double n_dot_j2;        // Secular mean motion rate due to J2
    double raan_dot_j2;     // Secular RAAN rate due to J2
    double argp_dot_j2;     // Secular argument of perigee rate due to J2

    // J2 parameters (precomputed for efficiency)
    double j2_factor;       // J2 * RE² / (2 * a²)
    double cos_inc;
    double sin_inc;
    double cos_inc_sq;
    double sin_inc_sq;

    // Original satellite index for output mapping
    int original_index;

    // Padding for alignment
    int _padding;
};

// SoA layout for GEO propagator output
struct GeoStateSoA {
    double* x;              // [n_sats * n_times]
    double* y;
    double* z;
    double* vx;
    double* vy;
    double* vz;
    int* error_code;
};

#endif // GEO_TYPES_CUH
