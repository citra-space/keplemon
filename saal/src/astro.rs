// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::os::raw::c_char;

use super::{GetSetString, environment, get_last_error_message, time};

unsafe extern "C" {
    //  Retrieves information about the current version of AstroFunc.dll. The information is placed in the string parameter you pass in.
    //  The returned string provides information about the version number, build date, and platform.
    pub fn AstroFuncGetInfo(infoStr: *const c_char);
    //  Converts a set of Keplerian elements to a set of equinoctial elements.
    pub fn KepToEqnx(xa_kep: *const [f64; 6], xa_eqnx: *mut [f64; 6]);
    //  Converts a set of osculating Keplerian elements to osculating position and velocity vectors.
    pub fn KepToPosVel(xa_kep: *const [f64; 6], pos: *mut [f64; 3], vel: *mut [f64; 3]);
    //  Converts a set of Keplerian elements to Ubar, Vbar, and Wbar vectors.
    pub fn KepToUVW(xa_kep: *const [f64; 6], uBar: *mut [f64; 3], vBar: *mut [f64; 3], wBar: *mut [f64; 3]);
    //  Converts a set of classical elements to a set of equinoctial elements.
    pub fn ClassToEqnx(xa_cls: *const [f64; 6], xa_eqnx: *mut [f64; 6]);
    //  Converts a set of equinoctial elements to a set of classical elements.
    pub fn EqnxToClass(xa_eqnx: *const [f64; 6], xa_cls: *mut [f64; 6]);
    //  Converts a set of equinoctial elements to a set of Keplerian elements.
    pub fn EqnxToKep(xa_eqnx: *const [f64; 6], xa_kep: *mut [f64; 6]);
    //  Converts a set of equinoctial elements to position and velocity vectors.
    pub fn EqnxToPosVel(xa_eqnx: *const [f64; 6], pos: *mut [f64; 3], vel: *mut [f64; 3]);
    //  Converts position and velocity vectors to a set of equinoctial elements.
    pub fn PosVelToEqnx(pos: *const [f64; 3], vel: *const [f64; 3], xa_eqnx: *mut [f64; 6]);
    //  Converts position and velocity vectors to a set of equinoctial elements with the given mu value.
    //  This function is used when working with the SP propagator to get a more accurate set of equinoctial elements.
    pub fn PosVelMuToEqnx(pos: *const [f64; 3], vel: *const [f64; 3], mu: f64, xa_eqnx: *mut [f64; 6]);
    //  Converts osculating position and velocity vectors to a set of osculating Keplerian elements.
    pub fn PosVelToKep(pos: *const [f64; 3], vel: *const [f64; 3], xa_kep: *mut [f64; 6]);
    //  Converts osculating position and velocity vectors to a set of osculating Keplerian elements with the given value of mu.
    //  This function is used when working with the SP propagator to get a more accurate set of Keplerian elements.
    pub fn PosVelMuToKep(pos: *const [f64; 3], vel: *const [f64; 3], mu: f64, xa_kep: *mut [f64; 6]);
    //  Converts position and velocity vectors to U, V, W vectors. See the remarks section for details.
    //  The resulting vectors have the following meanings.
    //  U vector: along radial direction
    //  V vector: W x U
    //  W vector: pos x vel
    pub fn PosVelToUUVW(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        uvec: *mut [f64; 3],
        vVec: *mut [f64; 3],
        wVec: *mut [f64; 3],
    );
    //  Converts position and velocity vectors to U, V, W vectors. See the remarks section for details.
    //  The resulting vectors have the following meanings.
    //  U vector: V x W
    //  V vector: along velocity direction
    //  W vector: pos x vel
    pub fn PosVelToPTW(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        uvec: *mut [f64; 3],
        vVec: *mut [f64; 3],
        wVec: *mut [f64; 3],
    );
    //  Solves Kepler's equation (M = E - e sin(E)) for the eccentric anomaly, E, by iteration.
    pub fn SolveKepEqtn(xa_kep: *const [f64; 6]) -> f64;
    //  Computes true anomaly from a set of Keplerian elements.
    pub fn CompTrueAnomaly(xa_kep: *const [f64; 6]) -> f64;
    //  Converts mean motion N to semi-major axis A.
    pub fn NToA(n: f64) -> f64;
    //  Converts semi-major axis A to mean motion N.
    pub fn AToN(a: f64) -> f64;
    //  Converts Kozai mean motion to Brouwer mean motion.
    //  SGP TLE's use Kozai mean motion while SGP4/SGP4-XP TLE's use Brouwer mean motion.
    pub fn KozaiToBrouwer(eccen: f64, incli: f64, nKozai: f64) -> f64;
    //  Converts Brouwer mean motion to Kozai mean motion.
    //  SGP TLE's use Kozai mean motion while SGP4/SGP4-XP TLE's use Brouwer mean motion.
    pub fn BrouwerToKozai(eccen: f64, incli: f64, nBrouwer: f64) -> f64;
    //  Converts a set of osculating Keplerian elements to a set of mean Keplerian elements using method 9 algorithm.
    pub fn KepOscToMean(xa_OscKep: *const [f64; 6], xa_MeanKep: *mut [f64; 6]);
    //  Converts an ECI position vector XYZ to geodetic latitude, longitude, and height.
    pub fn XYZToLLH(thetaG: f64, metricPos: *const [f64; 3], metricLLH: *mut [f64; 3]);
    //  Converts an ECI position vector XYZ to geodetic latitude, longitude, and height at the specified time.
    pub fn XYZToLLHTime(ds50UTC: f64, metricPos: *const [f64; 3], metricLLH: *mut [f64; 3]);
    //  Converts geodetic latitude, longitude, and height to an ECI position vector XYZ.
    pub fn LLHToXYZ(thetaG: f64, metricLLH: *const [f64; 3], metricXYZ: *mut [f64; 3]);
    //  Converts geodetic latitude, longitude, and height to an ECI position vector XYZ at the specified time.
    pub fn LLHToXYZTime(ds50UTC: f64, metricLLH: *const [f64; 3], metricXYZ: *mut [f64; 3]);
    //  Converts EFG position and velocity vectors to ECI position and velocity vectors.
    pub fn EFGToECI(
        thetaG: f64,
        posEFG: *const [f64; 3],
        velEFG: *const [f64; 3],
        posECI: *mut [f64; 3],
        velECI: *mut [f64; 3],
    );
    //  Converts EFG position and velocity vectors to ECI position and velocity vectors at the specified time.
    pub fn EFGToECITime(
        ds50UTC: f64,
        posEFG: *const [f64; 3],
        velEFG: *const [f64; 3],
        posECI: *mut [f64; 3],
        velECI: *mut [f64; 3],
    );
    //  Converts ECI position and velocity vectors to EFG position and velocity vectors.
    pub fn ECIToEFG(
        thetaG: f64,
        posECI: *const [f64; 3],
        velECI: *const [f64; 3],
        posEFG: *mut [f64; 3],
        velEFG: *mut [f64; 3],
    );
    //  Converts ECI position and velocity vectors to EFG position and velocity vectors at the specified time.
    pub fn ECIToEFGTime(
        ds50UTC: f64,
        posECI: *const [f64; 3],
        velECI: *const [f64; 3],
        posEFG: *mut [f64; 3],
        velEFG: *mut [f64; 3],
    );
    //  Converts ECR position and velocity vectors to EFG position and velocity vectors.
    pub fn ECRToEFG(
        polarX: f64,
        polarY: f64,
        posECR: *const [f64; 3],
        velECR: *const [f64; 3],
        posEFG: *mut [f64; 3],
        velEFG: *mut [f64; 3],
    );
    //  Converts ECR position and velocity vectors to EFG position and velocity vectors at the specified time.
    pub fn ECRToEFGTime(
        ds50UTC: f64,
        posECR: *const [f64; 3],
        velECR: *const [f64; 3],
        posEFG: *mut [f64; 3],
        velEFG: *mut [f64; 3],
    );
    //  Converts EFG position and velocity vectors to ECR position and velocity vectors.
    pub fn EFGToECR(
        polarX: f64,
        polarY: f64,
        posEFG: *const [f64; 3],
        velEFG: *const [f64; 3],
        posECR: *mut [f64; 3],
        velECR: *mut [f64; 3],
    );
    //  Converts EFG position and velocity vectors to ECR position and velocity vectors at the specified time.
    pub fn EFGToECRTime(
        ds50UTC: f64,
        posEFG: *const [f64; 3],
        velEFG: *const [f64; 3],
        posECR: *mut [f64; 3],
        velECR: *mut [f64; 3],
    );
    //  Converts an EFG position vector to geodetic latitude, longitude, and height.
    pub fn EFGPosToLLH(posEFG: *const [f64; 3], metricLLH: *mut [f64; 3]);
    //  Converts geodetic latitude, longitude, and height to an EFG position vector.
    pub fn LLHToEFGPos(metricLLH: *const [f64; 3], posEFG: *mut [f64; 3]);
    //  Rotates position and velocity vectors from J2000 to coordinates of the specified date, expressed in ds50TAI.
    pub fn RotJ2KToDate(
        spectr: i32,
        nutationTerms: i32,
        ds50TAI: f64,
        posJ2K: *const [f64; 3],
        velJ2K: *const [f64; 3],
        posDate: *mut [f64; 3],
        velDate: *mut [f64; 3],
    );
    //  Rotates position and velocity vectors from coordinates of date to J2000.
    pub fn RotDateToJ2K(
        spectr: i32,
        nutationTerms: i32,
        ds50TAI: f64,
        posDate: *const [f64; 3],
        velDate: *const [f64; 3],
        posJ2K: *mut [f64; 3],
        velJ2K: *mut [f64; 3],
    );
    //  Computes the Sun and Moon position at the specified time.
    pub fn CompSunMoonPos(
        ds50ET: f64,
        uvecSun: *mut [f64; 3],
        sunVecMag: *mut f64,
        uvecMoon: *mut [f64; 3],
        moonVecMag: *mut f64,
    );
    //  Computes the Sun position at the specified time.
    pub fn CompSunPos(ds50ET: f64, uvecSun: *mut [f64; 3], sunVecMag: *mut f64);
    //  Computes the Moon position at the specified time.
    pub fn CompMoonPos(ds50ET: f64, uvecMoon: *mut [f64; 3], moonVecMag: *mut f64);
    //  This function is intended for future use.  No information is currently available.
    //  This function is intended for future use.  No information is currently available.
    pub fn AstroConvFrTo(xf_Conv: i32, frArr: *const [f64; 128], toArr: *mut [f64; 128]);
    //  Converts right ascension and declination to vector triad LAD in topocentric equatorial coordinate system.
    pub fn RADecToLAD(ra: f64, dec: f64, l: *mut [f64; 3], a_tilde: *mut [f64; 3], d_tilde: *mut [f64; 3]);
    //  Converts azimuth and elevation to vector triad LAD in topocentric horizontal coordinate system.
    pub fn AzElToLAD(az: f64, el: f64, lh: *mut [f64; 3], ah: *mut [f64; 3], dh: *mut [f64; 3]);
    //  Converts satellite ECI position/velocity vectors and sensor location to topocentric components.
    //  The xa_topo array has the following structure:
    //  [0]: Resulting right ascension (RA) (deg)
    //  [1]: Declination (deg)
    //  [2]: Azimuth (deg)
    //  [3]: Elevation (deg)
    //  [4]: Range (km)
    //  [5]: RAdot (first derivative of right ascension) (deg/s)
    //  [6]: DecDot (first derivative of declination) (deg/s)
    //  [7]: AzDot (first derivative of azimuth) (deg/s)
    //  [8]: ElDot (first derivative of elevation) (deg/s)
    //  [9]: RangeDot (first derivative of range) (km/s)
    pub fn ECIToTopoComps(
        theta: f64,
        lat: f64,
        senPos: *const [f64; 3],
        satPos: *const [f64; 3],
        satVel: *const [f64; 3],
        xa_topo: *mut [f64; 10],
    );
    //  Converts right ascension and declination in the topocentric reference frame to Azimuth/Elevation in the local horizon reference frame.
    pub fn RaDecToAzEl(thetaG: f64, lat: f64, lon: f64, ra: f64, dec: f64, az: *mut f64, el: *mut f64);
    //  Converts right ascension and declination in the topocentric reference frame to Azimuth/Elevation in the local horizon reference frame.
    pub fn RaDecToAzElTime(ds50UTC: f64, lat: f64, lon: f64, ra: f64, dec: f64, az: *mut f64, el: *mut f64);
    //  Converts Azimuth/Elevation in the local horizon reference frame to Right ascension/Declination in the topocentric reference frame
    pub fn AzElToRaDec(thetaG: f64, lat: f64, lon: f64, az: f64, el: f64, ra: *mut f64, dec: *mut f64);
    //  Converts Azimuth/Elevation in the local horizon reference frame to Right ascension/Declination in the topocentric reference frame
    pub fn AzElToRaDecTime(ds50UTC: f64, lat: f64, lon: f64, az: f64, el: f64, ra: *mut f64, dec: *mut f64);
    //  Converts full state RAE (range, az, el, and their rates) to full state ECI (position and velocity)
    //  The xa_rae array has the following structure:
    //  [0]: Range (km)
    //  [1]: Azimuth (deg)
    //  [2]: Elevation (deg)
    //  [3]: Range Dot (km/s)
    //  [4]: Azimuth Dot (deg/s)
    //  [5]: Elevation Dot (deg/s)
    pub fn RAEToECI(
        theta: f64,
        astroLat: f64,
        xa_rae: *const [f64; 6],
        senPos: *const [f64; 3],
        satPos: *mut [f64; 3],
        satVel: *mut [f64; 3],
    );
    //  Computes initial values for the SGP drag term nDot and the SGP4 drag term BSTAR based upon eccentricity and semi-major axis.
    pub fn GetInitialDrag(semiMajorAxis: f64, eccen: f64, nDot: *mut f64, bstar: *mut f64);
    //  Converts covariance matrix PTW to UVW.
    //  PTW = P: TxW, T: along velocity direction, W: pos x vel.
    //  UVW = U: radial direction, V: in plane, perpendicular to U, W: pos x vel.
    pub fn CovMtxPTWToUVW(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        ptwCovMtx: *const [[f64; 6]; 6],
        uvwCovMtx: *mut [[f64; 6]; 6],
    );
    //  Converts covariance matrix UVW to PTW.
    //  PTW = P: TxW, T: along velocity direction, W: pos x vel.
    //  UVW = U: radial direction, V: in plane, perpendicular to U, W: pos x vel.
    pub fn CovMtxUVWToPTW(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        uvwCovMtx: *const [[f64; 6]; 6],
        ptwCovMtx: *mut [[f64; 6]; 6],
    );
    //  Computes Earth/Sensor/Earth Limb and Earth/Sensor/Satellite angles.
    pub fn EarthObstructionAngles(
        earthLimb: f64,
        satECI: *const [f64; 3],
        senECI: *const [f64; 3],
        earthSenLimb: *mut f64,
        earthSenSat: *mut f64,
        satEarthSen: *mut f64,
    );
    //  Determines if a point in space is sunlit at the input time ds50ET
    pub fn IsPointSunlit(ds50ET: f64, ptEci: *const [f64; 3]) -> i32;
    //  Rotates Right Ascension and Declination to specified epoch
    pub fn RotRADecl(
        nutationTerms: i32,
        dir: i32,
        ds50UTCIn: f64,
        raIn: f64,
        declIn: f64,
        ds50UTCOut: f64,
        raOut: *mut f64,
        declOut: *mut f64,
    );
    //  Rotates Right Ascension and Declination from TEME of Date to MEME of the specified year of equinox
    pub fn RotRADec_DateToEqnx(
        nutationTerms: i32,
        yrOfEqnx: i32,
        ds50UTCIn: f64,
        raIn: f64,
        declIn: f64,
        raOut: *mut f64,
        declOut: *mut f64,
    ) -> i32;
    //  Rotates Right Ascension and Declination from MEME of the specified year of equinox to TEME of Date
    pub fn RotRADec_EqnxToDate(
        nutationTerms: i32,
        yrOfEqnx: i32,
        ds50UTCIn: f64,
        raIn: f64,
        declIn: f64,
        raOut: *mut f64,
        declOut: *mut f64,
    ) -> i32;
    //  Rotates the Equinoctial covariance to UVW
    //  Note: This method uses the global Earth constants so make sure that you select the right Earth model by calling the EnvConst/EnvSetGeoIdx method
    //  The n terms must be normalized by n
    //  The input position, velocity and covariance must all have the same reference equator and equinox.
    pub fn CovMtxEqnxToUVW(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        covMtxEqnx: *const [[f64; 6]; 6],
        covMtxUVW: *mut [[f64; 6]; 6],
    );
    //  Rotates the UVW covariance to Equinoctial
    //  Note: This method uses the global Earth constants so make sure that you select the right Earth model by calling the EnvConst/EnvSetGeoIdx method
    //  The n terms are normalized by n
    //  The input position, velocity reference equator and equinox determine the output covariance reference frame.
    pub fn CovMtxUVWToEqnx(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        covMtxUVW: *const [[f64; 6]; 6],
        covMtxEqnx: *mut [[f64; 6]; 6],
    );
    //  Rotates the ECI covariance to UVW
    //  Note: This method uses the global Earth constants so make sure that you select the proper Earth model by calling the EnvConst/EnvSetGeoIdx method
    pub fn CovMtxECIToUVW(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        covMtxECI: *const [[f64; 6]; 6],
        covMtxUVW: *mut [[f64; 6]; 6],
    );
    //  Rotates the UVW covariance to ECI
    //  Note: This method uses the global Earth constants so make sure that you select the proper Earth model by calling the EnvConst/EnvSetGeoIdx method
    pub fn CovMtxUVWToECI(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        covMtxUVW: *const [[f64; 6]; 6],
        covMtxECI: *mut [[f64; 6]; 6],
    );
    //  Converts covariance matrix ECI to EFG.
    //  EFG = Earth Fixed Greenwich
    //  ECI = Earth Centered Inertial - need to determine TEME or J2K
    pub fn CovMtxECIToEFG(thetaG: f64, covECI: *const [[f64; 6]; 6], covEFG: *mut [[f64; 6]; 6]);
    //  Converts covariance matrix EFG to ECI.
    //  EFG = Earth Fixed Greenwich
    //  ECI = Earth Centered Inertial - need to determine TEME or J2K
    pub fn CovMtxEFGToECI(thetaG: f64, covEFG: *const [[f64; 6]; 6], covECI: *mut [[f64; 6]; 6]);
    //  Converts 6x6 symmetric Matrix/2D array to 1D array of 21 elements (lower triangular of a 6x6 symmetric matrix)
    pub fn Mtx6x6ToLTA21(symMtx6x6: *const [[f64; 6]; 6], lta21: *mut [f64; 21]);
    //  Converts 1D array of 21 elements (lower triangular of a 6x6 symmetric matrix) to a 6x6 symmetric matrix
    pub fn LTA21ToMtx6x6(lta21: *const [f64; 21], symMtx6x6: *mut [[f64; 6]; 6]);
    //  Converts 9x9 symmetric Matrix/2D array to 1D array of 45 elements (lower triangular of a 9x9 symmetric matrix)
    pub fn Mtx9x9ToLTA45(symMtx9x9: *const [[f64; 9]; 9], lta45: *mut [f64; 45]);
    //  Converts 1D array of 45 elements (lower triangular of a 9x9 symmetric matrix) to a 9x9 symmetric matrix
    pub fn LTA45ToMtx9x9(lta45: *const [f64; 45], symMtx9x9: *mut [[f64; 9]; 9]);
    //  Propagate xyzDate covariance forward to the propagation time
    pub fn PropCovFrState(
        rms: f64,
        consider: f64,
        stateArray: *const [f64; 54],
        cov: *const [[f64; 9]; 9],
        propCov: *mut [[f64; 6]; 6],
    );
    //  Rotates the ECI covariance to UVW
    //  Note: This method uses the global Earth constants so make sure that you select the proper Earth model by calling the EnvConst/EnvSetGeoIdx method
    pub fn CovMtxECIToEqnx(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        covMtxECI: *const [[f64; 9]; 9],
        covMtxEqnx: *mut [[f64; 9]; 9],
    );
    //  Rotates the UVW covariance to ECI
    //  Note: This method uses the global Earth constants so make sure that you select the proper Earth model by calling the EnvConst/EnvSetGeoIdx method
    pub fn CovMtxEqnxToECI9x9(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        covEqnx: *const [[f64; 9]; 9],
        covMtxECI: *mut [[f64; 9]; 9],
    );
    //  Rotates the UVW covariance to ECI
    //  Note: This method uses the global Earth constants so make sure that you select the proper Earth model by calling the EnvConst/EnvSetGeoIdx method
    pub fn CovMtxEqnxToUVW9x9(
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        covEqnx: *const [[f64; 9]; 9],
        covMtxUVW: *mut [[f64; 9]; 9],
    );
    //  Update (propagate) covariance to a future time with a supplied covariance, state transition matrix
    //  consider parameter and RMS. Consider parameter is applied to the drag term only.
    //  Full covariance matrix is multiplied by RMS squared.  State transition matrix can be obtained from
    //  SpProp.SpGetStateMtx or supplying your own. State matrix, input and output covariance must be in
    //  matching coordinate systems.
    pub fn CovMtxUpdate(
        rmsIn: f64,
        consider: f64,
        cov: *const [[f64; 9]; 9],
        stateArray: *const [f64; 54],
        propCov: *mut [[f64; 6]; 6],
    );
    //  Annual Aberration calculated using equations from Astronomical Algorithms, Jean Meeus, 2nd Edition with Corrections as of June 15, 2005
    pub fn AberrationAnnual(ra: f64, decl: f64, dS50UTC: f64, raDelta: *mut f64, decDelta: *mut f64);
    //  Diurnal Aberration is due to the rotation of the Earth about it's axis. This is only valid for ground based sensors.
    //  Diurnal Aberration calculated using equations from Explanatory Supplement to the Astronomical Almanac 3rd Edition, 2013
    pub fn AberrationDiurnal(
        ra: f64,
        decl: f64,
        dS50UTC: f64,
        senPos: *const [f64; 3],
        raDelta: *mut f64,
        decDelta: *mut f64,
    );
    //  Sets JPL parameters
    //  Notes: Set JPL parameters will be used by SP, SPG4-XP, and anything that requires access to JPL data
    pub fn JplSetParameters(jplFile: *const c_char, ds50Start: f64, ds50Stop: f64);
    //  Gets JPL parameters
    pub fn JplGetParameters(jplFile: *const c_char, ds50Start: *mut f64, ds50Stop: *mut f64);
    //  Resets JPL parameters & removes JPL ephemeris data
    pub fn JplReset();
    //  Computes various Sun and Moon vectors base on loaded JPL data at the specified time.
    //  Note: if JPL data isn't loaded or available, all output parameters are set to zero
    pub fn JplCompSunMoonVec(
        ds50UTC: f64,
        uvecSun: *mut [f64; 3],
        sunVecMag: *mut f64,
        uvecMoon: *mut [f64; 3],
        moonVecMag: *mut f64,
    );
    //  Computes Sun and Moon position vectors base on loaded JPL data at the specified time.
    //  Note: if JPL data isn't loaded or available, all output parameters are set to zero
    pub fn JplCompSunMoonPos(ds50UTC: f64, sunVec: *mut [f64; 3], moonVec: *mut [f64; 3]);
    //  Removes the JPL ephemeris from memory
    pub fn RemoveJpl();
    //  Rotates position and velocity vectors from TEME of Epoch to TEME of Date
    pub fn TemeEpochToDate(
        nutationTerms: i32,
        epochDs50TAI: f64,
        dateDs50TAI: f64,
        posEpoch: *const [f64; 3],
        velEpoch: *const [f64; 3],
        posDate: *mut [f64; 3],
        velDate: *mut [f64; 3],
    );
}

