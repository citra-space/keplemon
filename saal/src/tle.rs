// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{GetSetString, get_last_error_message};
use std::os::raw::c_char;
use std::result::Result;

unsafe extern "C" {

    //  Returns the information about the Tle DLL.
    //  The returned string provides information about the version number, build date, and the platform of the Tle DLL.
    pub fn TleGetInfo(infoStr: *const c_char);
    //  Loads TLEs (satellites) contained in a text file into the TLE DLL's binary tree.
    //  You may use this function repeatedly to load TLEs from different input files. However, only unique satKeys are loaded. Duplicated TLEs won't be stored.
    //
    //  TLEs can be included directly in the specified file, or they can be read from a separate file identified with "ELTFIL=[path\filename]" or "VECFIL=[path\filename]".
    //
    //  The input file is read in two passes. The function first looks for "ELTFIL=" and "VECFIL=" lines, then it looks for TLEs which were included directly. The result of this is that data entered using both methods will be processed, but the "ELTFIL=" and "VECFIL=" data will be processed first.
    pub fn TleLoadFile(tleFile: *const c_char) -> i32;
    //  Saves currently loaded TLEs to a file.
    //  In append mode, if the specified file does not exist it will be created.
    //  If you call this routine immediately after TleLoadFile(), the TLE contents in the two files should be the same (minus duplicated TLE's or bad TLE's).
    //
    //  The purpose of this function is to save the current state of the loaded TLE's, usually used in GUI applications, for future use.
    pub fn TleSaveFile(tleFile: *const c_char, saveMode: i32, xf_tleForm: i32) -> i32;
    //  Removes a TLE represented by the satKey from memory.
    //  If the users enter an invalid satKey (a non-existing satKey), the function will return a non-zero value indicating an error.
    pub fn TleRemoveSat(satKey: i64) -> i32;
    //  Removes all the TLEs from memory.
    pub fn TleRemoveAllSats() -> i32;
    //  Returns the number of TLEs currently loaded.
    //  See TleGetLoaded for an example.
    //  This function is useful for dynamically allocating memory for the array that is passed to the function TleGetLoaded().
    pub fn TleGetCount() -> i32;
    //  Retrieves all of the currently loaded satKeys. These satKeys can be used to access the internal data for the TLE's.
    //  It is recommended that TleGetCount() be  used to determine how many satellites are currently loaded. This value can then be used to dynamically allocate an array to hold the satKeys.
    //
    //  If you are going to pass a statically allocated array to this function, ensure it is large enough to hold all of the returned satKeys.
    pub fn TleGetLoaded(order: i32, satKeys: *mut i64);
    //  Adds a TLE (satellite), using its directly specified first and second lines.
    //  The function will indicate an error if the specified two line element set corresponds to a satellite that is already in memory.
    //
    //  This function can be called repeatedly to add many TLEs, one at a time.
    pub fn TleAddSatFrLines(line1: *const c_char, line2: *const c_char) -> i64;
    //  Adds a TLE (satellite), using its CSV string format.
    pub fn TleAddSatFrCsv(csvLine: *const c_char) -> i64;
    //  Adds a GP TLE using its individually provided field values.
    //  The function will indicate an error if the specified two line element set corresponds to a satellite that is already in memory.
    //
    //  This function can be called repeatedly to add many satellites (one satellite at a time).
    //
    //  SGP satellites (ephType = 0) use Kozai mean motion. SGP4 satellites (ephType = 2) use Brouwer mean motion.
    pub fn TleAddSatFrFieldsGP(
        satNum: i32,
        secClass: c_char,
        satName: *const c_char,
        epochYr: i32,
        epochDays: f64,
        bstar: f64,
        ephType: i32,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
    ) -> i64;
    //  This function is similar to TleAddSatFrFieldsGP but includes nDotO2 and n2DotO6.
    //  nDotO2 and n2DotO6 values are not used in the SGP4 propagator. However, some users still want to preserve the integrity of all input data.
    pub fn TleAddSatFrFieldsGP2(
        satNum: i32,
        secClass: c_char,
        satName: *const c_char,
        epochYr: i32,
        epochDays: f64,
        bstar: f64,
        ephType: i32,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
        nDotO2: f64,
        n2DotO6: f64,
    ) -> i64;
    //  Updates a GP satellite's data in memory by providing its individual field values. Note: satNum, year, day, and ephtype can't be updated.
    //  The satellite's unique key will not be changed by this function. If you specify a satKey that does not correspond to a currently loaded satellite, the function will indicate an error.
    //
    //  Remember to use the correct mean motion depending on the satellite's ephType.
    pub fn TleUpdateSatFrFieldsGP(
        satKey: i64,
        secClass: c_char,
        satName: *const c_char,
        bstar: f64,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
    ) -> i32;
    //  This function is similar to TleUpdateSatFrFieldsGP but includes nDotO2 and n2DotO6. Note: satNum, year, day, and ephtype can't be updated.
    //  nDotO2 and n2DotO6 values are not used in the SGP4 propagator. However, some users still want to preserve the integrity of all input data.
    pub fn TleUpdateSatFrFieldsGP2(
        satKey: i64,
        secClass: c_char,
        satName: *const c_char,
        bstar: f64,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
        nDotO2: f64,
        n2DotO6: f64,
    ) -> i32;
    //  Adds an SP satellite using the individually provided field values.
    //  Only applies to SP propagator.
    pub fn TleAddSatFrFieldsSP(
        satNum: i32,
        secClass: c_char,
        satName: *const c_char,
        epochYr: i32,
        epochDays: f64,
        bTerm: f64,
        ogParm: f64,
        agom: f64,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
    ) -> i64;
    //  Updates an SP satellite's data in memory using its individually provided field values. Note: satNum, year, day, and ephtype can't be updated.
    //  Only applies to SP propagator.
    //  The satellite's unique key will not be changed by this function. If you specify a satKey that does not correspond to a currently loaded TLE, the function will indicate an error.
    pub fn TleUpdateSatFrFieldsSP(
        satKey: i64,
        secClass: c_char,
        satName: *const c_char,
        bterm: f64,
        ogParm: f64,
        agom: f64,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
    ) -> i32;
    //  Updates the value of a field of a TLE. This function can be used for both GP and SP satellites.
    //  <br>
    //  The table below indicates which index values correspond to which fields. Make sure to use the appropriate field index for GP TLEs and SP TLEs.  For indexes 5, 15 and 16, the interpretation depends on the ephemeris type of the TLE.
    //  satNum (1), Epoch (4), and Ephemeris Type (5) cannot be altered.
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  	<td>index</td>
    //  	<td>index Interpretation</td>
    //  </tr>
    //  <tr><td>1</td><td>Satellite number</td></tr>
    //  <tr><td>2</td><td>Security classification</td></tr>
    //  <tr><td>3</td><td>Satellite international designator</td></tr>
    //  <tr><td>4</td><td>Epoch</td></tr>
    //  <tr><td>5</td><td>Ephemeris type = 0,2: B* drag term (1/er) <br>Ephemeris type = 6   : SP radiation pressure
    //  coefficient agom (m2/kg)</td></tr>
    //  <tr><td>6</td><td>Ephemeris type</td></tr>
    //  <tr><td>7</td><td>Element set number</td></tr>
    //  <tr><td>8</td><td>Orbit inclination (degrees)</td></tr>
    //  <tr><td>9</td><td>Right ascension of ascending node (degrees)</td></tr>
    //  <tr><td>10</td><td>Eccentricity</td></tr>
    //  <tr><td>11</td><td>Argument of perigee (degrees)</td></tr>
    //  <tr><td>12</td><td>Mean anomaly (degrees)</td></tr>
    //  <tr><td>13</td><td>Mean motion (rev/day)</td></tr>
    //  <tr><td>14</td><td>Revolution number at epoch</td></tr>
    //  <tr><td>15</td><td>Ephemeris type = 0: SGP mean motion derivative (rev/day /2) or <br>Ephemeris type = 6: SP
    //  ballistic coefficient (m2/kg)</td></tr>
    //  <tr><td>16</td><td>Ephemeris type = 0: SGP mean motion second derivative (rev/day**2 /6) or <br>Ephemeris type = 6:
    //  SP Outgassing parameter/Thrust Acceleration (km/s2)</td></tr>
    //  </table>
    pub fn TleSetField(satKey: i64, xf_Tle: i32, valueStr: *const c_char) -> i32;
    //  Retrieves the value of a specific field of a TLE.
    //  <br>
    //  The table below indicates which index values correspond to which fields. Make sure to use the appropriate field index for GP TLEs and SP TLEs.  For indexes 5, 15 and 16, the interpretation depends on the ephemeris type of the TLE.
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  	<td>index</td>
    //  	<td>index Interpretation</td>
    //  </tr>
    //  <tr><td>1</td><td>Satellite number</td></tr>
    //  <tr><td>2</td><td>Security classification</td></tr>
    //  <tr><td>3</td><td>Satellite international designator</td></tr>
    //  <tr><td>4</td><td>Epoch</td></tr>
    //  <tr><td>5</td><td>Ephemeris type = 0,2: B* drag term (1/er) <br>Ephemeris type = 6   : SP radiation pressure
    //  coefficient agom (m2/kg)</td></tr>
    //  <tr><td>6</td><td>Ephemeris type</td></tr>
    //  <tr><td>7</td><td>Element set number</td></tr>
    //  <tr><td>8</td><td>Orbit inclination (degrees)</td></tr>
    //  <tr><td>9</td><td>Right ascension of ascending node (degrees)</td></tr>
    //  <tr><td>10</td><td>Eccentricity</td></tr>
    //  <tr><td>11</td><td>Argument of perigee (degrees)</td></tr>
    //  <tr><td>12</td><td>Mean anomaly (degrees)</td></tr>
    //  <tr><td>13</td><td>Mean motion (rev/day)</td></tr>
    //  <tr><td>14</td><td>Revolution number at epoch</td></tr>
    //  <tr><td>15</td><td>Ephemeris type = 0: SGP mean motion derivative (rev/day /2) or <br>Ephemeris type = 6: SP
    //  ballistic coefficient (m2/kg)</td></tr>
    //  <tr><td>16</td><td>Ephemeris type = 0: SGP mean motion second derivative (rev/day**2 /6) or <br>Ephemeris type = 6:
    //  SP Outgassing parameter/Thrust Acceleration (km/s2)</td></tr>
    //  </table>
    pub fn TleGetField(satKey: i64, xf_Tle: i32, valueStr: *const c_char) -> i32;
    //  Retrieves all of the data for a GP satellite in a single function call.
    //  This function only works for GP satellites. The field values are placed in the corresponding parameters of the function.
    pub fn TleGetAllFieldsGP(
        satKey: i64,
        satNum: *mut i32,
        secClass: *const c_char,
        satName: *const c_char,
        epochYr: *mut i32,
        epochDays: *mut f64,
        bstar: *mut f64,
        ephType: *mut i32,
        elsetNum: *mut i32,
        incli: *mut f64,
        node: *mut f64,
        eccen: *mut f64,
        omega: *mut f64,
        mnAnomaly: *mut f64,
        mnMotion: *mut f64,
        revNum: *mut i32,
    ) -> i32;
    //  Retrieves all of the data (including nDotO2 and n2DotO6) for a GP satellite in a single function call.
    //  This function is similar to TleGetAllFieldsGP but also includes nDotO2 and n2DotO6.
    //  This function only works for GP satellites. The field values are placed in the corresponding parameters of the function.
    pub fn TleGetAllFieldsGP2(
        satKey: i64,
        satNum: *mut i32,
        secClass: *const c_char,
        satName: *const c_char,
        epochYr: *mut i32,
        epochDays: *mut f64,
        bstar: *mut f64,
        ephType: *mut i32,
        elsetNum: *mut i32,
        incli: *mut f64,
        node: *mut f64,
        eccen: *mut f64,
        omega: *mut f64,
        mnAnomaly: *mut f64,
        mnMotion: *mut f64,
        revNum: *mut i32,
        nDotO2: *mut f64,
        n2DotO6: *mut f64,
    ) -> i32;
    //  Retrieves all of the data for an SP satellite in a single function call.
    //  Only applies to SP propagator.
    //  This function only works for SP satellites. The field values are placed in the corresponding parameters of the function.
    pub fn TleGetAllFieldsSP(
        satKey: i64,
        satNum: *mut i32,
        secClass: *const c_char,
        satName: *const c_char,
        epochYr: *mut i32,
        epochDays: *mut f64,
        bTerm: *mut f64,
        ogParm: *mut f64,
        agom: *mut f64,
        elsetNum: *mut i32,
        incli: *mut f64,
        node: *mut f64,
        eccen: *mut f64,
        omega: *mut f64,
        mnAnomaly: *mut f64,
        mnMotion: *mut f64,
        revNum: *mut i32,
    ) -> i32;
    //  Parses GP data from the input first and second lines of a two line element set or a CSV Tle.
    //  This function only parses data from the input TLE but DOES NOT load/add the input TLE to memory.
    pub fn TleParseGP(
        line1: *const c_char,
        line2: *const c_char,
        satNum: *mut i32,
        secClass: *const c_char,
        satName: *const c_char,
        epochYr: *mut i32,
        epochDays: *mut f64,
        nDotO2: *mut f64,
        n2DotO6: *mut f64,
        bstar: *mut f64,
        ephType: *mut i32,
        elsetNum: *mut i32,
        incli: *mut f64,
        node: *mut f64,
        eccen: *mut f64,
        omega: *mut f64,
        mnAnomaly: *mut f64,
        mnMotion: *mut f64,
        revNum: *mut i32,
    ) -> i32;
    //  Parses GP data from the input first and second lines of a two line element set or a CSV tle and store that data back into the output parameters.
    //  This function only parses data from the input TLE but DOES NOT load/add the input TLE to memory.
    pub fn TleLinesToArray(
        line1: *const c_char,
        line2: *const c_char,
        xa_tle: *mut [f64; 64],
        xs_tle: *const c_char,
    ) -> i32;
    //  Parses SP data from the input first and second lines of a two line element set.
    //  Only applies to SP propagator.
    //  This function only parses data from the input TLE but DOES NOT load/add the input TLE to memory.
    pub fn TleParseSP(
        line1: *const c_char,
        line2: *const c_char,
        satNum: *mut i32,
        secClass: *const c_char,
        satName: *const c_char,
        epochYr: *mut i32,
        epochDays: *mut f64,
        bTerm: *mut f64,
        ogParm: *mut f64,
        agom: *mut f64,
        elsetNum: *mut i32,
        incli: *mut f64,
        node: *mut f64,
        eccen: *mut f64,
        omega: *mut f64,
        mnAnomaly: *mut f64,
        mnMotion: *mut f64,
        revNum: *mut i32,
    ) -> i32;
    //  Returns the first and second lines representation of a TLE of a satellite.
    pub fn TleGetLines(satKey: i64, line1: *const c_char, line2: *const c_char) -> i32;
    //  Returns the CSV string representation of a TLE of a satellite.
    pub fn TleGetCsv(satKey: i64, csvLine: *const c_char) -> i32;
    //  Constructs a TLE from individually provided GP data fields.
    //  This function only parses data from the input fields but DOES NOT load/add the TLE to memory.
    //  Returned line1 and line2 will be empty if the function fails to construct the lines as requested.
    pub fn TleGPFieldsToLines(
        satNum: i32,
        secClass: c_char,
        satName: *const c_char,
        epochYr: i32,
        epochDays: f64,
        nDotO2: f64,
        n2DotO6: f64,
        bstar: f64,
        ephType: i32,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
        line1: *const c_char,
        line2: *const c_char,
    );
    //  Constructs a TLE from individually provided GP data fields.
    //  This function only parses data from the input fields but DOES NOT load/add the TLE to memory.
    //  Returned line1 and line2 will be empty if the function fails to construct the lines as requested.
    pub fn TleGPFieldsToCsv(
        satNum: i32,
        secClass: c_char,
        satName: *const c_char,
        epochYr: i32,
        epochDays: f64,
        nDotO2: f64,
        n2DotO6: f64,
        bstar: f64,
        ephType: i32,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
        csvLine: *const c_char,
    );
    //  Constructs a TLE from GP data stored in the input parameters.
    //  This function only parses data from the input data but DOES NOT load/add the TLE to memory.
    //  <br>
    //  Returned line1 and line2 will be empty if the function fails to construct the lines as requested.
    pub fn TleGPArrayToLines(
        xa_tle: *const [f64; 64],
        xs_tle: *const c_char,
        line1: *const c_char,
        line2: *const c_char,
    );
    //  Constructs a TLE from GP data stored in the input parameters.
    //  This function only parses data from the input data but DOES NOT load/add the TLE to memory.
    //  Returned line1 and line2 will be empty if the function fails to construct the lines as requested.
    pub fn TleGPArrayToCsv(xa_tle: *const [f64; 64], xs_tle: *const c_char, csvline: *const c_char);
    //  Constructs a TLE from individually provided SP data fields.
    //  Only applies to SP propagator.
    //  This function only parses data from the input fields but DOES NOT load/add the TLE to memory.
    //  Returned line1 and line2 will be empty if the function fails to construct the lines as requested.
    pub fn TleSPFieldsToLines(
        satNum: i32,
        secClass: c_char,
        satName: *const c_char,
        epochYr: i32,
        epochDays: f64,
        bTerm: f64,
        ogParm: f64,
        agom: f64,
        elsetNum: i32,
        incli: f64,
        node: f64,
        eccen: f64,
        omega: f64,
        mnAnomaly: f64,
        mnMotion: f64,
        revNum: i32,
        line1: *const c_char,
        line2: *const c_char,
    );
    //  Returns the first satKey from the currently loaded set of TLEs that contains the specified satellite number.
    //  This function is useful when Tle.dll is used in applications that require only one record (one TLE entry) for one satellite, and which refer to that TLE by its satellite number. This function can be used to retrieve a satKey in that situation, which is useful since the Standardized Astrodynamic Algorithms library works only with satKeys.
    //  A negative value will be returned if there is an error.
    pub fn TleGetSatKey(satNum: i32) -> i64;
    //  Computes a satKey from the input data.
    //  There is no need for a matching satellite to be loaded prior to using this function. The function simply computes the satKey from the provided fields.
    //
    //  This is the proper way to reconstruct a satKey from its fields. If you use your own routine to do this, the computed satKey might be different.
    //  A negative value will be returned if there is an error.
    pub fn TleFieldsToSatKey(satNum: i32, epochYr: i32, epochDays: f64, ephType: i32) -> i64;
    //  Adds a TLE (satellite), using its data stored in the input parameters.
    pub fn TleAddSatFrArray(xa_tle: *const [f64; 64], xs_tle: *const c_char) -> i64;
    //  Updates existing TLE data with the provided new data stored in the input parameters. Note: satNum, year, day, and ephtype can't be updated.
    //  nDotO2 and n2DotO6 values are not used in the SGP4 propagator. However, some users still want to preserve the integrity of all input data.
    pub fn TleUpdateSatFrArray(satKey: i64, xa_tle: *const [f64; 64], xs_tle: *const c_char) -> i32;
    //  Retrieves TLE data and stored it in the passing parameters
    pub fn TleDataToArray(satKey: i64, xa_tle: *mut [f64; 64], xs_tle: *const c_char) -> i32;
    //  Converts TLE two line format to CSV format
    pub fn TleLinesToCsv(line1: *const c_char, line2: *const c_char, csvline: *const c_char) -> i32;
    //  Converts TLE CSV format to two line format
    pub fn TleCsvToLines(csvLine: *const c_char, newSatno: i32, line1: *const c_char, line2: *const c_char) -> i32;
    //  Finds the check sums of TLE lines
    pub fn GetCheckSums(
        line1: *const c_char,
        line2: *const c_char,
        chkSum1: *mut i32,
        chkSum2: *mut i32,
        errCode: *mut i32,
    );
}

