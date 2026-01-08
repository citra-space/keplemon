// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod astro;
pub mod environment;
// pub mod ephemeris;
#[cfg(feature = "python")]
mod bindings;
mod get_set_string;
pub mod obs;
pub mod satellite;
pub mod sensor;
pub mod sgp4;
#[cfg(test)]
pub(crate) mod test_lock;
pub mod time;
pub mod tle;

use ctor::ctor;
pub use get_set_string::GetSetString;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::os::raw::c_char;
use std::path::PathBuf;

unsafe extern "C" {
    //  Returns information about the DllMain DLL.
    //  The returned string provides information about the version number, build date, and the platform.
    pub fn DllMainGetInfo(infoStr: *const c_char);
    //  Loads DllMain-related parameters (AS_MOIC) from a text file
    pub fn DllMainLoadFile(dllMainFile: *const c_char) -> i32;
    //  Returns a character string describing the last error that occurred.
    //  As a common practice, this function is called to retrieve the error message when an error occurs.
    //
    //  This function works with or without an opened log file.
    //
    //  If you call this function before you have called DllMainInit(), the function will return an invalid string. This could result in undefined behavior.
    pub fn GetLastErrMsg(lastErrMsg: *const c_char);
    //  Returns a character string describing the last informational message that was recorded.
    //  This function is usually called right after space objects (TLEs, VCMs, sensors, observations, etc.) in an input text file were loaded. It gives information about how many records were successfully loaded, how many were bad, and how many were duplicated.
    //
    //  This function works with or without an opened log file.
    //
    //  If you call this function before you have called DllMainInit(), the function will return an invalid string. This could result in undefined behavior.
    //  This function provides a quick way to check whether all of the prerequisite DLLs have been loaded and initialized correctly. Improper initialization of the Standardized Astrodynamic Algorithms DLLs is one of the most common causes of program crashes.
    pub fn GetLastInfoMsg(lastInfoMsg: *const c_char);
    //  Sets ELSET key mode
    //  This mode can also be turned on if the user loads an input text file that includes this line - "AS_DMA_ON" -
    //  and is currently calling any of these methods: DllMainLoadFile(), TleLoadFile(), SpVecLoadFile(), or VcmLoadFile()
    pub fn SetElsetKeyMode(elset_keyMode: i32) -> i32;
    //  Gets current ELSET key mode
    pub fn GetElsetKeyMode() -> i32;
    //  Sets key mode for ALL (elsets/obs/sensors). This takes precedence over individual elset/obs/sensor key mode
    //  This mode can also be turned on if the user loads an input text file that includes this line - "AS_DMA_ALL_ON"
    pub fn SetAllKeyMode(all_keyMode: i32) -> i32;
    //  Gets current ALL (elsets/obs/sensors) key mode
    pub fn GetAllKeyMode() -> i32;
    //  Resets ALL (elsets/obs/sensors) key mode to its default value which then allows individual elsets/obs/sensors to use their own key mode settings.
    //  Also reset DUPLICATION key mode to its default value.
    pub fn ResetAllKeyMode();
    //  Sets DUPLICATION key mode - change the default behavior of returning a key which already exists in memory: zero versus actual value
    pub fn SetDupKeyMode(dupKeyMode: i32) -> i32;
    //  Gets current DUPLICATION key mode
    pub fn GetDupKeyMode() -> i32;
}

// log message string length
pub const LOGMSGLEN: i32 = 128;

// DHN 06Feb12 - Increase file path length to 512 characters from 128 characters to handle longer file path
pub const FILEPATHLEN: i32 = 512;

// DHN 10Feb12 - Uniformally using 512 characters to passing/receiving string in all Get/Set Field functions
pub const GETSETSTRLEN: usize = 512;

pub const INFOSTRLEN: i32 = 128;

// DHN 10Feb12 - All input card types' (elsets, ob, sensors, ...) can now have maximum of 512 characters
pub const INPUTCARDLEN: i32 = 512;

