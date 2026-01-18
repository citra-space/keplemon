// SGP4 Constants Header
// Based on Vallado's SGP4 implementation with WGS-72 gravity model
// (matches AFSPC/saal library for compatibility)

#ifndef SGP4_CONSTANTS_CUH
#define SGP4_CONSTANTS_CUH

// Mathematical constants
#define PI 3.14159265358979323846
#define TWOPI (2.0 * PI)
#define DEG2RAD (PI / 180.0)
#define RAD2DEG (180.0 / PI)

// WGS-72 Physical constants (matches AFSPC implementation)
#define XKE 0.0743669161331734132      // sqrt(GM) in (er^3/2 / min) for WGS-72
#define TUMIN 13.44683969695931        // 1.0 / XKE
#define X2O3 (2.0 / 3.0)

// WGS-72 Earth constants
#define RE 6378.135                    // Earth equatorial radius (km) - WGS-72
#define J2 0.001082616                 // J2 harmonic (WGS-72)
#define J3 -0.00000253881              // J3 harmonic (WGS-72)
#define J4 -0.00000165597              // J4 harmonic (WGS-72)
#define J3OJ2 (J3 / J2)                // J3/J2 ratio for long period terms

// Derived velocity constant: converts internal velocity units to km/s
// VKMPERSEC = XKE * RE / 60 = sqrt(GM_er3/min2) * km/er / (s/min)
#define VKMPERSEC 7.905365719014155    // km/s per velocity unit

// Time constants
#define MINUTES_PER_DAY 1440.0

// Thresholds
#define DEEP_SPACE_PERIOD_MIN 225.0    // minutes (period above which deep space is used)

// ═══════════════════════════════════════════════════════════════════════════
// DEEP SPACE CONSTANTS (Lunar/Solar perturbations)
// ═══════════════════════════════════════════════════════════════════════════

// Moon constants
#define ZNS 1.19459e-5                 // Mean motion of sun (rad/min)
#define C1SS 2.9864797e-6              // Solar secular coefficient
#define ZES 0.01675                    // Solar eccentricity
#define ZNL 1.5835218e-4               // Mean motion of moon (rad/min)
#define C1L 0.00015835218              // Lunar secular coefficient
#define ZEL 0.05490                    // Lunar eccentricity

// Resonance constants
#define ROOT22 1.7891679e-6
#define ROOT32 3.7393792e-7
#define ROOT44 7.3636953e-9
#define ROOT52 1.1428639e-7
#define ROOT54 2.1765803e-9
#define G22 5.7686396
#define G32 0.95240898
#define G44 1.8014998
#define G52 1.0508330
#define G54 4.4108898
#define Q22 1.7891679e-6
#define Q31 2.1460748e-6
#define Q33 2.2123015e-7

// Thresholds for resonance
#define RPTIM 4.37526908801129966e-3   // Solar rate (rad/min)
#define STEP 720.0                     // Step size for resonance integration
#define STEP2 259200.0                 // 180 days in minutes

// Deep space inclination limits
#define INCLM_LIM 5.2359877e-2         // ~3 degrees in radians

#endif // SGP4_CONSTANTS_CUH
