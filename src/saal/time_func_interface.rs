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
    //  Initializes the TimeFunc DLL for use in the program.
    //  If this function returns an error, it is recommended that you stop the program immediately.
    //
    //  An error will occur if you forget to load and initialize all the prerequisite DLLs, as listed in the DLL Prerequisites section of the accompanying documentation, before using this DLL.
    pub fn TimeFuncInit(apAddr: i64) -> i32;
    //  Returns the information about the TimeFunc DLL.  The information is placed in the string parameter you pass in.
    //  The returned string provides information about the version number, build date, and the platform of the TimeFunc DLL.
    pub fn TimeFuncGetInfo(infoStr: *const c_char);
    //  Loads timing constants data from an input file.
    //  Time constants can be included directly in the main input file or they can be read from a separate file identified with "TIMFIL=[pathname\filename]".
    //
    //  The input file is read in two passes. The function first looks for "TIMFIL=" lines, then it looks for timing constant data which was included directly. The result of this is that data entered using both methods will be processed, but the "TIMFIL=" data will be processed first.
    //
    //  The time constants are also read in from each VCM. However, only the most recent time constants among VCMs are stored in the memory, see VCM.dll documentation.
    //  See the "Time Constants Data Description" section in the accompanying TimeFunc documentation file for supported formats.
    pub fn TConLoadFile(tconFile: *const c_char) -> i32;
    //  Loads timing constants data and prediction control (6P-card) from an input file.
    //  Time constants can be included directly in the main input file or they can be read from a separate file identified with "TIMFIL=[pathname\filename]".
    //
    //  The input file is read in two passes. The function first looks for "TIMFIL=" lines, then it looks for timing constant data which was included directly. The result of this is that data entered using both methods will be processed, but the "TIMFIL=" data will be processed first.
    //
    //  The time constants are also read in from each VCM. However, only the most recent time constants among VCMs are stored in the memory, see VCM.dll documentation.
    pub fn TimeFuncLoadFile(tconFile: *const c_char) -> i32;
    //  Checks to see if timing constants have been loaded into memory.
    //  The timing constants can be loaded from a timing constants file or from VCM(s).  See TConLoadFile, TConAddOne, and TConAddARec functions.
    pub fn IsTConFileLoaded() -> i32;
    //  Saves currently loaded timing constants data to a file.
    //  The data will be saved in the format specified by the form parameter, regardless of the format or method originally used to load it.
    pub fn TConSaveFile(tconFile: *const c_char, saveMode: i32, saveForm: i32) -> i32;
    //  Adds a timing constant record to memory. Note that this function is solely for backward compatible with legacy software.
    //  Notes: only the latest timing record is stored in memory using this method. Input timing record will be skipped/ignored if it's earlier than the existing one
    //  The users should use TConLoadFile or TimeFuncLoadFile to load timing constants file instead.
    pub fn TConAddARec(
        refDs50UTC: f64,
        leapDs50UTC: f64,
        taiMinusUTC: f64,
        ut1MinusUTC: f64,
        ut1Rate: f64,
        polarX: f64,
        polarY: f64,
    ) -> i32;
    //  Adds one timing constant record to memory. This API can be used to avoid TConLoadFile's file I/O
    pub fn TConAddOne(
        refDs50UTC: f64,
        taiMinusUTC: f64,
        ut1MinusUTC: f64,
        ut1Rate: f64,
        polarX: f64,
        polarY: f64,
    ) -> i32;
    //  Retrieves the timing constants record, if exists, at the requested input time in ds50UTC.
    //  If the requested record is not found, 0's will be returned for all of the constants. You can use this fact to determine whether the record was found or not. Simply check the taiMinusUTC value after calling this function. Since that value can never be 0 for a valid record, if it is 0 the record was not found.
    pub fn UTCToTConRec(
        ds50UTC: f64,
        taiMinusUTC: *mut f64,
        ut1MinusUTC: *mut f64,
        ut1Rate: *mut f64,
        polarX: *mut f64,
        polarY: *mut f64,
    );
    //  Removes all the timing constants records in memory.
    pub fn TConRemoveAll() -> i32;
    //  Converts an internal time in ds50UTC to a string in DTG20 format. The resulting string takes the form "YYYY/DDD HHMM SS.SSS".
    //  The input ds50UTC must be greater than 2192.0, which corresponds to a time later than 1st Jan 1956. Any input value less than or equal to 2192.0 will yield "1956/001 0000 00.000".
    pub fn UTCToDTG20(ds50UTC: f64, dtg20: *const c_char);
    //  Converts a time in ds50UTC to a time in DTG19 format. The resulting string takes the form "YYYYMonDDHHMMSS.SSS".
    //  See "UTCToDTG20" for an example.
    //  The input ds50UTC must be greater than 2192.0, which corresponds to a time later than 1st Jan 1956. Any input value less than or equal to 2192.0 will yield "1956Jan01000000.000".
    //  Note, the return value is in the DTG19 format "YYYYMonDDHHMMSS.SSS", not the "YY DDD HH MM SS.SSS" format.
    pub fn UTCToDTG19(ds50UTC: f64, dtg19: *const c_char);
    //  Converts a time in ds50UTC to a time in DTG17 format. The resulting string takes the form "YYYY/DDD.DDDDDDDD" format.
    //  See "UTCToDTG20" for an example.
    //  The input ds50UTC must be greater than 2192.0, which corresponds to a time later than 1st Jan 1956. Any input value less than or equal to 2192.0 will yield "1956/001.00000000".
    pub fn UTCToDTG17(ds50UTC: f64, dtg17: *const c_char);
    //  Converts a time in ds50UTC to a time in DTG15 format. The resulting string takes the form "YYDDDHHMMSS.SSS".
    //  See "UTCToDTG20" for an example.
    //  The input ds50UTC must be greater than 2192.0, which corresponds to a time later than 1st Jan 1956. Any input value less than or equal to 2192.0 will yield "56001000000.000".
    pub fn UTCToDTG15(ds50UTC: f64, dtg15: *const c_char);
    //  Converts a time in one of the DTG formats to a time in ds50UTC. DTG15, DTG17, DTG19, and DTG20 formats are accepted.
    //  See "UTCToDTG20" for an example.
    //  During the conversion, this function processes only numbers and the '.' character. This means that you can format dtgStr in a format that makes sense. You can use spaces and the '/' character for example if you wish.
    //
    //  The function can process dates from 1950 to 2049. Any input outside this range will yield "0.0".
    //  This function supports DTG19 inputs in both "YY DDD HH MM SS.SSS" and "YYYYMonDDHHMMSS.SSS" formats.
    pub fn DTGToUTC(dtg: *const c_char) -> f64;
    //  Converts a time in ds50UTC to a time in ds50TAI using timing constants records in memory.
    //  If no timing constants records were loaded, ds50UTC and ds50TAI are the same.
    //  Partial days may be returned.
    pub fn UTCToTAI(ds50UTC: f64) -> f64;
    //  Converts a time in ds50UTC to a time in ds50UT1 using timing constants records in memory.
    //  If no timing constants records were loaded, ds50UTC and ds50UT1 are the same.
    //  Partial days may be returned.
    pub fn UTCToUT1(ds50UTC: f64) -> f64;
    //  Converts a time in ds50UTC to a time in ds50ET (Ephemeris Time/Terrestrial Time) using timing constants records in memory.
    pub fn UTCToET(ds50UTC: f64) -> f64;
    //  Converts a time in ds50TAI to a time in ds50UTC using timing constants records in memory.
    //  If no timing constants records were loaded, ds50TAI and ds50UTC are the same.
    //  Partial days may be returned.
    pub fn TAIToUTC(ds50TAI: f64) -> f64;
    //  Converts a time in ds50TAI to a time in ds50UT1 using timing constants records in memory.
    //  If no timing constants records were loaded, ds50TAI and ds50UT1 are the same.
    //  Partial days may be returned.
    pub fn TAIToUT1(ds50TAI: f64) -> f64;
    //  Converts a year and a number of days to a time in ds50UTC.
    //  Partial days may be returned.
    pub fn YrDaysToUTC(year: i32, dayOfYear: f64) -> f64;
    //  Converts a time in ds50UTC to a year and day of year.
    //  The input ds50UTC must be greater than 2192.0, which corresponds to a time later than 1st Jan 1956. Any input value less than or equal to 2192.0 will yield Year=1956, Day=1.0.
    pub fn UTCToYrDays(ds50UTC: f64, year: *mut i32, dayOfYear: *mut f64);
    //  Converts a set of time components (year, day of year, hour, minute, second) to a time in ds50UTC.
    //  Partial days may be returned.
    //  See "TimeComps2ToUTC" for a function which takes a month and day instead of a day of year value.
    pub fn TimeComps1ToUTC(year: i32, dayOfYear: i32, hh: i32, mm: i32, sss: f64) -> f64;
    //  Converts a time in ds50UTC to its individual components (year, day of year, hour, minute, second).
    //  See "TimeComps1ToUTC" for an example.
    //  The input ds50UTC must be greater than 2192.0, which corresponds to a time later than 1st Jan 1956. Any input value less than or equal to 2192.0 will be reset to that value.
    pub fn UTCToTimeComps1(
        ds50UTC: f64,
        year: *mut i32,
        dayOfYear: *mut i32,
        hh: *mut i32,
        mm: *mut i32,
        sss: *mut f64,
    );
    //  Converts a set of time components (year, month, day of month, hour, minute, second) to a time in ds50UTC.
    //  Partial days may be returned.
    //  See "TimeComps1ToUTC" for a function which takes a day of year value instead of a month and day.
    pub fn TimeComps2ToUTC(year: i32, mon: i32, dayOfMonth: i32, hh: i32, mm: i32, sss: f64) -> f64;
    //  Converts a time in ds50UTC to its individual components (year, month, day of month, hour, minute, second).
    //  See "TimeComps1ToUTC" for an example.
    //  The input ds50UTC must be greater than 2192.0, which corresponds to a time later than 1st Jan 1956. Any input value less than or equal to 2192.0 will be reset to that value.
    pub fn UTCToTimeComps2(
        ds50UTC: f64,
        year: *mut i32,
        month: *mut i32,
        dayOfMonth: *mut i32,
        hh: *mut i32,
        mm: *mut i32,
        sss: *mut f64,
    );
    //  Computes right ascension of Greenwich at the specified time in ds50UT1.
    //  The Fk constants as you currently have them set up in EnvConst.dll are used.
    //  EnvConst.dll is not marked as a direct dependency of TimeFunc.dll. However, it obviously must be loaded in order to be able to use this function since you must first obtain a handle via the EnvGetFkPtr() function.
    pub fn ThetaGrnwch(ds50UT1: f64, lenvFk: i64) -> f64;
    //  Computes right ascension of Greenwich at the specified time in ds50UT1 using the Fourth Fundamental Catalogue (FK4).
    //  There is no need to load or initialize EnvConst.dll when computing right ascension using this function.
    pub fn ThetaGrnwchFK4(ds50UT1: f64) -> f64;
    //  Computes right ascension of Greenwich at the specified time in ds50UT1 using the Fifth Fundamental Catalogue (FK5).
    //  There is no need to load or initialize EnvConst.dll when computing right ascension using this function.
    pub fn ThetaGrnwchFK5(ds50UT1: f64) -> f64;
    //  This function is intended for future use.  No information is currently available.
    //  This function is intended for future use.  No information is currently available.
    pub fn TimeConvFrTo(funcIdx: i32, frArr: *const f64, toArr: *mut f64);
    //  Returns prediction control parameters. The parameters are placed in the reference variables passed to this function.
    pub fn Get6P(
        startFrEpoch: *mut i32,
        stopFrEpoch: *mut i32,
        startTime: *mut f64,
        stopTime: *mut f64,
        interval: *mut f64,
    );
    //  Sets prediction control parameters.
    pub fn Set6P(startFrEpoch: i32, stopFrEpoch: i32, startTime: f64, stopTime: f64, interval: f64);
    //  Returns current prediction control parameters in form of a 6P-Card string.
    pub fn Get6PCardLine(card6PLine: *const c_char);
    //  Returns the time span of the loaded timing constants - the earliest and latest of loaded timing constant records
    pub fn TConTimeSpan(numOfRecs: *mut i32, frTimeDs50UTC: *mut f64, toTimeDs50UTC: *mut f64);
}
// ========================= End of auto generated code ==========================

