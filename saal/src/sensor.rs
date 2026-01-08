// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::{GetSetString, IDX_ORDER_QUICK, IDX_ORDER_READ, astro, get_last_error_message};
use std::os::raw::c_char;

unsafe extern "C" {
    //  Notes: This function has been deprecated since v9.0.
    //  Initializes Sensor DLL for use in the program
    //  If this function returns an error, it is recommended that the users stop the program immediately.
    //  The error occurs if the users forget to load and initialize all the prerequisite dlls, as listed
    //  in the DLL Prerequisite section, before using this DLL.
    pub fn SensorInit(apAddr: i64) -> i32;
    //  Returns information about the current version of Sensor DLL.
    //  The information is placed in the string parameter passed in.
    //  The returned string provides information about the version number, build date, and the platform of the Sensor DLL.
    pub fn SensorGetInfo(infoStr: *const c_char);
    //  Loads sensor data, contained in a text file, into the set of loaded sensors
    //  The users can use this function repeatedly to load sensor data from different input files.
    //  However, only unique senKeys are stored in the binary tree.  New sensor data will overwrite the existing data.
    //
    //  Sensor data can be included directly in the main input file or they can be read from a separate file
    //  identified with "SENFIL=[pathname\filename]".
    //
    //  This function only reads sensor data from the main input file or the file identified with SENFIL in the input file.
    //  It won't read anything else.
    pub fn SensorLoadFile(senFile: *const c_char) -> i32;
    //  Loads a single sensor-typed card
    pub fn SensorLoadCard(card: *const c_char) -> i32;
    //  Saves the currently loaded sensor data to a file
    //  If the users call this routine immediately after the SensorLoadFile().
    //  The sensor data contents in the two file should be the same (minus duplicated sensors or bad sensors).
    //
    //  The purpose of this function is to save the current state of the loaded sensors, usually used in GUI applications, for future use.
    pub fn SensorSaveFile(sensorFile: *const c_char, saveMode: i32, saveForm: i32) -> i32;
    //  Removes a sensor, represented by the senKey, from the set of currently loaded sensors
    //  If the users enter an invalid senKey - a non-existing senKey, the function will return a non-zero value indicating an error.
    pub fn SensorRemove(senKey: i64) -> i32;
    //  Removes all currently loaded sensors from memory
    //  The function returns zero if all the loaded sensors are removed successfully from the SENSOR DLL's binary tree. Other values indicate an error.
    pub fn SensorRemoveAll() -> i32;
    //  Returns the number of sensors currently loaded
    //  See SensorGetLoaded for example.
    //  This function is useful for dynamically allocating memory for the array that is passed to the function SensorGetLoaded().
    pub fn SensorGetCount() -> i32;
    //  Retrieves all of the currently loaded senKeys.
    //  These senKeys can be used to access the internal data for the sensors.
    //  It is recommended that SensorGetCount() is used to count how many sensors are currently loaded in the SENSOR DLL's binary tree.
    //  The user can then use this number to dynamically allocate the senKeys array and pass it to this function.
    //
    //  If the users prefer to pass a static array to the function, make sure it is big enough to store all the senKeys in memory.
    pub fn SensorGetLoaded(order: i32, senKeys: *mut i64);
    //  Retrieves sensor location data for a sensor
    pub fn SensorGetLocAll(
        senKey: i64,
        astroLat: *mut f64,
        astroLon: *mut f64,
        senPos: *mut [f64; 3],
        senDesc: *const c_char,
        orbSatNum: *mut i32,
        secClass: *const c_char,
    ) -> i32;
    //  Adds/updates sensor location data (non DMA) for a sensor using individually provided field values
    //  This function is shared between ground sensors and orbiting sensors. The users need to provide the proper data fields for each sensor type.
    //
    //  If the function is invoked with a senKey that already exists in the SENSOR DLL's binary tree, the senKey's associated data will be updated with the provided data.
    //
    //  If it is an orbiting sensor, the users need to provide the associated satellite number.
    pub fn SensorSetLocAll(
        senKey: i64,
        astroLat: f64,
        astroLon: f64,
        senPos: *const [f64; 3],
        senDesc: *const c_char,
        orbSatNum: i32,
        secClass: c_char,
    ) -> i32;
    //  Retrieves the value of an individual field of sensor location data
    //  <br>
    //  Make sure to use the appropriate field index for ground sensors and orbiting sensors.
    //  <br>
    //  The table below shows the values for the xf_SenLoc parameter:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Index</b></td>
    //  <td><b>Index Interpretation</b></td>
    //  </tr>
    //  <tr><td>1</td><td>Sensor number</td></tr>
    //  <tr><td>2</td><td>Sensor astronomical latitude (deg)</td></tr>
    //  <tr><td>3</td><td>Sensor astronomical longitude (deg)</td></tr>
    //  <tr><td>4</td><td>Sensor position X (km)</td></tr>
    //  <tr><td>5</td><td>Sensor position Y (km)</td></tr>
    //  <tr><td>6</td><td>Sensor position Z (km)</td></tr>
    //  <tr><td>7</td><td>Sensor description</td></tr>
    //  <tr><td>8</td><td>Orbiting sensor's number (satNum)   </td></tr>
    //  <tr><td>9</td><td>Sensor classification </td></tr>
    //  </table>
    pub fn SensorGetLocField(senKey: i64, xf_SenLoc: i32, strValue: *const c_char) -> i32;
    //  Updates the value of an individual field of sensor location data
    //  See SensorGetLocField for a description of the xf_SenLoc values.
    //  The set value type was intentionally chosen as a character string because it allows the users to set value for different data types.
    //
    //  Make sure to use the appropriate field index for ground sensors and orbiting sensors.
    pub fn SensorSetLocField(senKey: i64, xf_SenLoc: i32, strValue: *const c_char) -> i32;
    //  Retrieves sensor limits data
    //  This function is shared between ground sensors and orbiting sensors. Make sure to interpret the data correctly.
    pub fn SensorGet1L(
        senKey: i64,
        viewType: *const c_char,
        obsType: *const c_char,
        rngUnits: *mut i32,
        maxRngLim: *mut f64,
        boresight1: *const c_char,
        elLim1: *mut f64,
        elLim2: *mut f64,
        azLim1: *mut f64,
        azLim2: *mut f64,
        interval: *mut f64,
        visFlg: *mut i32,
        rngLimFlg: *mut i32,
        maxPPP: *mut i32,
        minRngLim: *mut f64,
        plntryRes: *mut i32,
        rrLim: *mut f64,
    ) -> i32;
    //  Adds/updates sensor limits data (non DMA) via individually provided field values
    //  This function is shared between ground sensors and orbiting sensors. The users need to provide the proper data fields for each sensor type.
    //
    //  If the function is invoked with a senKey that already exists in the SENSOR DLL's binary tree, the senKey's associated data will be updated with the provided data.
    //
    //  If it is an orbiting sensor, the users need to provide the associated satellite number.
    pub fn SensorSet1L(
        senKey: i64,
        viewType: c_char,
        obsType: c_char,
        rngUnits: i32,
        maxRngLim: f64,
        boresight1: c_char,
        elLim1: f64,
        elLim2: f64,
        azLim1: f64,
        azLim2: f64,
        interval: f64,
        visFlg: i32,
        rngLimFlg: i32,
        maxPPP: i32,
        minRngLim: f64,
        plntryRes: i32,
        rrLim: f64,
    ) -> i32;
    //  Retrieves additional sensor limits data
    //  This function is shared between ground sensors and orbiting sensors. Make sure to interpret the data correctly.
    pub fn SensorGet2L(
        senKey: i64,
        boresight2: *const c_char,
        elLim3: *mut f64,
        elLim4: *mut f64,
        azLim3: *mut f64,
        azLim4: *mut f64,
        earthBckgrnd: *mut i32,
        earthLimb: *mut f64,
        solarXAngle: *mut f64,
        lunarXAngle: *mut f64,
        minIllum: *mut f64,
        twilit: *mut f64,
    ) -> i32;
    //  Adds/updates additional sensor limits data
    //  This function is shared between ground sensors and orbiting sensors. The users need to provide the proper data fields for each sensor type.
    //
    //  If the function is invoked with a senKey that already exists in the SENSOR DLL's binary tree, the senKey's associated data will be updated with the provided data.
    pub fn SensorSet2L(
        senKey: i64,
        boresight2: c_char,
        elLim3: f64,
        elLim4: f64,
        azLim3: f64,
        azLim4: f64,
        earthBckgrnd: i32,
        earthLimb: f64,
        solarXAngle: f64,
        lunarXAngle: f64,
        minIllum: f64,
        twilit: f64,
    ) -> i32;
    //  Retrieves an individual field of sensor limits data
    //  <br>
    //  Make sure to use the appropriate field index for ground sensors and orbiting sensors.
    //  <br>
    //  The table below shows the values for the xf_SenLim parameter:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Index</b></td>
    //  <td><b>Index Interpretation</b></td>
    //  </tr>
    //  <tr><td>11</td><td> Sensor view type</td></tr>
    //  <tr><td>12</td><td> Sensor observation type</td></tr>
    //  <tr><td>13</td><td> Unit on range/range rate</td></tr>
    //  <tr><td>14</td><td> Max observable range (km)</td></tr>
    //  <tr><td>15</td><td> Min observable range (km)</td></tr>
    //  <tr><td>16</td><td> Output interval (min)</td></tr>
    //  <tr><td>17</td><td> Visual pass control flag</td></tr>
    //  <tr><td>18</td><td> Range limit control flag </td></tr>
    //  <tr><td>19</td><td> Max number of points per pass</td></tr>
    //  <tr><td>20</td><td> Range rate/relative velocity limit (km/sec)</td></tr>
    //  <tr><td>31</td><td> Elevation limits (deg) 1, 2, 3, 4 or Sensor off-boresight angle (deg)</td></tr>
    //  <tr><td>32</td><td> </td></tr>
    //  <tr><td>33</td><td> </td></tr>
    //  <tr><td>34</td><td> </td></tr>
    //  <tr><td>35</td><td> Azimuth limits (deg) 1, 2,3, 4 or Sensor clock angle (deg)</td></tr>
    //  <tr><td>36</td><td> </td></tr>
    //  <tr><td>37</td><td> </td></tr>
    //  <tr><td>38</td><td> </td></tr>
    //  <tr><td>52</td><td> Orbiting sensor planetary restriction</td></tr>
    //  <tr><td>53</td><td> Orbiting sensor boresight vector 1</td></tr>
    //  <tr><td>54</td><td> Orbiting sensor boresight vector 2</td></tr>
    //  <tr><td>55</td><td> Allow orbiting sensor to view sat against earth background</td></tr>
    //  <tr><td>56</td><td> Orbiting sensor earth limb exclusion distance (km)</td></tr>
    //  <tr><td>57</td><td> Orbiting sensor solar exclusion angle (deg)   </td></tr>
    //  <tr><td>58</td><td> Orbiting sensor lunar exclusion angle (deg)</td></tr>
    //  <tr><td>59</td><td> Orbiting sensor min illumination angle (deg)</td></tr>
    //  <tr><td>60</td><td> Ground site twilight offset angle (deg)</td></tr>
    //  </table>
    pub fn SensorGetLimField(senKey: i64, xf_SenLim: i32, strValue: *const c_char) -> i32;
    //  Updates an individual field of sensor limits data
    //  See SensorGetLimField for a description of the xf_SenLim parameter.
    //  The set value type was intentionally chosen as a character string because it allows the users to set value for different data types.
    //
    //  Make sure to use the appropriate field index for ground sensors and orbiting sensors.
    pub fn SensorSetLimField(senKey: i64, xf_SenLim: i32, strValue: *const c_char) -> i32;
    //  Retrieves sensor sigma/bias data
    pub fn SensorGetBS(senKey: i64, xaf_senbs: *mut [f64; 16]) -> i32;
    //  Adds/updates sensor sigma/bias data (non DMA)
    pub fn SensorSetBS(senKey: i64, xaf_senbs: *const [f64; 16]) -> i32;
    //  Retrieves an individual field of sensor sigma/bias data
    pub fn SensorGetBSField(senKey: i64, xaf_senbs: i32, strValue: *const c_char) -> i32;
    //  Updates an individual field of sensor sigma/bias data
    pub fn SensorSetBSField(senKey: i64, xaf_senbs: i32, strValue: *const c_char) -> i32;
    //  Retrieves the sensor data in form of S-Card, L1-Card, and L2-Card of the sensor
    pub fn SensorGetLines(senKey: i64, sCard: *const c_char, l1Card: *const c_char, l2Card: *const c_char) -> i32;
    //  Retrieves the sensor Bias/Sigma card
    pub fn SensorGetBSLine(senKey: i64, bsCard: *const c_char) -> i32;
    //  Gets sensor's orbiting satellite's satKey
    pub fn SensorGetOrbSatKey(senKey: i64, orbSatKey: *mut i64) -> i32;
    //  Sets sensor's orbiting satellite's satKey
    pub fn SensorSetOrbSatKey(senKey: i64, orbSatKey: i64) -> i32;
    //  Loads Space Fence's detailed azimuth-elevation definition table
    pub fn SensorLoadAzElTable(senKey: i64, azElTableFile: *const c_char) -> i32;
    //  Adds a new sensor segment whose limits defined by the input parameters - a cone or a dome portion
    pub fn SensorAddSegment(senKey: i64, segType: i32, xa_seg: *const [f64; 16]) -> i32;
    //  Retrieves sensor segment data of the specified segment (segNum)
    pub fn SensorGetSegment(senKey: i64, segNum: i32, segType: *mut i32, xa_seg: *mut [f64; 16]) -> i32;
    //  Sets sensor key mode
    //  This mode can also be turned on if the user loads an input text file that includes this line - "AS_DMA_SEN_ON" -
    //  and is currently calling any of these methods: DllMainLoadFile(), or SenLoadFile()
    pub fn SetSenKeyMode(sen_keyMode: i32) -> i32;
    //  Gets current sensor key mode
    pub fn GetSenKeyMode() -> i32;
    //  Returs the sensor number associated with the input senKey
    pub fn SenNumOf(senKey: i64) -> i32;
    //  Returns the first senKey from the currently loaded set of sensors that contains the specified sensor number.
    //  A negative value will be returned if there is an error.
    pub fn SensorGetSenKey(senNum: i32) -> i64;
    //  This function is similar to SensorGetSenKey but designed to be used in Matlab.
    //  Matlab doesn't correctly return the 19-digit satellite key using SensorGetSenKey. This method is an alternative way to return the senKey output.
    //  A negative value will be returned in senKey if there is an error.
    pub fn SensorGetSenKeyML(senNum: i32, senKey: *mut i64);
    //  Adds a sensor (location, limits, sigmas/biases), using its data stored in the input arrays.
    pub fn SensorAddFrArray(xa_sen: *const [f64; 128], xs_sen: *const c_char) -> i64;
    //  Retrieves sensor data (location, limits, sigmas/biases) and stores it in the passing arrays.
    pub fn SensorDataToArray(senKey: i64, xa_sen: *mut [f64; 128], xs_sen: *const c_char) -> i32;
}

