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

#endif // SGP4_CONSTANTS_CUH
