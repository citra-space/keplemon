// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::{
    GetSetString, get_last_error_message,
    tle::{self, XA_TLE_AGOMGP},
};
use std::os::raw::c_char;

unsafe extern "C" {
    //  Notes: This function has been deprecated since v9.0.
    //  Initializes the Sgp4 DLL for use in the program.
    //  <br>
    //  If this function returns an error, it is recommended that you stop the program immediately.
    //  <br>
    //  An error will occur if you forget to load and initialize all the prerequisite DLLs, as listed in the DLL Prerequisites section of the accompanying documentation, before using this DLL.
    pub fn Sgp4Init(apAddr: i64) -> i32;
    //  Returns information about the current version of Sgp4Prop.dll. The information is placed in the string parameter you pass in.
    //  The returned string provides information about the version number, build date, and platform.
    pub fn Sgp4GetInfo(infoStr: *const c_char);
    //  Loads SGP4-related parameters (prediction controls, JPL settings) and SGP4 elsets from a text file
    pub fn Sgp4LoadFileAll(sgp4InputFile: *const c_char) -> i32;
    //  Saves currently loaded SGP4-related parameters (SGP4 application controls, prediction controls, integration controls) to a file
    //  The purpose of this function is to save the current SGP4-related settings, usually used in GUI applications, for future use.
    pub fn Sgp4SaveFile(sgp4File: *const c_char, saveMode: i32, saveForm: i32) -> i32;
    //  Initializes an SGP4 satellite from an SGP or SGP4 TLE.
    //  Internally, when this function is called, Tle.dll's set of TLEs is searched for the provided satKey. If found, the associated TLE data will be used to create an SGP4 satellite which then will be added to Sgp4Prop.dll's set of satellites. Subsequent calls to propagate this satellite will use the data in this set to compute the satellite's new state.
    //
    //  This routine should be called once for each satellite you wish to propagate before propagation begins, or any time the associated data that is stored by Tle.dll is changed.
    //
    //  The call to this routine needs to be placed before any calls to the SGP4 propagator routines (Sgp4PropMse(), Sgp4PropDs50UTC(), etc.).
    pub fn Sgp4InitSat(satKey: i64) -> i32;
    //  Removing a satellite from the propagator's set of satellites does not affect the corresponding TLE data loaded by calls to routines in Tle.dll.
    pub fn Sgp4RemoveSat(satKey: i64) -> i32;
    //  Removes all currently loaded satellites from memory.
    //  Calling this function removes all satellites from the set maintained by Sgp4Prop.dll. However, the TLE data loaded by Tle.dll is unaffected by this function.
    pub fn Sgp4RemoveAllSats() -> i32;
    //  Returns the number of GP objects currently created.
    pub fn Sgp4GetCount() -> i32;
    //  Propagates a satellite, represented by the satKey, to the time expressed in minutes since the satellite's epoch time.
    //  The resulting data about the satellite is placed in the various reference parameters.
    //  It is the users' responsibility to decide what to do with the returned value. For example, if the users want to check for decay or low altitude, they can put that logic into their own code.
    //
    //  This function can be called in random time requests.
    //  The following cases will result in an error:
    //  <ul>
    //  <li>Semi major axis A &lt;= 0 or A &gt;1.0D6</li>
    //  <li>Eccentricity E &gt;= 1.0 or E &lt; -1.0D-3</li>
    //  <li>Mean anomaly MA&gt;=1.0D10</li>
    //  <li>Hyperbolic orbit E<sup>2</sup>&gt;= 1.0</li>
    //  <li>satKey doesn't exist in the set of loaded satellites</li>
    //  <li>FK model not set to FK5</li>
    //  </ul>
    pub fn Sgp4PropMse(
        satKey: i64,
        mse: f64,
        ds50UTC: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        llh: *mut [f64; 3],
    ) -> i32;
    //  Propagates a satellite, represented by the satKey, to the time expressed in days since 1950, UTC.
    //  The resulting data about the satellite is placed in the pos (position), vel (velocity), and llh (Lat/Lon/Height) parameters.
    //  It is the users' responsibility to decide what to do with the returned value. For example, if the users want to check for decay or low altitude, they can put that logic into their own code.
    //  The following cases will result in an error:
    //  <ul>
    //  <li>Semi major axis A &lt;= 0 or A &gt;1.0D6</li>
    //  <li>Eccentricity E &gt;= 1.0 or E &lt; -1.0D-3</li>
    //  <li>Mean anomaly MA&gt;=1.0D10</li>
    //  <li>Hyperbolic orbit E<sup>2</sup>&gt;= 1.0</li>
    //  <li>satKey doesn't exist in the set of loaded satellites</li>
    //  <li>GEO model not set to WGS-72 and/or FK model not set to FK5</li>
    //  </ul>
    pub fn Sgp4PropDs50UTC(
        satKey: i64,
        ds50UTC: f64,
        mse: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        llh: *mut [f64; 3],
    ) -> i32;
    //  Propagates a satellite, represented by the satKey, to the time expressed in days since 1950, UTC.
    //  The resulting data about the satellite is placed in the pos (position), vel (velocity) parameters.
    pub fn Sgp4PropDs50UtcPosVel(satKey: i64, ds50UTC: f64, pos: *mut [f64; 3], vel: *mut [f64; 3]) -> i32;
    //  Propagates a satellite, represented by the satKey, to the time expressed in days since 1950, UTC.
    //  Only the geodetic information is returned by this function.
    //  It is the users' responsibility to decide what to do with the returned value. For example, if the users want to check for decay or low altitude, they can put that logic into their own code.
    //
    //  This function is similar to Sgp4PropDs50UTC but returns only LLH.  This function is designed especially for applications which plot ground traces.
    //  The following cases will result in an error:
    //  <ul>
    //  <li>Semi major axis A &lt;= 0 or A &gt;1.0D6</li>
    //  <li>Eccentricity E &gt;= 1.0 or E &lt; -1.0D-3</li>
    //  <li>Mean anomaly MA&gt;=1.0D10</li>
    //  <li>Hyperbolic orbit E<sup>2</sup>&gt;= 1.0</li>
    //  <li>satKey doesn't exist in the set of loaded satellites</li>
    //  <li>GEO model not set to WGS-72 and/or FK model not set to FK5</li>
    //  </ul>
    pub fn Sgp4PropDs50UtcLLH(satKey: i64, ds50UTC: f64, llh: *mut [f64; 3]) -> i32;
    //  Propagates a satellite, represented by the satKey, to the time expressed in days since 1950, UTC.
    //  Only the ECI position vector is returned by this function.
    //  It is the users' responsibility to decide what to do with the returned value. For example, if the users want to check for decay or low altitude, they can put that logic into their own code.
    //
    //  This function is similar to Sgp4PropDs50UTC but returns only ECI position vector.  This function is designed especially for applications which plot satellite position in 3D.
    //  The following cases will result in an error:
    //  <ul>
    //  <li>Semi major axis A &lt;= 0 or A &gt;1.0D6</li>
    //  <li>Eccentricity E &gt;= 1.0 or E &lt; -1.0D-3</li>
    //  <li>Mean anomaly MA&gt;=1.0D10</li>
    //  <li>Hyperbolic orbit E<sup>2</sup>&gt;= 1.0</li>
    //  <li>satKey doesn't exist in the set of loaded satellites</li>
    //  <li>GEO model not set to WGS-72 and/or FK model not set to FK5</li>
    //  </ul>
    pub fn Sgp4PropDs50UtcPos(satKey: i64, ds50UTC: f64, pos: *mut [f64; 3]) -> i32;
    //  Retrieves propagator's precomputed results. This function can be used to obtain results from
    //  a propagation which are not made available through calls to the propagation functions themselves.
    //  <br>
    //  See example in Sgp4PropMse or Sgp4PropDs50UTC.
    //  <br>
    //  This function should be called immediately after a successful call to Sgp4PropMse() or Sgp4PropDs50UTC() to retrieve the desired values.
    //  <br>
    //  It is the caller's responsibility to ensure that the array passed in the destArray parameter is large enough to hold the requested values. The required size can be found by looking at the destArray size column of the table below describing valid index values.
    //  <br>
    //  The destArray Arrangement column lists the order of the elements in the array. It is not necessarily the subscript of the element in the array since this is language-dependent. For example, in C/C++ the first element in every array is the zero-subscripted element. In other programming languages, the subscript of the first element is 1.
    //  <br>
    //  Note: This function is not thread safe, please use Sgp4PropAll() instead
    //  <br>
    //  The table below shows the values for the xf_Sgp4Out parameter:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Index</b></td>
    //  <td><b>Index Interpretation</b></td>
    //  <td><b>DestArray size</b></td>
    //  <td><b>DestArray Arrangement</b></td>
    //  </tr>
    //  <tr><td>1</td><td>Revolution number</td><td>1</td><td><ol><li>Revolution number (based on the Osculating Keplerian
    //  Elements)</li></ol></td></tr>
    //  <tr><td>2</td><td>Nodal Apogee Perigee</td><td>3</td><td><ol><li>nodal period (minutes)</li><li>apogee
    //  (km)</li><li>perigee (km)</li></ol></td></tr>
    //  <tr><td>3</td><td>Mean Keplerian Elements</td><td>6</td><td><ol><li>semi-major axis (km)</li><li>eccentricity
    //  (unitless)</li><li>inclination (degree)</li><li>mean anomaly (degree)</li><li>right ascension of the ascending node
    //  (degree)</li><li>argument of perigee (degree)</li></ol></td></tr>
    //  <tr><td>4</td><td>Osculating Keplerian Elements</td><td>6</td><td>Same as Mean Keplerian Elements</td></tr>
    //  </table>
    pub fn Sgp4GetPropOut(satKey: i64, xf_Sgp4Out: i32, destArr: *mut f64) -> i32;
    //  Propagates a satellite, represented by the satKey, to the time expressed in either minutes since epoch or days since 1950, UTC.
    //  All propagation data is returned by this function.
    pub fn Sgp4PropAll(satKey: i64, timeType: i32, timeIn: f64, xa_Sgp4Out: *mut [f64; 64]) -> i32;
    //  Converts osculating position and velocity vectors to a set of mean Keplerian SGP4 elements.
    //  The new position and velocity vectors are the results of using SGP4 propagator to propagate the computed sgp4MeanKep to the time specified in year and day of epoch time.
    //  They should be closely matched with the input osculating position and velocity vectors.
    //
    //  The mean Keplerian elements are SGP4's Brouwer mean motion not SGP's Kozai mean motion.
    //  Notes: Even if the function fails, the less acurate results may still be availalbe
    pub fn Sgp4PosVelToKep(
        yr: i32,
        day: f64,
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        posNew: *mut [f64; 3],
        velNew: *mut [f64; 3],
        xa_kep: *mut [f64; 6],
    ) -> i32;
    //  Converts osculating position and velocity vectors to TLE array - allows bstar/bterm, drag values to be used in the conversion if desired
    //  The function is similar to Sgp4PosVelToKep but allows the user to specify agom (XP mode) and bstar/bterm values, if desired, to be used in solving for the new Keplerian elements.
    //
    //  The updated elements returned in the xa_tle array is of type SGP and the mean motion is Kozai mean motion.
    //  Notes: Even if the function fails, the less acurate results may still be availalbe
    pub fn Sgp4PosVelToTleArr(pos: *const [f64; 3], vel: *const [f64; 3], xa_tle: *mut [f64; 64]) -> i32;
    //  Reepochs a loaded TLE, represented by the satKey, to a new epoch.
    pub fn Sgp4ReepochTLE(satKey: i64, reEpochDs50UTC: f64, line1Out: *const c_char, line2Out: *const c_char) -> i32;
    //  Reepochs a loaded TLE, represented by the satKey, to a new epoch in Csv format.
    pub fn Sgp4ReepochCsv(satKey: i64, reEpochDs50UTC: f64, csvLine: *const c_char) -> i32;
    //  Sets path to the Sgp4 Open License file if the license file
    //  Note: This function has been revised since v9.6. It's only needed if the "SGP4_Open_License.txt" isn't located in current folder or those folders specified in PATH/LD_LIBRARY_PATH environment
    pub fn Sgp4SetLicFilePath(licFilePath: *const c_char);
    //  Gets the current path to the Sgp4 Open License file
    //  Note: This function has been revised since v9.6. It's only needed if the "SGP4_Open_License.txt" isn't located in current folder or those folders specified in PATH/LD_LIBRARY_PATH environment
    pub fn Sgp4GetLicFilePath(licFilePath: *const c_char);
    //  Generates ephemerides for the input satellite, represented by its satKey, for the specified time span and step size
    //  Notes: if arrSize isn't big enough to store all the ephemeris points, the function will exit when the ephemArr reaches
    //  that many points (arrSize) and the errCode is set to IDX_ERR_WARN
    pub fn Sgp4GenEphems(
        satKey: i64,
        startTime: f64,
        endTime: f64,
        stepSize: f64,
        sgp4_ephem: i32,
        arrSize: i32,
        ephemArr: *mut f64,
        genEphemPts: *mut i32,
    ) -> i32;
    //  Generates ephemerides for the input TLE - in an array format - for the specified time span and step size (OS - in One Step)
    //  Notes: <br>
    //  - This function takes in TLE data directly and doesn't need to go through loading/geting satKey/initializing steps<br>
    //  - if arrSize isn't big enough to store all the ephemeris points, the function will exit when the ephemArr reaches
    //    that many points (arrSize) and the errCode is set to IDX_ERR_WARN
    pub fn Sgp4GenEphems_OS(
        xa_tle: *const [f64; 64],
        startTime: f64,
        endTime: f64,
        stepSize: f64,
        sgp4_ephem: i32,
        arrSize: i32,
        ephemArr: *mut f64,
        genEphemPts: *mut i32,
    ) -> i32;
    //  Propagates all input satellites, represented by their satKeys, to the time expressed in days since 1950, UTC.
    pub fn Sgp4PropAllSats(satKeys: *const i64, numOfSats: i32, ds50UTC: f64, ephemArr: *mut f64) -> i32;
    //  Provides the native XP equinoctial elements and rates at given time
    pub fn XpGetNativeElts(satKey: i64, ds50UTC: f64, xa_eqnx: *mut [f64; 6], xa_eqnx_dot: *mut [f64; 6]) -> i32;
    //  Reepochs to a csv and provides the native XP equinoctial elements and rates
    pub fn XpReepochGetNativeElts(
        satKey: i64,
        reEpochDs50UTC: f64,
        csvLine: *const c_char,
        xa_eqnx: *mut [f64; 6],
        xa_eqnx_dot: *mut [f64; 6],
    ) -> i32;
}
// Different return values of errCode from Sgp4 propagation
// SGP4 propagates successfully
pub static GP_ERR_NONE: i32 = 0;
// Bad FK model (FK5 must be selected)
pub static GP_ERR_BADFK: i32 = 1;
// A is negative
pub static GP_ERR_ANEGATIVE: i32 = 2;
// A is to large
pub static GP_ERR_ATOOLARGE: i32 = 3;
// Eccentricity is hyperbolic
pub static GP_ERR_EHYPERPOLIC: i32 = 4;
// Eccentricity is negative
pub static GP_ERR_ENEGATIVE: i32 = 5;
// Mean anomaly is too large
pub static GP_ERR_MATOOLARGE: i32 = 6;
// e**2 is too large
pub static GP_ERR_E2TOOLARGE: i32 = 7;

