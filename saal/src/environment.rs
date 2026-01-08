// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::GetSetString;
use std::os::raw::c_char;

unsafe extern "C" {

    //  Returns information about the EnvConst DLL.
    //  The returned string provides information about the version number, build date, and the platform of the EnvConst DLL.
    pub fn EnvGetInfo(infoStr: *const c_char);
    //  Reads Earth constants (GEO) model and fundamental catalogue (FK) model settings from a file.
    //  The users can use NAME=VALUE pair to setup the GEO and FK models in the input file.
    //
    //  For GEO model, the valid names are GEOCONST, BCONST and the valid values are WGS-72, WGS72,  72, WGS-84, WGS84, 84, EGM-96, EGM96, 96, EGM-08, EGM08, 08, JGM-2, JGM2, 2, SEM68R, 68, GEM5, 5, GEM9, and 9.
    //
    //  For FK model, the valid name is FKCONST and the valid values are: FK4, 4, FK5, 5.
    //
    //  All the string literals are case-insensitive.
    pub fn EnvLoadFile(envFile: *const c_char) -> i32;
    //  Saves the current Earth constants (GEO) model and fundamental catalogue (FK) model settings to a file.
    //  Returns zero indicating the GEO and FK settings have been successfully saved to the file. Other values indicate an error.
    pub fn EnvSaveFile(envConstFile: *const c_char, saveMode: i32, saveForm: i32) -> i32;
    //  Returns the current fundamental catalogue (FK) setting.
    //  The FK model is shared among all the Standardized Astrodynamic Algorithms DLLs in the program.
    pub fn EnvGetFkIdx() -> i32;
    //  Changes the fundamental catalogue (FK) setting to the specified value.
    //  If the users enter an invalid value for the fkIdx, the program will continue to use the current setting.
    //
    //  The FK model is globally shared among the Standardized Astrodynamic Algorithms DLLs. If its setting is changed, the new setting takes effect immediately.
    //  The FK model must be set to FK5 to use the SGP4 propagator.
    pub fn EnvSetFkIdx(xf_FkMod: i32);
    //  Returns the current Earth constants (GEO) setting.
    //  <br>
    //  The GEO model is shared among all the Standardized Astrodynamic Algorithms DLLs in the program.
    //  <br>
    //  The following table lists possible values of the return value GEO setting:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Value</b></td>
    //  <td><b>Value interpretation</b></td>
    //  </tr>
    //  <tr><td>84</td><td>WGS-84</td></tr>
    //  <tr><td>96</td><td>EGM-96</td></tr>
    //  <tr><td>08</td><td>EGM-08</td></tr>
    //  <tr><td>72</td><td>WGS-72 (default)</td></tr>
    //  <tr><td>2</td><td>JGM2</td></tr>
    //  <tr><td>68</td><td>STEM68R, SEM68R</td></tr>
    //  <tr><td>5</td><td>GEM5</td></tr>
    //  <tr><td>9</td><td>GEM9</td></tr>
    //  </table>
    pub fn EnvGetGeoIdx() -> i32;
    //  Changes the Earth constants (GEO) setting to the specified value.
    //  <br>
    //  If you specify an invalid value for xf_GeoMod, the program will continue to use the current setting.
    //  <br>
    //  The GEO model is globally shared among the Standardized Astrodynamic Algorithms DLLs. If its setting is changed, the new setting takes effect immediately
    //  <br>
    //  The following table lists possible values of the parameter value GEO setting:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Value</b></td>
    //  <td><b>Value interpretation</b></td>
    //  </tr>
    //  <tr><td>84</td><td>WGS-84</td></tr>
    //  <tr><td>96</td><td>EGM-96</td></tr>
    //  <tr><td>08</td><td>EGM-08</td></tr>
    //  <tr><td>72</td><td>WGS-72 (default)</td></tr>
    //  <tr><td>2</td><td>JGM2</td></tr>
    //  <tr><td>68</td><td>STEM68R, SEM68R</td></tr>
    //  <tr><td>5</td><td>GEM5</td></tr>
    //  <tr><td>9</td><td>GEM9</td></tr>
    //  </table>
    //  <br>
    //  The GEO model must be set to WGS-72 to use the SGP4 propagator.
    pub fn EnvSetGeoIdx(xf_GeoMod: i32);
    //  Returns the name of the current Earth constants (GEO) model.
    //  <br>
    //  The geoStr parameter may contain one of the following values:
    //  <table>
    //  <caption>table</caption>
    //  <tr><td>WGS-84</td></tr>
    //  <tr><td>EGM-96</td></tr>
    //  <tr><td>EGM-08</td></tr>
    //  <tr><td>WGS-72</td></tr>
    //  <tr><td>JGM2</td></tr>
    //  <tr><td>SEM68R</td></tr>
    //  <tr><td>GEM5</td></tr>
    //  <tr><td>GEM9</td></tr>
    //  </table>
    pub fn EnvGetGeoStr(geoStr: *const c_char);
    //  Changes the Earth constants (GEO) setting to the model specified by a string literal.
    //  <br>
    //  If you specify an invalid value for geoStr, the program will continue to use the current setting.
    //  <br>
    //  The GEO model is globally shared among the Standardized Astrodynamic Algorithms DLLs. If its setting is changed, the new setting takes effect immediately.
    //  <br>
    //  The following table lists possible values of the parameter value GEO setting:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>geoStr (any string in the row)</b></td>
    //  <td><b>Interpretation</b></td>
    //  </tr>
    //  <tr><td>'WGS-84', 'WGS84', '84'</td><td>WGS-84</td></tr>
    //  <tr><td>'EGM-96', 'EGM96', '96'</td><td>EGM-96</td></tr>
    //  <tr><td>'EGM-08', 'EGM08', '8'</td><td>EGM-08</td></tr>
    //  <tr><td>'WGS-72', 'WGS72', '72'</td><td>WGS-72 (default)</td></tr>
    //  <tr><td>'JGM-2, 'JGM2', '2'</td><td>JGM-2</td></tr>
    //  <tr><td>'SEM68R', '68'</td><td>STEM68R, SEM68R</td></tr>
    //  <tr><td>'GEM5', '5'</td><td>GEM5</td></tr>
    //  <tr><td>'GEM9', '9'</td><td>GEM9</td></tr>
    //  </table>
    //  <br>
    //  The GEO model must be set to WGS-72 to use the SGP4 propagator.
    pub fn EnvSetGeoStr(geoStr: *const c_char);
    //  Retrieves the value of one of the constants from the current Earth constants (GEO) model.
    pub fn EnvGetGeoConst(xf_GeoCon: i32) -> f64;
    //  Retrieves the value of one of the constants from the current fundamental catalogue (FK) model.
    pub fn EnvGetFkConst(xf_FkCon: i32) -> f64;
    //  Returns a handle that can be used to access the fundamental catalogue (FK) data structure.
    //  <br>
    //  This function is needed when calling the ThetaGrnwch function from TimeFunc.dll.
    //  <br>
    //  The handle returned by this function is sometimes called a pointer for historical reasons. The name EnvGetFkPtr comes from the fact that the handle used to be called a pointer.
    pub fn EnvGetFkPtr() -> i64;
    //  Specifies the shape of the earth that will be used by the Astro Standards software, either spherical earth or oblate earth
    pub fn EnvSetEarthShape(earthShape: i32);
    //  Returns the value representing the shape of the earth being used by the Astro Standards software, either spherical earth or oblate earth
    pub fn EnvGetEarthShape() -> i32;
}

