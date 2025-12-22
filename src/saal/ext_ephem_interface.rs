// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use super::main_interface::MainInterface;
use once_cell::sync::Lazy;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::os::raw::c_char;
use std::sync::Mutex;

static EXTEPH_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

extern "C" {
    //  Notes: This function has been deprecated since v9.0.
    //  Initializes ExtEphem DLL for use in the program
    //  If this function returns an error, it is recommended that the users stop the program immediately.
    //  The error occurs if the users forget to load and initialize all the prerequisite DLLs, as listed
    //  in the DLL Prerequisite section, before using this DLL.
    pub fn ExtEphInit(apAddr: i64) -> i32;
    //  Returns information about the current version of ExtEphem DLL.
    //  The information is placed in the string parameter passed in.
    //  The returned string provides information about the version number, build date, and the platform of the ExtEphem DLL.
    pub fn ExtEphGetInfo(infoStr: *const c_char);
    //  Loads a file containing EXTEPHEM's
    //  The users can use this function repeatedly to load EXTEPHEMs from different input files.
    //  However, only unique satKeys are stored in the binary tree. Duplicated EXTEPHEMs
    //  (determined by same file name, satellite number + epoch) won't be stored.
    //
    //  EXTEPHEMs can be included directly in the main input file or they can be read from a
    //  separate file identified with "EPHFIL =[pathname\filename]".
    //
    //  This function only reads EXTEPHEMs from the main input file or EXTEPHEMs from the file
    //  identified with EPHFIL in the input file. It won't read anything else.
    pub fn ExtEphLoadFile(extEphFile: *const c_char) -> i32;
    //  Saves the currently loaded EXTEPHEM's to a file (EPHFIL=input file name)
    pub fn ExtEphSaveFile(extEphFile: *const c_char, saveMode: i32, saveForm: i32) -> i32;
    //  Removes an EXTEPHEM represented by the satKey from memory
    //  If the users enter an invalid satKey (the satKey does not exist in memory), the function will return a non-zero value indicating an error.
    pub fn ExtEphRemoveSat(satKey: i64) -> i32;
    //  Removes all EXTEPHEMS from memory
    pub fn ExtEphRemoveAllSats() -> i32;
    //  Returns the number of EXTEPHEM's currently loaded
    //  See ExtEphGetLoaded for example.
    //  This function is useful for dynamically allocating memory for the array that is passed to the function ExtEphGetLoaded().
    pub fn ExtEphGetCount() -> i32;
    //  Retrieves all of the currently loaded satKeys. These satKeys can be used to access the external ephemeris data for the EXTEPHEM's
    //  It is recommended that ExtEphGetCount() is used to count how many satellites are currently loaded in the ExtEphem DLL's binary tree.
    //  The users then use this number to dynamically allocate the satKeys array and pass it to this function.
    //
    //  If the users prefer to pass a static array to the function, ensure that it is big enough to store all the satKeys in memory.
    pub fn ExtEphGetLoaded(order: i32, satKeys: *mut i64);
    //  Allows for an EXTEPHEM to be added to memory without using an input file. The function creates a place holder for an EXTEPHEM
    //  If the same satellite (same satNum and epochDs50UTC) was previously added to the ExtEphem DLL's binary tree,
    //  the function will generate a new unique satKey. This is very useful when the users want to compare ephemerides
    //  of the same satellite number and same epoch time from different sources.
    pub fn ExtEphAddSat(satNum: i32, epochDs50UTC: f64, ae: f64, ke: f64, coordSys: i32) -> i64;
    //  Adds an ephemeris point to the end of an EXTEPHEM's set of ephemeris points
    //  The new ephemeris point will only be added to the array if its time is greater than the times of all points already in the array.
    //  Therefore, the array is always in sorted order (t1 &lt; t2 &lt;... &lt; tn).
    pub fn ExtEphAddSatEphem(satKey: i64, ds50UTC: f64, pos: *const [f64; 3], vel: *const [f64; 3], revNum: i32)
        -> i32;
    //  Adds an ephemeris point (including covariance matrix) to the end of an EXTEPHEM's set of ephemeris points
    pub fn ExtEphAddSatEphemCovMtx(
        satKey: i64,
        ds50UTC: f64,
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        revNum: i32,
        covUVW: *const [f64; 21],
    ) -> i32;
    //  Adds an ephemeris point (including covariance matrix) to the end of an EXTEPHEM's set of ephemeris points
    pub fn ExtEphAddSatEphemExt(
        satKey: i64,
        ds50UTC: f64,
        pos: *const [f64; 3],
        vel: *const [f64; 3],
        revNum: i32,
        extArr: *const [f64; 128],
    ) -> i32;
    //  Loads satellite data from an external ephemeris file (any valid external ephemeris file formats) and returns a satKey on success
    pub fn ExtEphAddSatFrFile(extEphFile: *const c_char) -> i64;
    //  Gets number of epehemeris points associated with satKey
    pub fn ExtEphGetNumPts(satKey: i64, numOfPts: *mut i32) -> i32;
    //  Retrieves all data for an EXTEPHEM with a single function call
    pub fn ExtEphGetAllFields(
        satKey: i64,
        satNum: *mut i32,
        satName: *const c_char,
        recName: *const c_char,
        epochDs50UTC: *mut f64,
        ae: *mut f64,
        ke: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        coordSys: *mut i32,
        numOfPts: *mut i32,
        fileLoc: *const c_char,
    ) -> i32;
    //  Retrieves the value of a specific field of an EXTEPHEM
    //  <br>
    //  When using xf_ExtEph = 11, the input coordinate system is returned as an integer value.  The table below shows the coordinate system values:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Value</b></td>
    //  <td><b>Coordinate System</b></td>
    //  </tr>
    //  <tr><td>1  </td><td>ECI TEME of DATE</td></tr>
    //  <tr><td>2  </td><td>MEME of J2K</td></tr>
    //  <tr><td>3  </td><td>Earth Fixed Greenwich (EFG)</td></tr>
    //  <tr><td>4  </td><td>Earch Centered Rotation (ECR)</td></tr>
    //  <tr><td>100</td><td>Invalid</td></tr>
    //  </table>
    pub fn ExtEphGetField(satKey: i64, xf_ExtEph: i32, valueStr: *const c_char) -> i32;
    //  Updates the value of a specific field of an EXTEPHEM
    pub fn ExtEphSetField(satKey: i64, xf_ExtEph: i32, valueStr: *const c_char) -> i32;
    //  Retrieves the times (in days since 1950 UTC) of the start and end ephemeris points of the EXTEPHEM
    pub fn ExtEphStartEndTime(satKey: i64, startDs50UTC: *mut f64, endDs50UTC: *mut f64) -> i32;
    //  Retrieves the data for a specific point within an EXTEPHEM
    //  It is important to know that the array subscript starts at one (not zero).
    pub fn ExtEphGetEphemeris(
        satKey: i64,
        index: i32,
        ds50UTC: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        revNum: *mut i32,
    ) -> i32;
    //  Retrieves the data (including the covariance matrix) for a specific point within an EXTEPHEM
    pub fn ExtEphGetCovMtx(
        satKey: i64,
        index: i32,
        ds50UTC: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        revNum: *mut i32,
        covMtx: *mut [[f64; 6]; 6],
    ) -> i32;
    //  Interpolates the external ephemeris data to the requested time in minutes since the satellite's epoch time
    //  The coordinate system of the output position and velocity is the same as the input ephemerides.
    pub fn ExtEphMse(
        satKey: i64,
        mse: f64,
        ds50UTC: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        revNum: *mut i32,
    ) -> i32;
    //  Interpolates the external ephemeris data to the requested time in minutes since the satellite's epoch time
    pub fn ExtEphMseCovMtx(
        satKey: i64,
        mse: f64,
        ds50UTC: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        revNum: *mut i32,
        covMtx: *mut [[f64; 6]; 6],
    ) -> i32;
    //  Interpolates the external ephemeris data to the requested time in days since 1950, UTC
    //  The coordinate system of the output position and velocity is the same as the input ephemerides.
    pub fn ExtEphDs50UTC(
        satKey: i64,
        ds50UTC: f64,
        mse: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        revNum: *mut i32,
    ) -> i32;
    //  Interpolates the external ephemeris data to the requested time in days since 1950, UTC
    pub fn ExtEphDs50UTCCovMtx(
        satKey: i64,
        ds50UTC: f64,
        mse: *mut f64,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        revNum: *mut i32,
        covMtx: *mut [[f64; 6]; 6],
    ) -> i32;
    //  Extensible routine which retrieves/interpolates external ephemeris data based on user's request
    pub fn ExtEphXten(satKey: i64, xf_getEph: i32, inVal: f64, extArr: *mut [f64; 128]) -> i32;
    //  This function returns a string that represents the EXTFIL= directive used to read a particular EXTEPHEM
    pub fn ExtEphGetLine(satKey: i64, line: *const c_char) -> i32;
    //  Returns the first satKey that matches the satNum in the EXTEPHEM binary tree
    //  This function is useful when ExtEphem DLL is used in applications that requires only one record (one EXTEPHEM entry)
    //  for one satellite and the applications refer to that EXTEPHEM by its satellite number.
    //  However, the Astrodynamic Standard Shared library only uses satKeys; this function helps to return the associated satKey of that satellite.
    pub fn ExtEphGetSatKey(satNum: i32) -> i64;
    //  Creates satKey from EXTEPHEM's satelite number and date time group string
    //  This is the proper way to reconstruct a satKey from its fields. If the users use their own routine to do this, the computed satKey might be different.
    pub fn ExtEphFieldsToSatKey(satNum: i32, epochDtg: *const c_char) -> i64;
}

