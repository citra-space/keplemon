// SGP4 Constants Header
// Based on Vallado's SGP4 implementation

#ifndef SGP4_CONSTANTS_CUH
#define SGP4_CONSTANTS_CUH

// Mathematical constants
#define PI 3.14159265358979323846
#define TWOPI (2.0 * PI)
#define DEG2RAD (PI / 180.0)
#define RAD2DEG (180.0 / PI)

// Physical constants
#define XKE 0.0743669161331734132      // sqrt(GM) (er^3/2 / min)
#define VKMPERSEC 7.905366149846074e-3 // km/s per velocity unit
#define X2O3 (2.0 / 3.0)

// Earth constants
#define RE 6378.137                    // Earth radius (km)
#define J2 0.00108262998905            // J2 harmonic
#define J3 -0.00000253215306           // J3 harmonic  
#define J4 -0.00000161098761           // J4 harmonic

// Time constants
#define MINUTES_PER_DAY 1440.0

// Thresholds
#define DEEP_SPACE_PERIOD_MIN 225.0    // minutes

#endif // SGP4_CONSTANTS_CUH
