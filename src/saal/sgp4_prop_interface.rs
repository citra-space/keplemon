// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use super::main_interface::MainInterface;
use super::GetSetString;
use crate::exceptions::SAALError;
use pyo3::prelude::*;
use pyo3::py_run;
use std::os::raw::c_char;

extern "C" {
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
pub static SGP4_EPHEM_ECI: i32 = 1;
// MEME of J2K (4 terms)- 0: time in days since 1950 UTC, 1-3: pos (km), 4-6: vel (km/sec)
pub static SGP4_EPHEM_J2K: i32 = 2;

// Different dynamic step size options
// Use a simple algorithm to determine step size based on satellite's current position
pub static DYN_SS_BASIC: i32 = -1;

//*******************************************************************************

// ========================= End of auto generated code ==========================

#[inline]
pub fn load_key(sat_key: i64) -> Result<(), String> {
    let result = unsafe { Sgp4InitSat(sat_key) };
    match result {
        0 => Ok(()),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[inline]
pub fn remove_key(sat_key: i64) -> Result<(), String> {
    let result = unsafe { Sgp4RemoveSat(sat_key) };
    match result {
        0 => Ok(()),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[inline]
pub fn clear_keys() -> Result<(), String> {
    let result = unsafe { Sgp4RemoveAllSats() };
    match result {
        0 => Ok(()),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

type MSEPosVelLLH = (f64, [f64; 3], [f64; 3], [f64; 3]);
#[inline]
pub fn get_state_at_ds50(sat_key: i64, ds50_utc: f64) -> Result<MSEPosVelLLH, String> {
    let mut mse = 0.0;
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    let mut llh = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UTC(sat_key, ds50_utc, &mut mse, &mut pos, &mut vel, &mut llh) };
    match result {
        0 => Ok((mse, pos, vel, llh)),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[inline]
pub fn get_posvel_at_ds50(sat_key: i64, ds50_utc: f64) -> Result<([f64; 3], [f64; 3]), String> {
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UtcPosVel(sat_key, ds50_utc, &mut pos, &mut vel) };
    match result {
        0 => Ok((pos, vel)),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[inline]
pub fn get_llh_at_ds50(sat_key: i64, ds50_utc: f64) -> Result<[f64; 3], String> {
    let mut llh = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UtcLLH(sat_key, ds50_utc, &mut llh) };
    match result {
        0 => Ok(llh),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[inline]
pub fn get_pos_at_ds50(sat_key: i64, ds50_utc: f64) -> Result<[f64; 3], String> {
    let mut pos = [0.0; 3];
    let result = unsafe { Sgp4PropDs50UtcPos(sat_key, ds50_utc, &mut pos) };
    match result {
        0 => Ok(pos),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

pub fn get_all_at_ds50(sat_key: i64, ds50_utc: f64) -> Result<[f64; XA_SGP4OUT_SIZE], String> {
    let mut all = [0.0; XA_SGP4OUT_SIZE];
    let result = unsafe { Sgp4PropAll(sat_key, SGP4_TIMETYPE_DS50UTC, ds50_utc, &mut all) };
    match result {
        0 => Ok(all),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

pub fn get_equinoctial_at_ds50(sat_key: i64, ds50_utc: f64) -> Result<[f64; 6], String> {
    let mut xa_eqnx = [0.0; 6];
    let mut xa_eqnx_dot = [0.0; 6];
    let result = unsafe { XpGetNativeElts(sat_key, ds50_utc, &mut xa_eqnx, &mut xa_eqnx_dot) };
    match result {
        0 => Ok(xa_eqnx),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[pyfunction]
pub fn set_license_file_path(lic_file_path: &str) {
    let mut c_str = GetSetString::from_string(lic_file_path);
    unsafe { Sgp4SetLicFilePath(c_str.pointer()) };
}

#[pyfunction]
pub fn get_license_file_path() -> String {
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
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[pyfunction(name = "reepoch_tle")]
pub fn py_reepoch_tle(sat_key: i64, re_epoch_ds50_utc: f64) -> PyResult<(String, String)> {
    reepoch_tle(sat_key, re_epoch_ds50_utc).map_err(SAALError::new_err)
}

pub fn register_sgp4_prop_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let sgp4_prop_interface = PyModule::new(parent_module.py(), "sgp4_prop_interface")?;
    sgp4_prop_interface.add_function(wrap_pyfunction!(py_reepoch_tle, &sgp4_prop_interface)?)?;
    py_run!(
        parent_module.py(),
        sgp4_prop_interface,
        "import sys; sys.modules['keplemon._keplemon.saal.sgp4_prop_interface'] = sgp4_prop_interface"
    );

    parent_module.add_submodule(&sgp4_prop_interface)
}