#[inline]
pub fn ymd_components_to_ds50(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: f64) -> f64 {
    unsafe { TimeComps2ToUTC(year, month, day, hour, minute, second) }
}

#[pyfunction(name = "ymd_components_to_ds50")]
pub fn py_ymd_components_to_ds50(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: f64) -> f64 {
    ymd_components_to_ds50(year, month, day, hour, minute, second)
}

#[inline]
pub fn ds50_to_ymd_components(ds50: f64) -> (i32, i32, i32, i32, i32, f64) {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0;
    let mut minute = 0;
    let mut second = 0.0;
    unsafe {
        UTCToTimeComps2(
            ds50,
            &mut year,
            &mut month,
            &mut day,
            &mut hour,
            &mut minute,
            &mut second,
        )
    };
    (year, month, day, hour, minute, second)
}

#[inline]
pub fn dtg_to_ds50(dtg: &str) -> f64 {
    let mut inout = GetSetString::from_string(dtg);
    unsafe { DTGToUTC(inout.pointer()) }
}

#[inline]
pub fn ds50_to_dtg20(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG20(ds50, inout.pointer()) };
    inout.value()
}

#[inline]
pub fn ds50_to_dtg19(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG19(ds50, inout.pointer()) };
    inout.value()
}

#[inline]
pub fn ds50_to_dtg17(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG17(ds50, inout.pointer()) };
    inout.value()
}