// Different time types for passing to Sgp4PropAll
// propagation time is in minutes since epoch
pub static SGP4_TIMETYPE_MSE: i32 = 0;
// propagation time is in days since 1950, UTC
pub static SGP4_TIMETYPE_DS50UTC: i32 = 1;

// Sgp4 propagated output fields
// Revolution number
pub static XF_SGP4OUT_REVNUM: i32 = 1;
// Nodal period, apogee, perigee
pub static XF_SGP4OUT_NODAL_AP_PER: i32 = 2;
// Mean Keplerian
pub static XF_SGP4OUT_MEAN_KEP: i32 = 3;
// Osculating Keplerian
pub static XF_SGP4OUT_OSC_KEP: i32 = 4;

// Sgp4 propagated data
// Propagation time in days since 1950, UTC
pub const XA_SGP4OUT_DS50UTC: usize = 0;
// Propagation time in minutes since the satellite's epoch time
pub const XA_SGP4OUT_MSE: usize = 1;
// ECI X position (km) in True Equator and Mean Equinox of Epoch
pub const XA_SGP4OUT_POSX: usize = 2;
// ECI Y position (km) in True Equator and Mean Equinox of Epoch
pub const XA_SGP4OUT_POSY: usize = 3;
// ECI Z position (km) in True Equator and Mean Equinox of Epoch
pub const XA_SGP4OUT_POSZ: usize = 4;
// ECI X velocity (km/s) in True Equator and Mean Equinox of Epoch
pub const XA_SGP4OUT_VELX: usize = 5;
// ECI Y velocity (km/s) in True Equator and Mean Equinox of Epoch
pub const XA_SGP4OUT_VELY: usize = 6;
// ECI Z velocity (km/s) in True Equator and Mean Equinox of Epoch
pub const XA_SGP4OUT_VELZ: usize = 7;
// Geodetic latitude (deg)
pub const XA_SGP4OUT_LAT: usize = 8;
// Geodetic longitude (deg)
pub const XA_SGP4OUT_LON: usize = 9;
// Height above geoid (km)
pub const XA_SGP4OUT_HEIGHT: usize = 10;
// Revolution number
pub const XA_SGP4OUT_REVNUM: usize = 11;
// Nodal period (min)
pub const XA_SGP4OUT_NODALPER: usize = 12;
// Apogee (km)
pub const XA_SGP4OUT_APOGEE: usize = 13;
// Perigee (km)
pub const XA_SGP4OUT_PERIGEE: usize = 14;
// Mean semi-major axis (km)
pub const XA_SGP4OUT_MN_A: usize = 15;
// Mean eccentricity (unitless)
pub const XA_SGP4OUT_MN_E: usize = 16;
// Mean inclination (deg)
pub const XA_SGP4OUT_MN_INCLI: usize = 17;
// Mean mean anomaly (deg)
pub const XA_SGP4OUT_MN_MA: usize = 18;
// Mean right ascension of the asending node (deg)
pub const XA_SGP4OUT_MN_NODE: usize = 19;
// Mean argument of perigee (deg)
pub const XA_SGP4OUT_MN_OMEGA: usize = 20;
// Osculating semi-major axis (km)
pub const XA_SGP4OUT_OSC_A: usize = 21;
// Osculating eccentricity (unitless)
pub const XA_SGP4OUT_OSC_E: usize = 22;
// Osculating inclination (deg)
pub const XA_SGP4OUT_OSC_INCLI: usize = 23;
// Osculating mean anomaly (deg)
pub const XA_SGP4OUT_OSC_MA: usize = 24;
// Osculating right ascension of the asending node (deg)
pub const XA_SGP4OUT_OSC_NODE: usize = 25;
// Osculating argument of perigee (deg)
pub const XA_SGP4OUT_OSC_OMEGA: usize = 26;

