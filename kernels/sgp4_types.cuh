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
    
    // ═══════════════════════════════════════════════════════════════════
    // DEEP SPACE PARAMETERS
    // ═══════════════════════════════════════════════════════════════════
    
    // Greenwich sidereal time at epoch
    double gsto;
    
    // Lunar-solar terms (from DSCOM)
    double e3;
    double ee2;
    double peo;
    double pgho;
    double pho;
    double pinco;
    double plo;
    double se2;
    double se3;
    double sgh2;
    double sgh3;
    double sgh4;
    double sh2;
    double sh3;
    double si2;
    double si3;
    double sl2;
    double sl3;
    double sl4;
    double xgh2;
    double xgh3;
    double xgh4;
    double xh2;
    double xh3;
    double xi2;
    double xi3;
    double xl2;
    double xl3;
    double xl4;
    double zmol;
    double zmos;
    
    // Secular rates (from DSINIT)
    double dedt;
    double didt;
    double dmdt;
    double dnodt;
    double domdt;
    
    // Resonance terms (from DSINIT)
    double d2201;
    double d2211;
    double d3210;
    double d3222;
    double d4410;
    double d4422;
    double d5220;
    double d5232;
    double d5421;
    double d5433;
    double del1;
    double del2;
    double del3;
    double xfact;
    double xlamo;
    double xli;
    double xni;
    double atime;
    
    // Flags
    int is_deep_space;
    int irez;                // 0=none, 1=one-day, 2=half-day resonance
    int force_near_earth;    // 1=override is_deep_space, force SGP4 behavior
    int _padding;            // Maintain 8-byte alignment
};

// Output state (position and velocity in TEME frame)
// Must match Rust Sgp4StateGpu exactly
struct Sgp4State {
    double x, y, z;        // Position (km)
    double vx, vy, vz;     // Velocity (km/s)
    int error_code;        // 0 = success, 1 = decayed, 2 = other error
    int _padding;          // Alignment padding to 8-byte boundary
};

// ═══════════════════════════════════════════════════════════════════════════
// STRUCT OF ARRAYS (SoA) LAYOUT FOR OPTIMIZED MEMORY COALESCING
// ═══════════════════════════════════════════════════════════════════════════

// SoA output state for coalesced memory writes
// Each pointer points to an array of [n_sats * n_times] elements
// Layout uses time-major ordering: states_x[time_idx * n_sats + sat_idx]
// This ensures adjacent threads write to adjacent memory addresses
struct Sgp4StateSoA {
    double* x;          // [n_sats * n_times] Position X (km)
    double* y;          // [n_sats * n_times] Position Y (km)
    double* z;          // [n_sats * n_times] Position Z (km)
    double* vx;         // [n_sats * n_times] Velocity X (km/s)
    double* vy;         // [n_sats * n_times] Velocity Y (km/s)
    double* vz;         // [n_sats * n_times] Velocity Z (km/s)
    int* error_code;    // [n_sats * n_times] Error codes
};

#endif // SGP4_TYPES_CUH
