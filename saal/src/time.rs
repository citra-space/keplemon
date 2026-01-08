// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::{GetSetString, get_last_error_message};
use std::os::raw::c_char;

unsafe extern "C" {
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

/// Return the TimeFunc DLL info string (version, build date, platform).
///
/// Example:
/// ```rust
/// let info = saal::time::get_dll_info();
/// println!("{}", info.contains(saal::DLL_VERSION));
/// ```
///
/// Output:
/// ```bash
/// true
/// ```
pub fn get_dll_info() -> String {
    let mut info_str = GetSetString::new();
    unsafe {
        TimeFuncGetInfo(info_str.pointer());
    }
    info_str.value()
}

/// Convert UTC date/time components to ds50 UTC.
///
/// Units: returns days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let ds50 = saal::time::ymd_components_to_ds50(1956, 1, 1, 0, 0, 0.0);
/// println!("{ds50:.1}");
/// ```
///
/// Output:
/// ```bash
/// 2192.0
/// ```
pub fn ymd_components_to_ds50(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: f64) -> f64 {
    unsafe { TimeComps2ToUTC(year, month, day, hour, minute, second) }
}

/// Convert ds50 UTC to UTC date/time components.
///
/// Units: input is days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let (year, month, day, hour, minute, second) = saal::time::ds50_to_ymd_components(2192.0);
/// println!("{year}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:06.3}");
/// ```
///
/// Output:
/// ```bash
/// 1956-01-01 00:00:00.000
/// ```
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

/// Convert a DTG string (DTG15/17/19/20) to ds50 UTC.
///
/// Units: returns days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let ds50 = saal::time::dtg_to_ds50("1956/001 0000 00.000");
/// println!("{ds50:.1}");
/// ```
///
/// Output:
/// ```bash
/// 2192.0
/// ```
pub fn dtg_to_ds50(dtg: &str) -> f64 {
    let mut inout: GetSetString = dtg.into();
    unsafe { DTGToUTC(inout.pointer()) }
}

/// Convert ds50 UTC to a DTG20 string.
///
/// Units: input is days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let dtg = saal::time::ds50_to_dtg20(2192.0);
/// println!("{dtg}");
/// ```
///
/// Output:
/// ```bash
/// 1956/001 0000 00.000
/// ```
pub fn ds50_to_dtg20(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG20(ds50, inout.pointer()) };
    inout.value()
}

/// Convert ds50 UTC to a DTG19 string.
///
/// Units: input is days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let dtg = saal::time::ds50_to_dtg19(2192.0);
/// println!("{dtg}");
/// ```
///
/// Output:
/// ```bash
/// 1956Jan01000000.000
/// ```
pub fn ds50_to_dtg19(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG19(ds50, inout.pointer()) };
    inout.value()
}

/// Convert ds50 UTC to a DTG17 string.
///
/// Units: input is days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let dtg = saal::time::ds50_to_dtg17(2192.0);
/// println!("{dtg}");
/// ```
///
/// Output:
/// ```bash
/// 1956/001.00000000
/// ```
pub fn ds50_to_dtg17(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG17(ds50, inout.pointer()) };
    inout.value()
}

/// Convert ds50 UTC to a DTG15 string.
///
/// Units: input is days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let dtg = saal::time::ds50_to_dtg15(2192.0);
/// println!("{dtg}");
/// ```
///
/// Output:
/// ```bash
/// 56001000000.000
/// ```
pub fn ds50_to_dtg15(ds50: f64) -> String {
    let mut inout = GetSetString::new();
    unsafe { UTCToDTG15(ds50, inout.pointer()) };
    inout.value()
}

/// Convert year and day-of-year to ds50 UTC.
///
/// Units: returns days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let ds50 = saal::time::year_doy_to_ds50(1956, 1.0);
/// println!("{ds50:.1}");
/// ```
///
/// Output:
/// ```bash
/// 2192.0
/// ```
pub fn year_doy_to_ds50(year: i32, doy: f64) -> f64 {
    unsafe { YrDaysToUTC(year, doy) }
}

/// Convert ds50 UTC to year and day-of-year.
///
/// Units: input is days since 1950-01-01 00:00:00 UTC.
///
/// Example:
/// ```rust
/// let (year, doy) = saal::time::ds50_to_year_doy(2192.0);
/// println!("{year} {doy:.1}");
/// ```
///
/// Output:
/// ```bash
/// 1956 1.0
/// ```
pub fn ds50_to_year_doy(ds50: f64) -> (i32, f64) {
    let mut year = 0;
    let mut doy = 0.0;
    unsafe { UTCToYrDays(ds50, &mut year, &mut doy) };
    (year, doy)
}

