// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::os::raw::c_char;

unsafe extern "C" {
    //  Notes: This function has been deprecated since v9.0.
    //  Initializes SatState DLL for use in the program
    //  If this function returns an error, it is recommended that the users stop the program immediately.
    //  The error occurs if the users forget to load and initialize all the prerequisite DLLs,
    //  as listed in the DLL Prerequisite section, before using this DLL.
    pub fn SatStateInit(apAddr: i64) -> i32;
    //  Returns information about the current version of SatState DLL.
    //  The information is placed in the string parameter passed in.
    //  The returned string provides information about the version number, build date, and the platform of the SatState DLL.
    pub fn SatStateGetInfo(infoStr: *const c_char);
    //  Loads any orbital element types (TLE's/SPVEC's/VCM's), EXTEPHEM's, and/or propagator controls from an input text file
    //  Internally, if taskMode = 1, this function calls SpProp.SpLoadFile();
    //  and if taskMode = 2, this function calls Tle.TleLoadFile(), SpVec.SpVecLoadFile(), Vcm.VcmLoadFile(), ExtEphem.ExtEphLoadFile();
    //  if taskMode = 3, both tasks (1 and 2) are executed.
    pub fn SatStateLoadFile(inputFile: *const c_char, xf_Task: i32) -> i32;
    //  Saves currently loaded orbital element types (TLE's/SPVEC's/VCM's), EXTEPHEM's, and/or propagator controls to a file
    //  The purpose of this function is to save the current SatState-related settings, usually used in GUI applications, for future use.
    //
    //  Internally, if taskMode = 1, this function calls SpProp.SpSaveFile();
    //  and if taskMode = 2, this function calls Tle.TleSaveFile(), SpVec.SpVecSavedFile(), Vcm.VcmSaveFile(), ExtEphem.ExtEphSaveFile();
    //  if taskMode = 3, both tasks (1 and 2) are executed.
    pub fn SatStateSaveFile(outFile: *const c_char, saveMode: i32, saveForm: i32, xf_Task: i32) -> i32;
    //  Removes a satellite from the appropriate elset DLL's set of loaded satellites.
    //  The function will automatically choose the proper set of elsets from which to remove the satellite.
    //  The choices are: Tle.dll, SpVec.dll, Vcm.dll, or ExtEphem.dll
    //  If the users enter an invalid satKey (a satKey that does not exist in memory), the function will return a non-zero value indicating an error.
    pub fn SatStateRemoveSat(satKey: i64) -> i32;
    //  Removes a satellite from the appropriate sets of loaded satellites (elset dll and propagator dll).
    //  The function will remove the satellite from the elset DLL's sets as in SatStateRemoveSat() and
    //  from the appropriate propagator's set of initialized satellites if it exists there.
    pub fn SatStateRemoveSatAll(satKey: i64) -> i32;
    //  Removes all satellites from all of the loaded data sets.
    //  It will remove all satellites from the following sets: Tle, SpVec, Vcm, ExtEphem, Sgp4Prop, and SpProp
    pub fn SatStateRemoveAllSats() -> i32;
    //  Resets propagator settings back to their default values
    pub fn SatStateReset();
    //  Returns the total number of satellites (TLE's, SPVEC's, VCM's, and EXTEPHEM's) currently loaded in memory
    //  See SatStateGetLoaded for example.
    //  This function is useful for dynamically allocating memory for the array that is passed to the function SatStateGetLoaded.
    pub fn SatStateGetCount() -> i32;
    //  Retrieves all of the currently loaded satKeys.
    //  These satKeys can be used to access the internal data for the satellites.
    //  It is recommended that SatStateGetCount() is used to count how many satellites are currently loaded in memory.
    //  The user can then use this number to dynamically allocate the satKeys array and pass it to this function.
    //
    //  If the user prefers to pass a static array to the function, make sure it is big enough to store all the satKeys in memory.
    pub fn SatStateGetLoaded(order: i32, satKeys: *mut i64);
    //  Returns the first satKey that contains the specified satellite number in all sets of loaded satellites.
    //  These sets will be searched: Tle, SpVec, Vcm, and ExtEphem.
    //  This function is useful when a satellite is used in applications that require only one record for one
    //  satellite and the applications refer to that satellite by its satellite number.
    //  However, the Astrodynamic Standard Shared library is only working with satKeys, this function helps to return the associated satKey of that satellite.
    pub fn SatStateNumToKey(satNum: i32) -> i64;
    //  Retrieves the data which is common to all satellite types.
    //  All common fields are retrieved with a single function call.
    //  The apogee height and perigee height are defined as the distance above an ellipsoid
    //  created using the earth flattening factor from the selected geopotential model.
    //  Note:  When using SP elsets (TLE type 6, SPVEC, or VCM), calling SatStateGetSatDataAll
    //  will implicitly call SpInit in order to extract the mu value from the GEO file the elset is tied to.
    //  The elset must have a valid GEO directory available or an error will be returned.
    pub fn SatStateGetSatDataAll(
        satKey: i64,
        satNum: *mut i32,
        satName: *const c_char,
        eltType: *mut i32,
        revNum: *mut i32,
        epochDs50UTC: *mut f64,
        bField: *mut f64,
        elsetNum: *mut i32,
        incli: *mut f64,
        node: *mut f64,
        eccen: *mut f64,
        omega: *mut f64,
        mnAnomaly: *mut f64,
        mnMotion: *mut f64,
        period: *mut f64,
        perigeeHt: *mut f64,
        apogeeHt: *mut f64,
        perigee: *mut f64,
        apogee: *mut f64,
        a: *mut f64,
    ) -> i32;
    //  Retrieves an individual field of a satellite.
    pub fn SatStateGetSatDataField(satKey: i64, xf_Sat: i32, retVal: *const c_char) -> i32;
    //  Initializes a TLE, SPVEC, or VCM in preparation for propagation, or an EXTEPHEM in preparation for interpolation
    pub fn SatStateInitSat(satKey: i64) -> i32;
    //  Propagates a TLE/SPVEC/VCM, or interpolates an EXTEPHEM.
    //  The satellite is propagated/interpolated to the specified time calculated in minutes since the satellite's epoch time
    pub fn SatStateMse(
        satKey: i64,
        mse: f64,
        ds50UTC: *mut f64,
        revNum: *mut i32,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        llh: *mut [f64; 3],
    ) -> i32;
    //  Propagates a TLE/SPVEC/VCM, or interpolates an EXTEPHEM.
    //  The satellite is propagated/interpolated to the specified time calculated in days since 1950, UTC.
    pub fn SatStateDs50UTC(
        satKey: i64,
        ds50UTC: f64,
        mse: *mut f64,
        revNum: *mut i32,
        pos: *mut [f64; 3],
        vel: *mut [f64; 3],
        llh: *mut [f64; 3],
    ) -> i32;
    //  Returns additional propagated/interpolated results (reserved for future implementation)
    //  Reserved for future implementation
    //  Use this function immediately after the call to SatStateMse or SatStateDs50UTC.
    pub fn SatStateGetPropOut(satKey: i64, index: i32, destArr: *mut [f64; 128]) -> i32;
    //  Returns various ephemeris comparison results between two satellite states.
    //  <br>
    //  The "in-track" is NOT the velocity direction, but is defined as completing the right handed coordinate system
    //  defined by the position vector (radial) and the angular momentum vector (cross-track).
    pub fn SatStateEphCom(
        primSatKey: i64,
        secSatKey: i64,
        ds50UTC: f64,
        uvwFlag: i32,
        xa_Delta: *mut [f64; 100],
    ) -> i32;
    //  Returns various ephemeris comparison results between two satellite states (_OS one step) .
    //  <br>
    //  The "in-track" is NOT the velocity direction, but is defined as completing the right handed coordinate system
    //  defined by the position vector (radial) and the angular momentum vector (cross-track).
    pub fn SatStateEphCom_OS(
        priPosVel: *const [f64; 6],
        secPosVel: *const [f64; 6],
        ds50UTC: f64,
        uvwFlag: i32,
        xa_Delta: *mut [f64; 100],
    );
    //  Determines if a satellite contains covariance matrix.
    //  0=no, 1=yes
    pub fn SatStateHasCovMtx(satKey: i64) -> i32;
    //  Propagates/Interpolates UVW covariance matrix from VCM/External ephemeris to the time in days since 1950
    pub fn SatStateGetCovUVW(satKey: i64, ds50UTC: f64, covUVW: *mut [[f64; 6]; 6]) -> i32;
    //  Generate external ephemeris file for the specified satellite (via its unique satKey)
    //  Note: No need to initialize the satellite before this call. If it was intialized, it will be removed after this call
    pub fn SatStateGenEphFile(
        satKey: i64,
        startDs50UTC: f64,
        stopDs50UTC: f64,
        stepSizeSecs: f64,
        ephFileName: *const c_char,
        ephFileType: i32,
    ) -> i32;
    //  Finds the time of ascending nodal crossing of the specified satellite prior to an input time in days since 1950 TAI
    pub fn GetNodalCrossingPriorToTime(satKey: i64, ds50TAI: f64) -> f64;
    //  Get the Gobs parameters for a TLE
    pub fn GetGobsParams(satKey: i64, ds50UTC: f64, xa_gobs: *mut [f64; 32], errCode: *mut i32);
    //  Does an XP GOBS comparison
    pub fn GobsCom(
        primSatKey: i64,
        secSatKey: i64,
        ds50UTC: f64,
        xa_gobs_lim: *const [f64; 16],
        xa_gobs_delta: *mut [f64; 32],
    ) -> i32;
    //  Does an XP GOBS comparison using gobs arrays
    pub fn GobsComArr(
        xa_gobs_prim: *const [f64; 32],
        xa_gobs_sec: *const [f64; 32],
        xa_gobs_lim: *const [f64; 16],
        xa_gobs_delta: *mut [f64; 16],
    );
}