// TLE types (TLE ephemeris types) - They are different than ELTTYPE
// TLE SGP elset (Kozai mean motion)
pub const TLETYPE_SGP: i32 = 0;
// TLE SGP4 elset (Brouwer mean motion)
pub const TLETYPE_SGP4: i32 = 2;
// TLE SGP4-XP elset (Brouwer mean motion)
pub const TLETYPE_XP: i32 = 4;
// TLE SP elset (osculating elements)
pub const TLETYPE_SP: i32 = 6;

// Indexes of TLE data fields
// Satellite number
pub const XF_TLE_SATNUM: usize = 1;
// Security classification U: unclass, C: confidential, S: Secret
pub const XF_TLE_CLASS: usize = 2;
// Satellite name A8
pub const XF_TLE_SATNAME: usize = 3;
// Satellite's epoch time "YYYYJJJ.jjjjjjjj"
pub const XF_TLE_EPOCH: usize = 4;
// GP B* drag term (1/er)  (not the same as XF_TLE_BTERM)
pub const XF_TLE_BSTAR: usize = 5;
// Satellite ephemeris type: 0=SGP, 2=SGP4, 4=SGP4-XP, 6=SP
pub const XF_TLE_EPHTYPE: usize = 6;
// Element set number
pub const XF_TLE_ELSETNUM: usize = 7;
// Orbit inclination (deg)
pub const XF_TLE_INCLI: usize = 8;
// Right ascension of asending node (deg)
pub const XF_TLE_NODE: usize = 9;
// Eccentricity
pub const XF_TLE_ECCEN: usize = 10;
// Argument of perigee (deg)
pub const XF_TLE_OMEGA: usize = 11;
// Mean anomaly (deg)
pub const XF_TLE_MNANOM: usize = 12;
// Mean motion (rev/day) (ephType=0: Kozai, ephType=2: Brouwer)
pub const XF_TLE_MNMOTN: usize = 13;
// Revolution number at epoch
pub const XF_TLE_REVNUM: usize = 14;