// Index of Keplerian elements
// semi-major axis (km)
pub const XA_KEP_A: usize = 0;
// eccentricity (unitless)
pub const XA_KEP_E: usize = 1;
// inclination (deg)
pub const XA_KEP_INCLI: usize = 2;
// mean anomaly (deg)
pub const XA_KEP_MA: usize = 3;
// right ascension of the asending node (deg)
pub const XA_KEP_NODE: usize = 4;
// argument of perigee (deg)
pub const XA_KEP_OMEGA: usize = 5;
pub static XA_KEP_SIZE: usize = 6;

// Index of classical elements
// N mean motion (revs/day)
pub static XA_CLS_N: usize = 0;
// eccentricity (unitless)
pub static XA_CLS_E: usize = 1;
// inclination (deg)
pub static XA_CLS_INCLI: usize = 2;
// mean anomaly (deg)
pub static XA_CLS_MA: usize = 3;
// right ascension of the asending node (deg)
pub static XA_CLS_NODE: usize = 4;
// argument of perigee (deg)
pub static XA_CLS_OMEGA: usize = 5;
pub static XA_CLS_SIZE: usize = 6;

// Index of equinoctial elements
// Af (unitless)
pub const XA_EQNX_AF: usize = 0;
// Ag (unitless)
pub const XA_EQNX_AG: usize = 1;
// chi (unitless)
pub const XA_EQNX_CHI: usize = 2;
// psi (unitless)
pub const XA_EQNX_PSI: usize = 3;
// L mean longitude (deg)
pub const XA_EQNX_L: usize = 4;
// N mean motion (revs/day)
pub const XA_EQNX_N: usize = 5;
pub const XA_EQNX_SIZE: usize = 6;

