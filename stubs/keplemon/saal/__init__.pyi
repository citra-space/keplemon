# flake8: noqa
from keplemon.enums import SAALKeyMode

class TLEInterface:
    # TLE types (TLE ephemeris types) - They are different than ELTTYPE
    # TLE SGP elset (Kozai mean motion)
    TLETYPE_SGP: int
    # TLE SGP4 elset (Brouwer mean motion)
    TLETYPE_SGP4: int
    # TLE SGP4-XP elset (Brouwer mean motion)
    TLETYPE_XP: int
    # TLE SP elset (osculating elements)
    TLETYPE_SP: int

    # Indexes of TLE data fields
    # Satellite number
    XF_TLE_SATNUM: int
    # Security classification U: unclass, C: confidential, S: Secret
    XF_TLE_CLASS: int
    # Satellite name A8
    XF_TLE_SATNAME: int
    # Satellite's epoch time "YYYYJJJ.jjjjjjjj"
    XF_TLE_EPOCH: int
    # GP B* drag term (1/er)  (not the same as XF_TLE_BTERM)
    XF_TLE_BSTAR: int
    # Satellite ephemeris type: 0=SGP, 2=SGP4, 4=SGP4-XP, 6=SP
    XF_TLE_EPHTYPE: int
    # Element set number
    XF_TLE_ELSETNUM: int
    # Orbit inclination (deg)
    XF_TLE_INCLI: int
    # Right ascension of asending node (deg)
    XF_TLE_NODE: int
    # Eccentricity
    XF_TLE_ECCEN: int
    # Argument of perigee (deg)
    XF_TLE_OMEGA: int
    # Mean anomaly (deg)
    XF_TLE_MNANOM: int
    # Mean motion (rev/day) (ephType=0: Kozai, ephType=2: Brouwer)
    XF_TLE_MNMOTN: int
    # Revolution number at epoch
    XF_TLE_REVNUM: int

    # GP Mean motion derivative (rev/day /2)
    XF_TLE_NDOT: int
    # GP Mean motion second derivative (rev/day**2 /6)
    XF_TLE_NDOTDOT: int
    # Solar radiation pressure GP (m2/kg)
    XF_TLE_AGOMGP: int

    # SP Radiation Pressure Coefficient
    XF_TLE_SP_AGOM: int
    # SP ballistic coefficient (m2/kg)
    XF_TLE_SP_BTERM: int
    # SP outgassing parameter (km/s2)
    XF_TLE_SP_OGPARM: int

    # Original satellite number
    XF_TLE_ORGSATNUM: int
    # GP ballistic coefficient (m2/kg) (not the same as XF_TLE_BSTAR)
    XF_TLE_BTERM: int
    # Time of last observation relative to epoch +/- fractional days
    XF_TLE_OBSTIME: int
    # Last calculated error growth rate (km/day)
    XF_TLE_EGR: int
    # Last calculated energy dissipation rate (w/kg)
    XF_TLE_EDR: int
    # Median Vismag
    XF_TLE_VISMAG: int
    # Median RCS - diameter in centimeters (cm)
    XF_TLE_RCS: int
    # Object Type (Payload, Rocket Body, Platform, Debris, Unknown)
    XF_TLE_OBJTYPE: int
    # Satellite name A12 (upto 12 character long)
    XF_TLE_SATNAME_12: int

    # Indexes of TLE numerical data in an array
    # Line 1
    # Satellite number
    XA_TLE_SATNUM: int
    # Satellite's epoch time in DS50UTC
    XA_TLE_EPOCH: int
    # GP Mean motion derivative (rev/day /2)
    XA_TLE_NDOT: int
    # GP Mean motion second derivative (rev/day**2 /6)
    XA_TLE_NDOTDOT: int
    # GP B* drag term (1/er)
    XA_TLE_BSTAR: int
    # Satellite ephemeris type: 0=SGP, 2=SGP4, 4=SGP4-XP, 6=SP
    XA_TLE_EPHTYPE: int

    # Line 2
    # Orbit inclination (deg)
    XA_TLE_INCLI: int
    # Right ascension of asending node (deg)
    XA_TLE_NODE: int
    # Eccentricity
    XA_TLE_ECCEN: int
    # Argument of perigee (deg)
    XA_TLE_OMEGA: int
    # Mean anomaly (deg)
    XA_TLE_MNANOM: int
    # Mean motion (rev/day) (ephType=0, 4: Kozai, ephType=2: Brouwer)
    XA_TLE_MNMOTN: int
    # Revolution number at epoch
    XA_TLE_REVNUM: int
    # Element set number
    XA_TLE_ELSETNUM: int

    # CSV (or TLE-XP, ephemType=4) specific fields
    # Original satellite number
    XA_TLE_ORGSATNUM: int
    # SP/SGP4-XP ballistic coefficient (m2/kg)
    XA_TLE_BTERM: int
    # Time of last observation relative to epoch +/- fractional days
    XA_TLE_OBSTIME: int
    # Last calculated error growth rate (km/day)
    XA_TLE_EGR: int
    # Last calculated energy dissipation rate (w/kg)
    XA_TLE_EDR: int
    # Median Vismag
    XA_TLE_VISMAG: int
    # Median RCS - diameter in centimeters (cm)
    XA_TLE_RCS: int

    # CSV (or TLE-XP, ephemType=4)
    # Solar Radiation Pressure Coefficient GP (m2/kg)
    XA_TLE_AGOMGP: int

    # SP specific fields
    # SP ballistic coefficient (m2/kg)
    XA_TLE_SP_BTERM: int
    # SP outgassing parameter (km/s2)
    XA_TLE_SP_OGPARM: int
    # SP Radiation Pressure Coefficient
    XA_TLE_SP_AGOM: int

    XA_TLE_SIZE: int

    # Indexes of TLE text data in an array of chars
    # Security classification of line 1 and line 2
    XS_TLE_SECCLASS_1: int
    # Satellite name
    XS_TLE_SATNAME_12: int
    # Object Type (Payload, Rocket Body, Platform, Debris, Unknown) - csv only
    XS_TLE_OBJTYPE_11: int

    XS_TLE_SIZE: int

    # TLE's text data fields - new convention (start index, string length)
    # Security classification of line 1 and line 2
    XS_TLE_SECCLASS_0_1: int
    # Satellite name
    XS_TLE_SATNAME_1_12: int
    # Object Type (Payload, Rocket Body, Platform, Debris, Unknown) - csv only
    XS_TLE_OBJTYPE_13_1: int

    XS_TLE_LENGTH: int

    # Indexes of different TLE file's formats
    # Original TLE format
    XF_TLEFORM_ORG: int
    # CSV format
    XF_TLEFORM_CSV: int
    @staticmethod
    def get_check_sums(line_1: str, line_2: str) -> tuple[int, int]: ...