// Indexes of available satellite data fields
// Satellite epoch time in days since 1950 UTC
pub static XF_SATFIELD_EPOCHUTC: i32 = 1;
// Mean anomaly (deg)
pub static XF_SATFIELD_MNANOM: i32 = 2;
// Right ascension of asending node (deg)
pub static XF_SATFIELD_NODE: i32 = 3;
// Argument of perigee (deg)
pub static XF_SATFIELD_OMEGA: i32 = 4;
// Satellite's period (min)
pub static XF_SATFIELD_PERIOD: i32 = 5;
// Eccentricity
pub static XF_SATFIELD_ECCEN: i32 = 6;
// Orbit inclination (deg)
pub static XF_SATFIELD_INCLI: i32 = 7;
// Mean motion (rev/day)
pub static XF_SATFIELD_MNMOTION: i32 = 8;
// GP B* drag term (1/er)  or SP Radiation Pressure Coefficient
pub static XF_SATFIELD_BFIELD: i32 = 9;
// Perigee height above the geoid (km)
pub static XF_SATFIELD_PERIGEEHT: i32 = 10;
// Apogee height above the geoid (km)
pub static XF_SATFIELD_APOGEEHT: i32 = 11;
// Perigee height above the center of the earth (km)
pub static XF_SATFIELD_PERIGEE: i32 = 12;
// Apogee height above the center of the earth (km)
pub static XF_SATFIELD_APOGEE: i32 = 13;
// Semimajor axis (km)
pub static XF_SATFIELD_A: i32 = 14;
// Mean motion derivative (rev/day**2 /2)
pub static XF_SATFIELD_NDOT: i32 = 15;
// Satellite category (Synchronous, Deep space, Decaying, Routine)
pub static XF_SATFIELD_SATCAT: i32 = 16;
// Astat 3 Height multiplier
pub static XF_SATFIELD_HTM3: i32 = 17;
// Center of mass offset (m)
pub static XF_SATFIELD_CMOFFSET: i32 = 18;
// Unused
pub static XF_SATFIELD_N2DOT: i32 = 19;
// GP node dot (deg/s)
pub static XF_SATFIELD_NODEDOT: i32 = 20;
// GP only - the last time when propagation has error
pub static XF_SATFIELD_ERRORTIME: i32 = 21;
// value of mu
pub static XF_SATFIELD_MU: i32 = 22;