// Indexes of Earth Constant fields
// Earth flattening (reciprocal; unitless)
pub static XF_GEOCON_FF: i32 = 1;
// J2 (unitless)
pub static XF_GEOCON_J2: i32 = 2;
// J3 (unitless)
pub static XF_GEOCON_J3: i32 = 3;
// J4 (unitless)
pub static XF_GEOCON_J4: i32 = 4;
// Ke (er**1.5/min)
pub static XF_GEOCON_KE: i32 = 5;
// Earth radius (km/er)
pub static XF_GEOCON_KMPER: i32 = 6;
// Earth rotation rate w.r.t. fixed equinox (rad/min)
pub static XF_GEOCON_RPTIM: i32 = 7;

// J2/2 (unitless)
pub static XF_GEOCON_CK2: i32 = 8;
// -3/8 J4 (unitless)
pub static XF_GEOCON_CK4: i32 = 9;
// Converts km/sec to er/kem
pub static XF_GEOCON_KS2EK: i32 = 10;
// Earth rotation rate w.r.t. fixed equinox (rad/kemin)
pub static XF_GEOCON_THDOT: i32 = 11;
// J5 (unitless)
pub static XF_GEOCON_J5: i32 = 12;
// Gravitational parameter km^3/(solar s)^2
pub static XF_GEOCON_MU: i32 = 13;