// Indexes of AstroConvFrTo
// SGP4 (A, E, Incli, BStar) to SGP (nDot, n2Dot)
pub static XF_CONV_SGP42SGP: i32 = 101;

// Indexes for topocentric components
// Right ascension (deg)
pub static XA_TOPO_RA: usize = 0;
// Declination (deg)
pub static XA_TOPO_DEC: usize = 1;
// Azimuth (deg)
pub static XA_TOPO_AZ: usize = 2;
// Elevation (deg)
pub static XA_TOPO_EL: usize = 3;
// Range (km)
pub static XA_TOPO_RANGE: usize = 4;
// Right ascension dot (deg/s)
pub static XA_TOPO_RADOT: usize = 5;
// Declincation dot (deg/s)
pub static XA_TOPO_DECDOT: usize = 6;
// Azimuth dot (deg/s)
pub static XA_TOPO_AZDOT: usize = 7;
// Elevation dot (deg/s)
pub static XA_TOPO_ELDOT: usize = 8;
// Range dot (km/s)
pub static XA_TOPO_RANGEDOT: usize = 9;
pub static XA_TOPO_SIZE: usize = 10;

// Indexes for RAE components
// Range (km)
pub static XA_RAE_RANGE: usize = 0;
// Azimuth (deg)
pub static XA_RAE_AZ: usize = 1;
// Elevation (deg)
pub static XA_RAE_EL: usize = 2;
// Range dot (km/s)
pub static XA_RAE_RANGEDOT: usize = 3;
// Azimuth dot (deg/s)
pub static XA_RAE_AZDOT: usize = 4;
// Elevation dot (deg/s)
pub static XA_RAE_ELDOT: usize = 5;
pub static XA_RAE_SIZE: usize = 6;

