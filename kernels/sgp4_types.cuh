// SGP4 Type Definitions for CUDA
// Shared structures between CUDA and Rust
// IMPORTANT: These must match the Rust structs exactly!

#ifndef SGP4_TYPES_CUH
#define SGP4_TYPES_CUH

// Raw TLE data input (from Rust)
// Size: 10 doubles = 80 bytes
struct TleData {
    double epoch_jd;
    double inclination;      // degrees
    double raan;             // degrees  
    double eccentricity;
    double arg_perigee;      // degrees
    double mean_anomaly;     // degrees
    double mean_motion;      // revs/day
    double bstar;
    double ndot;             // first derivative of mean motion
    double nddot;            // second derivative of mean motion
};

// Precomputed SGP4 parameters (after initialization)
// Must match Rust Sgp4ParamsGpu exactly
struct Sgp4Params {
    // TLE epoch and elements (in radians and native units) - 10 doubles
    double epoch_jd;
    double inclo;
    double nodeo;
    double ecco;
    double argpo;
    double mo;
    double no_kozai;
    double bstar;
    double ndot;
    double nddot;
    
    // Derived initialization constants - 30 doubles
    double a;
    double alta;
    double altp;
    double con41;
    double con42;
    double cosio;
    double cosio2;
    double cosio4;
    double cc1;
    double cc4;
    double cc5;
    double d2;
    double d3;
    double d4;
    double delmo;
    double eta;
    double argpdot;
    double omgcof;
    double sinmao;
    double t2cof;
    double t3cof;
    double t4cof;
    double t5cof;
    double x1mth2;
    double x7thm1;
    double xlcof;
    double xmcof;
    double xnodcf;
    double nodedot;
    double mdot;
    double no_unkozai;
    double aycof;
    double delmo_const;
    
    // Deep space flag - 4 ints for alignment
    int is_deep_space;
    int _padding[3];
};

// Output state (position and velocity in TEME frame)
// Must match Rust Sgp4StateGpu exactly
struct Sgp4State {
    double x, y, z;        // Position (km)
    double vx, vy, vz;     // Velocity (km/s)
    int error_code;        // 0 = success, 1 = decayed, 2 = other error
    int _padding;          // Alignment padding to 8-byte boundary
};

#endif // SGP4_TYPES_CUH