#[inline]
pub fn ds50_to_dtg15(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG15(ds50, inout.pointer()) };
    inout.value()
}

#[inline]
pub fn year_doy_to_ds50(year: i32, doy: f64) -> f64 {
    unsafe { YrDaysToUTC(year, doy) }
}

#[inline]
pub fn ds50_to_year_doy(ds50: f64) -> (i32, f64) {
    let mut year = 0;
    let mut doy = 0.0;
    unsafe { UTCToYrDays(ds50, &mut year, &mut doy) };
    (year, doy)
}

#[inline]
pub fn ds50_tai_to_utc(ds50_tai: f64) -> f64 {
    unsafe { TAIToUTC(ds50_tai) }
}

#[inline]
pub fn ds50_utc_to_tai(ds50_utc: f64) -> f64 {
    unsafe { UTCToTAI(ds50_utc) }
}

#[inline]
pub fn ds50_utc_to_ut1(ds50_utc: f64) -> f64 {
    unsafe { UTCToUT1(ds50_utc) }
}

#[pyfunction(name = "ds50_utc_to_ut1")]
pub fn py_ds50_utc_to_ut1(ds50_utc: f64) -> f64 {
    ds50_utc_to_ut1(ds50_utc)
}

#[inline]
pub fn ds50_utc_to_tt(ds50_utc: f64) -> f64 {
    unsafe { UTCToET(ds50_utc) }
}