pub fn get_dll_info() -> String {
    let mut c_info = GetSetString::new();
    unsafe { SensorGetInfo(c_info.pointer()) };
    c_info.value()
}

// Sensor segment types
// bounded-cone-typed limit: Boresight Az/El, Min/Max halfcone angle/Range, minimum cut-off elevation
pub static SEG_BCONE: i32 = 1;
// dome-typed limit: Min/Max Az/El/Range
pub static SEG_DOME: i32 = 2;

// Indexes of dome segment parameters
// start azimuth (deg)
pub static XA_SEG_DOME_AZFR: i32 = 0;
// end azimuth (deg)
pub static XA_SEG_DOME_AZTO: i32 = 1;
// lower-bound elevation (deg)
pub static XA_SEG_DOME_ELFR: i32 = 2;
// higher-bound elevation (deg)
pub static XA_SEG_DOME_ELTO: i32 = 3;
// minimum range (km)
pub static XA_SEG_DOME_MINRNG: i32 = 4;
// maximum range (km)
pub static XA_SEG_DOME_MAXRNG: i32 = 5;

pub static XA_SEG_DOME_SIZE: i32 = 16;

// Indexes of bounded-cone segment parameters
// boresight azimuth (deg)
pub static XA_SEG_BCONE_BSAZ: i32 = 0;
// boresight elevation (deg)
pub static XA_SEG_BCONE_BSEL: i32 = 1;
// offboresight lower angle (deg)
pub static XA_SEG_BCONE_ANGFR: i32 = 2;
// offboresight higher angle (deg)
pub static XA_SEG_BCONE_ANGTO: i32 = 3;
// minimum range (km)
pub static XA_SEG_BCONE_MINRNG: i32 = 4;
// maximum range (km)
pub static XA_SEG_BCONE_MAXRNG: i32 = 5;
// minimum cut-off elevation (deg)
pub static XA_SEG_BCONE_MINEL: i32 = 6;