// Indexes of coordinate systems
// ECI TEME of DATE
pub static COORD_ECI: i32 = 1;
// MEME of J2K
pub static COORD_J2K: i32 = 2;
// Earth Fixed Greenwich (EFG)
pub static COORD_EFG: i32 = 3;
// Earch Centered Rotation (ECR)
pub static COORD_ECR: i32 = 4;
// Lat Lon Height and a vector offset (range, azimuth, elevation)
pub static COORD_LLH: i32 = 5;
// Sensor site (ECR) and a vector offset (range, azimuth, elevation)
pub static COORD_SEN: i32 = 6;

// ECI TEME of DATE, fixed point
pub static COORD_ECIFP: i32 = 11;
// MEME of J2K, fixed point
pub static COORD_J2KFP: i32 = 12;
// Earth Fixed Greenwich (EFG), fixed point
pub static COORD_EFGFP: i32 = 13;
// Earch Centered Rotation (ECR), fixed point
pub static COORD_ECRFP: i32 = 14;
// Lat Lon Height and an offset vector (range, azimuth, elevation)
pub static COORD_LLHOV: i32 = 15;
// Sensor site (ECR) and an offset vector (range, azimuth, elevation)
pub static COORD_SENOV: i32 = 16;
// Current position (LLH), heading (azimuth), and constant speed of an mobile object that travels in a rhumb line course
pub static COORD_HCSRL: i32 = 17;
// List of waypoints (LLH) that describes the movement of an object that travels in a rhumb line course
pub static COORD_WPTRL: i32 = 18;
// Current position (LLH), initial heading (azimuth), and constant speed of an mobile object that travels in a great circle course
pub static COORD_HCSGC: i32 = 19;
// List of waypoints (LLH) that describes the movement of an object that travels in a great circle course
pub static COORD_WPTGC: i32 = 20;

