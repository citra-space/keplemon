// SGP4 Type Definitions for CUDA
// Shared structures between CUDA and Rust

#ifndef SGP4_TYPES_CUH
#define SGP4_TYPES_CUH

// Raw TLE data input (from Rust)
struct alignas(16) TleData {
    double epoch_jd;
    double inclination;      // degrees
    double raan;            // degrees  
    double eccentricity;
    double arg_perigee;     // degrees
    double mean_anomaly;    // degrees
    double mean_motion;     // revs/day
    double bstar;
    double ndot;            // first derivative of mean motion
    double nddot;           // second derivative of mean motion
};

// Precomputed SGP4 parameters (after initialization)
struct alignas(16) Sgp4Params {
    // TLE epoch and elements (in radians and native units)
    double epoch_jd;
    double inclo, nodeo, ecco, argpo, mo, no_kozai;
    double bstar, ndot, nddot;
    
    // Derived initialization constants
    double a, alta, altp;
    double con41, con42, cosio, cosio2, cosio4;
    double cc1, cc4, cc5, d2, d3, d4;
    double delmo, eta, argpdot, omgcof;
    double sinmao, t2cof, t3cof, t4cof, t5cof;
    double x1mth2, x7thm1, xlcof, xmcof, xnodcf, nodedot;
    double mdot, no_unkozai;
    double aycof;
    double delmo_const;
    
    // Deep space flag and params (if needed)
    int is_deep_space;
    // Deep space parameters would go here if implementing
};

// Output state (position and velocity in TEME frame)
struct alignas(16) Sgp4State {
    double x, y, z;        // Position (km)
    double vx, vy, vz;     // Velocity (km/s)
    int error_code;        // 0 = success, 1 = decayed, 2 = other error
    double padding;        // Alignment padding
};

#endif // SGP4_TYPES_CUH