#[inline]
pub fn ds50_tai_to_ut1(ds50_tai: f64) -> f64 {
    unsafe { TAIToUT1(ds50_tai) }
}

pub fn load_time_constants(path: &str) -> Result<(), String> {
    let path = std::ffi::CString::new(path).unwrap();
    let err_code = unsafe { TConLoadFile(path.as_ptr()) };
    if err_code == 0 {
        Ok(())
    } else {
        Err(MainInterface::get_last_error_message())
    }
}

#[pyfunction(name = "load_time_constants")]
pub fn py_load_time_constants(path: String) -> PyResult<()> {
    match load_time_constants(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(PyErr::new::<SAALError, _>(e)),
    }
}

#[inline]
pub fn get_fk4_greenwich_angle(ds50_ut1: f64) -> f64 {
    unsafe { ThetaGrnwchFK4(ds50_ut1) }
}

#[pyfunction(name = "get_fk4_greenwich_angle")]
pub fn py_get_fk4_greenwich_angle(ds50_ut1: f64) -> f64 {
    get_fk4_greenwich_angle(ds50_ut1)
}

#[inline]
pub fn get_fk5_greenwich_angle(ds50_ut1: f64) -> f64 {
    unsafe { ThetaGrnwchFK5(ds50_ut1) }
}

#[pyfunction(name = "get_fk5_greenwich_angle")]
pub fn py_get_fk5_greenwich_angle(ds50_ut1: f64) -> f64 {
    get_fk5_greenwich_angle(ds50_ut1)
}

pub fn time_constants_loaded() -> bool {
    unsafe { IsTConFileLoaded() != 0 }
}

#[pyfunction(name = "time_constants_loaded")]
pub fn py_time_constants_loaded() -> bool {
    time_constants_loaded()
}

pub fn register_time_func_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let time_func_interface = PyModule::new(parent_module.py(), "time_func_interface")?;
    time_func_interface.add_function(wrap_pyfunction!(py_time_constants_loaded, &time_func_interface)?)?;
    time_func_interface.add_function(wrap_pyfunction!(py_load_time_constants, &time_func_interface)?)?;
    time_func_interface.add_function(wrap_pyfunction!(py_ds50_utc_to_ut1, &time_func_interface)?)?;
    time_func_interface.add_function(wrap_pyfunction!(py_get_fk4_greenwich_angle, &time_func_interface)?)?;
    time_func_interface.add_function(wrap_pyfunction!(py_get_fk5_greenwich_angle, &time_func_interface)?)?;
    time_func_interface.add_function(wrap_pyfunction!(py_ymd_components_to_ds50, &time_func_interface)?)?;
    py_run!(
        parent_module.py(),
        time_func_interface,
        "import sys; sys.modules['keplemon._keplemon.saal.time_func_interface'] = time_func_interface"
    );

    parent_module.add_submodule(&time_func_interface)
}