//*******************************************************************************

// Indexes of available deltas
// delta position (km)
pub static XA_DELTA_POS: usize = 0;
// delta time (sec)
pub static XA_DELTA_TIME: usize = 1;
// delta position in radial direction (km)
pub static XA_DELTA_PRADIAL: usize = 2;
// delta position in in-track direction (km)
pub static XA_DELTA_PINTRCK: usize = 3;
// delta position in cross-track direction (km)
pub static XA_DELTA_PCRSSTRCK: usize = 4;
// delta velocity (km/sec)
pub static XA_DELTA_VEL: usize = 5;
// delta velocity in radial direction (km/sec)
pub static XA_DELTA_VRADIAL: usize = 6;
// delta velocity in in-track direction (km/sec)
pub static XA_DELTA_VINTRCK: usize = 7;
// delta velocity in cross-track direction (km/sec)
pub static XA_DELTA_VCRSSTRCK: usize = 8;
// delta Beta (deg)
pub static XA_DELTA_BETA: usize = 9;
// delta height (km)
pub static XA_DELTA_HEIGHT: usize = 10;
// delta angular momentum (deg)
pub static XA_DELTA_ANGMOM: usize = 11;
// 3D position Mahalanobis distance in UVW Space (Bubble Covariance, only if covariance propagation is available or turned on)
pub static XA_DELTA_MHLNBS_UVW: i32 = 12;
// 3D position Mahalanobis distance in Height-Time_Beta Space (Banana Covariance, only if covariance propagation is available or turned on)
pub static XA_DELTA_MHLNBS_HTB: i32 = 13;