// Different orbital element types
// Element type - SGP Tle type 0
pub const ELTTYPE_TLE_SGP: isize = 1;
// Element type - SGP4 Tle type 2
pub const ELTTYPE_TLE_SGP4: isize = 2;
// Element type - SP Tle type 6
pub const ELTTYPE_TLE_SP: isize = 3;
// Element type - SP Vector
pub const ELTTYPE_SPVEC_B1P: isize = 4;
// Element type - VCM
pub const ELTTYPE_VCM: isize = 5;
// Element type - External ephemeris
pub const ELTTYPE_EXTEPH: isize = 6;
// Element type - SGP Tle type 4 - XP
pub const ELTTYPE_TLE_XP: isize = 7;

//*******************************************************************************

// Propagation types
// GP/SGP4/SGP4-XP propagator
pub const PROPTYPE_GP: i32 = 1;
// SP propagator
pub const PROPTYPE_SP: i32 = 2;
// External ephemeris
pub const PROPTYPE_X: i32 = 3;
// Unknown
pub const PROPTYPE_UK: i32 = 4;
//*******************************************************************************

// Add sat error
// Bad satellite key
pub const BADSATKEY: i32 = -1;
// Duplicate satellite key
pub const DUPSATKEY: i32 = 0;

//*******************************************************************************

// satellite/observation/sensor key possible errors
// Bad (satellite/observation/sensor) key
pub const BADKEY: i32 = -1;
// Duplicate (satellite/observation/sensor) key
pub const DUPKEY: i32 = 0;

//*******************************************************************************

//*******************************************************************************

// Different key mode options for all elset-satKey/obs-obsKey/sensor-senKey
// Default - duplicate elsets/observations/sensors can not be loaded in their binary trees
pub const ALL_KEYMODE_NODUP: i32 = 0;
// Allow duplicate elsets/obs/sensor to be loaded and have direct memory access (DMA - no duplication check and no binary tree)
pub const ALL_KEYMODE_DMA: i32 = 1;

//*******************************************************************************

// Different key mode options for elset satKey
// Default - duplicate elsets can not be loaded in binary tree
pub const ELSET_KEYMODE_NODUP: i32 = 0;
// Allow duplicate elsets to be loaded and have direct memory access (DMA - no duplication check and no binary tree)
pub const ELSET_KEYMODE_DMA: i32 = 1;

//*******************************************************************************

// Different duplication key mode options (apply to non DMA mode only)
// Returning (satellite/sensor/obs) key is zero to signify the existing data/key was already in memory
pub const DUPKEY_ZERO: i32 = 0;
// Return actual (satellite/sensor/obs) key regardless of the key/data duplication
pub const DUPKEY_ACTUAL: i32 = 1;

//*******************************************************************************

// Input time is in minutes since epoch
pub const TIME_IS_MSE: i32 = 1;
// Input time is in days since 1950 TAI
pub const TIME_IS_TAI: i32 = 2;
// Input time is in days since 1950 UTC
pub const TIME_IS_UTC: i32 = 3;

//*******************************************************************************

// ========================= End of auto generated code ==========================

pub const MAX_ALPHA_5_SAT_ID: i32 = 339999;
pub const DEFAULT_NORAD_ID: i32 = 99999;
pub const DEFAULT_SENSOR_NUMBER: i32 = 999;

// Options used in GetLoaded()
// ascending order
pub const IDX_ORDER_ASC: i32 = 0;
// descending order
pub const IDX_ORDER_DES: i32 = 1;
// order as read
pub const IDX_ORDER_READ: i32 = 2;
// tree traversal order
pub const IDX_ORDER_QUICK: i32 = 9;

pub const DLL_VERSION: &str = env!("SAAL_MANIFEST_VERSION");

/// Return the last error message reported by the DLL.
pub fn get_last_error_message() -> String {
    let mut msg = GetSetString::new();
    unsafe { GetLastErrMsg(msg.pointer()) };
    msg.value()
}