// Indexes of FK Constant fields
// Earth rotation rate w.r.t. moving equinox (rad/day)
pub static XF_FKCON_C1: i32 = 1;
// Earth rotation acceleration(rad/day**2)
pub static XF_FKCON_C1DOT: i32 = 2;
// Greenwich angle (1970; rad)
pub static XF_FKCON_THGR70: i32 = 3;

// Indexes represent geopotential models GEO
// Earth constants - JGM2
pub const XF_GEOMOD_JGM2: i32 = 2;
// Earth constants - GEM5
pub const XF_GEOMOD_GEM5: i32 = 5;
// Earth constants - EGM-08
pub const XF_GEOMOD_EGM08: i32 = 8;
// Earth constants - GEM9
pub const XF_GEOMOD_GEM9: i32 = 9;
// Earth constants - STEM68
pub const XF_GEOMOD_STEM68: i32 = 68;
// Earth constants - WGS-72
pub const XF_GEOMOD_WGS72: i32 = 72;
// Earth constants - WGS-84
pub const XF_GEOMOD_WGS84: i32 = 84;
// Earth constants - EGM-96
pub const XF_GEOMOD_EGM96: i32 = 96;
// Invalid earth model
pub const XF_GEOMOD_UNKNOWN: i32 = 100;

//*******************************************************************************

// Indexes represent fundamental catalogue FK
// Fundamental Catalog - FK5
pub const XF_FKMOD_4: i32 = 4;
// Fundamental Catalog - FK4
pub const XF_FKMOD_5: i32 = 5;

// ========================= End of auto generated code ==========================

/// Return the EnvConst DLL info string (version, build date, platform).
///
/// Example:
/// ```rust
/// let info = saal::environment::get_dll_info();
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
        EnvGetInfo(info_str.pointer());
    }
    info_str.value()
}

/// Return the Earth radius from the current GEO model.
///
/// Units: kilometers.
///
/// Example:
/// ```rust
/// let km = saal::environment::get_earth_radius();
/// println!("{km:.3}");
/// ```
///
/// Output:
/// ```bash
/// 6378.135
/// ```
pub fn get_earth_radius() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_KMPER) }
}

/// Return the Earth rotation rate from the current FK model.
///
/// Units: radians/day.
///
/// Example:
/// ```rust
/// let rate = saal::environment::get_earth_rotation_rate();
/// println!("{rate:.18}");
/// ```
///
/// Output:
/// ```bash
/// 0.017202791694070362
/// ```
pub fn get_earth_rotation_rate() -> f64 {
    unsafe { EnvGetFkConst(XF_FKCON_C1) }
}

