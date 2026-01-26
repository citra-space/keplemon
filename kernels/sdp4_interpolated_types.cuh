// ═══════════════════════════════════════════════════════════════════════════════════
// SDP4 Interpolated Propagator Type Definitions
// ═══════════════════════════════════════════════════════════════════════════════════
//
// This file defines types for the GPU-accelerated SDP4-compatible interpolated propagator.
// The key innovation is pre-sampled resonance interpolation, which eliminates the
// iterative DSPACE loop that causes thread divergence in standard GPU SDP4.
//
// Design:
//   1. At initialization, pre-compute resonance effects at regular intervals
//   2. Store samples in GPU memory (~2KB per satellite for ±30 days at 6h intervals)
//   3. At propagation time, interpolate resonance contribution using cubic spline
//   4. Eliminates iteration → all threads execute identical instructions → ~20-50x speedup
//
// ═══════════════════════════════════════════════════════════════════════════════════

#ifndef SDP4_INTERPOLATED_TYPES_CUH
#define SDP4_INTERPOLATED_TYPES_CUH

// Maximum number of resonance samples per satellite
// 241 samples = ±30 days at 6-hour intervals = (30*4 + 1) * 2 = 241
#define SDP4_MAX_RESONANCE_SAMPLES 241

// Resonance sample interval in minutes (6 hours)
#define SDP4_RESONANCE_SAMPLE_INTERVAL_MIN 360.0

// Maximum propagation span in minutes (±30 days)
#define SDP4_MAX_PROPAGATION_SPAN_MIN 43200.0

// ═══════════════════════════════════════════════════════════════════════════════════
// SDP4 ANALYTICAL PARAMETERS STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════════
// Contains all precomputed parameters needed for analytical SDP4 propagation.
// This struct is computed once during initialization and used for all propagations.

struct Sdp4InterpolatedParams {
    // ═══════════════════════════════════════════════════════════════════════════════
    // TLE EPOCH AND MEAN ELEMENTS
    // ═══════════════════════════════════════════════════════════════════════════════
    double epoch_jd;        // Julian Date of TLE epoch
    double inclo;           // Inclination at epoch (radians)
    double nodeo;           // Right ascension of ascending node at epoch (radians)
    double ecco;            // Eccentricity at epoch
    double argpo;           // Argument of perigee at epoch (radians)
    double mo;              // Mean anomaly at epoch (radians)
    double no_kozai;        // Mean motion (Kozai) (radians/min)
    double no_unkozai;      // Mean motion (un-Kozai'd) (radians/min)
    double bstar;           // B* drag coefficient

    // ═══════════════════════════════════════════════════════════════════════════════
    // DERIVED CONSTANTS (from SGP4 initialization)
    // ═══════════════════════════════════════════════════════════════════════════════
    double a;               // Semi-major axis (Earth radii)
    double alta;            // Apogee altitude (Earth radii)
    double altp;            // Perigee altitude (Earth radii)
    double con41;           // 1 - 5*cos²(i)
    double con42;           // -con41 - 2
    double cosio;           // cos(i)
    double sinio;           // sin(i)
    double cosio2;          // cos²(i)
    double cosio4;          // cos⁴(i)
    double eta;             // e*cos(ω)
    double argpdot;         // Secular rate of argument of perigee
    double omgcof;          // Coefficient for ω secular rate
    double sinmao;          // sin(M₀)
    double x1mth2;          // 1 - cos²(i)
    double x7thm1;          // 7*cos²(i) - 1
    double xlcof;           // Coefficient for Lyddane modification
    double xmcof;           // Coefficient for mean anomaly correction
    double xnodcf;          // Coefficient for node correction
    double cc1;             // C1 coefficient
    double cc4;             // C4 coefficient
    double cc5;             // C5 coefficient
    double d2;              // D2 coefficient
    double d3;              // D3 coefficient
    double d4;              // D4 coefficient
    double delmo;           // Modified e*sin(M)
    double t2cof;           // T² coefficient
    double t3cof;           // T³ coefficient
    double t4cof;           // T⁴ coefficient
    double t5cof;           // T⁵ coefficient
    double mdot;            // Mean anomaly rate (radians/min)
    double nodedot;         // Node rate (radians/min)
    double aycof;           // Coefficient for ay computation
    double delmo_const;     // Constant for delmo computation