class MainInterface:
    @staticmethod
    def get_key_mode() -> SAALKeyMode: ...
    @staticmethod
    def set_key_mode(mode: SAALKeyMode) -> None: ...

class SAALSensor:
    key: int
    """Unique key used to retrieve this sensor from the SensorInterface"""
    number: int
    """3-digit sensor number"""
    minimum_range: float | None
    """Minimum range used for access checks"""
    maximum_range: float | None
    """Maximum range used for access checks"""
    range_rate_limit: float | None
    """Range rate limit used for access checks"""
    apply_range_limits: bool
    """Whether to apply range limits during access checks"""
    mobile: bool
    """Whether the sensor is mobile"""
    latitude: float
    """Geodetic latitude of the sensor (deg)"""
    longitude: float
    """Geodetic longitude of the sensor (deg)"""
    altitude: float
    """Altitude of the sensor (km)"""
    astronomical_latitude: float
    """Astronomical latitude of the sensor (deg)"""
    astronomical_longitude: float
    """Astronomical longitude of the sensor (deg)"""
    azimuth_noise: float | None
    """Azimuth noise standard deviation (deg)"""
    elevation_noise: float | None
    """Elevation noise standard deviation (deg)"""
    range_noise: float | None
    """Range noise standard deviation (km)"""
    range_rate_noise: float | None
    """Range rate noise standard deviation (km/s)"""
    azimuth_rate_noise: float | None
    """Azimuth rate noise standard deviation (deg/s)"""
    elevation_rate_noise: float | None
    """Elevation rate noise standard deviation (deg/s)"""
    angular_noise: float | None
    """Combined angular noise standard deviation (deg)"""
    description: str | None
    """24-character sensor description/narrative/notes"""