// Invalid coordinate system
pub static COORD_INVALID: i32 = 100;

// UVW convariance matrix - TEME of DATE
pub static COVMTX_UVW_DATE: i32 = 0;
// Cartesian covariance matrix - TEME of DATE
pub static COVMTX_XYZ_DATE: i32 = 10;
// Equinoctial covariance matrix - TEME of DATE
pub static COVMTX_EQNX_DATE: i32 = 20;
// UVW convariance matrix - MEME of J2K
pub static COVMTX_UVW_J2K: i32 = 30;
// Cartesian covariance matrix - MEME of J2K
pub static COVMTX_XYZ_J2K: i32 = 40;
// Equinoctial covariance matrix - MEME of J2K
pub static COVMTX_EQNX_J2K: i32 = 50;

// Get ephemeris data using time in minutes since epoch
pub static XF_GETEPH_MSE: i32 = 1;
// Get ephemeris data using time in days since 1950 UTC
pub static XF_GETEPH_UTC: i32 = 2;
// Get ephemeris data using index of the element in the array
pub static XF_GETEPH_IDX: i32 = 3;

// Indexes of EXTEPH data fields
// Satellite number I5
pub static XF_EXTEPH_SATNUM: i32 = 1;
// Epoch YYDDDHHMMSS.SSS
pub static XF_EXTEPH_EPOCH: i32 = 2;
// Earth radius (km)
pub static XF_EXTEPH_AE: i32 = 3;
// Ke
pub static XF_EXTEPH_KE: i32 = 4;
// position X (km) F16.8
pub static XF_EXTEPH_POSX: i32 = 5;
// position Y (km) F16.8
pub static XF_EXTEPH_POSY: i32 = 6;
// position Z (km) F16.8
pub static XF_EXTEPH_POSZ: i32 = 7;
// velocity X (km/s) F16.12
pub static XF_EXTEPH_VELX: i32 = 8;
// velocity Y (km/s) F16.12
pub static XF_EXTEPH_VELY: i32 = 9;
// velocity Z (km/s) F16.12
pub static XF_EXTEPH_VELZ: i32 = 10;
// Input coordinate systems
pub static XF_EXTEPH_COORD: i32 = 11;
// Num of ephemeris points
pub static XF_EXTEPH_NUMOFPTS: i32 = 12;
// Ephemeris file path
pub static XF_EXTEPH_FILEPATH: i32 = 13;
// International Designator
pub static XF_EXTEPH_SATNAME: i32 = 14;
// Record name
pub static XF_EXTEPH_RECNAME: i32 = 15;