pub static XA_DELTA_SIZE: usize = 100;

// Indexes of Satellite data fields
// Satellite number I5
pub static XF_SAT_NUM: i32 = 1;
// Satellite international designator A8
pub static XF_SAT_NAME: i32 = 2;
// Element type I1 (old name XF_SAT_EPHTYPE)
pub static XF_SAT_ELTTYPE: i32 = 3;
// Obsolete - should use new name XF_SAT_ELTTYPE instead
pub static XF_SAT_EPHTYPE: i32 = 3;
// Epoch revolution number I6
pub static XF_SAT_REVNUM: i32 = 4;
// Epoch time in days since 1950
pub static XF_SAT_EPOCH: i32 = 5;
// BStar drag component (GP) or Ballistic coefficient-BTerm (SP) (m^2/kg)
pub static XF_SAT_BFIELD: i32 = 6;
// Element set number
pub static XF_SAT_ELSETNUM: i32 = 7;
// Inclination (deg)
pub static XF_SAT_INCLI: i32 = 8;
// Right ascension of ascending node (deg)
pub static XF_SAT_NODE: i32 = 9;
// Eccentricity
pub static XF_SAT_ECCEN: i32 = 10;
// Argument of perigee (deg)
pub static XF_SAT_OMEGA: i32 = 11;
// Mean anomaly (deg)
pub static XF_SAT_MNANOM: i32 = 12;
// Mean motion (revs/day)
pub static XF_SAT_MNMOTN: i32 = 13;
// Satellite period (min)
pub static XF_SAT_PERIOD: i32 = 14;
// Perigee Height(km)
pub static XF_SAT_PERIGEEHT: i32 = 15;
// Apogee Height (km)
pub static XF_SAT_APOGEEHT: i32 = 16;
// Perigee(km)
pub static XF_SAT_PERIGEE: i32 = 17;
// Apogee (km)
pub static XF_SAT_APOGEE: i32 = 18;
// Semi-major axis (km)
pub static XF_SAT_A: i32 = 19;