// GP Mean motion derivative (rev/day /2)
pub const XF_TLE_NDOT: usize = 15;
// GP Mean motion second derivative (rev/day**2 /6)
pub const XF_TLE_NDOTDOT: usize = 16;
// Solar radiation pressure GP (m2/kg)
pub const XF_TLE_AGOMGP: usize = 16;

// SP Radiation Pressure Coefficient
pub const XF_TLE_SP_AGOM: usize = 5;
// SP ballistic coefficient (m2/kg)
pub const XF_TLE_SP_BTERM: usize = 15;
// SP outgassing parameter (km/s2)
pub const XF_TLE_SP_OGPARM: usize = 16;

// Original satellite number
pub const XF_TLE_ORGSATNUM: usize = 17;
// GP ballistic coefficient (m2/kg) (not the same as XF_TLE_BSTAR)
pub const XF_TLE_BTERM: usize = 18;
// Time of last observation relative to epoch +/- fractional days
pub const XF_TLE_OBSTIME: usize = 19;
// Last calculated error growth rate (km/day)
pub const XF_TLE_EGR: usize = 20;
// Last calculated energy dissipation rate (w/kg)
pub const XF_TLE_EDR: usize = 21;
// Median Vismag
pub const XF_TLE_VISMAG: usize = 22;
// Median RCS - diameter in centimeters (cm)
pub const XF_TLE_RCS: usize = 23;
// Object Type (Payload, Rocket Body, Platform, Debris, Unknown)
pub const XF_TLE_OBJTYPE: usize = 24;
// Satellite name A12 (upto 12 character long)
pub const XF_TLE_SATNAME_12: usize = 25;

// Indexes of TLE numerical data in an array
// Line 1
// Satellite number
pub const XA_TLE_SATNUM: usize = 0;
// Satellite's epoch time in DS50UTC
pub const XA_TLE_EPOCH: usize = 1;
// GP Mean motion derivative (rev/day /2)
pub const XA_TLE_NDOT: usize = 2;
// GP Mean motion second derivative (rev/day**2 /6)
pub const XA_TLE_NDOTDOT: usize = 3;
// GP B* drag term (1/er)
pub const XA_TLE_BSTAR: usize = 4;
// Satellite ephemeris type: 0=SGP, 2=SGP4, 4=SGP4-XP, 6=SP
pub const XA_TLE_EPHTYPE: usize = 5;

// Line 2
// Orbit inclination (deg)
pub const XA_TLE_INCLI: usize = 20;
// Right ascension of asending node (deg)
pub const XA_TLE_NODE: usize = 21;
// Eccentricity
pub const XA_TLE_ECCEN: usize = 22;
// Argument of perigee (deg)
pub const XA_TLE_OMEGA: usize = 23;
// Mean anomaly (deg)
pub const XA_TLE_MNANOM: usize = 24;
// Mean motion (rev/day) (ephType=0, 4: Kozai, ephType=2: Brouwer)
pub const XA_TLE_MNMOTN: usize = 25;
// Revolution number at epoch
pub const XA_TLE_REVNUM: usize = 26;
// Element set number
pub const XA_TLE_ELSETNUM: usize = 30;

// CSV (or TLE-XP, ephemType=4) specific fields
// Original satellite number
pub const XA_TLE_ORGSATNUM: usize = 31;
// SP/SGP4-XP ballistic coefficient (m2/kg)
pub const XA_TLE_BTERM: usize = 32;
// Time of last observation relative to epoch +/- fractional days
pub const XA_TLE_OBSTIME: usize = 33;
// Last calculated error growth rate (km/day)
pub const XA_TLE_EGR: usize = 34;
// Last calculated energy dissipation rate (w/kg)
pub const XA_TLE_EDR: usize = 35;
// Median Vismag
pub const XA_TLE_VISMAG: usize = 36;
// Median RCS - diameter in centimeters (cm)
pub const XA_TLE_RCS: usize = 37;

// CSV (or TLE-XP, ephemType=4)
// Solar Radiation Pressure Coefficient GP (m2/kg)
pub const XA_TLE_AGOMGP: usize = 38;

// SP specific fields
// SP ballistic coefficient (m2/kg)
pub const XA_TLE_SP_BTERM: usize = 2;
// SP outgassing parameter (km/s2)
pub const XA_TLE_SP_OGPARM: usize = 3;
// SP Radiation Pressure Coefficient
pub const XA_TLE_SP_AGOM: usize = 4;

pub const XA_TLE_SIZE: usize = 64;

// Indexes of TLE text data in an array of chars
// Security classification of line 1 and line 2
pub const XS_TLE_SECCLASS_1: usize = 0;
// Satellite name
pub const XS_TLE_SATNAME_12: usize = 1;
// Object Type (Payload, Rocket Body, Platform, Debris, Unknown) - csv only
pub const XS_TLE_OBJTYPE_11: usize = 13;

pub const XS_TLE_SIZE: usize = 512;

// TLE's text data fields - new convention (start index, string length)
// Security classification of line 1 and line 2
pub const XS_TLE_SECCLASS_0_1: usize = 0;
// Satellite name
pub const XS_TLE_SATNAME_1_12: usize = 1;
// Object Type (Payload, Rocket Body, Platform, Debris, Unknown) - csv only
pub const XS_TLE_OBJTYPE_13_1: usize = 13;

pub const XS_TLE_LENGTH: usize = 512;

// Indexes of different TLE file's formats
// Original TLE format
pub const XF_TLEFORM_ORG: usize = 0;
// CSV format
pub const XF_TLEFORM_CSV: usize = 1;

// ========================= End of auto generated code ==========================

pub const DESIGNATOR_LENGTH: usize = 8;

#[derive(Clone, Debug)]
pub struct ParsedTLE {
    pub epoch: f64,
    pub norad_id: i32,
    pub inclination: f64,
    pub raan: f64,
    pub eccentricity: f64,
    pub argument_of_perigee: f64,
    pub mean_anomaly: f64,
    pub mean_motion: f64,
    mean_motion_1st_derivative: Option<f64>,
    mean_motion_2nd_derivative: Option<f64>,
    b_star: Option<f64>,
    ephemeris_type: i32,
    pub element_set_number: i32,
    pub revolution_number: i32,
    pub designator: Option<String>,
    pub classification: String,
    ballistic_coefficient: Option<f64>,
    srp_coefficient: Option<f64>,
}