class SensorInterface:

    XA_SEN_GEN_SENNUM: int
    """Index for sensor number in sensor data array"""

    XA_SEN_GEN_MINRNG: int
    """Index for minimum range in sensor data array"""

    XA_SEN_GEN_MAXRNG: int
    """Index for maximum range in sensor data array"""

    XA_SEN_GEN_RRLIM: int
    """Index for range rate limit in sensor data array"""

    XA_SEN_GEN_RNGLIMFLG: int
    """Index for range limit flag in sensor data array"""

    XA_SEN_GEN_SMSEN: int
    """Index for sensor mobility flag in sensor data array"""

    XA_SEN_GRN_LOCTYPE: int
    """Index for ground sensor location type in sensor data array"""

    XA_SEN_GRN_POS1: int
    """Index for ground sensor position component 1 in sensor data array"""

    XA_SEN_GRN_POS2: int
    """Index for ground sensor position component 2 in sensor data array"""

    XA_SEN_GRN_POS3: int
    """Index for ground sensor position component 3 in sensor data array"""

    XA_SEN_GRN_ASTROLAT: int
    """Index for ground sensor astronomical latitude in sensor data array"""

    XA_SEN_GRN_ASTROLON: int
    """Index for ground sensor astronomical longitude in sensor data array"""

    XA_SEN_GRN_ECITIME: int
    """Index for ground sensor ECI time in sensor data array"""

    XA_SEN_GEN_ELSIGMA: int
    """Index for elevation noise standard deviation in sensor data array"""

    XA_SEN_GEN_AZSIGMA: int
    """Index for azimuth noise standard deviation in sensor data array"""

    XA_SEN_GEN_RGSIGMA: int
    """Index for range noise standard deviation in sensor data array"""

    XA_SEN_GEN_RRSIGMA: int
    """Index for range rate noise standard deviation in sensor data array"""

    XA_SEN_GEN_ARSIGMA: int
    """Index for azimuth rate noise standard deviation in sensor data array"""

    XA_SEN_GEN_ERSIGMA: int
    """Index for elevation rate noise standard deviation in sensor data array"""

    XA_SEN_GEN_ELBIAS: int
    """Index for elevation bias in sensor data array"""

    XA_SEN_GEN_AZBIAS: int
    """Index for azimuth bias in sensor data array"""

    XA_SEN_GEN_RGBIAS: int
    """Index for range bias in sensor data array"""

    XA_SEN_GEN_RRBIAS: int
    """Index for range rate bias in sensor data array"""

    XA_SEN_GEN_TIMEBIAS: int
    """Index for time bias in sensor data array"""

    XA_SEN_SIZE: int
    """Array size for sensor data"""

    @staticmethod
    def get_astronomical_ll(sen_key: int) -> list[float]:
        """Get the sensor's astronomical latitude and longitude

        !!! warning
            West is positive longitude in this system.

        Args:
            sen_key (int): The sensor key in the SAAL binary tree

        Returns:
            [latitude (deg), longitude (deg)]
        """

    @staticmethod
    def get_lla(sen_key: int) -> list[float]: ...
    @staticmethod
    def get_loaded_keys() -> list[int]: ...
    @staticmethod
    def load_card(card: str) -> None: ...
    @staticmethod
    def remove_key(sen_key: int) -> None: ...
    @staticmethod
    def count_loaded() -> int: ...
    @staticmethod
    def get(sen_key: int) -> "SAALSensor": ...
    @staticmethod
    def load_file(file_path: str) -> None: ...
    @staticmethod
    def remove_all() -> None: ...
    @staticmethod
    def get_all() -> list["SAALSensor"]: ...
    @staticmethod
    def prune_missing_locations() -> None: ...
    @staticmethod
    def get_arrays(sen_key: int) -> tuple[list[float], str]: ...