/// Convert ds50 TAI to ds50 UTC using loaded time constants.
///
/// Units: days since 1950-01-01 00:00:00 (TAI in, UTC out).
///
/// Example:
/// ```rust
/// let utc = saal::time::tai_to_utc(8431.000138888889);
/// println!("{utc:.1}");
/// ```
///
/// Output:
/// ```bash
/// 8431.0
/// ```
pub fn tai_to_utc(ds50_tai: f64) -> f64 {
    unsafe { TAIToUTC(ds50_tai) }
}

/// Convert ds50 UTC to ds50 TAI using loaded time constants.
///
/// Units: days since 1950-01-01 00:00:00 (UTC in, TAI out).
///
/// Example:
/// ```rust
/// let tai = saal::time::utc_to_tai(8431.0);
/// println!("{tai:.15}");
/// ```
///
/// Output:
/// ```bash
/// 8431.000138888889
/// ```
pub fn utc_to_tai(ds50_utc: f64) -> f64 {
    unsafe { UTCToTAI(ds50_utc) }
}

/// Convert ds50 UTC to ds50 UT1 using loaded time constants.
///
/// If no timing constants are loaded, the input is returned unchanged.
/// Units: days since 1950-01-01 00:00:00 (UTC in, UT1 out).
///
/// Example:
/// ```rust
/// let ut1 = saal::time::utc_to_ut1(8431.0);
/// println!("{ut1:.11}");
/// ```
///
/// Output:
/// ```bash
/// 8431.00000830081
/// ```
pub fn utc_to_ut1(ds50_utc: f64) -> f64 {
    unsafe { UTCToUT1(ds50_utc) }
}

/// Convert ds50 UTC to ds50 TT (ET) using loaded time constants.
///
/// Units: days since 1950-01-01 00:00:00 (UTC in, TT out).
///
/// Example:
/// ```rust
/// let tt = saal::time::utc_to_tt(8431.0);
/// println!("{tt:.11}");
/// ```
///
/// Output:
/// ```bash
/// 8431.00051138889
/// ```
pub fn utc_to_tt(ds50_utc: f64) -> f64 {
    unsafe { UTCToET(ds50_utc) }
}

/// Convert ds50 TAI to ds50 UT1 using loaded time constants.
///
/// Units: days since 1950-01-01 00:00:00 (TAI in, UT1 out).
///
/// Example:
/// ```rust
/// let ut1 = saal::time::tai_to_ut1(8431.000138888889);
/// println!("{ut1:.11}");
/// ```
///
/// Output:
/// ```bash
/// 8431.00000830081
/// ```
pub fn tai_to_ut1(ds50_tai: f64) -> f64 {
    unsafe { TAIToUT1(ds50_tai) }
}

/// Load timing constants from a file.
///
/// Example:
/// ```rust
/// let path = std::env::temp_dir().join("saal_missing_timing.txt");
/// let _ = std::fs::remove_file(&path);
/// let result = saal::time::load_constants(path.to_str().unwrap());
/// println!("{}", result.is_err());
/// ```
///
/// Output:
/// ```bash
/// true
/// ```
pub fn load_constants(path: &str) -> Result<(), String> {
    let path = std::ffi::CString::new(path).unwrap();
    let err_code = unsafe { TConLoadFile(path.as_ptr()) };
    if err_code == 0 {
        Ok(())
    } else {
        Err(get_last_error_message())
    }
}

/// Compute Greenwich right ascension using the FK4 catalog at ds50 UT1.
///
/// Units: returns radians; input is days since 1950-01-01 00:00:00 UT1.
///
/// Example:
/// ```rust
/// let utc = saal::time::ymd_components_to_ds50(1973, 1, 2, 0, 0, 0.0);
/// let ut1 = saal::time::utc_to_ut1(utc);
/// let ang = saal::time::get_fk4_greenwich_angle(ut1);
/// println!("{ang:.16}");
/// ```
///
/// Output:
/// ```bash
/// 1.7712987335192203
/// ```
pub fn get_fk4_greenwich_angle(ds50_ut1: f64) -> f64 {
    unsafe { ThetaGrnwchFK4(ds50_ut1) }
}