pub static XA_SEG_BCONE_SIZE: i32 = 16;

// Different key mode options for senKey
// Default - duplicate sensor can not be loaded in binary tree
pub static SEN_KEYMODE_NODUP: i32 = 0;
// Allow duplicate obs to be loaded and have direct memory access (DMA - no duplication check and no binary tree)
pub static SEN_KEYMODE_DMA: i32 = 1;

// sensor key possible errors
// Bad sensor key
pub static BADSENKEY: i32 = -1;
// Duplicate sensor key
pub static DUPSENKEY: i32 = 0;

// Different sensor types
// bounded-cone tracker
pub static VT_BCT: i32 = 2;
// conventional tracker
pub static VT_CON: i32 = 3;
// optical tracker
pub static VT_OPT: i32 = 4;
// constant azimuth fan sweeping vertical plane
pub static VT_FAN: i32 = 7;
// orbiting sensor (same as 'M' or 'O')
pub static VT_ORB: i32 = 9;
// space fence's field of regard FOR ('R' = 86)
pub static VT_FOR: i32 = 82;
// space fence's surveillance fan FOV ('V' = 82)
pub static VT_FOV: i32 = 86;

// Different sensor location types
// Sensor location is in ECR
pub const SENLOC_TYPE_ECR: i32 = 1;
// Sensor location is in EFG
pub const SENLOC_TYPE_EFG: i32 = 2;
// Sensor location is in LLH
pub const SENLOC_TYPE_LLH: i32 = 3;
// Sensor location is in ECI (TEME of Date) (associated with time of ECI position)
pub const SENLOC_TYPE_ECI: i32 = 4;