    // ═══════════════════════════════════════════════════════════════════════════════
    // GREENWICH SIDEREAL TIME AT EPOCH
    // ═══════════════════════════════════════════════════════════════════════════════
    double gsto;            // Greenwich sidereal time at epoch (radians)

    // ═══════════════════════════════════════════════════════════════════════════════
    // LUNAR-SOLAR CONSTANTS (from DSCOM)
    // ═══════════════════════════════════════════════════════════════════════════════
    // These coefficients capture the lunar and solar gravitational perturbations
    double zmos;            // Solar mean anomaly at epoch
    double zmol;            // Lunar mean anomaly at epoch

    // Secular lunar-solar coefficients
    double se2, se3;        // e-dependent solar terms
    double sgh2, sgh3, sgh4; // ω-dependent solar terms
    double sh2, sh3;        // i-dependent solar terms (used if inclination > 0.2 rad)
    double si2, si3;        // Ω-dependent solar terms
    double sl2, sl3, sl4;   // λ-dependent solar terms

    // Periodic lunar-solar coefficients (from DSCOM xinc loop)
    double xgh2, xgh3, xgh4; // ω-dependent lunar terms
    double xh2, xh3;        // i-dependent lunar terms
    double xi2, xi3;        // Ω-dependent lunar terms
    double xl2, xl3, xl4;   // λ-dependent lunar terms
    double e3, ee2;         // Additional e-dependent terms

    // DPPER coefficients (from DSINIT/DSCOM)
    double peo, pgho, pho;  // Periodic coefficients for e, ω, i
    double pinco, plo;      // Periodic coefficients for Ω, λ

    // ═══════════════════════════════════════════════════════════════════════════════
    // SECULAR RATES (from DSINIT)
    // ═══════════════════════════════════════════════════════════════════════════════
    // These rates are applied analytically (no iteration needed)
    double dedt;            // de/dt secular rate
    double didt;            // di/dt secular rate
    double dmdt;            // dM/dt secular rate (additional to mdot)
    double dnodt;           // dΩ/dt secular rate (additional to nodedot)
    double domdt;           // dω/dt secular rate (additional to argpdot)

    // ═══════════════════════════════════════════════════════════════════════════════
    // RESONANCE TERMS
    // ═══════════════════════════════════════════════════════════════════════════════
    int irez;               // Resonance flag: 0=none, 1=synchronous (GEO), 2=half-day (GPS)

    // Synchronous (1-day) resonance terms (irez=1)
    double del1, del2, del3;

    // Half-day resonance terms (irez=2)
    double d2201, d2211;
    double d3210, d3222;
    double d4410, d4422;
    double d5220, d5232;
    double d5421, d5433;

    // Common resonance variables
    double xfact;           // Resonance factor
    double xlamo;           // λ at epoch for resonance

    // ═══════════════════════════════════════════════════════════════════════════════
    // RESONANCE SAMPLE METADATA
    // ═══════════════════════════════════════════════════════════════════════════════
    // These describe the pre-computed resonance samples stored in a separate buffer
    int n_samples;          // Number of resonance samples
    double sample_t0;       // Start time of samples relative to epoch (minutes)
    double sample_dt;       // Time interval between samples (minutes)

    // ═══════════════════════════════════════════════════════════════════════════════
    // ORIGINAL INDEX FOR OUTPUT MAPPING
    // ═══════════════════════════════════════════════════════════════════════════════
    int original_index;     // Satellite index for result reordering

    // Padding for alignment
    int _padding;
};

// ═══════════════════════════════════════════════════════════════════════════════════
// RESONANCE SAMPLE STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════════
// Pre-computed resonance contributions at regular time intervals.
// Stored separately to allow different sample counts per satellite.

struct Sdp4ResonanceSample {
    double xli;             // Integrated mean motion anomaly
    double xni;             // Instantaneous mean motion
    double atime;           // Time of this sample (minutes since epoch)
};

// ═══════════════════════════════════════════════════════════════════════════════════
// OUTPUT STATE STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════════
// Reuses the Sgp4State structure from tle_propagator_types.cuh for compatibility

// Note: Sgp4State is already defined in tle_propagator_types.cuh, which this file includes via
// the .cu file. We don't redefine it here.

#endif // SDP4_INTERPOLATED_TYPES_CUH
