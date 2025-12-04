// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use super::GetSetString;
use crate::enums::SAALKeyMode;
use pyo3::prelude::*;
use std::os::raw::c_char;

extern "C" {
    //  Returns information about the DllMain DLL.
    //  The returned string provides information about the version number, build date, and the platform.
    pub fn DllMainGetInfo(infoStr: *const c_char);
    //  Loads DllMain-related parameters (AS_MOIC) from a text file
    pub fn DllMainLoadFile(dllMainFile: *const c_char) -> i32;
    //  Opens a log file and enables the writing of diagnostic information into it.
    //  All of the DLLs in the library will write diagnostic information into the log file once this function has been called.
    //  If the file specified by logFileName already exists, its contents are erased.
    //
    //  Enabling logging can potentially result in large amounts of diagnostic information being generated, which can lead to large amounts of storage being consumed as well as performance decreases. For this reason, it is recommended that this function only be used for debugging purposes.
    pub fn OpenLogFile(fileName: *const c_char) -> i32;
    //  Closes the currently open log file and reset the last logged error message to null.
    //  Remember to close the log file before exiting the program.
    pub fn CloseLogFile();
    //  Writes a message into the log file.
    //  Make sure the log file is open by calling OpenLogFile before using this function.
    //
    //  The message is limited to 128 characters. If the message is longer than this, it will be truncated.
    pub fn LogMessage(msgStr: *const c_char);
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
    //  Tests different input/output data types that are supported by the Astrodynamic Standards library.
    pub fn TestInterface(
        cIn: c_char,
        cOut: *const c_char,
        intIn: i32,
        intOut: *mut i32,
        longIn: i64,
        longOut: *mut i64,
        realIn: f64,
        realOut: *mut f64,
        strIn: *const c_char,
        strOut: *const c_char,
        int1DIn: *const [i32; 3],
        int1DOut: *mut [i32; 3],
        long1DIn: *const [i64; 3],
        long1DOut: *mut [i64; 3],
        real1DIn: *const [f64; 3],
        real1DOut: *mut [f64; 3],
        int2DIn: *const [[i32; 3]; 2],
        int2DOut: *mut [[i32; 3]; 2],
        long2DIn: *const [[i64; 3]; 2],
        long2DOut: *mut [[i64; 3]; 2],
        real2DIn: *const [[f64; 3]; 2],
        real2DOut: *mut [[f64; 3]; 2],
    );
    //  Tests different input/output data types that are supported by the Astrodynamic Standards library.
    pub fn TestInterface2(
        cInOut: *const c_char,
        intInOut: *mut i32,
        longInOut: *mut i64,
        realInOut: *mut f64,
        strInOut: *const c_char,
        int1DInOut: *mut [i32; 3],
        long1DInOut: *mut [i64; 3],
        real1DInOut: *mut [f64; 3],
        int2DInOut: *mut [[i32; 3]; 2],
        long2DInOut: *mut [[i64; 3]; 2],
        real2DInOut: *mut [[f64; 3]; 2],
    );
    //  Tests input and output arrays with unknown length that are supported by the Astrodynamic Standards library.
    pub fn TestInterface3(unk1DIn: *const i32, unk1DOut: *mut i32, unk2DIn: *const i32, unk2DOut: *mut i32);
    //  Returns data parsed from user's AS_MOIC-typed input card - up to 128 fields are allowed.
    pub fn GetMOICData(arrSize: i32, xa_moic: *mut f64);
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
pub static LOGMSGLEN: i32 = 128;

// DHN 06Feb12 - Increase file path length to 512 characters from 128 characters to handle longer file path
pub static FILEPATHLEN: i32 = 512;

// DHN 10Feb12 - Uniformally using 512 characters to passing/receiving string in all Get/Set Field functions
pub static GETSETSTRLEN: usize = 512;

pub static INFOSTRLEN: i32 = 128;

// DHN 10Feb12 - All input card types' (elsets, ob, sensors, ...) can now have maximum of 512 characters
pub static INPUTCARDLEN: i32 = 512;

// Different orbital element types
// Element type - SGP Tle type 0
pub static ELTTYPE_TLE_SGP: isize = 1;
// Element type - SGP4 Tle type 2
pub static ELTTYPE_TLE_SGP4: isize = 2;
// Element type - SP Tle type 6
pub static ELTTYPE_TLE_SP: isize = 3;
// Element type - SP Vector
pub static ELTTYPE_SPVEC_B1P: isize = 4;
// Element type - VCM
pub static ELTTYPE_VCM: isize = 5;
// Element type - External ephemeris
pub static ELTTYPE_EXTEPH: isize = 6;
// Element type - SGP Tle type 4 - XP
pub static ELTTYPE_TLE_XP: isize = 7;

//*******************************************************************************

// Propagation types
// GP/SGP4/SGP4-XP propagator
pub static PROPTYPE_GP: i32 = 1;
// SP propagator
pub static PROPTYPE_SP: i32 = 2;
// External ephemeris
pub static PROPTYPE_X: i32 = 3;
// Unknown
pub static PROPTYPE_UK: i32 = 4;
//*******************************************************************************

// Add sat error
// Bad satellite key
pub static BADSATKEY: i32 = -1;
// Duplicate satellite key
pub static DUPSATKEY: i32 = 0;

//*******************************************************************************

// satellite/observation/sensor key possible errors
// Bad (satellite/observation/sensor) key
pub static BADKEY: i32 = -1;
// Duplicate (satellite/observation/sensor) key
pub static DUPKEY: i32 = 0;

//*******************************************************************************

//*******************************************************************************

// Different key mode options for all elset-satKey/obs-obsKey/sensor-senKey
// Default - duplicate elsets/observations/sensors can not be loaded in their binary trees
pub const ALL_KEYMODE_NODUP: isize = 0;
// Allow duplicate elsets/obs/sensor to be loaded and have direct memory access (DMA - no duplication check and no binary tree)
pub const ALL_KEYMODE_DMA: isize = 1;

//*******************************************************************************

// Different key mode options for elset satKey
// Default - duplicate elsets can not be loaded in binary tree
pub static ELSET_KEYMODE_NODUP: i32 = 0;
// Allow duplicate elsets to be loaded and have direct memory access (DMA - no duplication check and no binary tree)
pub static ELSET_KEYMODE_DMA: i32 = 1;

//*******************************************************************************

// Different duplication key mode options (apply to non DMA mode only)
// Returning (satellite/sensor/obs) key is zero to signify the existing data/key was already in memory
pub static DUPKEY_ZERO: i32 = 0;
// Return actual (satellite/sensor/obs) key regardless of the key/data duplication
pub static DUPKEY_ACTUAL: i32 = 1;

//*******************************************************************************

// Input time is in minutes since epoch
pub static TIME_IS_MSE: i32 = 1;
// Input time is in days since 1950 TAI
pub static TIME_IS_TAI: i32 = 2;
// Input time is in days since 1950 UTC
pub static TIME_IS_UTC: i32 = 3;

//*******************************************************************************

// ========================= End of auto generated code ==========================

pub const MAX_ALPHA_5_SAT_ID: i32 = 339999;

#[pyclass]
pub struct MainInterface;

impl MainInterface {
    pub fn get_last_error_message() -> String {
        let mut msg = GetSetString::new();
        unsafe { GetLastErrMsg(msg.pointer()) };
        msg.value()
    }

    pub fn get_key_mode() -> Result<SAALKeyMode, String> {
        let key_mode = unsafe { GetAllKeyMode() };
        match key_mode as isize {
            ALL_KEYMODE_DMA => Ok(SAALKeyMode::DirectMemoryAccess),
            ALL_KEYMODE_NODUP => Ok(SAALKeyMode::NoDuplicates),
            _ => Err("Unknown key mode".to_string()),
        }
    }

    pub fn set_key_mode(key_mode: SAALKeyMode) -> Result<(), String> {
        let key_mode = match key_mode {
            SAALKeyMode::DirectMemoryAccess => ALL_KEYMODE_DMA,
            SAALKeyMode::NoDuplicates => ALL_KEYMODE_NODUP,
        };
        let result = unsafe { SetAllKeyMode(key_mode as i32) };
        match result {
            0 => Ok(()),
            _ => Err(MainInterface::get_last_error_message()),
        }
    }
}

#[pymethods]
impl MainInterface {
    // Options used in GetLoaded()
    // ascending order
    pub const IDX_ORDER_ASC: usize = 0;
    // descending order
    pub const IDX_ORDER_DES: usize = 1;
    // order as read
    pub const IDX_ORDER_READ: usize = 2;
    // tree traversal order
    pub const IDX_ORDER_QUICK: usize = 9;

    #[staticmethod]
    #[pyo3(name = "get_last_error_message")]
    pub fn py_get_last_error_message() -> PyResult<String> {
        Ok(MainInterface::get_last_error_message())
    }

    #[staticmethod]
    #[pyo3(name = "get_key_mode")]
    pub fn py_get_key_mode() -> PyResult<SAALKeyMode> {
        MainInterface::get_key_mode().map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)
    }

    #[staticmethod]
    #[pyo3(name = "set_key_mode")]
    pub fn py_set_key_mode(key_mode: SAALKeyMode) -> PyResult<()> {
        MainInterface::set_key_mode(key_mode).map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)
    }
}