impl ParsedTLE {
    pub fn get_lines(&self, remove_nulls: bool) -> Result<(String, String), String> {
        let xa_tle: [f64; XA_TLE_SIZE] = self.into();
        let designator = match &self.designator {
            Some(d) => Some(d.clone()),
            None => {
                if remove_nulls {
                    Some("UNKNOWN".to_string())
                } else {
                    None
                }
            }
        };
        let xs_tle = join_xs_tle(&self.classification, &designator).unwrap();
        let (mut line_1, mut line_2) = arrays_to_lines(xa_tle, &xs_tle)?;
        if remove_nulls {
            fix_blank_exponent_sign(&mut line_1);
            add_check_sums(&mut line_1, &mut line_2)?;
        }
        Ok((line_1, line_2))
    }

    pub fn set_mean_motion_1st_derivative(&mut self, value: Option<f64>) {
        match self.ephemeris_type {
            0 | 2 | 4 => {
                self.mean_motion_1st_derivative = Some(value.unwrap_or(0.0));
            }
            _ => {
                self.mean_motion_1st_derivative = None;
            }
        }
    }

    pub fn set_mean_motion_2nd_derivative(&mut self, value: Option<f64>) {
        match self.ephemeris_type {
            0 | 2 => {
                self.mean_motion_2nd_derivative = Some(value.unwrap_or(0.0));
            }
            _ => {
                self.mean_motion_2nd_derivative = None;
            }
        }
    }

    pub fn set_b_star(&mut self, value: Option<f64>) {
        match self.ephemeris_type {
            0 | 2 => {
                self.b_star = Some(value.unwrap_or(0.0));
            }
            _ => {
                self.b_star = None;
            }
        }
    }

    pub fn set_ballistic_coefficient(&mut self, value: Option<f64>) {
        match self.ephemeris_type {
            4 | 6 => {
                self.ballistic_coefficient = Some(value.unwrap_or(0.0));
            }
            _ => {
                self.ballistic_coefficient = None;
            }
        }
    }

    pub fn set_srp_coefficient(&mut self, value: Option<f64>) {
        match self.ephemeris_type {
            4 | 6 => {
                self.srp_coefficient = Some(value.unwrap_or(0.0));
            }
            _ => {
                self.srp_coefficient = None;
            }
        }
    }

    pub fn set_ephemeris_type(&mut self, ephemeris_type: i32) {
        self.ephemeris_type = ephemeris_type;
        self._validate_fields().unwrap();
    }

    pub fn get_ephemeris_type(&self) -> i32 {
        self.ephemeris_type
    }

    pub fn get_mean_motion_1st_derivative(&self) -> Option<f64> {
        self.mean_motion_1st_derivative
    }

    pub fn get_mean_motion_2nd_derivative(&self) -> Option<f64> {
        self.mean_motion_2nd_derivative
    }

    pub fn get_b_star(&self) -> Option<f64> {
        self.b_star
    }

    pub fn get_ballistic_coefficient(&self) -> Option<f64> {
        self.ballistic_coefficient
    }

    pub fn get_srp_coefficient(&self) -> Option<f64> {
        self.srp_coefficient
    }

    fn _validate_fields(&mut self) -> Result<(), String> {
        match self.ephemeris_type {
            0 | 2 => {
                self.ballistic_coefficient = None;
                self.srp_coefficient = None;
                if self.mean_motion_1st_derivative.is_none() {
                    self.mean_motion_1st_derivative = Some(0.0);
                }
                if self.mean_motion_2nd_derivative.is_none() {
                    self.mean_motion_2nd_derivative = Some(0.0);
                }
                if self.b_star.is_none() {
                    self.b_star = Some(0.0);
                }
            }
            4 => {
                self.mean_motion_2nd_derivative = None;
                self.b_star = None;
                if self.mean_motion_1st_derivative.is_none() {
                    self.mean_motion_1st_derivative = Some(0.0);
                }
                if self.ballistic_coefficient.is_none() {
                    self.ballistic_coefficient = Some(0.0);
                }
                if self.srp_coefficient.is_none() {
                    self.srp_coefficient = Some(0.0);
                }
            }
            6 => {
                self.mean_motion_1st_derivative = None;
                self.mean_motion_2nd_derivative = None;
                self.b_star = None;
                if self.ballistic_coefficient.is_none() {
                    self.ballistic_coefficient = Some(0.0);
                }
                if self.srp_coefficient.is_none() {
                    self.srp_coefficient = Some(0.0);
                }
            }
            _ => return Err("Invalid ephemeris type".to_string()),
        }
        Ok(())
    }
}

pub fn fix_blank_exponent_sign(line_1: &mut str) {
    unsafe {
        let bytes = line_1.as_bytes_mut();
        for pos in [50usize, 59usize] {
            if let Some(byte) = bytes.get_mut(pos)
                && *byte == b' '
            {
                *byte = b'+';
            }
        }
    }
}

pub fn add_check_sums(line_1: &mut String, line_2: &mut String) -> Result<(), String> {
    let (sum_1, sum_2) = get_check_sums(line_1, line_2)?;
    add_check_sum_digit(line_1, sum_1);
    add_check_sum_digit(line_2, sum_2);
    Ok(())
}

fn add_check_sum_digit(line: &mut String, sum: i32) {
    let digit = (sum.rem_euclid(10) as u8) + b'0';
    line.push(digit as char);
}

impl Default for ParsedTLE {
    fn default() -> Self {
        ParsedTLE {
            epoch: 0.0,
            norad_id: 0,
            inclination: 0.0,
            raan: 0.0,
            eccentricity: 0.0,
            argument_of_perigee: 0.0,
            mean_anomaly: 0.0,
            mean_motion: 0.0,
            mean_motion_1st_derivative: None,
            mean_motion_2nd_derivative: None,
            b_star: None,
            ephemeris_type: 0,
            element_set_number: 0,
            revolution_number: 0,
            designator: None,
            classification: String::from("U"),
            ballistic_coefficient: None,
            srp_coefficient: None,
        }
    }
}

fn get_classification_str(xs_tle: &str) -> &str {
    &xs_tle[XS_TLE_SECCLASS_1..XS_TLE_SECCLASS_1 + 1]
}

fn get_designator_string(xs_tle: &str) -> Option<String> {
    let designator = xs_tle[XS_TLE_SATNAME_12..XS_TLE_SATNAME_12 + DESIGNATOR_LENGTH]
        .trim()
        .to_string();
    if designator.is_empty() { None } else { Some(designator) }
}

fn join_xs_tle(classification: &str, designator: &Option<String>) -> Result<String, String> {
    let mut xs_tle = GetSetString::new();
    xs_tle.set(XS_TLE_SECCLASS_0_1, classification)?;
    if let Some(desig) = designator {
        xs_tle.set(XS_TLE_SATNAME_1_12, desig)?;
    }
    Ok(xs_tle.value())
}

impl From<([f64; XA_TLE_SIZE], String)> for ParsedTLE {
    fn from(value: ([f64; XA_TLE_SIZE], String)) -> Self {
        let xa_tle = value.0;
        let xs_tle = value.1;
        let classification = get_classification_str(&xs_tle).to_string();
        let ephemeris_type = xa_tle[XA_TLE_EPHTYPE] as i32;
        let mean_motion_1st_derivative = match ephemeris_type {
            0 | 2 | 4 => Some(xa_tle[XA_TLE_NDOT]),
            _ => None,
        };
        let mean_motion_2nd_derivative = match ephemeris_type {
            0 | 2 => Some(xa_tle[XA_TLE_NDOTDOT]),
            _ => None,
        };
        let b_star = match ephemeris_type {
            0 | 2 => Some(xa_tle[XA_TLE_BSTAR]),
            _ => None,
        };
        let ballistic_coefficient = match ephemeris_type {
            4 | 6 => Some(xa_tle[XA_TLE_BTERM]),
            _ => None,
        };

        let srp_coefficient = match ephemeris_type {
            4 | 6 => Some(xa_tle[XA_TLE_AGOMGP]),
            _ => None,
        };
        ParsedTLE {
            epoch: xa_tle[XA_TLE_EPOCH],
            norad_id: xa_tle[XA_TLE_SATNUM] as i32,
            inclination: xa_tle[XA_TLE_INCLI],
            raan: xa_tle[XA_TLE_NODE],
            eccentricity: xa_tle[XA_TLE_ECCEN],
            argument_of_perigee: xa_tle[XA_TLE_OMEGA],
            mean_anomaly: xa_tle[XA_TLE_MNANOM],
            mean_motion: xa_tle[XA_TLE_MNMOTN],
            mean_motion_1st_derivative,
            mean_motion_2nd_derivative,
            b_star,
            ephemeris_type,
            element_set_number: xa_tle[XA_TLE_ELSETNUM] as i32,
            revolution_number: xa_tle[XA_TLE_REVNUM] as i32,
            designator: get_designator_string(&xs_tle),
            classification,
            ballistic_coefficient,
            srp_coefficient,
        }
    }
}
impl From<&ParsedTLE> for [f64; XA_TLE_SIZE] {
    fn from(value: &ParsedTLE) -> Self {
        let mut xa_tle = [0.0; XA_TLE_SIZE];
        xa_tle[XA_TLE_SATNUM] = value.norad_id as f64;
        xa_tle[XA_TLE_EPOCH] = value.epoch;
        xa_tle[XA_TLE_EPHTYPE] = value.ephemeris_type.into();
        xa_tle[XA_TLE_INCLI] = value.inclination;
        xa_tle[XA_TLE_NODE] = value.raan;
        xa_tle[XA_TLE_ECCEN] = value.eccentricity;
        xa_tle[XA_TLE_OMEGA] = value.argument_of_perigee;
        xa_tle[XA_TLE_MNANOM] = value.mean_anomaly;
        xa_tle[XA_TLE_MNMOTN] = value.mean_motion;
        xa_tle[XA_TLE_ELSETNUM] = value.element_set_number as f64;
        xa_tle[XA_TLE_REVNUM] = value.revolution_number as f64;

        match value.ephemeris_type {
            0 | 2 => {
                if let Some(v) = value.mean_motion_1st_derivative {
                    xa_tle[XA_TLE_NDOT] = v;
                }
                if let Some(v) = value.mean_motion_2nd_derivative {
                    xa_tle[XA_TLE_NDOTDOT] = v;
                }
                if let Some(v) = value.b_star {
                    xa_tle[XA_TLE_BSTAR] = v;
                }
            }

            4 | 6 => {
                if let Some(v) = value.mean_motion_1st_derivative {
                    xa_tle[XA_TLE_NDOT] = v;
                }
                if let Some(v) = value.ballistic_coefficient {
                    xa_tle[XA_TLE_BTERM] = v;
                }
                if let Some(v) = value.srp_coefficient {
                    xa_tle[XA_TLE_AGOMGP] = v;
                }
            }
            _ => {}
        }

        xa_tle
    }
}
impl From<ParsedTLE> for ([f64; XA_TLE_SIZE], String) {
    fn from(value: ParsedTLE) -> Self {
        let xa_tle = (&value).into();
        let xs_tle = join_xs_tle(&value.classification, &value.designator).unwrap();
        (xa_tle, xs_tle)
    }
}