class SAALObservation:

    security_character: str
    """Classification of the observation"""

    satellite_number: int
    """Identifier for the target"""

    sensor_number: int
    """Identifier for the observer"""

    epoch_ds50utc: float
    """Epoch of the observation in DS50 UTC format"""

    elevation_or_declination: float
    """TEME Elevation or Declination in degrees"""

    azimuth_or_right_ascension: float
    """TEME Azimuth or Right Ascension in degrees"""

    slant_range: float
    """Range from observer to target in kilometers"""

    range_rate: float
    """Rate of change of slant range in kilometers/second"""

    elevation_rate: float
    """Rate of change of elevation/declination in degrees/second"""

    azimuth_rate: float
    """Rate of change of azimuth/right ascension in degrees/second"""

    range_acceleration: float
    """Acceleration of slant range in kilometers/second^2"""

    observation_type: str
    """B3 type of observation"""

    track_position_indicator: int
    """Indicator of track position (3==beginning, 4==middle, 5==end)"""

    association_status: int
    """Association status of the observation"""

    site_tag: int
    """Identifier for the target as labeled by the observer"""

    spadoc_tag: int
    """Identifier for the target as labeled by SPADOC"""

    position: list[float]
    """Sensor position in TEME coordinates (X, Y, Z) in kilometers"""

    def __init__(self, b3_string: str) -> None:
        """Parser for B3 observation strings"""