// Year of Equinox indicator
// Date of observation
pub static YROFEQNX_OBTIME: isize = 0;
// 0 Jan of Date
pub static YROFEQNX_CURR: isize = 1;
// J2000
pub static YROFEQNX_2000: isize = 2;
// B1950
pub static YROFEQNX_1950: isize = 3;

// ========================= End of auto generated code ==========================

pub fn get_dll_info() -> String {
    let mut info = GetSetString::new();
    unsafe {
        AstroFuncGetInfo(info.pointer());
    }
    info.value()
}

pub fn sma_to_mean_motion(semi_major_axis: f64) -> f64 {
    unsafe { AToN(semi_major_axis) }
}

pub fn keplerian_to_cartesian(xa_kep: &[f64; XA_KEP_SIZE]) -> [f64; 6] {
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    unsafe {
        KepToPosVel(xa_kep, &mut pos, &mut vel);
    }
    [pos[0], pos[1], pos[2], vel[0], vel[1], vel[2]]
}

pub fn cartesian_to_keplerian(posvel: &[f64; 6]) -> [f64; XA_KEP_SIZE] {
    let mut xa_kep = [0.0; XA_KEP_SIZE];
    let pos = [posvel[0], posvel[1], posvel[2]];
    let vel = [posvel[3], posvel[4], posvel[5]];
    unsafe {
        PosVelToKep(&pos, &vel, &mut xa_kep);
    }
    xa_kep
}

