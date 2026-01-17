// SGP4 Initialization Kernel
// Processes TLE data and computes derived constants for batch propagation

#include "sgp4_types.cuh"
#include "sgp4_constants.cuh"

// Device function for SGP4 initialization
__device__ void sgp4_init_single(const TleData& tle, Sgp4Params& p) {
    // Convert TLE elements to radians and native units
    p.epoch_jd = tle.epoch_jd;
    p.inclo = tle.inclination * DEG2RAD;
    p.nodeo = tle.raan * DEG2RAD;
    p.ecco = tle.eccentricity;
    p.argpo = tle.arg_perigee * DEG2RAD;
    p.mo = tle.mean_anomaly * DEG2RAD;
    p.bstar = tle.bstar;
    p.ndot = tle.ndot;
    p.nddot = tle.nddot;
    
    // Convert mean motion from revs/day to radians/minute
    p.no_kozai = tle.mean_motion * TWOPI / MINUTES_PER_DAY;
    
    // Pre-compute trigonometric values
    double cosio = cos(p.inclo);
    p.cosio = cosio;
    p.cosio2 = cosio * cosio;
    p.cosio4 = p.cosio2 * p.cosio2;
    double sinio = sin(p.inclo);
    
    // Common expressions
    p.con41 = 3.0 * p.cosio2 - 1.0;
    p.con42 = 1.0 - 5.0 * p.cosio2;
    p.x1mth2 = 1.0 - p.cosio2;
    p.x7thm1 = 7.0 * p.cosio2 - 1.0;
    
    // WGS-72 constants for un-Kozai the mean motion
    double ak = pow(XKE / p.no_kozai, X2O3);
    double d1 = 0.75 * J2 * (3.0 * p.cosio2 - 1.0) / 
                (pow(1.0 - p.ecco * p.ecco, 1.5));
    double del = d1 / (ak * ak);
    double ao = ak * (1.0 - del * (1.0/3.0 + del * (1.0 + 134.0/81.0 * del)));
    double delo = d1 / (ao * ao);
    double no_unkozai = p.no_kozai / (1.0 + delo);
    p.no_unkozai = no_unkozai;
    
    // Semi-major axis
    double a = pow(XKE / no_unkozai, X2O3);
    p.a = a;
    
    // Eccentricity squared
    double e2 = p.ecco * p.ecco;
    double omeosq = 1.0 - e2;
    
    // Perigee and apogee altitudes
    double rp = a * (1.0 - p.ecco);
    double ra = a * (1.0 + p.ecco);
    p.altp = (rp - 1.0) * RE;  // km
    p.alta = (ra - 1.0) * RE;  // km
    
    // Determine deep space vs near-earth
    double period = TWOPI / no_unkozai;  // minutes
    p.is_deep_space = (period >= DEEP_SPACE_PERIOD_MIN) ? 1 : 0;
    
    // Common calculations
    double s = RE / a + 1.0;
    double qoms2t = pow((120.0 - 78.0) / RE, 4.0);
    double temp0 = qoms2t * pow(s, 4.0);
    
    double temp1 = cos(p.inclo);
    double temp2 = temp0 * J2;
    double temp3 = temp2 * J2;
    
    p.mdot = no_unkozai + 0.5 * temp2 * no_unkozai / omeosq * 
             (-1.0 + 3.0 * p.cosio2);
    p.argpdot = -0.5 * temp2 / omeosq * (5.0 * p.cosio2 - 1.0);
    p.nodedot = -temp2 * cosio / omeosq;
    
    double betal = sqrt(omeosq);
    p.eta = a * p.ecco * betal;
    p.delmo = pow(1.0 + p.eta * cos(p.mo), 3.0);
    
    // C coefficients for near-earth satellites
    if (!p.is_deep_space) {
        double c1sq = 1.5 * J2 * (3.0 * p.cosio2 - 1.0) / 
                      (omeosq * omeosq);
        p.cc1 = p.bstar * 2.0 / (4.0 * a * betal * omeosq) * c1sq;
        
        double cosio4 = p.cosio4;
        double temp4 = temp0 * J2;
        double c4 = 2.0 * no_unkozai * temp4 / omeosq;
        double c5 = 0.5 * temp4 * a * betal / omeosq;
        
        p.cc4 = c4;
        p.cc5 = c5;
        
        // D terms
        double theta2 = p.cosio2;
        double theta4 = theta2 * theta2;
        double temp = c1sq - 2.0 * theta2;
        p.d2 = 4.0 * a * p.cc1 * temp;
        
        temp = 3.0 * temp - 12.0 * theta2 * theta2;
        p.d3 = (17.0 * a + s) * p.cc1 * temp / 3.0;
        
        temp = 3.0 * (1.0 - 13.0 * theta2 + 5.0 * theta4);
        p.d4 = 0.5 * a * p.cc1 * (5.0 + temp) * temp;
        
        // T coefficients
        p.t2cof = 1.5 * p.cc1;
        p.t3cof = p.d2 + 2.0 * p.cc1 * p.cc1;
        p.t4cof = 0.25 * (3.0 * p.d3 + p.cc1 * (12.0 * p.d2 + 10.0 * p.cc1 * p.cc1));
        p.t5cof = 0.2 * (3.0 * p.d4 + 12.0 * p.cc1 * p.d3 + 
                  6.0 * p.d2 * p.d2 + 15.0 * p.cc1 * p.cc1 * (2.0 * p.d2 + p.cc1 * p.cc1));
        
        // Additional coefficients
        p.sinmao = sin(p.mo);
        p.omgcof = p.bstar * 1.5 * J2 * (5.0 * p.cosio2 - 1.0);
        p.xmcof = -X2O3 * qoms2t * p.bstar * RE / p.eta;
        p.xlcof = 0.125 * 1.5 * J2 * sinio * (3.0 + 5.0 * cosio) / 
                  (1.0 + cosio);
        p.aycof = 0.25 * 1.5 * J2 * sinio;
        p.xnodcf = 3.5 * omeosq * p.nodedot * p.cosio;
    } else {
        // Deep space satellites - simplified initialization
        // (Full deep space periodic effects would go here)
        p.cc1 = p.cc4 = p.cc5 = 0.0;
        p.d2 = p.d3 = p.d4 = 0.0;
        p.t2cof = p.t3cof = p.t4cof = p.t5cof = 0.0;
        p.sinmao = 0.0;
        p.omgcof = p.xmcof = p.xlcof = p.aycof = p.xnodcf = 0.0;
    }
}

// Kernel to initialize batch of satellites
__global__ void sgp4_init_batch(
    const TleData* __restrict__ tle_data,  // [n_sats]
    Sgp4Params* __restrict__ params,        // [n_sats] output
    int n_sats
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    
    if (idx >= n_sats) return;
    
    const TleData& tle = tle_data[idx];
    Sgp4Params& p = params[idx];
    
    sgp4_init_single(tle, p);
}