// Sensor Data -
//*******************************************************************************
// generic data for all sensor types
//*******************************************************************************
// sensor number
pub const XA_SEN_GEN_SENNUM: usize = 0;
// sensor minimum range (km) (=0 no minimum range check)
pub const XA_SEN_GEN_MINRNG: usize = 3;
// sensor maxinmum range (km) (=0 no maximum range check)
pub const XA_SEN_GEN_MAXRNG: usize = 4;
// range rate/relative velocity limit (km/sec) (=0 no range rate check)
pub const XA_SEN_GEN_RRLIM: usize = 5;
// min/max range check (=0 apply min/max range limits, =1 accept all passes regardless of range)
pub const XA_SEN_GEN_RNGLIMFLG: usize = 6;
// is special ground-based mobil sensor flag / column 9 in 1L card (=0 non mobile sensor, =1 mobile sensor) {0, 1}
pub const XA_SEN_GEN_SMSEN: usize = 7;

//*******************************************************************************

// sensor location - for all ground-based (or fixed point) sensor types (non-orbiting)
// location type (see SENLOC_TYPE_? for available types)
pub const XA_SEN_GRN_LOCTYPE: usize = 10;
// sensor location ECR/EFG/ECI X component (km) or LLH/Lat (deg)
pub const XA_SEN_GRN_POS1: usize = 11;
// sensor location ECR/EFG/ECI Y component (km) or LLH/Lon (+: East/-: West) (deg)
pub const XA_SEN_GRN_POS2: usize = 12;
// sensor location ECR/EFG/ECI Z component (km) or LLH/Height (km)
pub const XA_SEN_GRN_POS3: usize = 13;
// astronomical latitude (deg) (+: North, -: South) [-90, 90]
pub const XA_SEN_GRN_ASTROLAT: usize = 14;
// astronomical longitude (deg) (+: West, -: East) [-360, 360]
pub const XA_SEN_GRN_ASTROLON: usize = 15;
// time of ECI position (ds50UTC) - only when location type = SENLOC_TYPE_ECI
pub const XA_SEN_GRN_ECITIME: usize = 16;

//*******************************************************************************
// sensor limits - for LAMOD only (not needed by Rotas/BatchDC)
//*******************************************************************************
// Bounded cone tracker (conical sensor) (VT_BCT)
// conical sensor boresight azimuth angle (deg) [-360, 360]
pub static XA_SEN_BCT_BRSGHTAZ: i32 = 20;
// conical sensor boresight elevation angle (deg) [-90, 90]
pub static XA_SEN_BCT_BRSGHTEL: i32 = 21;
// coninal sensor off-boresight angle/half cone angle (deg) [0, 90]
pub static XA_SEN_BCT_HALFANG: i32 = 22;
// conical sensor minimum elevation angle (deg) [0, 90]
pub static XA_SEN_BCT_MINEL: i32 = 23;

//*******************************************************************************
// Conventinoal tracker (VT_CON)
// is special mobil sensor flag / column 9 in 1L card (=0 non mobile sensor, =1 mobile sensor) {0, 1}
pub static XA_SEN_CON_SMSEN: i32 = 7;
// low elevation limit #1 (deg) [
pub static XA_SEN_CON_ELFR1: i32 = 20;
// high elevation limit #1 (deg)
pub static XA_SEN_CON_ELTO1: i32 = 21;
// low azimuth limit #1 (deg)
pub static XA_SEN_CON_AZFR1: i32 = 22;
// high azimuth limit #1 (deg)
pub static XA_SEN_CON_AZTO1: i32 = 23;
// low elevation limit #2 (deg)
pub static XA_SEN_CON_ELFR2: i32 = 24;
// high elevation limit #2 (deg)
pub static XA_SEN_CON_ELTO2: i32 = 25;
// low azimuth limit #2 (deg)
pub static XA_SEN_CON_AZFR2: i32 = 26;
// high azimuth limit #2 (deg)
pub static XA_SEN_CON_AZTO2: i32 = 27;

//*******************************************************************************
// Optical tracker (VT_OPT)
// low elevation limit #1 (deg)
pub static XA_SEN_OPT_ELFR1: i32 = 20;
// high elevation limit #1 (deg)
pub static XA_SEN_OPT_ELTO1: i32 = 21;
// low azimuth limit #1 (deg)
pub static XA_SEN_OPT_AZFR1: i32 = 22;
// high azimuth limit #1 (deg)
pub static XA_SEN_OPT_AZTO1: i32 = 23;
// low elevation limit #2 (deg)
pub static XA_SEN_OPT_ELFR2: i32 = 24;
// high elevation limit #2 (deg)
pub static XA_SEN_OPT_ELTO2: i32 = 25;
// low azimuth limit #2 (deg)
pub static XA_SEN_OPT_AZFR2: i32 = 26;
// high azimuth limit #2 (deg)
pub static XA_SEN_OPT_AZTO2: i32 = 27;

// optical sensor solar exclusion angle (to check for solar aspect angle limit)
pub static XA_SEN_OPT_SEA: i32 = 40;
// ground site twilight offset angle (deg) [0, 20)
pub static XA_SEN_OPT_TWILGHT: i32 = 41;
// visual pass check (sensor in dark, satellite illuminated)
pub static XA_SEN_OPT_VISCHK: i32 = 42;

//*******************************************************************************
// Constant azimuth fan (VT_FAN)
// fan's constant azimuth (deg) - fan sweeps a vertical plane  [0, 180)
pub static XA_SEN_FAN_AZ: i32 = 20;
// fan's tilt angle (deg)  (-: North/West tilt, +: South/East tilt) [-90, 90]
pub static XA_SEN_FAN_TILTANGLE: i32 = 21;
// fan's off-boresight angle (deg) [0, 90]
pub static XA_SEN_FAN_OBSANGLE: i32 = 23;

//*******************************************************************************
// Orbiting sensor (VT_ORB)
// orbiting sensor's satellite number
pub static XA_SEN_ORB_SATNUM: i32 = 10;