pub fn set_jpl_ephemeris_file_path(file_path: &str) {
    let mut jpl_path: GetSetString = file_path.into();
    let ds50_start = time::year_doy_to_ds50(1960, 1.0);
    let ds50_stop = time::year_doy_to_ds50(2050, 1.0);
    unsafe {
        JplSetParameters(jpl_path.pointer(), ds50_start, ds50_stop);
    }
}

pub fn j2000_to_teme(ds50_utc: f64, j2000_posvel: &[f64; 6]) -> [f64; 6] {
    let mut pos_teme = [0.0; 3];
    let mut vel_teme = [0.0; 3];
    let pos_j2000 = [j2000_posvel[0], j2000_posvel[1], j2000_posvel[2]];
    let vel_j2000 = [j2000_posvel[3], j2000_posvel[4], j2000_posvel[5]];
    let ds50_tai = time::utc_to_tai(ds50_utc);
    unsafe {
        RotJ2KToDate(0, 106, ds50_tai, &pos_j2000, &vel_j2000, &mut pos_teme, &mut vel_teme);
    }
    [
        pos_teme[0],
        pos_teme[1],
        pos_teme[2],
        vel_teme[0],
        vel_teme[1],
        vel_teme[2],
    ]
}

pub fn j2000_to_efg(ds50_utc: f64, j2000_posvel: &[f64; 6]) -> [f64; 6] {
    teme_to_efg(ds50_utc, &j2000_to_teme(ds50_utc, j2000_posvel))
}

pub fn j2000_to_ecr(ds50_utc: f64, j2000_posvel: &[f64; 6]) -> [f64; 6] {
    teme_to_ecr(ds50_utc, &j2000_to_teme(ds50_utc, j2000_posvel))
}

pub fn teme_to_j2000(ds50_utc: f64, teme_posvel: &[f64; 6]) -> [f64; 6] {
    let mut pos_j2000 = [0.0; 3];
    let mut vel_j2000 = [0.0; 3];
    let ds50_tai = time::utc_to_tai(ds50_utc);
    let pos = [teme_posvel[0], teme_posvel[1], teme_posvel[2]];
    let vel = [teme_posvel[3], teme_posvel[4], teme_posvel[5]];
    unsafe {
        RotDateToJ2K(0, 106, ds50_tai, &pos, &vel, &mut pos_j2000, &mut vel_j2000);
    }
    [
        pos_j2000[0],
        pos_j2000[1],
        pos_j2000[2],
        vel_j2000[0],
        vel_j2000[1],
        vel_j2000[2],
    ]
}