// Indexes of SatState's load/save file task mode
// Only load/save propagator control parameters
pub static XF_TASK_CTRLONLY: i32 = 1;
// Only load/save orbital elements/external ephemeris data
pub static XF_TASK_SATONLY: i32 = 2;
// Load/Save both 1 and 2
pub static XF_TASK_BOTH: i32 = 3;

// Different external ephemeris file types
// ITC file format
pub static EPHFILETYPE_ITC: i32 = 1;
// ITC compact (without covariance matrix) file format
pub static EPHFILETYPE_ITC_WOCOV: i32 = 2;

// Gobs records
// Satellite number
pub static XA_GOBS_SATNUM: i32 = 0;
// East Longitude
pub static XA_GOBS_LONE: i32 = 1;
// Longitude Drift Rate
pub static XA_GOBS_DRIFT: i32 = 2;
// satellite's relative energy (deg^2/sec^2) - only for GOBS
pub static XA_GOBS_RELENERGY: i32 = 3;
// sin(incl)*sin(r.a. node)
pub static XA_GOBS_WX: i32 = 4;
// -sin(incl)*cos(r.a. node)
pub static XA_GOBS_WY: i32 = 5;
// cos(incl)
pub static XA_GOBS_WZ: i32 = 6;
// abar x
pub static XA_GOBS_ABARX: i32 = 7;
// abar y
pub static XA_GOBS_ABARY: i32 = 8;
// abar z
pub static XA_GOBS_ABARZ: i32 = 9;
// AGOM
pub static XA_GOBS_AGOM: i32 = 10;
// Trough/Drift Flag, 0 - 75 deg trough, 1 - 255 deg trough, 2 - both troughs, 3 - unstable point, 4 - East drift, 5 - West drift
pub static XA_GOBS_TROUGH: i32 = 11;

pub static XA_GOBS_SIZE: i32 = 32;

// Indexes of GOBS limits
// 0 - ignore trough logic, 1 - implement trough logic
pub static XA_GOBS_LIM_TROUGH: i32 = 0;
// Primary satellite is plane changer
pub static XA_GOBS_LIM_PCP: i32 = 1;
// Secondary satellite is plane changer
pub static XA_GOBS_LIM_PCS: i32 = 2;
// Primary satellite is plane changer
pub static XA_GOBS_LIM_ACTIVEP: i32 = 3;
// Secondary satellite is plane changer
pub static XA_GOBS_LIM_ACTIVES: i32 = 4;
// Min Longitude of sat
pub static XA_GOBS_LIM_LONGMIN: i32 = 5;
// Max Longitude of sat
pub static XA_GOBS_LIM_LONGMAX: i32 = 6;
// Min Agom of sat
pub static XA_GOBS_LIM_AGOMMIN: i32 = 7;
// Max Agom of sat
pub static XA_GOBS_LIM_AGOMMAX: i32 = 8;

pub static XA_GOBS_LIM_SIZE: i32 = 16;

// Indexes of available deltas
// Primary satellite number
pub static XA_GOBS_DELTA_PRIMESAT: i32 = 0;
// Secondary satellite number
pub static XA_GOBS_DELTA_SECONDARYSAT: i32 = 1;
// GOBS correlation score
pub static XA_GOBS_DELTA_ASTAT: i32 = 2;
// delta orbital plane
pub static XA_GOBS_DELTA_DOP: i32 = 3;
// delta shape
pub static XA_GOBS_DELTA_DABAR: i32 = 4;
// delta Relative Energy (deg^2/day^2)
pub static XA_GOBS_DELTA_DRELENERGY: i32 = 5;
// Longitude of Primary
pub static XA_GOBS_DELTA_LONGP: i32 = 6;
// Minimum Longitude of Secondary
pub static XA_GOBS_DELTA_LONGMIN: i32 = 7;
// Maximum Longitude of Secondary
pub static XA_GOBS_DELTA_LONGMAX: i32 = 8;
// 0 - opposite throughs or drift rates, 1 - trough or drift rates match
pub static XA_GOBS_DELTA_TROUGH: i32 = 9;
// 0|1    Plane Match Flag
pub static XA_GOBS_DELTA_PLANE: i32 = 10;
// 0|1    Shape Match Flag
pub static XA_GOBS_DELTA_SHAPE: i32 = 11;
// 0|1    Energy Match Flag
pub static XA_GOBS_DELTA_ENERGY: i32 = 12;
// 0|1|2  Longitude Match Flag (2 is fuzzy match)
pub static XA_GOBS_DELTA_LONG: i32 = 13;
// 0|1    Agom Match Flag
pub static XA_GOBS_DELTA_AGOM: i32 = 14;