// orbiting sensor's off-boresight angle - low elevation limit #1 (deg) [0, 180]
pub static XA_SEN_ORB_ELMIN1: i32 = 20;
// orbiting sensor's off-boresight angle - high elevation limit #1 (deg) [0, 180]
pub static XA_SEN_ORB_ELMAX1: i32 = 21;
// orbiting sensor's off-boresight clock angle - low azimuth limit #1 (deg) [-360, 360]
pub static XA_SEN_ORB_AZMIN1: i32 = 22;
// orbiting sensor's off-boresight clock angle - high azimuth limit #1 (deg) [-360, 360]
pub static XA_SEN_ORB_AZMAX1: i32 = 23;
// orbiting sensor's off-boresight angle - low elevation limit #2 (deg) [0, 180]
pub static XA_SEN_ORB_ELMIN2: i32 = 24;
// orbiting sensor's off-boresight angle - high elevation limit #2 (deg) [0, 180]
pub static XA_SEN_ORB_ELMAX2: i32 = 25;
// orbiting sensor's off-boresight clock angle - low azimuth limit #2 (deg) [-360, 360]
pub static XA_SEN_ORB_AZMIN2: i32 = 26;
// orbiting sensor's off-boresight clock angle - high azimuth limit #2 (deg) [-360, 360]
pub static XA_SEN_ORB_AZMAX2: i32 = 27;

// orbiting sensor earth limb exclusion distance (km)
pub static XA_SEN_ORB_EARTHLIMB: i32 = 40;
// orbiting sensor solar exclusion angle (deg)
pub static XA_SEN_ORB_SEA: i32 = 41;
// orbiting sensor lunar exclusion angle (deg)
pub static XA_SEN_ORB_LEA: i32 = 42;
// orbiting sensor minimum illumination angle (deg) [0, 180)
pub static XA_SEN_ORB_MINILLUM: i32 = 43;
// orbiting sensor allow earth in the back ground {0, 1}
pub static XA_SEN_ORB_EARTHBRND: i32 = 44;
// orbiting sensor planetary restriction {0, 1}
pub static XA_SEN_ORB_PLNTRYRES: i32 = 45;

//*******************************************************************************

// Space fence Field Of Regard (VT_FOR)
// Sensor limits are defined in the az/el table file (using XS_SEN_FOR_AZFILE_255_256)

//*******************************************************************************

// Space fence Field Of View (VT_FOV)
// north/south beam width (deg)
pub static XA_SEN_FOV_BEAMWIDTH: i32 = 20;
// fence tilt angle (deg)
pub static XA_SEN_FOV_TILTANGLE: i32 = 21;

//*******************************************************************************

// Output control parameters (shouldn't be part of sensor data - just for backward compatibility)
// unit on range/range rate (0= km, km/sec, 1=nm, nm/sec) {0, 1}
pub static XA_SEN_GEN_UNIT: i32 = 90;
// output interval (min)
pub static XA_SEN_GEN_INTERVAL: i32 = 91;
// max number of points per pass
pub static XA_SEN_GEN_PTSPERPAS: i32 = 92;

// sensor sigmas/biases - ROTAS/BATCHDC
// azimuth sigma (deg)
pub const XA_SEN_GEN_AZSIGMA: usize = 110;
// elevation sigma (deg)
pub const XA_SEN_GEN_ELSIGMA: usize = 111;
// range sigma (km)
pub const XA_SEN_GEN_RGSIGMA: usize = 112;
// range-rate sigma (km/sec)
pub const XA_SEN_GEN_RRSIGMA: usize = 113;
// az rate sigma (deg/sec)
pub const XA_SEN_GEN_ARSIGMA: usize = 114;
// el rate sigma (deg/sec)
pub const XA_SEN_GEN_ERSIGMA: usize = 115;
// azimuth bias (deg)
pub const XA_SEN_GEN_AZBIAS: usize = 116;
// elevation bias (deg)
pub const XA_SEN_GEN_ELBIAS: usize = 117;
// range bias (km)
pub const XA_SEN_GEN_RGBIAS: usize = 118;
// range-rate bias (km/sec)
pub const XA_SEN_GEN_RRBIAS: usize = 119;
// time bias (sec)
pub const XA_SEN_GEN_TIMEBIAS: usize = 120;

pub const XA_SEN_SIZE: usize = 128;

// Indexes of SENSOR text data in an array of chars
// The last two numbers in the field name are the zero-based starting character position and the number of characters in the text field
// sensor security classification (1-character length)
pub const XS_SEN_SECCLASS_0_1: usize = 0;
// sensor viewing type (1-character length)
pub const XS_SEN_VIEWTYPE_1_1: usize = 1;
// sensor observation type (1-character length)
pub const XS_SEN_OBSTYPE_2_1: usize = 2;
// sensor description/narrative/notes (24-character length)
pub const XS_SEN_DSCRPTN_3_24: usize = 3;
// orbiting sensor's boresight vector #1 (1-character length) {'N', 'D', 'U', 'F', 'V', 'A', 'B', 'W', 'L'. 'R'}
pub const XS_SEN_ORB_BSVEC1_27_1: usize = 27;
// orbiting sensor's boresight vector #2 (1-character length) {'N', 'D', 'U', 'F', 'V', 'A', 'B', 'W', 'L'. 'R'}
pub const XS_SEN_ORB_BSVEC2_28_1: usize = 28;
// for VT_FOR only, az/el table file path (256-character length)
pub const XS_SEN_FOR_AZFILE_255_256: usize = 255;

pub const XS_SEN_LENGTH: usize = 512;

// Indexes of Sensor data fields
// Sensor number
pub static XF_SENLOC_NUM: i32 = 1;
// Sensor location - astronomical longitude (deg) (+=West/-=East)
pub static XF_SENLOC_LAT: i32 = 2;
// Sensor location - astronomical latitude (deg)  (+=North/-=South)
pub static XF_SENLOC_LON: i32 = 3;
// Sensor ECR position X (km)
pub static XF_SENLOC_POSX: i32 = 4;
// Sensor ECR position Y (km)
pub static XF_SENLOC_POSY: i32 = 5;
// Sensor ECR position Z (km)
pub static XF_SENLOC_POSZ: i32 = 6;
// Sensor description
pub static XF_SENLOC_DESC: i32 = 7;
// Orbiting sensor's number (satNum)
pub static XF_SENLOC_ORBSATNUM: i32 = 8;
// Sensor classification
pub static XF_SENLOC_SECCLASS: i32 = 9;

// Sensor view type
pub static XF_SENLIM_VIEWTYPE: i32 = 11;
// Sensor observation type
pub static XF_SENLIM_OBSTYPE: i32 = 12;
// Unit on range/range rate
pub static XF_SENLIM_UNIT: i32 = 13;
// Max observable range (km)
pub static XF_SENLIM_MAXRNG: i32 = 14;
// Min observable range (km)
pub static XF_SENLIM_MINRNG: i32 = 15;
// Output interval (min)
pub static XF_SENLIM_INTERVAL: i32 = 16;
// Visual pass control flag
pub static XF_SENLIM_OPTVISFLG: i32 = 17;
// Range limit control flag
pub static XF_SENLIM_RNGLIMFLG: i32 = 18;
// Max number of points per pass
pub static XF_SENLIM_PTSPERPAS: i32 = 19;
// Range rate/relative velocity limit (km/sec)
pub static XF_SENLIM_RRLIM: i32 = 20;