pub fn teme_to_efg(ds50_utc: f64, teme_posvel: &[f64; 6]) -> [f64; 6] {
    let mut pos_efg = [0.0; 3];
    let mut vel_efg = [0.0; 3];
    let pos = [teme_posvel[0], teme_posvel[1], teme_posvel[2]];
    let vel = [teme_posvel[3], teme_posvel[4], teme_posvel[5]];
    unsafe {
        ECIToEFGTime(ds50_utc, &pos, &vel, &mut pos_efg, &mut vel_efg);
    }
    [pos_efg[0], pos_efg[1], pos_efg[2], vel_efg[0], vel_efg[1], vel_efg[2]]
}

pub fn efg_to_ecr(ds50_utc: f64, efg_posvel: &[f64; 6]) -> [f64; 6] {
    let mut pos_ecr = [0.0; 3];
    let mut vel_ecr = [0.0; 3];
    let pos_efg = [efg_posvel[0], efg_posvel[1], efg_posvel[2]];
    let vel_efg = [efg_posvel[3], efg_posvel[4], efg_posvel[5]];
    unsafe {
        EFGToECRTime(ds50_utc, &pos_efg, &vel_efg, &mut pos_ecr, &mut vel_ecr);
    }
    [pos_ecr[0], pos_ecr[1], pos_ecr[2], vel_ecr[0], vel_ecr[1], vel_ecr[2]]
}

pub fn teme_to_ecr(ds50_utc: f64, teme_posvel: &[f64; 6]) -> [f64; 6] {
    efg_to_ecr(ds50_utc, &teme_to_efg(ds50_utc, teme_posvel))
}

pub fn ecr_to_efg(ds50_utc: f64, ecr_posvel: &[f64; 6]) -> [f64; 6] {
    let mut pos_efg = [0.0; 3];
    let mut vel_efg = [0.0; 3];
    let pos_ecr = [ecr_posvel[0], ecr_posvel[1], ecr_posvel[2]];
    let vel_ecr = [ecr_posvel[3], ecr_posvel[4], ecr_posvel[5]];
    unsafe {
        ECRToEFGTime(ds50_utc, &pos_ecr, &vel_ecr, &mut pos_efg, &mut vel_efg);
    }
    [pos_efg[0], pos_efg[1], pos_efg[2], vel_efg[0], vel_efg[1], vel_efg[2]]
}

pub fn efg_to_teme(ds50_utc: f64, efg_posvel: &[f64; 6]) -> [f64; 6] {
    let mut pos_teme = [0.0; 3];
    let mut vel_teme = [0.0; 3];
    let pos_efg = [efg_posvel[0], efg_posvel[1], efg_posvel[2]];
    let vel_efg = [efg_posvel[3], efg_posvel[4], efg_posvel[5]];
    unsafe {
        EFGToECITime(ds50_utc, &pos_efg, &vel_efg, &mut pos_teme, &mut vel_teme);
    }
    [
        pos_teme[0],
        pos_teme[1],
        pos_teme[2],
        vel_teme[0],
        vel_teme[1],
        vel_teme[2],
    ]
}

pub fn ecr_to_teme(ds50_utc: f64, ecr_posvel: &[f64; 6]) -> [f64; 6] {
    efg_to_teme(ds50_utc, &ecr_to_efg(ds50_utc, ecr_posvel))
}

pub fn ecr_to_j2000(ds50_utc: f64, ecr_posvel: &[f64; 6]) -> [f64; 6] {
    teme_to_j2000(ds50_utc, &ecr_to_teme(ds50_utc, ecr_posvel))
}

pub fn efg_to_j2000(ds50_utc: f64, efg_posvel: &[f64; 6]) -> [f64; 6] {
    teme_to_j2000(ds50_utc, &efg_to_teme(ds50_utc, efg_posvel))
}

pub fn kozai_to_brouwer(eccentricity: f64, inclination: f64, mean_motion: f64) -> f64 {
    unsafe { KozaiToBrouwer(eccentricity, inclination, mean_motion) }
}

pub fn brouwer_to_kozai(eccentricity: f64, inclination: f64, mean_motion: f64) -> f64 {
    unsafe { BrouwerToKozai(eccentricity, inclination, mean_motion) }
}

pub fn mean_motion_to_sma(mean_motion: f64) -> f64 {
    unsafe { NToA(mean_motion) }
}

pub fn lla_to_teme(ds50_utc: f64, pos_lla: &[f64; 3]) -> [f64; 3] {
    let mut pos_teme = [0.0; 3];
    unsafe {
        LLHToXYZTime(ds50_utc, pos_lla, &mut pos_teme);
    }
    pos_teme
}

pub fn topo_meme_to_teme(yr_of_equinox: i32, ds50_utc: f64, ra: f64, dec: f64) -> (f64, f64) {
    let mut ra_out = 0.0;
    let mut dec_out = 0.0;
    unsafe {
        RotRADec_EqnxToDate(106, yr_of_equinox, ds50_utc, ra, dec, &mut ra_out, &mut dec_out);
    }
    (ra_out, dec_out)
}

pub fn topo_teme_to_meme(yr_of_equinox: i32, ds50_utc: f64, ra: f64, dec: f64) -> (f64, f64) {
    let mut ra_out = 0.0;
    let mut dec_out = 0.0;
    unsafe {
        RotRADec_DateToEqnx(106, yr_of_equinox, ds50_utc, ra, dec, &mut ra_out, &mut dec_out);
    }
    (ra_out, dec_out)
}

pub fn osculating_to_mean(xa_osc: &[f64; XA_KEP_SIZE]) -> [f64; XA_KEP_SIZE] {
    let mut xa_mean = [0.0; XA_KEP_SIZE];
    unsafe {
        KepOscToMean(xa_osc, &mut xa_mean);
    }
    xa_mean
}

pub fn equinoctial_to_keplerian(xa_eqnx: &[f64; XA_EQNX_SIZE]) -> [f64; XA_KEP_SIZE] {
    let mut xa_kep = [0.0; XA_KEP_SIZE];
    unsafe {
        EqnxToKep(xa_eqnx, &mut xa_kep);
    }
    xa_kep
}

pub fn keplerian_to_equinoctial(xa_kep: &[f64; XA_KEP_SIZE]) -> [f64; XA_EQNX_SIZE] {
    let mut xa_eqnx = [0.0; XA_EQNX_SIZE];
    unsafe {
        KepToEqnx(xa_kep, &mut xa_eqnx);
    }
    xa_eqnx
}