pub static XA_GOBS_DELTA_SIZE: i32 = 16;

//*******************************************************************************

// ========================= End of auto generated code ==========================

pub fn get_relative_array(
    target_posvel: &[f64; 6],
    chase_posvel: &[f64; 6],
    utc_ds50: f64,
    frame: i32,
) -> [f64; XA_DELTA_SIZE] {
    let mut xa_delta: [f64; XA_DELTA_SIZE] = [0.0; XA_DELTA_SIZE];
    unsafe {
        SatStateEphCom_OS(target_posvel, chase_posvel, utc_ds50, frame, &mut xa_delta);
    }
    xa_delta
}

pub fn get_prior_nodal_crossing(sat_key: i64, tai_ds50: f64) -> f64 {
    unsafe { GetNodalCrossingPriorToTime(sat_key, tai_ds50) }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::test_lock::TEST_LOCK;

    const TARGET_TEME_X: f64 = 42164.0;
    const TARGET_TEME_Y: f64 = 0.0;
    const TARGET_TEME_Z: f64 = 0.0;
    const TARGET_TEME_VX: f64 = 0.0;
    const TARGET_TEME_VY: f64 = 3.0746;
    const TARGET_TEME_VZ: f64 = 0.0;
    const CHASE_TEME_X: f64 = 42160.0;
    const CHASE_TEME_Y: f64 = 1.0;
    const CHASE_TEME_Z: f64 = 1.0;
    const CHASE_TEME_VX: f64 = 0.0;
    const CHASE_TEME_VY: f64 = 3.0746;
    const CHASE_TEME_VZ: f64 = 0.0;

    #[test]
    fn test_get_relative_state() {
        let _lock = TEST_LOCK.lock().unwrap();
        let target_posvel = [
            TARGET_TEME_X,
            TARGET_TEME_Y,
            TARGET_TEME_Z,
            TARGET_TEME_VX,
            TARGET_TEME_VY,
            TARGET_TEME_VZ,
        ];
        let chase_posvel = [
            CHASE_TEME_X,
            CHASE_TEME_Y,
            CHASE_TEME_Z,
            CHASE_TEME_VX,
            CHASE_TEME_VY,
            CHASE_TEME_VZ,
        ];
        let utc_ds50 = 25567.0; // Some arbitrary date
        let delta = get_relative_array(&target_posvel, &chase_posvel, utc_ds50, 1);

        assert_abs_diff_eq!(delta[XA_DELTA_PRADIAL], -4.0, epsilon = 1e-4);
        assert_abs_diff_eq!(delta[XA_DELTA_PINTRCK], 1.0, epsilon = 1e-4);
        assert_abs_diff_eq!(delta[XA_DELTA_PCRSSTRCK], 1.0, epsilon = 1e-4);
        assert_abs_diff_eq!(delta[XA_DELTA_VRADIAL], 0.0, epsilon = 1e-4);
        assert_abs_diff_eq!(delta[XA_DELTA_VINTRCK], 0.0, epsilon = 1e-4);
        assert_abs_diff_eq!(delta[XA_DELTA_VCRSSTRCK], 0.0, epsilon = 1e-4);
    }
}