pub const XA_SGP4OUT_SIZE: usize = 64;

// Different options for generating ephemerides from SGP4
// ECI TEME of DATE     - 0: time in days since 1950 UTC, 1-3: pos (km), 4-6: vel (km/sec)
pub const SGP4_EPHEM_ECI: i32 = 1;
// MEME of J2K (4 terms)- 0: time in days since 1950 UTC, 1-3: pos (km), 4-6: vel (km/sec)
pub const SGP4_EPHEM_J2K: i32 = 2;

// Different dynamic step size options
// Use a simple algorithm to determine step size based on satellite's current position
pub static DYN_SS_BASIC: i32 = -1;

//*******************************************************************************

// ========================= End of auto generated code ==========================

pub fn get_ephemeris(sat_key: i64, start: f64, stop: f64, step: f64, frame: i32) -> Result<Vec<f64>, String> {
    let step_days = step / (24.0 * 60.0);
    let num_steps = (((stop - start) / step_days).ceil()) as i32 + 1;
    let array_size = num_steps * 7;
    let mut ephem_arr = vec![0.0; (array_size) as usize];
    let mut gen_ephem_pts = 0;
    let result = unsafe {
        Sgp4GenEphems(
            sat_key,
            start,
            stop,
            step,
            frame,
            array_size,
            ephem_arr.as_mut_ptr(),
            &mut gen_ephem_pts,
        )
    };
    match result {
        0 => {
            ephem_arr.truncate((gen_ephem_pts as usize) * 7);
            Ok(ephem_arr)
        }
        _ => Err(get_last_error_message()),
    }
}