pub fn covariance_equinoctial_to_uvw(teme_posvel: &[f64; 6], cov_eqnx: &[[f64; 6]; 6]) -> [[f64; 6]; 6] {
    let mut cov_uvw = [[0.0; 6]; 6];
    let pos = [teme_posvel[0], teme_posvel[1], teme_posvel[2]];
    let vel = [teme_posvel[3], teme_posvel[4], teme_posvel[5]];
    unsafe {
        CovMtxEqnxToUVW(&pos, &vel, cov_eqnx, &mut cov_uvw);
    }
    cov_uvw
}

pub fn covariance_uvw_to_teme(teme_posvel: &[f64; 6], cov_uvw: &[[f64; 6]; 6]) -> [[f64; 6]; 6] {
    let mut cov_teme = [[0.0; 6]; 6];
    let pos = [teme_posvel[0], teme_posvel[1], teme_posvel[2]];
    let vel = [teme_posvel[3], teme_posvel[4], teme_posvel[5]];
    unsafe {
        CovMtxUVWToECI(&pos, &vel, cov_uvw, &mut cov_teme);
    }
    cov_teme
}

pub fn gst_ra_dec_to_az_el(gst: f64, lla: &[f64; 3], ra: f64, dec: f64) -> [f64; 2] {
    let mut az = 0.0;
    let mut el = 0.0;
    unsafe {
        RaDecToAzEl(gst, lla[0], lla[1], ra, dec, &mut az, &mut el);
    }

    [az, el]
}

pub fn time_ra_dec_to_az_el(ds50_utc: f64, lla: &[f64; 3], ra: f64, dec: f64) -> [f64; 2] {
    let mut az = 0.0;
    let mut el = 0.0;
    unsafe {
        RaDecToAzElTime(ds50_utc, lla[0], lla[1], ra, dec, &mut az, &mut el);
    }

    [az, el]
}

pub fn horizon_to_teme(
    lst: f64,
    lat: f64,
    sensor_teme: &[f64; 3],
    xa_rae: &[f64; XA_RAE_SIZE],
) -> Result<[f64; 6], String> {
    let mut teme_pos = [0.0; 3];
    let mut teme_vel = [0.0; 3];

    unsafe {
        RAEToECI(lst, lat, xa_rae, sensor_teme, &mut teme_pos, &mut teme_vel);
    }

    if teme_pos.iter().all(|&x| x == 0.0) && teme_vel.iter().all(|&x| x == 0.0) {
        Err(get_last_error_message())
    } else {
        Ok([
            teme_pos[0],
            teme_pos[1],
            teme_pos[2],
            teme_vel[0],
            teme_vel[1],
            teme_vel[2],
        ])
    }
}

pub fn gst_teme_to_lla(gst: f64, teme_pos: &[f64; 3]) -> [f64; 3] {
    let mut pos_lla = [0.0; 3];
    unsafe {
        XYZToLLH(gst, teme_pos, &mut pos_lla);
    }
    pos_lla
}

pub fn time_teme_to_lla(ds50_utc: f64, teme_pos: &[f64; 3]) -> [f64; 3] {
    let mut pos_lla = [0.0; 3];
    unsafe {
        XYZToLLHTime(ds50_utc, teme_pos, &mut pos_lla);
    }
    pos_lla
}

pub fn efg_to_lla(efg_pos: &[f64; 3]) -> Result<[f64; 3], String> {
    let mut pos_lla = [0.0; 3];
    if efg_pos.iter().all(|&x| x == 0.0) {
        return Err("Input EFG position is zero vector.".to_string());
    }
    unsafe {
        EFGPosToLLH(efg_pos, &mut pos_lla);
    }
    if pos_lla.iter().all(|&x| x == 0.0) {
        Err(get_last_error_message())
    } else {
        Ok(pos_lla)
    }
}

pub fn teme_to_topo(
    lst: f64,
    lat: f64,
    sen_teme_pos: &[f64; 3],
    sat_teme_posvel: &[f64; 6],
) -> Result<[f64; XA_TOPO_SIZE], String> {
    let mut xa_topo = [0.0; XA_TOPO_SIZE];
    let sat_pos = [sat_teme_posvel[0], sat_teme_posvel[1], sat_teme_posvel[2]];
    let sat_vel = [sat_teme_posvel[3], sat_teme_posvel[4], sat_teme_posvel[5]];
    unsafe {
        ECIToTopoComps(lst, lat, sen_teme_pos, &sat_pos, &sat_vel, &mut xa_topo);
    }

    if xa_topo.iter().all(|&x| x == 0.0) {
        Err(get_last_error_message())
    } else {
        Ok(xa_topo)
    }
}

pub fn get_jpl_sun_and_moon_position(ds50utc: f64) -> ([f64; 3], [f64; 3]) {
    let mut sun_pos = [0.0; 3];
    let mut moon_pos = [0.0; 3];
    unsafe {
        JplCompSunMoonPos(ds50utc, &mut sun_pos, &mut moon_pos);
    }
    (sun_pos, moon_pos)
}

pub fn point_is_sunlit(ds50_tt: f64, teme_pos: &[f64; 3]) -> bool {
    unsafe { IsPointSunlit(ds50_tt, teme_pos) == 1 }
}