// Elevation limits #1 (low, deg) or orbiting sensor off-boresight angle (low, deg) or conical sensor boresight elvation (deg)
pub static XF_SENLIM_ELLIM1: i32 = 31;
// Elevation limits #2 (high, deg) or orbiting sensor off-boresight angle (high, deg) or conical sensor boresight minimum angle (deg)
pub static XF_SENLIM_ELLIM2: i32 = 32;
// Elevation limits #3 (low, deg) or orbiting sensor off-boresight angle (low, deg) or
pub static XF_SENLIM_ELLIM3: i32 = 33;
// Elevation limits #4 (high, deg) or orbiting sensor off-boresight angle (high, deg)
pub static XF_SENLIM_ELLIM4: i32 = 34;
// Azimuth limits #1 (low, deg) or orbiting sensor clock angle (from, deg) or conical sensor boresight azimuth (deg)
pub static XF_SENLIM_AZLIM1: i32 = 35;
// Azimuth limits #2 (high, deg) or orbiting sensor clock angle (to, deg) or conical sensor off-boresight azimuth angle (deg)
pub static XF_SENLIM_AZLIM2: i32 = 36;
// Azimuth limits #3 (low, deg) or orbiting sensor clock angle (from, deg)
pub static XF_SENLIM_AZLIM3: i32 = 37;
// Azimuth limits #4 (high, deg) or orbiting sensor clock angle (to, deg)
pub static XF_SENLIM_AZLIM4: i32 = 38;

// Orbiting sensor planetary restriction
pub static XF_SENLIM_PLNTRYRES: i32 = 52;
// Orbiting sensor boresight vector 1
pub static XF_SENLIM_BOREVEC1: i32 = 53;
// Orbiting sensor boresight vector 2
pub static XF_SENLIM_BOREVEC2: i32 = 54;
// Allow orbiting sensor to view sat against earth background
pub static XF_SENLIM_KEARTH: i32 = 55;
// Orbiting sensor earth limb exclusion distance (km)
pub static XF_SENLIM_ELIMB: i32 = 56;
// Orbiting sensor solar exclusion angle (deg)
pub static XF_SENLIM_SOLEXCANG: i32 = 57;
// Orbiting sensor lunar exclusion angle (deg)
pub static XF_SENLIM_LUNEXCANG: i32 = 58;

// Orbiting sensor min illumination angle (deg)
pub static XF_SENLIM_MINIL: i32 = 59;
// Ground site twilight offset angle (deg)
pub static XF_SENLIM_TWILIT: i32 = 60;
// Is special mobil sensor flag / column 9 in 1L card
pub static XF_SENLIM_SMSEN: i32 = 61;
// Number of additional segments added to sensor limits
pub static XF_SENLIM_NUMSEGS: i32 = 62;
// Space fence FOR's Az/El table file name
pub static XF_SENLIM_FILE: i32 = 63;
// Number of rows in space fence FOR's Az/El table
pub static XF_SENLIM_AZELROWS: i32 = 64;

//*******************************************************************************

// Indexes of sensor location data
// Sensor nubmer
pub static XA_SENLOC_NUM: i32 = 0;
// Sensor location - astronomical latitude (deg)  (+=North/-=South)
pub static XA_SENLOC_LAT: i32 = 1;
// Sensor location - astronomical longitude (deg) (+=West/-=East)
pub static XA_SENLOC_LON: i32 = 2;
// Sensor ECR position X (km)
pub static XA_SENLOC_POSX: i32 = 3;
// Sensor ECR position Y (km)
pub static XA_SENLOC_POSY: i32 = 4;
// Sensor ECR position Z (km)
pub static XA_SENLOC_POSZ: i32 = 5;

pub static XA_SENLOC_SIZE: i32 = 16;

//*******************************************************************************

// Indexes of Sensor's sigmas & biases data fields
// azimuth sigma (deg)
pub static XAF_SENBS_AZSIGMA: i32 = 0;
// elevation sigma (deg)
pub static XAF_SENBS_ELSIGMA: i32 = 1;
// range sigma (km)
pub static XAF_SENBS_RGSIGMA: i32 = 2;
// range-rate sigma (km/sec)
pub static XAF_SENBS_RRSIGMA: i32 = 3;
// az rate sigma (deg/sec)
pub static XAF_SENBS_ARSIGMA: i32 = 4;
// el rate sigma (deg/sec)
pub static XAF_SENBS_ERSIGMA: i32 = 5;

// azimuth bias (deg)
pub static XAF_SENBS_AZBIAS: i32 = 8;
// elevation bias (deg)
pub static XAF_SENBS_ELBIAS: i32 = 9;
// range bias (km)
pub static XAF_SENBS_RGBIAS: i32 = 10;
// range-rate bias (km/sec)
pub static XAF_SENBS_RRBIAS: i32 = 11;
// time bias (sec)
pub static XAF_SENBS_TIMEBIAS: i32 = 15;

pub static XAF_SENBS_SIZE: i32 = 16;

// Named constants for different obs types
// obs contains range rate only data
pub static OT_RRATE: i32 = 0;
// obs contains azimuth, elevation data
pub static OT_AZEL: i32 = 1;
// obs contains azimuth, elevation, and range data
pub static OT_AZEL_RNG: i32 = 2;
// obs contains azimuth, elevation, range, and range rate data
pub static OT_AZEL_RNGRRATE: i32 = 3;
// obs contains azimuth, elevation, range, range rate, azimuth rate, elevation rate, and range acceleration data
pub static OT_AZEL_ALL: i32 = 4;
// obs contains right ascention and declination data
pub static OT_RADEC: i32 = 5;
// obs contains range only data
pub static OT_RNGONLY: i32 = 6;
// obs contains azimuth, elevation, and sensor location data
pub static OT_AZEL_SENPOS: i32 = 8;
// obs contains right ascension, declination, and sensor location data
pub static OT_RADEC_SENPOS: i32 = 9;
// obs contains ECI position and velocity data
pub static OT_VEL: i32 = 10;
// obs contains ECI position data
pub static OT_POS: i32 = 11;
// internal use only - new obs type for RA/Dec or Star to be used in FOV/targets
pub static OT_STAR: i32 = 12;
// obs contains TDOA/FDOA data
pub static OT_RF: i32 = 13;
// obs contains Satellite Laser Ranging (SLR) - range only, with tropospheric refraction term data
pub static OT_SLR: i32 = 16;
// obs contains azimuth, elevation, sensor location, and range data
pub static OT_M: i32 = 18;
// obs contains right ascension, declination, sensor location, and range data
pub static OT_O: i32 = 19;
// obs contains range rate only data - for use in obs selection criteria
pub static OT_RRATE_SELOB: i32 = 999;