class ObsInterface:
    """Interface for working with observation data"""

    XA_OBS_SECCLASS: int
    """Index for security classification (1=Unclassified, 2=Confidential, 3=Secret)"""

    XA_OBS_SATNUM: int
    """Index for satellite number"""

    XA_OBS_SENNUM: int
    """Index for sensor number"""

    XA_OBS_DS50UTC: int
    """Index for observation time in days since 1950 UTC"""

    XA_OBS_OBSTYPE: int
    """Index for observation type"""

    XA_OBS_ELORDEC: int
    """Index for elevation (ob type 1,2,3,4,8) or declination (ob type 5,9) (deg)"""

    XA_OBS_AZORRA: int
    """Index for azimuth (ob type 1,2,3,4,8) or right ascension (ob type 5,9) (deg)"""

    XA_OBS_RANGE: int
    """Index for range (km)"""

    XA_OBS_RANGERATE: int
    """Index for range rate (km/s) for non-optical obs type"""

    XA_OBS_ELRATE: int
    """Index for elevation rate (deg/s)"""

    XA_OBS_AZRATE: int
    """Index for azimuth rate (deg/s)"""

    XA_OBS_RANGEACCEL: int
    """Index for range acceleration (km/s^2)"""

    XA_OBS_TRACKIND: int
    """Index for track position indicator (3=start track, 4=in track, 5=end track)"""

    XA_OBS_ASTAT: int
    """Index for association status assigned by SPADOC"""

    XA_OBS_SITETAG: int
    """Index for original satellite number"""

    XA_OBS_SPADOCTAG: int
    """Index for SPADOC-assigned tag number"""

    XA_OBS_POSX: int
    """Index for position X/ECI or X/EFG (km)"""

    XA_OBS_POSY: int
    """Index for position Y/ECI or Y/EFG (km)"""

    XA_OBS_POSZ: int
    """Index for position Z/ECI or Z/EFG (km)"""

    XA_OBS_VELX: int
    """Index for velocity X/ECI (km/s) or Edot/EFG (km) for ob type 7 TTY"""

    XA_OBS_VELY: int
    """Index for velocity Y/ECI (km/s) or Fdot/EFG (km) for ob type 7 TTY"""

    XA_OBS_VELZ: int
    """Index for velocity Z/ECI (km/s) or Gdot/EFG (km) for ob type 7 TTY"""

    XA_OBS_YROFEQNX: int
    """Index for year of equinox indicator for obs type 5/9 (0=Time of obs, 1=0 Jan of date, 2=J2000, 3=B1950)"""

    XA_OBS_ABERRATION: int
    """Index for aberration indicator (0=not corrected, 1=corrected)"""

    XA_OBS_AZORRABIAS: int
    """Index for AZ/RA bias (deg)"""

    XA_OBS_ELORDECBIAS: int
    """Index for EL/DEC bias (deg)"""

    XA_OBS_RGBIAS: int
    """Index for range bias (km)"""

    XA_OBS_RRBIAS: int
    """Index for range-rate bias (km/sec)"""

    XA_OBS_TIMEBIAS: int
    """Index for time bias (sec)"""

    XA_OBS_RAZORRABIAS: int
    """Index for AZ/RA rate bias (deg/sec)"""

    XA_OBS_RELORDECBIAS: int
    """Index for EL/DEC rate bias (deg/sec)"""

    XA_OBS_SIGMATYPE: int
    """Index for individual obs's sigmas type (0=N/A, 6=6 elements, 21=21 elements, 7=CSV obs)"""

    XA_OBS_SIGMAEL1: int
    """Index for sigma covariance element 1 - 6 elements - Az sigma"""

    XA_OBS_SIGMAEL2: int
    """Index for sigma covariance element 2 - 6 elements - El sigma"""

    XA_OBS_SIGMAEL3: int
    """Index for sigma covariance element 3 - 6 elements - Range sigma"""

    XA_OBS_SIGMAEL4: int
    """Index for sigma covariance element 4 - 6 elements - Range rate sigma"""

    XA_OBS_SIGMAEL5: int
    """Index for sigma covariance element 5 - 6 elements - Az rate sigma"""

    XA_OBS_SIGMAEL6: int
    """Index for sigma covariance element 6 - 6 elements - El rate sigma"""

    XA_OBS_SIGMAEL7: int
    """Index for sigma covariance element 7"""

    XA_OBS_SIGMAEL8: int
    """Index for sigma covariance element 8"""

    XA_OBS_SIGMAEL9: int
    """Index for sigma covariance element 9"""

    XA_OBS_SIGMAEL10: int
    """Index for sigma covariance element 10"""

    XA_OBS_SIGMAEL11: int
    """Index for sigma covariance element 11"""

    XA_OBS_SIGMAEL12: int
    """Index for sigma covariance element 12"""

    XA_OBS_SIGMAEL13: int
    """Index for sigma covariance element 13"""

    XA_OBS_SIGMAEL14: int
    """Index for sigma covariance element 14"""

    XA_OBS_SIGMAEL15: int
    """Index for sigma covariance element 15"""

    XA_OBS_SIGMAEL16: int
    """Index for sigma covariance element 16"""

    XA_OBS_SIGMAEL17: int
    """Index for sigma covariance element 17"""

    XA_OBS_SIGMAEL18: int
    """Index for sigma covariance element 18"""

    XA_OBS_SIGMAEL19: int
    """Index for sigma covariance element 19"""

    XA_OBS_SIGMAEL20: int
    """Index for sigma covariance element 20"""

    XA_OBS_SIGMAEL21: int
    """Index for sigma covariance element 21"""

    XA_OBS_SIZE: int
    """Array size for observation data"""
    @staticmethod
    def get_csv_from_b3() -> str: ...
    @staticmethod
    def load_from_b3(b3_string: str) -> int: ...
    @staticmethod
    def remove_key(key: int) -> None: ...
    @staticmethod
    def get_field(key: int, field: str) -> str: ...
    @staticmethod
    def get_array(key: int) -> list[float]: ...
    @staticmethod
    def count_loaded() -> int: ...