pub fn get_earth_obstruction_angles(sat_teme_pos: &[f64; 3], sensor_teme_pos: &[f64; 3]) -> (f64, f64, f64) {
    let mut earth_sensor_limb = 0.0;
    let mut earth_sensor_sat = 0.0;
    let mut sat_earth_sensor = 0.0;
    unsafe {
        EarthObstructionAngles(
            environment::get_earth_radius(),
            sat_teme_pos,
            sensor_teme_pos,
            &mut earth_sensor_limb,
            &mut earth_sensor_sat,
            &mut sat_earth_sensor,
        );
    }
    (earth_sensor_limb, earth_sensor_sat, sat_earth_sensor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::{XF_GEOMOD_WGS72, XF_GEOMOD_WGS84};
    use crate::test_lock::TEST_LOCK;
    use crate::{DLL_VERSION, environment};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_get_dll_info_contains_version() {
        let _lock = TEST_LOCK.lock().unwrap();
        let info = get_dll_info();
        assert!(info.contains(DLL_VERSION));
    }

    #[test]
    fn test_keplerian_to_equinoctial() {
        let _lock = TEST_LOCK.lock().unwrap();
        let kep = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612];
        let eqnx = keplerian_to_equinoctial(&kep);

        assert_abs_diff_eq!(eqnx[0], 0.005756008409, epsilon = 1.0e-12);
        assert_abs_diff_eq!(eqnx[1], 0.002453246053, epsilon = 1.0e-12);
        assert_abs_diff_eq!(eqnx[2], 0.130405060328, epsilon = 1.0e-12);
        assert_abs_diff_eq!(eqnx[3], -0.503224317374, epsilon = 1.0e-12);
        assert_abs_diff_eq!(eqnx[4], 617.8480000000, epsilon = 1.0e-12);
        assert_abs_diff_eq!(eqnx[5], 2.005848298418, epsilon = 1.0e-12);
    }

    #[test]
    fn test_equinoctial_to_keplerian() {
        let _lock = TEST_LOCK.lock().unwrap();
        let eqnx = [
            0.005756008409,
            0.002453246053,
            0.130405060328,
            -0.503224317374,
            617.8480000,
            2.005848298418,
        ];
        let kep = equinoctial_to_keplerian(&eqnx);

        assert_abs_diff_eq!(kep[0], 26558.4820, epsilon = 1.0e-4);
        assert_abs_diff_eq!(kep[1], 0.0062570000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[2], 54.9350000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[3], 234.7640000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[4], 165.4720000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[5], 217.6120000, epsilon = 1.0e-7);
    }

    #[test]
    fn test_keplerian_to_cartesian() {
        let _lock = TEST_LOCK.lock().unwrap();
        let kep = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612];
        let posvel = keplerian_to_cartesian(&kep);

        assert_abs_diff_eq!(posvel[0], -3032.21272487, epsilon = 1.0e-7);
        assert_abs_diff_eq!(posvel[1], -15025.7763831, epsilon = 1.0e-7);
        assert_abs_diff_eq!(posvel[2], 21806.4954366, epsilon = 1.0e-7);
        assert_abs_diff_eq!(posvel[3], 3.754350020203, epsilon = 1.0e-7);
        assert_abs_diff_eq!(posvel[4], -0.889562019024, epsilon = 1.0e-7);
        assert_abs_diff_eq!(posvel[5], -0.114933710268, epsilon = 1.0e-7);
    }

    #[test]
    fn test_cartesian_to_keplerian() {
        let _lock = TEST_LOCK.lock().unwrap();
        let posvel = [
            -3032.21272487,
            -15025.7763831,
            21806.4954366,
            3.7543500202,
            -0.889562019026,
            -0.114933710268,
        ];
        let kep = cartesian_to_keplerian(&posvel);

        assert_abs_diff_eq!(kep[0], 26558.4820, epsilon = 1.0e-4);
        assert_abs_diff_eq!(kep[1], 0.0062570000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[2], 54.9350000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[3], 234.7640000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[4], 165.4720000, epsilon = 1.0e-7);
        assert_abs_diff_eq!(kep[5], 217.6120000, epsilon = 1.0e-7);
    }

    #[test]
    fn test_mean_motion_conversions() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mean_motion = 1.0027382962;
        let semi_major_axis = mean_motion_to_sma(mean_motion);
        assert_abs_diff_eq!(semi_major_axis, 42164.171420, epsilon = 1.0e-6);

        let mean_motion_back = sma_to_mean_motion(42164.17142);
        assert_abs_diff_eq!(mean_motion_back, mean_motion, epsilon = 1.0e-7);
    }

    #[test]
    fn test_kozai_brouwer_conversions() {
        let _lock = TEST_LOCK.lock().unwrap();
        let kozai = 14.2024103100000;
        let brouwer = 14.2107268431215;
        let ecc = 1.1127E-002;
        let inc = 99.4371000000000;

        let to_brouwer = kozai_to_brouwer(ecc, inc, kozai);
        assert_abs_diff_eq!(to_brouwer, brouwer, epsilon = 1.0e-7);

        let to_kozai = brouwer_to_kozai(ecc, inc, brouwer);
        assert_abs_diff_eq!(to_kozai, kozai, epsilon = 1.0e-7);
    }

    #[test]
    fn test_osculating_to_mean() {
        let _lock = TEST_LOCK.lock().unwrap();
        let osc = [7200.0, 0.006257, 54.935, 234.764, 165.472, 217.612];
        let mean = osculating_to_mean(&osc);

        assert_abs_diff_eq!(mean[0], 7206.06814087, epsilon = 1.0e-7);
        assert_abs_diff_eq!(mean[1], 0.00646986051778, epsilon = 1.0e-7);
        assert_abs_diff_eq!(mean[2], 54.9518948032, epsilon = 1.0e-7);
        assert_abs_diff_eq!(mean[3], 232.98566416, epsilon = 1.0e-7);
        assert_abs_diff_eq!(mean[4], 165.473342418, epsilon = 1.0e-7);
        assert_abs_diff_eq!(mean[5], 219.392452222, epsilon = 1.0e-7);
    }

    #[test]
    fn test_gst_teme_to_lla() {
        let _lock = TEST_LOCK.lock().unwrap();
        environment::set_geopotential_model(XF_GEOMOD_WGS84);
        let xyz = [6524.834, 6862.875, 6448.296];
        let llh = gst_teme_to_lla(0.0, &xyz);
        environment::set_geopotential_model(XF_GEOMOD_WGS72);

        assert_abs_diff_eq!(llh[0], 34.352495, epsilon = 1.0e-5);
        assert_abs_diff_eq!(llh[1], 46.446417, epsilon = 1.0e-5);
        assert_abs_diff_eq!(llh[2], 5085.218731, epsilon = 1.0e-5);
    }

    #[test]
    fn test_efg_to_lla() {
        let _lock = TEST_LOCK.lock().unwrap();
        environment::set_geopotential_model(XF_GEOMOD_WGS84);
        let efg = [6524.834, 6862.875, 6448.296];
        let llh = efg_to_lla(&efg).unwrap();
        environment::set_geopotential_model(XF_GEOMOD_WGS72);

        assert_abs_diff_eq!(llh[0], 34.352495, epsilon = 1.0e-5);
        assert_abs_diff_eq!(llh[1], 46.446417, epsilon = 1.0e-5);
        assert_abs_diff_eq!(llh[2], 5085.218731, epsilon = 1.0e-5);
    }
}