pub fn get_dll_info() -> String {
    let mut c_info = GetSetString::new();
    unsafe { TleGetInfo(c_info.pointer()) };
    c_info.value()
}

pub fn lines_to_arrays(line_1: &str, line_2: &str) -> Result<([f64; XA_TLE_SIZE], String), String> {
    let mut xa_tle = [0.0; XA_TLE_SIZE];
    let mut xs_tle = GetSetString::new();
    let mut c_line_1: GetSetString = line_1.into();
    let mut c_line_2: GetSetString = line_2.into();
    let result = unsafe {
        TleLinesToArray(
            c_line_1.pointer(),
            c_line_2.pointer(),
            xa_tle.as_mut_ptr() as *mut [f64; XA_TLE_SIZE],
            xs_tle.pointer(),
        )
    };
    match result {
        0 => Ok((xa_tle, xs_tle.value())),
        _ => Err(get_last_error_message()),
    }
}

pub fn remove(sat_key: i64) {
    unsafe { TleRemoveSat(sat_key) };
}

pub fn load_file(file_path: &str) -> Result<i32, String> {
    let mut tle_path: GetSetString = file_path.into();
    let result = unsafe { TleLoadFile(tle_path.pointer()) };
    match result {
        n if n >= 0 => Ok(n),
        _ => Err(get_last_error_message()),
    }
}