/// Return the Earth rotation acceleration from the current FK model.
///
/// Units: radians/day^2.
///
/// Example:
/// ```rust
/// let accel = saal::environment::get_earth_rotation_acceleration();
/// println!("{accel:.15e}");
/// ```
///
/// Output:
/// ```bash
/// 5.075514194322695e-15
/// ```
pub fn get_earth_rotation_acceleration() -> f64 {
    unsafe { EnvGetFkConst(XF_FKCON_C1DOT) }
}

/// Return the Earth's gravitational parameter from the current GEO model.
///
/// Units: km^3/s^2.
///
/// Example:
/// ```rust
/// let mu = saal::environment::get_earth_mu();
/// println!("{mu:.1}");
/// ```
///
/// Output:
/// ```bash
/// 398600.8
/// ```
pub fn get_earth_mu() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_MU) }
}

/// Return the Earth flattening (reciprocal) from the current GEO model.
///
/// Units: unitless.
///
/// Example:
/// ```rust
/// let f = saal::environment::get_earth_flattening();
/// println!("{f:.15}");
/// ```
///
/// Output:
/// ```bash
/// 0.003352779454168
/// ```
pub fn get_earth_flattening() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_FF) }
}

/// Return the J2 coefficient from the current GEO model.
///
/// Units: unitless.
///
/// Example:
/// ```rust
/// let j2 = saal::environment::get_j2();
/// println!("{j2:.9}");
/// ```
///
/// Output:
/// ```bash
/// 0.001082616
/// ```
pub fn get_j2() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_J2) }
}

/// Return the J3 coefficient from the current GEO model.
///
/// Units: unitless.
///
/// Example:
/// ```rust
/// let j3 = saal::environment::get_j3();
/// println!("{j3:.11}");
/// ```
///
/// Output:
/// ```bash
/// -0.00000253881
/// ```
pub fn get_j3() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_J3) }
}

/// Return the J4 coefficient from the current GEO model.
///
/// Units: unitless.
///
/// Example:
/// ```rust
/// let j4 = saal::environment::get_j4();
/// println!("{j4:.11}");
/// ```
///
/// Output:
/// ```bash
/// -0.00000165597
/// ```
pub fn get_j4() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_J4) }
}

/// Return the J5 coefficient from the current GEO model.
///
/// Units: unitless.
///
/// Example:
/// ```rust
/// let j5 = saal::environment::get_j5();
/// println!("{j5:.7e}");
/// ```
///
/// Output:
/// ```bash
/// -2.1848270e-07
/// ```
pub fn get_j5() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_J5) }
}

/// Load Earth constants and fundamental catalog settings from a file.
///
/// Example:
/// ```rust
/// let path = std::env::temp_dir().join("saal_missing_env.txt");
/// let _ = std::fs::remove_file(&path);
/// let result = saal::environment::load_from_file(path.to_str().unwrap());
/// println!("{}", result.is_err());
/// ```
///
/// Output:
/// ```bash
/// true
/// ```
pub fn load_from_file(file_path: &str) -> Result<(), String> {
    let mut env_file: GetSetString = file_path.into();
    let result = unsafe { EnvLoadFile(env_file.pointer()) };
    match result {
        0 => Ok(()),
        _ => Err(format!("Failed to load environment from file: {}", file_path)),
    }
}

/// Return the current fundamental catalog selection (FK4 or FK5).
///
/// Example:
/// ```rust
/// let catalog = saal::environment::get_fundamental_catalog().unwrap();
/// println!("{}", catalog as i32);
/// ```
///
/// Output:
/// ```bash
/// 5
/// ```
pub fn get_fundamental_catalog() -> Result<i32, String> {
    let fk_idx = unsafe { EnvGetFkIdx() };
    match fk_idx {
        XF_FKMOD_4 => Ok(fk_idx),
        XF_FKMOD_5 => Ok(fk_idx),
        _ => Err(format!("Unknown fundamental catalog index: {}", fk_idx)),
    }
}