/// Compute Greenwich right ascension using the FK5 catalog at ds50 UT1.
///
/// Units: returns radians; input is days since 1950-01-01 00:00:00 UT1.
///
/// Example:
/// ```rust
/// let utc = saal::time::ymd_components_to_ds50(1973, 1, 2, 0, 0, 0.0);
/// let ut1 = saal::time::utc_to_ut1(utc);
/// let ang = saal::time::get_fk5_greenwich_angle(ut1);
/// println!("{ang:.16}");
/// ```
///
/// Output:
/// ```bash
/// 1.7713027012394775
/// ```
pub fn get_fk5_greenwich_angle(ds50_ut1: f64) -> f64 {
    unsafe { ThetaGrnwchFK5(ds50_ut1) }
}

/// Return whether timing constants are loaded.
///
/// Example:
/// ```rust
/// let loaded = saal::time::constants_loaded();
/// println!("{loaded}");
/// ```
///
/// Output:
/// ```bash
/// true
/// ```
pub fn constants_loaded() -> bool {
    unsafe { IsTConFileLoaded() != 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DLL_VERSION;
    use crate::test_lock::TEST_LOCK;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_get_dll_info() {
        let _lock = TEST_LOCK.lock().unwrap();
        let info = get_dll_info();
        assert!(info.contains(DLL_VERSION));
    }

    #[test]
    fn test_ymd_components_to_ds50() {
        let _lock = TEST_LOCK.lock().unwrap();
        let ds50 = ymd_components_to_ds50(1956, 1, 1, 0, 0, 0.0);
        assert_eq!(ds50, 2192.0);
    }

    #[test]
    fn test_ds50_to_ymd_components() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (year, month, day, hour, minute, second) = ds50_to_ymd_components(2192.0);
        assert_eq!((year, month, day, hour, minute, second), (1956, 1, 1, 0, 0, 0.0));
    }

    #[test]
    fn test_dtg_to_ds50() {
        let _lock = TEST_LOCK.lock().unwrap();
        let ds50 = dtg_to_ds50("1956/001 0000 00.000");
        assert_eq!(ds50, 2192.0);
    }

    #[test]
    fn test_ds50_to_dtg_formats() {
        let _lock = TEST_LOCK.lock().unwrap();
        assert_eq!(ds50_to_dtg20(2192.0), "1956/001 0000 00.000");
        assert_eq!(ds50_to_dtg19(2192.0), "1956Jan01000000.000");
        assert_eq!(ds50_to_dtg17(2192.0), "1956/001.00000000");
        assert_eq!(ds50_to_dtg15(2192.0), "56001000000.000");
    }

    #[test]
    fn test_year_doy_conversions() {
        let _lock = TEST_LOCK.lock().unwrap();
        let ds50 = year_doy_to_ds50(1956, 1.0);
        assert_eq!(ds50, 2192.0);
        let (year, doy) = ds50_to_year_doy(2192.0);
        assert_eq!((year, doy), (1956, 1.0));
    }

    #[test]
    fn test_constants_loaded() {
        let _lock = TEST_LOCK.lock().unwrap();
        assert!(constants_loaded());
    }

    #[test]
    fn test_conversions() {
        let _lock = TEST_LOCK.lock().unwrap();
        let utc = ymd_components_to_ds50(1973, 1, 30, 0, 0, 0.0);
        let tai = 8431.000138888889;
        let ut1 = 8431.00000830081;
        let tt = 8431.00051138889;

        assert_abs_diff_eq!(utc_to_tai(utc), tai, epsilon = 1.0e-10);
        assert_abs_diff_eq!(tai_to_utc(tai), utc, epsilon = 1.0e-10);
        assert_abs_diff_eq!(utc_to_ut1(utc), ut1, epsilon = 1.0e-10);
        assert_abs_diff_eq!(utc_to_tt(utc), tt, epsilon = 1.0e-10);
        assert_abs_diff_eq!(tai_to_ut1(tai), ut1, epsilon = 1.0e-10);
    }

    #[test]
    fn test_greenwich_angles() {
        let _lock = TEST_LOCK.lock().unwrap();
        let utc = ymd_components_to_ds50(1973, 1, 2, 0, 0, 0.0);
        let ut1 = utc_to_ut1(utc);
        let fk4 = get_fk4_greenwich_angle(ut1);
        let fk5 = get_fk5_greenwich_angle(ut1);

        assert_abs_diff_eq!(fk4, 1.7712987335192203, epsilon = 1.0e-7);
        assert_abs_diff_eq!(fk5, 1.7713027012394775, epsilon = 1.0e-7);
    }
}