// ========================= End of auto generated code ==========================

pub struct ParsedSensor {
    pub key: i64,
    pub number: i32,
    pub minimum_range: Option<f64>,
    pub maximum_range: Option<f64>,
    pub range_rate_limit: Option<f64>,
    pub apply_range_limits: bool,
    pub mobile: bool,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub astronomical_latitude: f64,
    pub astronomical_longitude: f64,
    pub azimuth_noise: Option<f64>,
    pub elevation_noise: Option<f64>,
    pub range_noise: Option<f64>,
    pub range_rate_noise: Option<f64>,
    pub azimuth_rate_noise: Option<f64>,
    pub elevation_rate_noise: Option<f64>,
    pub description: Option<String>,
}

impl ParsedSensor {
    pub fn from_key(key: i64) -> Result<ParsedSensor, String> {
        let (xa_sen, xs_sen) = get_arrays(key)?;

        // get XS_SEN_DSCRPTN_3_24 + 24 characters
        let description = match xs_sen[XS_SEN_DSCRPTN_3_24..XS_SEN_DSCRPTN_3_24 + 24].trim() {
            "" => None,
            desc => Some(desc.to_string()),
        };

        let minimum_range = if xa_sen[XA_SEN_GEN_MINRNG] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_MINRNG])
        } else {
            None
        };

        let maximum_range = if xa_sen[XA_SEN_GEN_MAXRNG] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_MAXRNG])
        } else {
            None
        };

        let range_rate_limit = if xa_sen[XA_SEN_GEN_RRLIM] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_RRLIM])
        } else {
            None
        };
        let lla = get_lla(key)?;
        let mut latitude = None;
        let mut longitude = None;
        let mut altitude = None;
        if let Some(lla_values) = lla {
            latitude = Some(lla_values[0]);
            longitude = Some(lla_values[1]);
            altitude = Some(lla_values[2]);
        }

        let azimuth_noise = if xa_sen[XA_SEN_GEN_AZSIGMA] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_AZSIGMA])
        } else {
            None
        };

        let elevation_noise = if xa_sen[XA_SEN_GEN_ELSIGMA] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_ELSIGMA])
        } else {
            None
        };

        let range_noise = if xa_sen[XA_SEN_GEN_RGSIGMA] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_RGSIGMA])
        } else {
            None
        };

        let range_rate_noise = if xa_sen[XA_SEN_GEN_RRSIGMA] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_RRSIGMA])
        } else {
            None
        };

        let azimuth_rate_noise = if xa_sen[XA_SEN_GEN_ARSIGMA] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_ARSIGMA])
        } else {
            None
        };

        let elevation_rate_noise = if xa_sen[XA_SEN_GEN_ERSIGMA] != 0.0 {
            Some(xa_sen[XA_SEN_GEN_ERSIGMA])
        } else {
            None
        };

        Ok(ParsedSensor {
            key,
            number: xa_sen[XA_SEN_GEN_SENNUM] as i32,
            minimum_range,
            maximum_range,
            range_rate_limit,
            apply_range_limits: xa_sen[XA_SEN_GEN_RNGLIMFLG] == 0.0,
            mobile: xa_sen[XA_SEN_GEN_SMSEN] == 1.0,
            latitude,
            longitude,
            altitude,
            astronomical_latitude: xa_sen[XA_SEN_GRN_ASTROLAT],
            astronomical_longitude: xa_sen[XA_SEN_GRN_ASTROLON],
            azimuth_noise,
            elevation_noise,
            range_noise,
            range_rate_noise,
            azimuth_rate_noise,
            elevation_rate_noise,
            description,
        })
    }
}

pub fn parse_key(sen_key: i64) -> Result<ParsedSensor, String> {
    ParsedSensor::from_key(sen_key)
}

pub fn prune_missing_locations() -> Result<(), String> {
    let keys = get_keys(IDX_ORDER_QUICK);

    for key in keys {
        let lla = get_lla(key).unwrap_or(None);
        if lla.is_none() {
            remove(key)?;
        }
    }

    Ok(())
}

pub fn parse_all() -> Result<Vec<ParsedSensor>, String> {
    let keys = get_keys(IDX_ORDER_READ);
    let mut sensors = Vec::new();

    for key in keys {
        let sensor = ParsedSensor::from_key(key)?;
        sensors.push(sensor);
    }

    Ok(sensors)
}

pub fn get_astronomical_ll(sen_key: i64) -> Result<[f64; 2], String> {
    let mut xa_sen = [0f64; XA_SEN_SIZE];
    let mut xs_sen = GetSetString::new();

    let result = unsafe { SensorDataToArray(sen_key, &mut xa_sen, xs_sen.pointer()) };
    if result != 0 {
        return Err(get_last_error_message());
    }

    Ok([xa_sen[XA_SEN_GRN_ASTROLAT], xa_sen[XA_SEN_GRN_ASTROLON]])
}

pub fn get_lla(sen_key: i64) -> Result<Option<[f64; 3]>, String> {
    let mut xa_sen = [0f64; XA_SEN_SIZE];
    let mut xs_sen = GetSetString::new();

    let result = unsafe { SensorDataToArray(sen_key, &mut xa_sen, xs_sen.pointer()) };

    if result != 0 {
        return Err(get_last_error_message());
    }

    match xa_sen[XA_SEN_GRN_LOCTYPE] as i32 {
        SENLOC_TYPE_LLH => Ok(Some([
            xa_sen[XA_SEN_GRN_POS1],
            xa_sen[XA_SEN_GRN_POS2],
            xa_sen[XA_SEN_GRN_POS3],
        ])),
        SENLOC_TYPE_EFG | SENLOC_TYPE_ECR => {
            match astro::efg_to_lla(&[
                xa_sen[XA_SEN_GRN_POS1],
                xa_sen[XA_SEN_GRN_POS2],
                xa_sen[XA_SEN_GRN_POS3],
            ]) {
                Ok(lla) => Ok(Some(lla)),
                Err(_) => Ok(None),
            }
        }
        SENLOC_TYPE_ECI => Ok(None),
        _ => Err("Unknown sensor location type.".to_string()),
    }
}