pub fn clear() -> Result<(), String> {
    let result = unsafe { TleRemoveAllSats() };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_count() -> i32 {
    unsafe { TleGetCount() }
}

pub fn parse_lines(line_1: &str, line_2: &str) -> Result<ParsedTLE, String> {
    let (xa_tle, xs_tle) = lines_to_arrays(line_1, line_2)?;
    Ok(ParsedTLE::from((xa_tle, xs_tle)))
}

pub fn get_arrays(sat_key: i64) -> Result<([f64; XA_TLE_SIZE], String), String> {
    let mut xa_tle = [0.0; XA_TLE_SIZE];
    let mut xs_tle = GetSetString::new();
    let result = unsafe {
        TleDataToArray(
            sat_key,
            xa_tle.as_mut_ptr() as *mut [f64; XA_TLE_SIZE],
            xs_tle.pointer(),
        )
    };
    match result {
        0 => Ok((xa_tle, xs_tle.value())),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_lines(sat_key: i64) -> Result<(String, String), String> {
    let mut line_1 = GetSetString::new();
    let mut line_2 = GetSetString::new();
    let result = unsafe { TleGetLines(sat_key, line_1.pointer(), line_2.pointer()) };
    match result {
        0 => Ok((line_1.value().trim().to_string(), line_2.value().trim().to_string())),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_keys(order: i32) -> Vec<i64> {
    let count = get_count() as usize;
    let mut keys = vec![0_i64; count];
    unsafe {
        TleGetLoaded(order, keys.as_mut_ptr());
    }
    keys
}

pub fn load_arrays(xa_tle: [f64; XA_TLE_SIZE], xs_tle: &str) -> Result<i64, String> {
    let mut c_xs_tle: GetSetString = xs_tle.into();
    let key = unsafe { TleAddSatFrArray(&xa_tle, c_xs_tle.pointer()) };
    if key > 0 {
        Ok(key)
    } else {
        Err(get_last_error_message())
    }
}

pub fn load_lines(line_1: &str, line_2: &str) -> i64 {
    unsafe {
        let mut c_line_1: GetSetString = line_1.into();
        let mut c_line_2: GetSetString = line_2.into();
        TleAddSatFrLines(c_line_1.pointer(), c_line_2.pointer())
    }
}

pub fn arrays_to_lines(xa_tle: [f64; XA_TLE_SIZE], xs_tle: &str) -> Result<(String, String), String> {
    let mut c_line_1 = GetSetString::new();
    let mut c_line_2 = GetSetString::new();
    let mut c_xs_tle: GetSetString = xs_tle.into();
    unsafe { TleGPArrayToLines(&xa_tle, c_xs_tle.pointer(), c_line_1.pointer(), c_line_2.pointer()) };

    Ok((c_line_1.value().trim().to_string(), c_line_2.value().trim().to_string()))
}

pub fn get_check_sums(line_1: &str, line_2: &str) -> Result<(i32, i32), String> {
    let mut chk_sum_1: i32 = 0;
    let mut chk_sum_2: i32 = 0;
    let mut err_code: i32 = 0;
    let mut c_line_1: GetSetString = line_1.into();
    let mut c_line_2: GetSetString = line_2.into();
    unsafe {
        GetCheckSums(
            c_line_1.pointer(),
            c_line_2.pointer(),
            &mut chk_sum_1,
            &mut chk_sum_2,
            &mut err_code,
        )
    };
    if err_code == 0 {
        Ok((chk_sum_1, chk_sum_2))
    } else {
        Err(get_last_error_message())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::test_lock::TEST_LOCK;
    use crate::{DLL_VERSION, IDX_ORDER_DES, IDX_ORDER_READ};

    const SGP_LINE_1: &str = "1 11111U 98067A   25363.54791667 +.00012345  10000-1  20000-1 0 0900";
    const SGP_LINE_2: &str = "2 11111  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345";
    const NULL_LINE_1: &str = "1 11111U          25363.54791667 +.00012345  00000 0  00000 0 0 0900";
    const FMTD_LINE_1: &str = "1 11111U UNKNOWN  25363.54791667 +.00012345  00000+0  00000+0 0 09004";
    const SGP4_LINE_1: &str = "1 22222C 15058A   25363.54791667 +.00012345  10000-1  20000-1 2 0900";
    const SGP4_LINE_2: &str = "2 22222  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345";
    const XP_LINE_1: &str = "1 33333S 21001A   25363.54791667 +.00012345  10000-1  20000-1 4 0900";
    const XP_LINE_2: &str = "2 33333  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345";
    const SP_LINE_1: &str = "1 44444U 67001A   25363.54791667 +.02000000  00000 0  10000-1 6 0900";
    const SP_LINE_2: &str = "2 44444  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345";
    const SGP_NORAD_ID: f64 = 11111.0;
    const SGP_B_STAR: f64 = 0.02;
    const SGP_MEAN_MOTION_1ST_DERIVATIVE: f64 = 0.00012345;
    const SGP_MEAN_MOTION_2ND_DERIVATIVE: f64 = 0.01;
    const SGP_BALLISTIC_COEFFICIENT: f64 = 0.0;
    const SGP_EPHEMERIS_TYPE: f64 = 0.0;
    const SGP_SRP_COEFFICIENT: f64 = 0.0;
    const SGP4_NORAD_ID: f64 = 22222.0;
    const SGP4_B_STAR: f64 = 0.02;
    const SGP4_MEAN_MOTION_1ST_DERIVATIVE: f64 = 0.00012345;
    const SGP4_MEAN_MOTION_2ND_DERIVATIVE: f64 = 0.01;
    const SGP4_BALLISTIC_COEFFICIENT: f64 = 0.0;
    const SGP4_EPHEMERIS_TYPE: f64 = 2.0;
    const SGP4_SRP_COEFFICIENT: f64 = 0.0;
    const XP_NORAD_ID: f64 = 33333.0;
    const XP_MEAN_MOTION_1ST_DERIVATIVE: f64 = 0.00012345;
    const XP_BALLISTIC_COEFFICIENT: f64 = 0.02;
    const XP_EPHEMERIS_TYPE: f64 = 4.0;
    const XP_SRP_COEFFICIENT: f64 = 0.01;
    const SP_NORAD_ID: f64 = 44444.0;
    const SP_BALLISTIC_COEFFICIENT: f64 = 0.02;
    const SP_EPHEMERIS_TYPE: f64 = 6.0;
    const SP_SRP_COEFFICIENT: f64 = 0.01;
    const SGP_DESIGNATOR: &str = "98067A";
    const SGP4_DESIGNATOR: &str = "15058A";
    const XP_DESIGNATOR: &str = "21001A";
    const SP_DESIGNATOR: &str = "67001A";
    const EPOCH: f64 = 27757.54791667;
    const INCLINATION: f64 = 30.0;
    const RAAN: f64 = 40.0;
    const ECCENTRICITY: f64 = 0.0005;
    const ARGUMENT_OF_PERIGEE: f64 = 60.0;
    const MEAN_ANOMALY: f64 = 70.0;
    const MEAN_MOTION: f64 = 1.2345678;
    const B_STAR: f64 = 0.002;
    const SGP_XS_TLE: &str = "U98067A";
    const SGP4_XS_TLE: &str = "C15058A";
    const XP_XS_TLE: &str = "S21001A";
    const SP_XS_TLE: &str = "U67001A";

    #[test]
    fn test_get_lines() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp_key = load_lines(SGP_LINE_1, SGP_LINE_2);
        let sgp4_key = load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = load_lines(XP_LINE_1, XP_LINE_2);
        let sp_key = load_lines(SP_LINE_1, SP_LINE_2);
        let (line_1, line_2) = get_lines(sgp_key).unwrap();
        let (line_1_sgp4, line_2_sgp4) = get_lines(sgp4_key).unwrap();
        let (line_1_xp, line_2_xp) = get_lines(xp_key).unwrap();
        let (line_1_sp, line_2_sp) = get_lines(sp_key).unwrap();
        assert_eq!(line_1_sgp4, SGP4_LINE_1);
        assert_eq!(line_2_sgp4, SGP4_LINE_2);
        assert_eq!(line_1_xp, XP_LINE_1);
        assert_eq!(line_2_xp, XP_LINE_2);
        assert_eq!(line_1_sp, SP_LINE_1);
        assert_eq!(line_2_sp, SP_LINE_2);
        assert_eq!(line_1, SGP_LINE_1);
        assert_eq!(line_2, SGP_LINE_2);

        let _ = clear();
    }

    #[test]
    fn test_get_arrays() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp_key = load_lines(SGP_LINE_1, SGP_LINE_2);
        let sgp4_key = load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = load_lines(XP_LINE_1, XP_LINE_2);
        let sp_key = load_lines(SP_LINE_1, SP_LINE_2);
        let (expected_xa_tle, expected_xs_tle) = lines_to_arrays(SGP_LINE_1, SGP_LINE_2).unwrap();
        let (expected_xa_sgp4, expected_xs_sgp4) = lines_to_arrays(SGP4_LINE_1, SGP4_LINE_2).unwrap();
        let (expected_xa_xp, expected_xs_xp) = lines_to_arrays(XP_LINE_1, XP_LINE_2).unwrap();
        let (expected_xa_sp, expected_xs_sp) = lines_to_arrays(SP_LINE_1, SP_LINE_2).unwrap();
        let (xa_tle, xs_tle) = get_arrays(sgp_key).unwrap();
        let (xa_sgp4, xs_sgp4) = get_arrays(sgp4_key).unwrap();
        let (xa_xp, xs_xp) = get_arrays(xp_key).unwrap();
        let (xa_sp, xs_sp) = get_arrays(sp_key).unwrap();
        assert_eq!(xs_tle, expected_xs_tle);
        assert_eq!(xa_tle, expected_xa_tle);
        assert_eq!(xs_sgp4, expected_xs_sgp4);
        assert_eq!(xa_sgp4, expected_xa_sgp4);
        assert_eq!(xs_xp, expected_xs_xp);
        assert_eq!(xa_xp, expected_xa_xp);
        assert_eq!(xs_sp, expected_xs_sp);
        assert_eq!(xa_sp, expected_xa_sp);
        let _ = clear();
    }

    #[test]
    fn test_get_keys() {
        let _lock = TEST_LOCK.lock().unwrap();
        let sgp4_key = load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let sgp_key = load_lines(SGP_LINE_1, SGP_LINE_2);
        let xp_key = load_lines(XP_LINE_1, XP_LINE_2);
        let sp_key = load_lines(SP_LINE_1, SP_LINE_2);
        let final_count = get_count();
        assert_eq!(final_count, 4);

        let keys_asc = get_keys(IDX_ORDER_READ);
        assert_eq!(keys_asc, vec![sgp4_key, sgp_key, xp_key, sp_key]);

        let keys_desc = get_keys(IDX_ORDER_DES);
        assert_eq!(keys_desc, vec![sp_key, xp_key, sgp4_key, sgp_key]);

        let _ = clear();
    }

    #[test]
    fn test_remove_nulls() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (line_1, _line_2) = lines_to_arrays(NULL_LINE_1, SGP_LINE_2)
            .map(|(xa_tle, xs_tle)| ParsedTLE::from((xa_tle, xs_tle)))
            .and_then(|parsed_tle| parsed_tle.get_lines(true))
            .unwrap();
        assert_eq!(line_1, FMTD_LINE_1);
    }

    #[test]
    fn test_parsed_tles_to_lines() {
        let _lock = TEST_LOCK.lock().unwrap();
        // SGP
        let mut parsed_sgp = ParsedTLE {
            norad_id: SGP_NORAD_ID as i32,
            designator: Some(SGP_DESIGNATOR.to_string()),
            ephemeris_type: 0,
            classification: String::from("U"),
            epoch: EPOCH,
            inclination: INCLINATION,
            raan: RAAN,
            eccentricity: ECCENTRICITY,
            argument_of_perigee: ARGUMENT_OF_PERIGEE,
            mean_anomaly: MEAN_ANOMALY,
            mean_motion: MEAN_MOTION,
            ..Default::default()
        };
        parsed_sgp.set_b_star(Some(SGP_B_STAR));
        parsed_sgp.set_mean_motion_1st_derivative(Some(SGP_MEAN_MOTION_1ST_DERIVATIVE));
        parsed_sgp.set_mean_motion_2nd_derivative(Some(SGP_MEAN_MOTION_2ND_DERIVATIVE));
        parsed_sgp.set_ballistic_coefficient(Some(1.1)); // should be ignored
        parsed_sgp.set_srp_coefficient(Some(2.2)); // should be ignored
        parsed_sgp.element_set_number = 900;
        parsed_sgp.revolution_number = 12345;

        let mut parsed_sgp4 = ParsedTLE {
            norad_id: SGP4_NORAD_ID as i32,
            designator: Some(SGP4_DESIGNATOR.to_string()),
            ephemeris_type: 2,
            classification: String::from("C"),
            epoch: EPOCH,
            inclination: INCLINATION,
            raan: RAAN,
            eccentricity: ECCENTRICITY,
            argument_of_perigee: ARGUMENT_OF_PERIGEE,
            mean_anomaly: MEAN_ANOMALY,
            mean_motion: MEAN_MOTION,
            ..Default::default()
        };
        parsed_sgp4.set_b_star(Some(SGP4_B_STAR));
        parsed_sgp4.set_mean_motion_1st_derivative(Some(SGP4_MEAN_MOTION_1ST_DERIVATIVE));
        parsed_sgp4.set_mean_motion_2nd_derivative(Some(SGP4_MEAN_MOTION_2ND_DERIVATIVE));
        parsed_sgp4.set_ballistic_coefficient(Some(1.1)); // should be ignored
        parsed_sgp4.set_srp_coefficient(Some(2.2)); // should be ignored
        parsed_sgp4.element_set_number = 900;
        parsed_sgp4.revolution_number = 12345;

        let mut parsed_xp = ParsedTLE {
            norad_id: XP_NORAD_ID as i32,
            designator: Some(XP_DESIGNATOR.to_string()),
            ephemeris_type: 4,
            classification: String::from("S"),
            epoch: EPOCH,
            inclination: INCLINATION,
            raan: RAAN,
            eccentricity: ECCENTRICITY,
            argument_of_perigee: ARGUMENT_OF_PERIGEE,
            mean_anomaly: MEAN_ANOMALY,
            mean_motion: MEAN_MOTION,
            ..Default::default()
        };
        parsed_xp.set_mean_motion_1st_derivative(Some(XP_MEAN_MOTION_1ST_DERIVATIVE));
        parsed_xp.set_ballistic_coefficient(Some(XP_BALLISTIC_COEFFICIENT));
        parsed_xp.set_srp_coefficient(Some(XP_SRP_COEFFICIENT));
        parsed_xp.set_b_star(Some(1.1)); // should be ignored
        parsed_xp.set_mean_motion_2nd_derivative(Some(2.2)); // should be ignored
        parsed_xp.element_set_number = 900;
        parsed_xp.revolution_number = 12345;

        let mut parsed_sp = ParsedTLE {
            norad_id: SP_NORAD_ID as i32,
            designator: Some(SP_DESIGNATOR.to_string()),
            ephemeris_type: 6,
            classification: String::from("U"),
            epoch: EPOCH,
            inclination: INCLINATION,
            raan: RAAN,
            eccentricity: ECCENTRICITY,
            argument_of_perigee: ARGUMENT_OF_PERIGEE,
            mean_anomaly: MEAN_ANOMALY,
            mean_motion: MEAN_MOTION,
            ..Default::default()
        };
        parsed_sp.set_ballistic_coefficient(Some(SP_BALLISTIC_COEFFICIENT));
        parsed_sp.set_srp_coefficient(Some(SP_SRP_COEFFICIENT));
        parsed_sp.set_b_star(Some(1.1)); // should be ignored
        parsed_sp.set_mean_motion_1st_derivative(Some(2.2)); // should be ignored
        parsed_sp.set_mean_motion_2nd_derivative(Some(3.3)); // should be ignored
        parsed_sp.element_set_number = 900;
        parsed_sp.revolution_number = 12345;

        let (sgp_line_1, sgp_line_2) = parsed_sgp.get_lines(false).unwrap();
        let (sgp4_line_1, sgp4_line_2) = parsed_sgp4.get_lines(false).unwrap();
        let (xp_line_1, xp_line_2) = parsed_xp.get_lines(false).unwrap();
        let (sp_line_1, sp_line_2) = parsed_sp.get_lines(false).unwrap();
        assert_eq!(sgp_line_1, SGP_LINE_1);
        assert_eq!(sgp_line_2, SGP_LINE_2);
        assert_eq!(parsed_sgp.ballistic_coefficient, None);
        assert_eq!(parsed_sgp.srp_coefficient, None);
        assert_eq!(sgp4_line_1, SGP4_LINE_1);
        assert_eq!(sgp4_line_2, SGP4_LINE_2);
        assert_eq!(parsed_sgp4.ballistic_coefficient, None);
        assert_eq!(parsed_sgp4.srp_coefficient, None);
        assert_eq!(xp_line_1, XP_LINE_1);
        assert_eq!(xp_line_2, XP_LINE_2);
        assert_eq!(parsed_xp.b_star, None);
        assert_eq!(parsed_xp.mean_motion_2nd_derivative, None);
        assert_eq!(sp_line_1, SP_LINE_1);
        assert_eq!(sp_line_2, SP_LINE_2);
        assert_eq!(parsed_sp.b_star, None);
        assert_eq!(parsed_sp.mean_motion_1st_derivative, None);
        assert_eq!(parsed_sp.mean_motion_2nd_derivative, None);
    }

    #[test]
    fn test_arrays_to_parsed_tles() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (xa_sgp, xs_sgp) = lines_to_arrays(SGP_LINE_1, SGP_LINE_2).unwrap();
        let (xa_sgp4, xs_sgp4) = lines_to_arrays(SGP4_LINE_1, SGP4_LINE_2).unwrap();
        let (xa_xp, xs_xp) = lines_to_arrays(XP_LINE_1, XP_LINE_2).unwrap();
        let (xa_sp, xs_sp) = lines_to_arrays(SP_LINE_1, SP_LINE_2).unwrap();

        // SGP
        let parsed_sgp: ParsedTLE = (xa_sgp, xs_sgp).into();
        assert_eq!(parsed_sgp.norad_id, SGP_NORAD_ID as i32);
        assert_eq!(parsed_sgp.designator.as_deref(), Some(SGP_DESIGNATOR));
        assert_abs_diff_eq!(parsed_sgp.b_star.unwrap(), SGP_B_STAR, epsilon = 1e-10);
        assert_abs_diff_eq!(
            parsed_sgp.mean_motion_1st_derivative.unwrap(),
            SGP_MEAN_MOTION_1ST_DERIVATIVE,
            epsilon = 1e-10
        );
        assert_abs_diff_eq!(
            parsed_sgp.mean_motion_2nd_derivative.unwrap(),
            SGP_MEAN_MOTION_2ND_DERIVATIVE,
            epsilon = 1e-10
        );
        assert_eq!(parsed_sgp.ballistic_coefficient, None);
        assert_eq!(parsed_sgp.ephemeris_type, 0);
        assert_eq!(parsed_sgp.srp_coefficient, None);
        assert_eq!(parsed_sgp.classification, String::from("U"));
        assert_eq!(parsed_sgp.epoch, EPOCH);
        assert_eq!(parsed_sgp.inclination, INCLINATION);
        assert_eq!(parsed_sgp.raan, RAAN);
        assert_eq!(parsed_sgp.eccentricity, ECCENTRICITY);
        assert_eq!(parsed_sgp.argument_of_perigee, ARGUMENT_OF_PERIGEE);
        assert_eq!(parsed_sgp.mean_anomaly, MEAN_ANOMALY);
        assert_eq!(parsed_sgp.mean_motion, MEAN_MOTION);

        // SGP4
        let parsed_sgp4: ParsedTLE = (xa_sgp4, xs_sgp4).into();
        assert_eq!(parsed_sgp4.norad_id, SGP4_NORAD_ID as i32);
        assert_eq!(parsed_sgp4.designator.as_deref(), Some(SGP4_DESIGNATOR));
        assert_abs_diff_eq!(parsed_sgp4.b_star.unwrap(), SGP4_B_STAR, epsilon = 1e-10);
        assert_abs_diff_eq!(
            parsed_sgp4.mean_motion_1st_derivative.unwrap(),
            SGP4_MEAN_MOTION_1ST_DERIVATIVE,
            epsilon = 1e-10
        );
        assert_abs_diff_eq!(
            parsed_sgp4.mean_motion_2nd_derivative.unwrap(),
            SGP4_MEAN_MOTION_2ND_DERIVATIVE,
            epsilon = 1e-10
        );
        assert_eq!(parsed_sgp4.ballistic_coefficient, None);
        assert_eq!(parsed_sgp4.ephemeris_type, 2);
        assert_eq!(parsed_sgp4.srp_coefficient, None);
        assert_eq!(parsed_sgp4.classification, String::from("C"));
        assert_eq!(parsed_sgp4.epoch, EPOCH);
        assert_eq!(parsed_sgp4.inclination, INCLINATION);
        assert_eq!(parsed_sgp4.raan, RAAN);
        assert_eq!(parsed_sgp4.eccentricity, ECCENTRICITY);
        assert_eq!(parsed_sgp4.argument_of_perigee, ARGUMENT_OF_PERIGEE);
        assert_eq!(parsed_sgp4.mean_anomaly, MEAN_ANOMALY);
        assert_eq!(parsed_sgp4.mean_motion, MEAN_MOTION);

        // XP
        let parsed_xp: ParsedTLE = (xa_xp, xs_xp).into();
        assert_eq!(parsed_xp.norad_id, XP_NORAD_ID as i32);
        assert_eq!(parsed_xp.designator.as_deref(), Some(XP_DESIGNATOR));
        assert_eq!(parsed_xp.b_star, None);
        assert_abs_diff_eq!(
            parsed_xp.mean_motion_1st_derivative.unwrap(),
            XP_MEAN_MOTION_1ST_DERIVATIVE,
            epsilon = 1e-10
        );
        assert_eq!(parsed_xp.mean_motion_2nd_derivative, None);
        assert_abs_diff_eq!(
            parsed_xp.ballistic_coefficient.unwrap(),
            XP_BALLISTIC_COEFFICIENT,
            epsilon = 1e-10
        );
        assert_eq!(parsed_xp.ephemeris_type, 4);
        assert_abs_diff_eq!(parsed_xp.srp_coefficient.unwrap(), XP_SRP_COEFFICIENT, epsilon = 1e-10);
        assert_eq!(parsed_xp.classification, String::from("S"));
        assert_eq!(parsed_xp.epoch, EPOCH);
        assert_eq!(parsed_xp.inclination, INCLINATION);
        assert_eq!(parsed_xp.raan, RAAN);
        assert_eq!(parsed_xp.eccentricity, ECCENTRICITY);
        assert_eq!(parsed_xp.argument_of_perigee, ARGUMENT_OF_PERIGEE);
        assert_eq!(parsed_xp.mean_anomaly, MEAN_ANOMALY);
        assert_eq!(parsed_xp.mean_motion, MEAN_MOTION);

        // SP
        let parsed_sp: ParsedTLE = (xa_sp, xs_sp).into();
        assert_eq!(parsed_sp.norad_id, SP_NORAD_ID as i32);
        assert_eq!(parsed_sp.designator.as_deref(), Some(SP_DESIGNATOR));
        assert_eq!(parsed_sp.b_star, None);
        assert_eq!(parsed_sp.mean_motion_1st_derivative, None);
        assert_eq!(parsed_sp.mean_motion_2nd_derivative, None);
        assert_abs_diff_eq!(
            parsed_sp.ballistic_coefficient.unwrap(),
            SP_BALLISTIC_COEFFICIENT,
            epsilon = 1e-10
        );
        assert_eq!(parsed_sp.ephemeris_type, 6);
        assert_abs_diff_eq!(parsed_sp.srp_coefficient.unwrap(), SP_SRP_COEFFICIENT, epsilon = 1e-10);
        assert_eq!(parsed_sp.classification, String::from("U"));
        assert_eq!(parsed_sp.epoch, EPOCH);
        assert_eq!(parsed_sp.inclination, INCLINATION);
        assert_eq!(parsed_sp.raan, RAAN);
        assert_eq!(parsed_sp.eccentricity, ECCENTRICITY);
        assert_eq!(parsed_sp.argument_of_perigee, ARGUMENT_OF_PERIGEE);
        assert_eq!(parsed_sp.mean_anomaly, MEAN_ANOMALY);
        assert_eq!(parsed_sp.mean_motion, MEAN_MOTION);
    }

    #[test]
    fn test_arrays_to_lines() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (xa_sgp, xs_sgp) = lines_to_arrays(SGP_LINE_1, SGP_LINE_2).unwrap();
        let (xa_sgp4, xs_sgp4) = lines_to_arrays(SGP4_LINE_1, SGP4_LINE_2).unwrap();
        let (xa_xp, xs_xp) = lines_to_arrays(XP_LINE_1, XP_LINE_2).unwrap();
        let (xa_sp, xs_sp) = lines_to_arrays(SP_LINE_1, SP_LINE_2).unwrap();

        // SGP
        let (line1_sgp, line2_sgp) = arrays_to_lines(xa_sgp, &xs_sgp).unwrap();
        assert_eq!(line1_sgp.trim(), SGP_LINE_1);
        assert_eq!(line2_sgp.trim(), SGP_LINE_2);

        // SGP4
        let (line1_sgp4, line2_sgp4) = arrays_to_lines(xa_sgp4, &xs_sgp4).unwrap();
        assert_eq!(line1_sgp4.trim(), SGP4_LINE_1);
        assert_eq!(line2_sgp4.trim(), SGP4_LINE_2);

        // XP
        let (line1_xp, line2_xp) = arrays_to_lines(xa_xp, &xs_xp).unwrap();
        assert_eq!(line1_xp.trim(), XP_LINE_1);
        assert_eq!(line2_xp.trim(), XP_LINE_2);

        // SP
        let (line1_sp, line2_sp) = arrays_to_lines(xa_sp, &xs_sp).unwrap();
        assert_eq!(line1_sp.trim(), SP_LINE_1);
        assert_eq!(line2_sp.trim(), SP_LINE_2);
    }

    #[test]
    fn test_lines_to_arrays() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (xa_sgp, xs_sgp) = lines_to_arrays(SGP_LINE_1, SGP_LINE_2).unwrap();
        let (xa_sgp4, xs_sgp4) = lines_to_arrays(SGP4_LINE_1, SGP4_LINE_2).unwrap();
        let (xa_xp, xs_xp) = lines_to_arrays(XP_LINE_1, XP_LINE_2).unwrap();
        let (xa_sp, xs_sp) = lines_to_arrays(SP_LINE_1, SP_LINE_2).unwrap();

        // SGP
        assert_eq!(xa_sgp[XA_TLE_EPOCH], EPOCH);
        assert_eq!(xa_sgp[XA_TLE_SATNUM], SGP_NORAD_ID);
        assert_eq!(xa_sgp[XA_TLE_INCLI], INCLINATION);
        assert_eq!(xa_sgp[XA_TLE_NODE], RAAN);
        assert_eq!(xa_sgp[XA_TLE_ECCEN], ECCENTRICITY);
        assert_eq!(xa_sgp[XA_TLE_OMEGA], ARGUMENT_OF_PERIGEE);
        assert_eq!(xa_sgp[XA_TLE_MNANOM], MEAN_ANOMALY);
        assert_eq!(xa_sgp[XA_TLE_MNMOTN], MEAN_MOTION);
        assert_abs_diff_eq!(xa_sgp[XA_TLE_BSTAR], SGP_B_STAR, epsilon = 1e-8);
        assert_eq!(xa_sgp[XA_TLE_EPHTYPE], SGP_EPHEMERIS_TYPE);
        assert_eq!(xa_sgp[XA_TLE_NDOT], SGP_MEAN_MOTION_1ST_DERIVATIVE);
        assert_abs_diff_eq!(xa_sgp[XA_TLE_NDOTDOT], SGP_MEAN_MOTION_2ND_DERIVATIVE, epsilon = 1e-8);
        assert_eq!(xa_sgp[XA_TLE_BTERM], SGP_BALLISTIC_COEFFICIENT);
        assert_eq!(xa_sgp[XA_TLE_AGOMGP], SGP_SRP_COEFFICIENT);
        assert_eq!(xs_sgp.trim(), SGP_XS_TLE);

        // SGP4
        assert_eq!(xa_sgp4[XA_TLE_EPOCH], EPOCH);
        assert_eq!(xa_sgp4[XA_TLE_SATNUM], SGP4_NORAD_ID);
        assert_eq!(xa_sgp4[XA_TLE_INCLI], INCLINATION);
        assert_eq!(xa_sgp4[XA_TLE_NODE], RAAN);
        assert_eq!(xa_sgp4[XA_TLE_ECCEN], ECCENTRICITY);
        assert_eq!(xa_sgp4[XA_TLE_OMEGA], ARGUMENT_OF_PERIGEE);
        assert_eq!(xa_sgp4[XA_TLE_MNANOM], MEAN_ANOMALY);
        assert_eq!(xa_sgp4[XA_TLE_MNMOTN], MEAN_MOTION);
        assert_abs_diff_eq!(xa_sgp4[XA_TLE_BSTAR], SGP4_B_STAR, epsilon = 1e-8);
        assert_eq!(xa_sgp4[XA_TLE_EPHTYPE], SGP4_EPHEMERIS_TYPE);
        assert_eq!(xa_sgp4[XA_TLE_NDOT], SGP4_MEAN_MOTION_1ST_DERIVATIVE);
        assert_abs_diff_eq!(xa_sgp4[XA_TLE_NDOTDOT], SGP4_MEAN_MOTION_2ND_DERIVATIVE, epsilon = 1e-8);
        assert_eq!(xa_sgp4[XA_TLE_BTERM], SGP4_BALLISTIC_COEFFICIENT);
        assert_eq!(xa_sgp4[XA_TLE_AGOMGP], SGP4_SRP_COEFFICIENT);
        assert_eq!(xs_sgp4.trim(), SGP4_XS_TLE);

        // XP
        assert_eq!(xa_xp[XA_TLE_EPOCH], EPOCH);
        assert_eq!(xa_xp[XA_TLE_SATNUM], XP_NORAD_ID);
        assert_eq!(xa_xp[XA_TLE_INCLI], INCLINATION);
        assert_eq!(xa_xp[XA_TLE_NODE], RAAN);
        assert_eq!(xa_xp[XA_TLE_ECCEN], ECCENTRICITY);
        assert_eq!(xa_xp[XA_TLE_OMEGA], ARGUMENT_OF_PERIGEE);
        assert_eq!(xa_xp[XA_TLE_MNANOM], MEAN_ANOMALY);
        assert_eq!(xa_xp[XA_TLE_MNMOTN], MEAN_MOTION);
        assert_eq!(xa_xp[XA_TLE_EPHTYPE], XP_EPHEMERIS_TYPE);
        assert_eq!(xa_xp[XA_TLE_NDOT], XP_MEAN_MOTION_1ST_DERIVATIVE);
        assert_abs_diff_eq!(xa_xp[XA_TLE_BTERM], XP_BALLISTIC_COEFFICIENT);
        assert_abs_diff_eq!(xa_xp[XA_TLE_AGOMGP], XP_SRP_COEFFICIENT);
        assert_eq!(xs_xp.trim(), XP_XS_TLE);

        // SP
        assert_eq!(xa_sp[XA_TLE_EPOCH], EPOCH);
        assert_eq!(xa_sp[XA_TLE_SATNUM], SP_NORAD_ID);
        assert_eq!(xa_sp[XA_TLE_INCLI], INCLINATION);
        assert_eq!(xa_sp[XA_TLE_NODE], RAAN);
        assert_eq!(xa_sp[XA_TLE_ECCEN], ECCENTRICITY);
        assert_eq!(xa_sp[XA_TLE_OMEGA], ARGUMENT_OF_PERIGEE);
        assert_eq!(xa_sp[XA_TLE_MNANOM], MEAN_ANOMALY);
        assert_eq!(xa_sp[XA_TLE_MNMOTN], MEAN_MOTION);
        assert_eq!(xa_sp[XA_TLE_EPHTYPE], SP_EPHEMERIS_TYPE);
        assert_abs_diff_eq!(xa_sp[XA_TLE_BTERM], SP_BALLISTIC_COEFFICIENT);
        assert_abs_diff_eq!(xa_sp[XA_TLE_AGOMGP], SP_SRP_COEFFICIENT);
        assert_eq!(xs_sp.trim(), SP_XS_TLE);
    }

    #[test]
    fn test_load_lines() {
        let _lock = TEST_LOCK.lock().unwrap();

        let sgp_key = load_lines(SGP_LINE_1, SGP_LINE_2);
        let sgp4_key = load_lines(SGP4_LINE_1, SGP4_LINE_2);
        let xp_key = load_lines(XP_LINE_1, XP_LINE_2);
        let sp_key = load_lines(SP_LINE_1, SP_LINE_2);
        let count = get_count();
        assert!(sgp_key > 0);
        assert!(sgp4_key > 0);
        assert!(xp_key > 0);
        assert!(sp_key > 0);
        assert_eq!(count, 4);
        let _ = clear();
    }

    #[test]
    fn test_get_dll_info() {
        let info = get_dll_info();
        assert!(info.contains(DLL_VERSION));
    }

    #[test]
    fn test_tle_file() {
        let _lock = TEST_LOCK.lock().unwrap();
        let result = load_file("tests/data/2025-12-30-celestrak.tle");
        let count = get_count();
        let _ = clear();
        assert!(result.is_ok());
        assert_eq!(count, 14001);
    }

    #[test]
    fn test_load_3le_file() {
        let _lock = TEST_LOCK.lock().unwrap();
        let result = load_file("tests/data/2025-12-30-celestrak.3le");
        let count = get_count();
        let _ = clear();
        assert!(result.is_ok());
        assert_eq!(count, 14001);
    }
}