/// Return the current global key mode for all keys.
///
/// Example:
/// ```rust
/// let mode = saal::get_key_mode().unwrap() as i32;
/// println!("{mode}");
/// ```
///
/// Output:
/// ```bash
/// 1
/// ```
pub fn get_key_mode() -> Result<i32, String> {
    let key_mode = unsafe { GetAllKeyMode() };
    match key_mode {
        ALL_KEYMODE_DMA => Ok(key_mode),
        ALL_KEYMODE_NODUP => Ok(key_mode),
        _ => Err(get_last_error_message()),
    }
}

/// Set the global key mode for all keys.
///
/// Example:
/// ```rust
/// saal::set_key_mode(saal::ALL_KEYMODE_NODUP).unwrap();
/// println!("{}", saal::get_key_mode().unwrap());
/// ```
///
/// Output:
/// ```bash
/// 0
/// ```
pub fn set_key_mode(key_mode: i32) -> Result<(), String> {
    let result = unsafe { SetAllKeyMode(key_mode) };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

/// Reset the global key mode and duplicate key mode to defaults.
///
/// Example:
/// ```rust
/// saal::set_key_mode(saal::ALL_KEYMODE_NODUP).unwrap();
/// saal::reset_key_mode();
/// println!("{}", saal::get_key_mode().unwrap());
/// ```
///
/// Output:
/// ```bash
/// 1
/// ```
pub fn reset_key_mode() {
    unsafe { ResetAllKeyMode() };
}

/// Return the last informational message reported by the DLL.
pub fn get_last_info_message() -> String {
    let mut msg = GetSetString::new();
    unsafe { GetLastInfoMsg(msg.pointer()) };
    msg.value()
}

/// Return the DllMain DLL info string (version, build date, platform).
///
/// Example:
/// ```rust
/// let info = saal::get_dll_info();
/// println!("{}", info.contains(saal::DLL_VERSION));
/// ```
///
/// Output:
/// ```bash
/// true
/// ```
pub fn get_dll_info() -> String {
    let mut info = GetSetString::new();
    unsafe { DllMainGetInfo(info.pointer()) };
    info.value()
}

/// Load DllMain parameters from a file.
///
/// Example:
/// ```rust
/// let path = std::env::temp_dir().join("saal_missing_input.txt");
/// let _ = std::fs::remove_file(&path);
/// let result = saal::load_from_file(path.to_str().unwrap());
/// println!("{}", result.is_err());
/// ```
///
/// Output:
/// ```bash
/// true
/// ```
pub fn load_from_file(file_path: &str) -> Result<(), String> {
    let mut dll_path: GetSetString = file_path.into();
    let result = unsafe { DllMainLoadFile(dll_path.pointer()) };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

/// Set the ELSET key mode.
///
/// Example:
/// ```rust
/// saal::set_elset_key_mode(saal::ELSET_KEYMODE_DMA).unwrap();
/// println!("{}", saal::get_elset_key_mode().unwrap());
/// ```
///
/// Output:
/// ```bash
/// 1
/// ```
pub fn set_elset_key_mode(elset_key_mode: i32) -> Result<(), String> {
    let result = unsafe { SetElsetKeyMode(elset_key_mode) };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

/// Return the current ELSET key mode.
///
/// Example:
/// ```rust
/// saal::set_elset_key_mode(saal::ELSET_KEYMODE_NODUP).unwrap();
/// println!("{}", saal::get_elset_key_mode().unwrap());
/// ```
///
/// Output:
/// ```bash
/// 0
/// ```
pub fn get_elset_key_mode() -> Result<i32, String> {
    let elset_key_mode = unsafe { GetElsetKeyMode() };
    match elset_key_mode {
        ELSET_KEYMODE_DMA => Ok(elset_key_mode),
        ELSET_KEYMODE_NODUP => Ok(elset_key_mode),
        _ => Err(get_last_error_message()),
    }
}

/// Set the behavior of returned keys when a duplicate is loaded in NoDuplicates mode.
///
/// Check `DUPKEY_*` constants for return options.
///
/// Example:
/// ```rust
/// saal::set_duplicate_key_mode(saal::DUPKEY_ACTUAL).unwrap();
/// println!("{}", saal::get_duplicate_key_mode().unwrap());
/// ```
///
/// Output:
/// ```bash
/// 1
/// ```
pub fn set_duplicate_key_mode(dup_key_mode: i32) -> Result<(), String> {
    let result = unsafe { SetDupKeyMode(dup_key_mode) };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

/// Return the current duplicate key mode behavior.
///
/// Example:
/// ```rust
/// saal::set_duplicate_key_mode(saal::DUPKEY_ZERO).unwrap();
/// println!("{}", saal::get_duplicate_key_mode().unwrap());
/// ```
///
/// Output:
/// ```bash
/// 0
/// ```
pub fn get_duplicate_key_mode() -> Result<i32, String> {
    let dup_key_mode = unsafe { GetDupKeyMode() };
    match dup_key_mode {
        DUPKEY_ZERO => Ok(dup_key_mode),
        DUPKEY_ACTUAL => Ok(dup_key_mode),
        _ => Err(get_last_error_message()),
    }
}

#[ctor]
fn initialize() {
    set_key_mode(ALL_KEYMODE_DMA).unwrap();
    if let Some(path) = get_time_constants_path() {
        time::load_constants(path.to_str().unwrap()).unwrap();
    }
    if let Some(path) = get_jpl_file_path() {
        astro::set_jpl_ephemeris_file_path(path.to_str().unwrap());
    }
    if let Some(asset_dir) = get_asset_directory() {
        sgp4::set_license_directory(asset_dir.to_str().unwrap());
    }
}

fn asset_directory_override() -> Option<PathBuf> {
    std::env::var("SAAL_ASSET_DIRECTORY").ok().map(PathBuf::from)
}

fn build_asset_directory() -> Option<PathBuf> {
    option_env!("SAAL_BUILD_ASSET_DIR")
        .map(PathBuf::from)
        .filter(|path| path.exists())
}

fn get_asset_directory() -> Option<PathBuf> {
    if let Some(path) = asset_directory_override()
        && path.exists()
    {
        return Some(path);
    }

    Some(std::env::current_exe().ok()?.parent()?.to_path_buf())
}

fn get_time_constants_path() -> Option<PathBuf> {
    let asset_dir = get_asset_directory()?;
    let time_constants_path = asset_dir.join("time_constants.dat");
    if time_constants_path.exists() {
        return Some(time_constants_path);
    }
    None
}

fn get_jpl_file_path() -> Option<PathBuf> {
    let asset_dir = get_asset_directory()?;
    let jpl_path = asset_dir.join("JPLcon_1950_2050.405");
    if jpl_path.exists() {
        return Some(jpl_path);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_lock::TEST_LOCK;

    #[test]
    fn test_get_dll_info_contains_version() {
        let _lock = TEST_LOCK.lock().unwrap();
        let info = get_dll_info();
        assert!(info.contains(DLL_VERSION));
    }

    #[test]
    fn test_get_key_mode_default() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mode = get_key_mode().unwrap() as i32;
        assert_eq!(mode, 1);
    }

    #[test]
    fn test_set_duplicate_key_mode_return_key() {
        let _lock = TEST_LOCK.lock().unwrap();
        set_duplicate_key_mode(DUPKEY_ACTUAL).unwrap();
        let mode = get_duplicate_key_mode().unwrap();
        assert_eq!(mode, DUPKEY_ACTUAL);
    }

    #[test]
    fn test_get_duplicate_key_mode_return_zero() {
        let _lock = TEST_LOCK.lock().unwrap();
        set_duplicate_key_mode(DUPKEY_ZERO).unwrap();
        let mode = get_duplicate_key_mode().unwrap();
        assert_eq!(mode, DUPKEY_ZERO);
    }

    #[test]
    fn test_load_from_file_missing() {
        let _lock = TEST_LOCK.lock().unwrap();
        let path = std::env::temp_dir().join("saal_missing_input.txt");
        let _ = std::fs::remove_file(&path);
        let result = load_from_file(path.to_str().unwrap());
        assert!(result.is_err());
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn _pysaal(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    bindings::register_bindings(parent_module)?;
    Ok(())
}

// ========================= End of manually added code ==========================