pub fn get_keys(order: i32) -> Vec<i64> {
    let count = count_loaded();
    let mut keys = vec![0; count as usize];
    unsafe {
        SensorGetLoaded(order, keys.as_mut_ptr());
    }
    keys
}

pub fn load_card(card: &str) -> Result<(), String> {
    let mut input_card: GetSetString = card.into();
    let result = unsafe { SensorLoadCard(input_card.pointer()) };

    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

pub fn remove(sen_key: i64) -> Result<(), String> {
    let result = unsafe { SensorRemove(sen_key) };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

pub fn count_loaded() -> i32 {
    unsafe { SensorGetCount() }
}

pub fn load_file(file_path: &str) -> Result<(), String> {
    let mut input_file: GetSetString = file_path.into();
    let result = unsafe { SensorLoadFile(input_file.pointer()) };

    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

pub fn clear() -> Result<(), String> {
    let result = unsafe { SensorRemoveAll() };
    match result {
        0 => Ok(()),
        _ => Err(get_last_error_message()),
    }
}

pub fn get_arrays(sen_key: i64) -> Result<([f64; XA_SEN_SIZE], String), String> {
    let mut xa_sen = [0f64; XA_SEN_SIZE];
    let mut xs_sen = GetSetString::new();

    let result = unsafe { SensorDataToArray(sen_key, &mut xa_sen, xs_sen.pointer()) };

    match result {
        0 => Ok((xa_sen, xs_sen.value())),
        _ => Err(get_last_error_message()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_lock::TEST_LOCK;

    const SENSOR_CARD: &str = "211  3381724 -25333969 -1521161 -5083089  3530462  U SOCORRO CAM1              S";
    const NOISE_CARD: &str = "211 5   0.0003 0.0003 0.0000 0.0000  -0.0005 -0.0003  0.0000  0.0000  0.0000  BS";

    #[test]
    fn test_load_file() {
        let _lock = TEST_LOCK.lock().unwrap();
        load_file("tests/data/sensors.dat").unwrap();
        prune_missing_locations().unwrap();
        assert_eq!(count_loaded(), 98);
        let sensors = parse_all().unwrap();
        clear().unwrap();
        assert_eq!(count_loaded(), 0);
        assert_eq!(sensors[0].number, 211);
        assert_eq!(sensors[0].description.as_deref(), Some("SOCORRO CAM1"));
    }

    #[test]
    fn test_get_arrays() {
        let _lock = TEST_LOCK.lock().unwrap();
        load_card(SENSOR_CARD).unwrap();
        load_card(NOISE_CARD).unwrap();
        let keys = get_keys(IDX_ORDER_READ);
        let key = keys[keys.len() - 1];
        let (xa_sen, xs_sen) = get_arrays(key).unwrap();
        clear().unwrap();
        assert_eq!(xa_sen[XA_SEN_GEN_SENNUM], 211.0);
        assert_eq!(xa_sen[XA_SEN_GRN_POS1], -1521.161);
        assert_eq!(xa_sen[XA_SEN_GRN_POS2], -5083.089);
        assert_eq!(xa_sen[XA_SEN_GRN_POS3], 3530.462);
        assert_eq!(xa_sen[XA_SEN_GRN_ASTROLAT], 33.81724);
        assert_eq!(xa_sen[XA_SEN_GRN_ASTROLON], -253.33969);
        assert_eq!(xa_sen[XA_SEN_GRN_ECITIME], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_RNGLIMFLG], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_SMSEN], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_MINRNG], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_MAXRNG], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_RRLIM], 0.0);
        assert_eq!(xa_sen[XA_SEN_GRN_LOCTYPE], 1.0);
        assert_eq!(xa_sen[XA_SEN_GEN_AZSIGMA], 0.0003);
        assert_eq!(xa_sen[XA_SEN_GEN_ELSIGMA], 0.0003);
        assert_eq!(xa_sen[XA_SEN_GEN_ARSIGMA], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_ERSIGMA], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_RGSIGMA], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_RRSIGMA], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_AZBIAS], -0.0005);
        assert_eq!(xa_sen[XA_SEN_GEN_ELBIAS], -0.0003);
        assert_eq!(xa_sen[XA_SEN_GEN_RGBIAS], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_RRBIAS], 0.0);
        assert_eq!(xa_sen[XA_SEN_GEN_TIMEBIAS], 0.0);
        assert_eq!(xs_sen.trim(), "U33SOCORRO CAM1");
    }

    fn test_parse_key() {
        load_card(SENSOR_CARD).unwrap();
        load_card(NOISE_CARD).unwrap();
        let keys = get_keys(IDX_ORDER_READ);
        let key = keys[keys.len() - 1];
        let sensor = parse_key(key).unwrap();
        clear().unwrap();
        assert_eq!(sensor.number, 211);
        assert!(sensor.minimum_range.is_none());
        assert!(sensor.maximum_range.is_none());
        assert!(sensor.range_rate_limit.is_none());
        assert!(sensor.apply_range_limits);
        assert!(!sensor.mobile);
        assert_eq!(sensor.latitude.unwrap(), 33.817242266703744);
        assert_eq!(sensor.longitude.unwrap(), 253.33970533290127);
        assert_eq!(sensor.altitude.unwrap(), 1.509809294541032);
        assert_eq!(sensor.astronomical_latitude, 33.81724);
        assert_eq!(sensor.astronomical_longitude, -253.33969);
        assert_eq!(sensor.azimuth_noise.unwrap(), 0.0003);
        assert_eq!(sensor.elevation_noise.unwrap(), 0.0003);
        assert!(sensor.range_noise.is_none());
        assert!(sensor.range_rate_noise.is_none());
        assert!(sensor.azimuth_rate_noise.is_none());
        assert!(sensor.elevation_rate_noise.is_none());
        assert_eq!(sensor.description.as_deref(), Some("SOCORRO CAM1"));
    }
}