pub fn array_to_ephemeris(
    xa_tle: &[f64; tle::XA_TLE_SIZE],
    start: f64,
    stop: f64,
    step: f64,
    frame: i32,
) -> Result<Vec<f64>, String> {
    let step_days = step / (24.0 * 60.0);
    let num_steps = (((stop - start) / step_days).ceil()) as i32 + 1;
    let array_size = num_steps * 7;
    let mut ephem_arr = vec![0.0; (array_size) as usize];
    let mut gen_ephem_pts = 0;
    let result = unsafe {
        Sgp4GenEphems_OS(
            xa_tle,
            start,
            stop,
            step,
            frame,
            array_size,
            ephem_arr.as_mut_ptr(),
            &mut gen_ephem_pts,
        )
    };
    match result {
        0 => {
            ephem_arr.truncate((gen_ephem_pts as usize) * 7);
            Ok(ephem_arr)
        }
        _ => Err(get_last_error_message()),
    }
}

pub fn fit_xp_array(
    epoch: f64,
    posvel: &[f64; 6],
    ballistic_coefficient: Option<f64>,
    srp_coefficient: Option<f64>,
) -> Result<[f64; tle::XA_TLE_SIZE], String> {
    let pos = [posvel[0], posvel[1], posvel[2]];
    let vel = [posvel[3], posvel[4], posvel[5]];
    let mut xa_tle = [0.0; tle::XA_TLE_SIZE];
    xa_tle[tle::XA_TLE_EPOCH] = epoch;
    xa_tle[tle::XA_TLE_EPHTYPE] = tle::TLETYPE_XP as f64;
    xa_tle[XA_TLE_AGOMGP] = srp_coefficient.unwrap_or(0.0);
    xa_tle[tle::XA_TLE_BTERM] = ballistic_coefficient.unwrap_or(0.0);
    let result = unsafe { Sgp4PosVelToTleArr(&pos, &vel, &mut xa_tle) };
    match result {
        0 => Ok(xa_tle),
        _ => Err(get_last_error_message()),
    }
}