// ========================= End of auto generated code ==========================

pub fn add_satellite(sat_num: i32, epoch_ds50_utc: f64, ae: f64, ke: f64, coord_sys: i32) -> i64 {
    unsafe { ExtEphAddSat(sat_num, epoch_ds50_utc, ae, ke, coord_sys) }
}

pub fn add_satellite_state(
    sat_key: i64,
    ds50_utc: f64,
    pos: &[f64; 3],
    vel: &[f64; 3],
    cov: Option<&[f64; 21]>,
) -> Result<(), String> {
    let status = match cov {
        Some(cov) => unsafe { ExtEphAddSatEphemCovMtx(sat_key, ds50_utc, pos, vel, 0, cov) },
        None => unsafe { ExtEphAddSatEphem(sat_key, ds50_utc, pos, vel, 0) },
    };
    match status {
        0 => Ok(()),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

pub fn remove_key(key: i64) {
    unsafe {
        ExtEphRemoveSat(key);
    }
}

pub fn get_posvel_at_ds50(key: i64, ds50_utc: f64) -> Result<([f64; 3], [f64; 3]), String> {
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    let mut rev_num = 0;
    let mut mse = 0.0;
    let status = unsafe { ExtEphDs50UTC(key, ds50_utc, &mut mse, &mut pos, &mut vel, &mut rev_num) };
    match status {
        0 => Ok((pos, vel)),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

pub fn get_posvel_at_index(key: i64, index: i32) -> Result<([f64; 3], [f64; 3]), String> {
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    let mut rev_num = 0;
    let mut ds50_utc = 0.0;
    let status = unsafe { ExtEphGetEphemeris(key, index, &mut ds50_utc, &mut pos, &mut vel, &mut rev_num) };
    match status {
        0 => Ok((pos, vel)),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

pub fn get_number_of_states(key: i64) -> Result<i32, String> {
    let mut num_of_pts = 0;
    let status = unsafe { ExtEphGetNumPts(key, &mut num_of_pts) };
    match status {
        0 => Ok(num_of_pts),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

pub fn get_ds50_utc_range(key: i64) -> Result<(f64, f64), String> {
    let mut start_ds50_utc = 0.0;
    let mut end_ds50_utc = 0.0;
    let status = unsafe { ExtEphStartEndTime(key, &mut start_ds50_utc, &mut end_ds50_utc) };
    match status {
        0 => Ok((start_ds50_utc, end_ds50_utc)),
        _ => Err(MainInterface::get_last_error_message()),
    }
}

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct EphemInterface {}

impl EphemInterface {
    pub fn add_satellite(sat_num: i32, epoch_ds50_utc: f64, ae: f64, ke: f64, coord_sys: i32) -> Result<i64, String> {
        let _lock = EXTEPH_LOCK.lock().unwrap();
        let key = unsafe { ExtEphAddSat(sat_num, epoch_ds50_utc, ae, ke, coord_sys) };
        if key >= 0 {
            Ok(key)
        } else {
            Err(MainInterface::get_last_error_message())
        }
    }
    pub fn add_state(
        sat_key: i64,
        ds50_utc: f64,
        pos: &[f64; 3],
        vel: &[f64; 3],
        cov: Option<&[f64; 21]>,
    ) -> Result<(), String> {
        let status = match cov {
            Some(cov) => unsafe { ExtEphAddSatEphemCovMtx(sat_key, ds50_utc, pos, vel, 0, cov) },
            None => unsafe { ExtEphAddSatEphem(sat_key, ds50_utc, pos, vel, 0) },
        };
        match status {
            0 => Ok(()),
            _ => Err(MainInterface::get_last_error_message()),
        }
    }
    pub fn get_posvel_at_ds50(key: i64, ds50_utc: f64) -> Result<([f64; 3], [f64; 3]), String> {
        let mut pos = [0.0; 3];
        let mut vel = [0.0; 3];
        let mut rev_num = 0;
        let mut mse = 0.0;
        let status = unsafe { ExtEphDs50UTC(key, ds50_utc, &mut mse, &mut pos, &mut vel, &mut rev_num) };
        match status {
            0 => Ok((pos, vel)),
            _ => Err(MainInterface::get_last_error_message()),
        }
    }
    pub fn get_state_at_index(key: i64, index: i32) -> Result<(f64, [f64; 3], [f64; 3], [[f64; 6]; 6]), String> {
        let mut pos = [0.0; 3];
        let mut vel = [0.0; 3];
        let mut cov = [[0.0; 6]; 6];
        let mut rev_num = 0;
        let mut ds50_utc = 0.0;
        let status = unsafe { ExtEphGetCovMtx(key, index, &mut ds50_utc, &mut pos, &mut vel, &mut rev_num, &mut cov) };
        match status {
            0 => Ok((ds50_utc, pos, vel, cov)),
            _ => Err(MainInterface::get_last_error_message()),
        }
    }

    pub fn get_number_of_states(key: i64) -> Result<i32, String> {
        let mut num_of_pts = 0;
        let status = unsafe { ExtEphGetNumPts(key, &mut num_of_pts) };
        match status {
            0 => Ok(num_of_pts),
            _ => Err(MainInterface::get_last_error_message()),
        }
    }

    pub fn get_ds50_utc_range(key: i64) -> Result<(f64, f64), String> {
        let mut start_ds50_utc = 0.0;
        let mut end_ds50_utc = 0.0;
        let status = unsafe { ExtEphStartEndTime(key, &mut start_ds50_utc, &mut end_ds50_utc) };
        match status {
            0 => Ok((start_ds50_utc, end_ds50_utc)),
            _ => Err(MainInterface::get_last_error_message()),
        }
    }

    pub fn clone(key: i64) -> Result<i64, String> {
        let mut sat_num = 0;
        let mut sat_name = [0 as c_char; 100];
        let mut rec_name = [0 as c_char; 100];
        let mut epoch_ds50_utc = 0.0;
        let mut ae = 0.0;
        let mut ke = 0.0;
        let mut pos = [0.0; 3];
        let mut vel = [0.0; 3];
        let mut coord_sys = 0;
        let mut num_of_pts = 0;
        let mut file_loc = [0 as c_char; 260];

        let status = unsafe {
            ExtEphGetAllFields(
                key,
                &mut sat_num,
                sat_name.as_mut_ptr(),
                rec_name.as_mut_ptr(),
                &mut epoch_ds50_utc,
                &mut ae,
                &mut ke,
                &mut pos,
                &mut vel,
                &mut coord_sys,
                &mut num_of_pts,
                file_loc.as_mut_ptr(),
            )
        };
        if status != 0 {
            return Err(MainInterface::get_last_error_message());
        }

        let new_key = EphemInterface::add_satellite(sat_num, epoch_ds50_utc, ae, ke, coord_sys)?;
        for i in 0..num_of_pts {
            let (ds50, pos, vel, _) = EphemInterface::get_state_at_index(key, i + 1)?;
            EphemInterface::add_state(new_key, ds50, &pos, &vel, None)?;
        }
        Ok(new_key)
    }

    pub fn remove_key(key: i64) {
        unsafe {
            ExtEphRemoveSat(key);
        }
    }
}

#[pymethods]
impl EphemInterface {
    #[classattr]
    pub const COORD_ECI: i32 = COORD_ECI;

    #[classattr]
    pub const COORD_J2K: i32 = COORD_J2K;

    #[classattr]
    pub const COORD_EFG: i32 = COORD_EFG;

    #[classattr]
    pub const COORD_ECR: i32 = COORD_ECR;

    #[staticmethod]
    #[pyo3(name = "add_satellite")]
    pub fn py_add_satellite(sat_num: i32, epoch_ds50_utc: f64, ae: f64, ke: f64, coord_sys: i32) -> PyResult<i64> {
        EphemInterface::add_satellite(sat_num, epoch_ds50_utc, ae, ke, coord_sys).map_err(PyException::new_err)
    }

    #[staticmethod]
    #[pyo3(name = "add_state")]
    pub fn py_add_state(
        sat_key: i64,
        ds50_utc: f64,
        pos: [f64; 3],
        vel: [f64; 3],
        cov: Option<[f64; 21]>,
    ) -> PyResult<()> {
        EphemInterface::add_state(sat_key, ds50_utc, &pos, &vel, cov.as_ref()).map_err(PyException::new_err)
    }

    #[staticmethod]
    #[pyo3(name = "remove_key")]
    pub fn py_remove_key(key: i64) {
        EphemInterface::remove_key(key);
    }

    #[staticmethod]
    #[pyo3(name = "get_posvel_at_ds50")]
    pub fn py_get_posvel_at_ds50(key: i64, ds50_utc: f64) -> PyResult<([f64; 3], [f64; 3])> {
        EphemInterface::get_posvel_at_ds50(key, ds50_utc).map_err(PyException::new_err)
    }

    #[staticmethod]
    #[pyo3(name = "get_posvel_at_index")]
    pub fn py_get_posvel_at_index(key: i64, index: i32) -> PyResult<(f64, [f64; 3], [f64; 3], [[f64; 6]; 6])> {
        EphemInterface::get_state_at_index(key, index).map_err(PyException::new_err)
    }

    #[staticmethod]
    #[pyo3(name = "get_number_of_states")]
    pub fn py_get_number_of_states(key: i64) -> PyResult<i32> {
        EphemInterface::get_number_of_states(key).map_err(PyException::new_err)
    }
}