/// Set the fundamental catalog selection.
///
/// Example:
/// ```rust
/// saal::environment::set_fundamental_catalog(4);
/// println!("{}", saal::environment::get_fundamental_catalog().unwrap());
/// ```
///
/// Output:
/// ```bash
/// 4
/// ```
pub fn set_fundamental_catalog(catalog: i32) {
    unsafe {
        EnvSetFkIdx(catalog);
    }
}

pub fn set_geopotential_model(geo_model: i32) {
    unsafe {
        EnvSetGeoIdx(geo_model);
    }
}

pub fn get_geopotential_model() -> Result<i32, String> {
    let geo_idx = unsafe { EnvGetGeoIdx() };
    match geo_idx {
        XF_GEOMOD_UNKNOWN => Err("Unknown geopotential model".to_string()),
        _ => Ok(geo_idx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DLL_VERSION;
    use crate::test_lock::TEST_LOCK;
    #[test]
    fn test_get_dll_info() {
        let _lock = TEST_LOCK.lock().unwrap();
        let info = get_dll_info();
        assert!(info.contains(DLL_VERSION));
    }

    #[test]
    fn test_get_earth_radius() {
        let _lock = TEST_LOCK.lock().unwrap();
        let radius = get_earth_radius();
        assert!(radius == 6378.135);
    }

    #[test]
    fn test_get_fundamental_catalog() {
        let _lock = TEST_LOCK.lock().unwrap();
        let catalog = get_fundamental_catalog().unwrap();
        assert!(catalog == 5);
    }

    #[test]
    fn test_get_j2() {
        let _lock = TEST_LOCK.lock().unwrap();
        let j2 = get_j2();
        assert_eq!(j2, 0.001082616);
    }

    #[test]
    fn test_get_j3() {
        let _lock = TEST_LOCK.lock().unwrap();
        let j3 = get_j3();
        assert_eq!(j3, -0.00000253881);
    }

    #[test]
    fn test_get_j4() {
        let _lock = TEST_LOCK.lock().unwrap();
        let j4 = get_j4();
        assert_eq!(j4, -0.00000165597);
    }

    #[test]
    fn test_get_j5() {
        let _lock = TEST_LOCK.lock().unwrap();
        let j5 = get_j5();
        assert_eq!(j5, -2.184827e-7);
    }

    #[test]
    fn test_get_earth_mu() {
        let _lock = TEST_LOCK.lock().unwrap();
        let mu = get_earth_mu();
        assert_eq!(mu, 398600.8);
    }

    #[test]
    fn test_get_earth_flattening() {
        let _lock = TEST_LOCK.lock().unwrap();
        let ff = get_earth_flattening();
        assert_eq!(ff, 1.0 / 298.26);
    }

    #[test]
    fn test_get_earth_rotation_rate() {
        let _lock = TEST_LOCK.lock().unwrap();
        let rate = get_earth_rotation_rate();
        assert_eq!(rate, 0.017202791694070362);
    }

    #[test]
    fn test_get_earth_rotation_acceleration() {
        let _lock = TEST_LOCK.lock().unwrap();
        let accel = get_earth_rotation_acceleration();
        assert_eq!(accel, 5.075514194322695e-15);
    }

    #[test]
    fn test_set_fundamental_catalog_four() {
        let _lock = TEST_LOCK.lock().unwrap();
        set_fundamental_catalog(4);
        let catalog = get_fundamental_catalog().unwrap();
        assert_eq!(catalog, 4);
        set_fundamental_catalog(5);
    }

    #[test]
    fn test_load_from_file_missing() {
        let _lock = TEST_LOCK.lock().unwrap();
        let path = std::env::temp_dir().join("saal_missing_env.txt");
        let _ = std::fs::remove_file(&path);
        let result = load_from_file(path.to_str().unwrap());
        assert!(result.is_err());
    }
}