pub fn fit_sgp4_array(epoch: f64, posvel: &[f64; 6], b_star: Option<f64>) -> Result<[f64; tle::XA_TLE_SIZE], String> {
    let pos = [posvel[0], posvel[1], posvel[2]];
    let vel = [posvel[3], posvel[4], posvel[5]];
    let mut xa_tle = [0.0; tle::XA_TLE_SIZE];
    xa_tle[tle::XA_TLE_EPOCH] = epoch;
    xa_tle[tle::XA_TLE_EPHTYPE] = tle::TLETYPE_SGP4 as f64;
    xa_tle[tle::XA_TLE_BSTAR] = b_star.unwrap_or(0.0);
    let result = unsafe { Sgp4PosVelToTleArr(&pos, &vel, &mut xa_tle) };
    match result {
        0 => Ok(xa_tle),
        _ => Err(get_last_error_message()),
    }
}

pub fn load(sat_key: i64) -> Result<(), String> {
    let result = unsafe { Sgp4InitSat(sat_key) };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_positions_velocities(sat_keys: &[i64], ds50_utc: f64) -> Result<Vec<f64>, String> {
    let num_of_sats = sat_keys.len() as i32;
    let mut ephem_arr = vec![0.0; (num_of_sats * 6) as usize];
    let result = unsafe { Sgp4PropAllSats(sat_keys.as_ptr(), num_of_sats, ds50_utc, ephem_arr.as_mut_ptr()) };
    match result {
        0 => Ok(ephem_arr),
        _ => Err(get_last_error_message()),
    }
}

pub fn remove(sat_key: i64) -> Result<(), String> {
    let result = unsafe { Sgp4RemoveSat(sat_key) };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

pub fn clear() -> Result<(), String> {
    let result = unsafe { Sgp4RemoveAllSats() };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

type MSEPosVelLLH = (f64, [f64; 3], [f64; 3], [f64; 3]);

pub fn get_position_velocity_lla(sat_key: i64, ds50_utc: f64) -> Result<MSEPosVelLLH, String> {
    let mut mse = 0.0;
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    let mut llh = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UTC(sat_key, ds50_utc, &mut mse, &mut pos, &mut vel, &mut llh) };
    match result {
        0 => Ok((mse, pos, vel, llh)),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_position_velocity(sat_key: i64, ds50_utc: f64) -> Result<([f64; 3], [f64; 3]), String> {
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UtcPosVel(sat_key, ds50_utc, &mut pos, &mut vel) };
    match result {
        0 => Ok((pos, vel)),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_lla(sat_key: i64, ds50_utc: f64) -> Result<[f64; 3], String> {
    let mut llh = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UtcLLH(sat_key, ds50_utc, &mut llh) };
    match result {
        0 => Ok(llh),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_position(sat_key: i64, ds50_utc: f64) -> Result<[f64; 3], String> {
    let mut pos = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UtcPos(sat_key, ds50_utc, &mut pos) };
    match result {
        0 => Ok(pos),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_full_state(sat_key: i64, ds50_utc: f64) -> Result<[f64; XA_SGP4OUT_SIZE], String> {
    let mut all = [0.0; XA_SGP4OUT_SIZE];
    let result = unsafe { Sgp4PropAll(sat_key, SGP4_TIMETYPE_DS50UTC, ds50_utc, &mut all) };
    match result {
        0 => Ok(all),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_equinoctial(sat_key: i64, ds50_utc: f64) -> Result<[f64; 6], String> {
    let mut xa_eqnx = [0.0; 6];
    let mut xa_eqnx_dot = [0.0; 6];
    let result = unsafe { XpGetNativeElts(sat_key, ds50_utc, &mut xa_eqnx, &mut xa_eqnx_dot) };
    match result {
        0 => Ok(xa_eqnx),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_dll_info() -> String {
    let mut info = GetSetString::new();
    unsafe {
        Sgp4GetInfo(info.pointer());
    }
    info.value()
}

pub fn get_count() -> i32 {
    unsafe { Sgp4GetCount() }
}

pub fn set_license_directory(file_path: &str) {
    let mut lic_file: GetSetString = file_path.into();
    unsafe { Sgp4SetLicFilePath(lic_file.pointer()) };
}

pub fn get_license_directory() -> String {
    let mut c_str = GetSetString::new();
    unsafe { Sgp4GetLicFilePath(c_str.pointer()) };
    c_str.value().trim().to_string()
}

pub fn reepoch_tle(sat_key: i64, re_epoch_ds50_utc: f64) -> Result<(String, String), String> {
    let mut line1_out = GetSetString::new();
    let mut line2_out = GetSetString::new();
    let result = unsafe { Sgp4ReepochTLE(sat_key, re_epoch_ds50_utc, line1_out.pointer(), line2_out.pointer()) };
    match result {
        0 => Ok((
            line1_out.value().trim().to_string(),
            line2_out.value().trim().to_string(),
        )),
        _ => Err(get_last_error_message()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_lock::TEST_LOCK;
    use crate::{DLL_VERSION, tle};
    use approx::assert_abs_diff_eq;

    const SGP4_LINE_1: &str = "1 22222C 15058A   25363.54791667 +.00012345  10000-1  20000-1 2 0900";
    const SGP4_LINE_2: &str = "2 22222  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345";
    const XP_LINE_1: &str = "1 33333U 15058A   25363.54791667 +.00012345  10000-1  20000-1 4  900";
    const XP_LINE_2: &str = "2 33333  30.0000  40.0000 0005000  60.0000  70.0000  8.2345678012345";
    const EPOCH: f64 = 27757.54791667;
    const SGP4_LATITUDE: f64 = 22.536547343263198;
    const SGP4_LONGITUDE: f64 = 238.66278387347936;
    const SGP4_ALTITUDE: f64 = 30319.722365834336;
    const XP_LATITUDE: f64 = 22.580834873791684;
    const XP_LONGITUDE: f64 = 238.65305913125454;
    const XP_ALTITUDE: f64 = 3977.969361992566;
    const SGP4_X: f64 = -33722.20240953347;
    const SGP4_Y: f64 = 3451.0939430966114;
    const SGP4_Z: f64 = 14050.115953447255;
    const SGP4_VX: f64 = -0.7534699464839536;
    const SGP4_VY: f64 = -3.0289958981453213;
    const SGP4_VZ: f64 = -1.0597673984106273;
    const XP_X: f64 = -9515.23633738959;
    const XP_Y: f64 = 975.4110099884765;
    const XP_Z: f64 = 3961.4106652075125;
    const XP_VX: f64 = -1.4191743539251394;
    const XP_VY: f64 = -5.702875678562367;
    const XP_VZ: f64 = -1.9961866743072767;
    const SGP4_MEAN_MOTION: f64 = 1.2345678;
    const SGP4_MEAN_INCLINATION: f64 = 30.0;
    const SGP4_MEAN_ECCENTRICITY: f64 = 0.0005;
    const SGP4_MEAN_RAAN: f64 = 40.0;
    const SGP4_MEAN_SMA: f64 = 36705.009522294495;
    const SGP4_MEAN_MA: f64 = 70.0;
    const SGP4_PERIOD: f64 = 1166.3030603694838;
    const SGP4_APOGEE: f64 = 36723.36202705564;
    const SGP4_PERIGEE: f64 = 36686.65701753335;
    const SGP4_MEAN_ARG_PERIGEE: f64 = 60.0;
    const SGP4_OSC_SMA: f64 = 36704.93201233728;
    const SGP4_OSC_ECCENTRICITY: f64 = 0.0006084790199714079;
    const SGP4_OSC_INCLINATION: f64 = 30.006798162224296;
    const SGP4_OSC_MA: f64 = 62.77053792235827;
    const SGP4_OSC_RAAN: f64 = 40.02170181812538;
    const SGP4_OSC_ARG_PERIGEE: f64 = 67.2046668254525;
    const XP_MEAN_MOTION: f64 = 8.2345678;
    const XP_MEAN_INCLINATION: f64 = 30.0;
    const XP_MEAN_ECCENTRICITY: f64 = 0.0005;
    const XP_MEAN_RAAN: f64 = 40.0;
    const XP_MEAN_SMA: f64 = 2656.509522294495;
    const XP_MEAN_MA: f64 = 70.0;
    const XP_PERIOD: f64 = 174.3030603694838;
    const XP_APOGEE: f64 = 2673.36202705564;
    const XP_PERIGEE: f64 = 2639.65701753335;
    const XP_MEAN_ARG_PERIGEE: f64 = 60.0;
    const XP_OSC_SMA: f64 = 2656.93201233728;
    const XP_OSC_ECCENTRICITY: f64 = 0.0006084790199714079;
    const XP_OSC_INCLINATION: f64 = 30.006798162224296;
    const XP_OSC_MA: f64 = 62.77053792235827;
    const XP_OSC_RAAN: f64 = 40.02170181812538;
    const XP_OSC_ARG_PERIGEE: f64 = 67.2046668254525;

    #[test]
    fn test_custom_license_directory() {
        let _lock = TEST_LOCK.lock().unwrap();
        let original_lic_path = get_license_directory();
        set_license_directory("tests/custom_license_directory");
        assert_eq!(get_license_directory(), "tests/custom_license_directory");
        let sgp4_key = tle::load_lines(SGP4_LINE_1, SGP4_LINE_2);
        load(sgp4_key).unwrap();
        let state = get_position(sgp4_key, EPOCH).unwrap();
        assert!(!state.is_empty());

        set_license_directory(&original_lic_path);
        assert_eq!(get_license_directory(), original_lic_path);
    }

    #[test]
    fn test_ephemeris_generation() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_key = tle::load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = tle::load_lines(XP_LINE_1, XP_LINE_2);
        let start = EPOCH - 1.0;
        let stop = EPOCH;
        let step = 5.0;
        let frame = SGP4_EPHEM_ECI;
        load(sgp4_key).unwrap();
        load(xp_key).unwrap();
        let sgp4_ephem_by_key = get_ephemeris(sgp4_key, start, stop, step, frame).unwrap();
        let xp_ephem_by_key = get_ephemeris(xp_key, start, stop, step, frame).unwrap();
        let (sgp4_xa, _) = tle::get_arrays(sgp4_key).unwrap();
        let (xp_xa, _) = tle::get_arrays(xp_key).unwrap();
        let sgp4_ephem = array_to_ephemeris(&sgp4_xa, start, stop, step, frame).unwrap();
        let xp_ephem = array_to_ephemeris(&xp_xa, start, stop, step, frame).unwrap();

        let _ = clear();
        let _ = tle::clear();
        assert_eq!(sgp4_ephem_by_key, sgp4_ephem);
        assert_eq!(xp_ephem_by_key, xp_ephem);
        assert_abs_diff_eq!(sgp4_ephem[sgp4_ephem.len() - 6], SGP4_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_ephem[sgp4_ephem.len() - 5], SGP4_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_ephem[sgp4_ephem.len() - 4], SGP4_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_ephem[sgp4_ephem.len() - 3], SGP4_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_ephem[sgp4_ephem.len() - 2], SGP4_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_ephem[sgp4_ephem.len() - 1], SGP4_VZ, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_ephem[xp_ephem.len() - 6], XP_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_ephem[xp_ephem.len() - 5], XP_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_ephem[xp_ephem.len() - 4], XP_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_ephem[xp_ephem.len() - 3], XP_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_ephem[xp_ephem.len() - 2], XP_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_ephem[xp_ephem.len() - 1], XP_VZ, epsilon = 1.0e-9);
        assert_eq!(sgp4_ephem.len(), 2023);
        assert_eq!(xp_ephem.len(), 2023);
    }

    #[test]
    fn test_fit_arrays() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_posvel = [SGP4_X, SGP4_Y, SGP4_Z, SGP4_VX, SGP4_VY, SGP4_VZ];
        let xp_posvel = [XP_X, XP_Y, XP_Z, XP_VX, XP_VY, XP_VZ];

        let sgp4_xa = fit_sgp4_array(EPOCH, &sgp4_posvel, Some(0.02)).unwrap();
        let xp_xa = fit_xp_array(EPOCH, &xp_posvel, Some(0.02), Some(0.01)).unwrap();
        assert_abs_diff_eq!(sgp4_xa[tle::XA_TLE_INCLI], SGP4_MEAN_INCLINATION, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_xa[tle::XA_TLE_ECCEN], SGP4_MEAN_ECCENTRICITY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_xa[tle::XA_TLE_NODE], SGP4_MEAN_RAAN, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_xa[tle::XA_TLE_MNANOM], SGP4_MEAN_MA, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_xa[tle::XA_TLE_MNMOTN], SGP4_MEAN_MOTION, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_xa[tle::XA_TLE_OMEGA], SGP4_MEAN_ARG_PERIGEE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_xa[tle::XA_TLE_INCLI], XP_MEAN_INCLINATION, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_xa[tle::XA_TLE_ECCEN], XP_MEAN_ECCENTRICITY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_xa[tle::XA_TLE_NODE], XP_MEAN_RAAN, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_xa[tle::XA_TLE_MNANOM], XP_MEAN_MA, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_xa[tle::XA_TLE_MNMOTN], XP_MEAN_MOTION, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_xa[tle::XA_TLE_OMEGA], XP_MEAN_ARG_PERIGEE, epsilon = 1.0e-9);
    }

    #[test]
    fn test_prop_all_sats() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_key = tle::load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = tle::load_lines(XP_LINE_1, XP_LINE_2);
        load(sgp4_key).unwrap();
        load(xp_key).unwrap();
        let sat_keys = vec![sgp4_key, xp_key];
        let ephem_arr = get_positions_velocities(&sat_keys, EPOCH).unwrap();
        let _ = clear();
        let _ = tle::clear();
        assert_abs_diff_eq!(ephem_arr[0], SGP4_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[1], SGP4_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[2], SGP4_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[3], SGP4_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[4], SGP4_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[5], SGP4_VZ, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[6], XP_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[7], XP_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[8], XP_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[9], XP_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[10], XP_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(ephem_arr[11], XP_VZ, epsilon = 1.0e-9);
    }

    #[test]
    fn test_get_all_at_ds50() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_key = tle::load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = tle::load_lines(XP_LINE_1, XP_LINE_2);
        load(sgp4_key).unwrap();
        load(xp_key).unwrap();
        let sgp4_all = get_full_state(sgp4_key, EPOCH).unwrap();
        let xp_all = get_full_state(xp_key, EPOCH).unwrap();
        let _ = clear();
        let _ = tle::clear();
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_DS50UTC], EPOCH, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_all[XA_SGP4OUT_MSE], 0.0, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_POSX], SGP4_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_POSY], SGP4_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_POSZ], SGP4_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_VELX], SGP4_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_VELY], SGP4_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_VELZ], SGP4_VZ, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_MN_INCLI], SGP4_MEAN_INCLINATION, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_MN_E], SGP4_MEAN_ECCENTRICITY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_MN_A], SGP4_MEAN_SMA, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_NODALPER], SGP4_PERIOD, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_APOGEE], SGP4_APOGEE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_PERIGEE], SGP4_PERIGEE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_MN_MA], SGP4_MEAN_MA, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_MN_OMEGA], SGP4_MEAN_ARG_PERIGEE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_MN_NODE], SGP4_MEAN_RAAN, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_OSC_A], SGP4_OSC_SMA, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_OSC_E], SGP4_OSC_ECCENTRICITY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_OSC_INCLI], SGP4_OSC_INCLINATION, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_OSC_MA], SGP4_OSC_MA, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_OSC_NODE], SGP4_OSC_RAAN, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_all[XA_SGP4OUT_OSC_OMEGA], SGP4_OSC_ARG_PERIGEE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_all[XA_SGP4OUT_POSX], XP_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_all[XA_SGP4OUT_POSY], XP_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_all[XA_SGP4OUT_POSZ], XP_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_all[XA_SGP4OUT_VELX], XP_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_all[XA_SGP4OUT_VELY], XP_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_all[XA_SGP4OUT_VELZ], XP_VZ, epsilon = 1.0e-9);
    }

    #[test]
    fn test_get_pos_at_ds50() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_key = tle::load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = tle::load_lines(XP_LINE_1, XP_LINE_2);

        load(sgp4_key).unwrap();
        load(xp_key).unwrap();

        let sgp4_pos = get_position(sgp4_key, EPOCH).unwrap();
        let xp_pos = get_position(xp_key, EPOCH).unwrap();

        let _ = clear();
        let _ = tle::clear();

        assert_abs_diff_eq!(sgp4_pos[0], SGP4_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_pos[1], SGP4_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_pos[2], SGP4_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_pos[0], XP_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_pos[1], XP_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_pos[2], XP_Z, epsilon = 1.0e-9);
    }

    #[test]
    fn test_get_posvel_at_ds50() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_key = tle::load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = tle::load_lines(XP_LINE_1, XP_LINE_2);

        load(sgp4_key).unwrap();
        load(xp_key).unwrap();

        let (sgp4_pos, sgp4_vel) = get_position_velocity(sgp4_key, EPOCH).unwrap();
        let (xp_pos, xp_vel) = get_position_velocity(xp_key, EPOCH).unwrap();

        let _ = clear();
        let _ = tle::clear();

        assert_abs_diff_eq!(sgp4_pos[0], SGP4_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_pos[1], SGP4_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_pos[2], SGP4_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_vel[0], SGP4_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_vel[1], SGP4_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_vel[2], SGP4_VZ, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_pos[0], XP_X, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_pos[1], XP_Y, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_pos[2], XP_Z, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_vel[0], XP_VX, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_vel[1], XP_VY, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_vel[2], XP_VZ, epsilon = 1.0e-9);
    }

    #[test]
    fn test_get_llh_at_ds50() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_key = tle::load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = tle::load_lines(XP_LINE_1, XP_LINE_2);

        load(sgp4_key).unwrap();
        load(xp_key).unwrap();

        let sgp4_lla = get_lla(sgp4_key, EPOCH).unwrap();
        let xp_lla = get_lla(xp_key, EPOCH).unwrap();

        let _ = clear();
        let _ = tle::clear();

        assert_abs_diff_eq!(sgp4_lla[0], SGP4_LATITUDE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_lla[1], SGP4_LONGITUDE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(sgp4_lla[2], SGP4_ALTITUDE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_lla[0], XP_LATITUDE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_lla[1], XP_LONGITUDE, epsilon = 1.0e-9);
        assert_abs_diff_eq!(xp_lla[2], XP_ALTITUDE, epsilon = 1.0e-9);
    }

    #[test]
    fn test_get_dll_info_contains_version() {
        let _lock = TEST_LOCK.lock().unwrap();
        let info = get_dll_info();
        assert!(info.contains(DLL_VERSION));
    }
}
