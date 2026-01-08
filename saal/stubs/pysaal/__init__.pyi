# flake8: noqa
"""Type hints for saal Python bindings.

Units:
    - ds50 values are days since 1950-01-01 00:00:00 in the given time scale.
    - Angles are radians unless noted otherwise.

Example:
    ```python
    from saal import TimeInterface

    ti = TimeInterface()
    ds50 = ti.ymd_components_to_ds50(1956, 1, 1, 0, 0, 0.0)
    print(f"{ds50:.1f}")
    ```

    Output:
    ```bash
    2192.0
    ```
"""

from __future__ import annotations

from typing import Optional

class MainInterface:
    """Access DllMain settings, messages, and key modes."""

    DLL_VERSION: str
    ALL_KEYMODE_NODUP: int
    ALL_KEYMODE_DMA: int
    ELSET_KEYMODE_NODUP: int
    ELSET_KEYMODE_DMA: int
    DUPKEY_ZERO: int
    DUPKEY_ACTUAL: int
    IDX_ORDER_ASC: int
    IDX_ORDER_DES: int
    IDX_ORDER_READ: int
    IDX_ORDER_QUICK: int
    TIME_IS_MSE: int
    TIME_IS_TAI: int
    TIME_IS_UTC: int

    def __init__(self, file_name: Optional[str] = None) -> None:
        """Create an interface and optionally load an input file."""
        ...

    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform).

        Example:
            ```python
            from saal import MainInterface

            main = MainInterface()
            print(MainInterface.DLL_VERSION in main.info)
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    @property
    def last_error_message(self) -> str:
        """Last error message reported by the DLL."""
        ...

    @property
    def last_info_message(self) -> str:
        """Last informational message reported by the DLL."""
        ...

    @property
    def key_mode(self) -> int:
        """Global key mode for all keys.

        Example:
            ```python
            from saal import MainInterface

            main = MainInterface()
            print(int(main.key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

    @key_mode.setter
    def key_mode(self, mode: int) -> None:
        """Set the global key mode for all keys.

        Example:
            ```python
            from saal import KeyMode, MainInterface

            main = MainInterface()
            main.key_mode = KeyMode.NoDuplicates
            print(int(main.key_mode))
            ```

            Output:
            ```bash
            0
            ```
        """
        ...

    def reset_key_mode(self) -> None:
        """Reset global key mode and duplicate key mode to defaults.

        Example:
            ```python
            from saal import KeyMode, MainInterface

            main = MainInterface()
            main.key_mode = KeyMode.NoDuplicates
            main.reset_key_mode()
            print(int(main.key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

    @property
    def elset_key_mode(self) -> int:
        """ELSET key mode.

        Example:
            ```python
            from saal import ElsetKeyMode, MainInterface

            main = MainInterface()
            main.elset_key_mode = ElsetKeyMode.NoDuplicates
            print(int(main.elset_key_mode))
            ```

            Output:
            ```bash
            0
            ```
        """
        ...

    @elset_key_mode.setter
    def elset_key_mode(self, mode: int) -> None:
        """Set the ELSET key mode.

        Example:
            ```python
            from saal import ElsetKeyMode, MainInterface

            main = MainInterface()
            main.elset_key_mode = ElsetKeyMode.DirectMemoryAccess
            print(int(main.elset_key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

    @property
    def duplicate_key_mode(self) -> int:
        """Duplicate key mode behavior."""
        ...

    @duplicate_key_mode.setter
    def duplicate_key_mode(self, mode: int) -> None:
        """Set the behavior of returned keys when a duplicate is loaded in NoDuplicates mode.

        Check DuplicateKeyMode for return options.

        Example:
            ```python
            from saal import DuplicateKeyMode, MainInterface

            main = MainInterface()
            main.duplicate_key_mode = DuplicateKeyMode.ReturnKey
            print(int(main.duplicate_key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

class AstroInterface:
    """Astronomical conversion utilities."""

    XF_CONV_SGP42SGP: int

    def __init__(self) -> None: ...
    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform)."""
        ...

    def keplerian_to_equinoctial(self, kep: list[float]) -> list[float]: ...
    def equinoctial_to_keplerian(self, eqnx: list[float]) -> list[float]: ...
    def keplerian_to_cartesian(self, kep: list[float]) -> list[float]: ...
    def cartesian_to_keplerian(self, posvel: list[float]) -> list[float]: ...
    def mean_motion_to_sma(self, mean_motion: float) -> float: ...
    def sma_to_mean_motion(self, semi_major_axis: float) -> float: ...
    def kozai_to_brouwer(self, eccentricity: float, inclination: float, mean_motion: float) -> float: ...
    def brouwer_to_kozai(self, eccentricity: float, inclination: float, mean_motion: float) -> float: ...
    def osculating_to_mean(self, osc: list[float]) -> list[float]: ...
    def gst_teme_to_lla(self, gst: float, teme_pos: list[float]) -> list[float]: ...
    def efg_to_lla(self, efg_pos: list[float]) -> list[float]: ...

class EnvironmentInterface:
    """Access Earth constants and fundamental catalog settings."""

    XF_FKMOD_4: int
    XF_FKMOD_5: int

    def __init__(self, file_name: Optional[str] = None) -> None:
        """Create an interface and optionally load an input file."""
        ...

    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform).

        Example:
            ```python
            from saal import EnvironmentInterface, MainInterface

            env = EnvironmentInterface()
            print(MainInterface.DLL_VERSION in env.info)
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    @property
    def earth_radius(self) -> float:
        """Earth radius from the current GEO model.

        Units: kilometers.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_radius:.3f}")
            ```

            Output:
            ```bash
            6378.135
            ```
        """
        ...

    @property
    def earth_rotation_rate(self) -> float:
        """Earth rotation rate from the current FK model.

        Units: radians/day.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_rotation_rate:.18f}")
            ```

            Output:
            ```bash
            0.017202791694070362
            ```
        """
        ...

    @property
    def earth_rotation_acceleration(self) -> float:
        """Earth rotation acceleration from the current FK model.

        Units: radians/day^2.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_rotation_acceleration:.15e}")
            ```

            Output:
            ```bash
            5.075514194322695e-15
            ```
        """
        ...

    @property
    def earth_mu(self) -> float:
        """Earth gravitational parameter from the current GEO model.

        Units: km^3/s^2.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_mu:.1f}")
            ```

            Output:
            ```bash
            398600.8
            ```
        """
        ...

    @property
    def earth_flattening(self) -> float:
        """Earth flattening (reciprocal) from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_flattening:.15f}")
            ```

            Output:
            ```bash
            0.003352779454168
            ```
        """
        ...

    @property
    def j2(self) -> float:
        """J2 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j2:.9f}")
            ```

            Output:
            ```bash
            0.001082616
            ```
        """
        ...

    @property
    def j3(self) -> float:
        """J3 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j3:.11f}")
            ```

            Output:
            ```bash
            -0.00000253881
            ```
        """
        ...

    @property
    def j4(self) -> float:
        """J4 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j4:.11f}")
            ```

            Output:
            ```bash
            -0.00000165597
            ```
        """
        ...

    @property
    def j5(self) -> float:
        """J5 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j5:.7e}")
            ```

            Output:
            ```bash
            -2.1848270e-07
            ```
        """
        ...

    @property
    def fundamental_catalog(self) -> int:
        """Fundamental catalog selection (FK4 or FK5).

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(int(env.fundamental_catalog))
            ```

            Output:
            ```bash
            5
            ```
        """
        ...

    @fundamental_catalog.setter
    def fundamental_catalog(self, catalog: int) -> None:
        """Set the fundamental catalog selection.

        Example:
            ```python
            from saal import EnvironmentInterface, FundamentalCatalog

            env = EnvironmentInterface()
            env.fundamental_catalog = FundamentalCatalog.Four
            print(int(env.fundamental_catalog))
            ```

            Output:
            ```bash
            4
            ```
        """
        ...

class TimeInterface:
    """Time conversion utilities and time constants access.

    ds50 values are days since 1950-01-01 00:00:00 in the given time scale.
    """

    def __init__(self, file_name: Optional[str] = None) -> None:
        """Create an interface and optionally load time constants."""
        ...

    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform).

        Example:
            ```python
            from saal import MainInterface, TimeInterface

            ti = TimeInterface()
            print(MainInterface.DLL_VERSION in ti.info)
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    def ymd_components_to_ds50(
        self,
        year: int,
        month: int,
        day: int,
        hour: int,
        minute: int,
        second: float,
    ) -> float:
        """Convert UTC date/time components to ds50 UTC.

        Units: returns days since 1950-01-01 00:00:00 UTC.

        Example:
            ```python
            from saal import TimeInterface

            ti = TimeInterface()
            ds50 = ti.ymd_components_to_ds50(1956, 1, 1, 0, 0, 0.0)
            print(f"{ds50:.1f}")
            ```

            Output:
            ```bash
            2192.0
            ```
        """
        ...

    def ds50_to_ymd_components(self, ds50_utc: float) -> tuple[int, int, int, int, int, float]:
        """Convert ds50 UTC to (year, month, day, hour, minute, second)."""
        ...

    def dtg_to_ds50(self, dtg: str) -> float:
        """Convert a DTG string to ds50 UTC."""
        ...

    def ds50_to_dtg20(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 20 characters."""
        ...

    def ds50_to_dtg19(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 19 characters."""
        ...

    def ds50_to_dtg17(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 17 characters."""
        ...

    def ds50_to_dtg15(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 15 characters."""
        ...

    def year_doy_to_ds50(self, year: int, doy: float) -> float:
        """Convert (year, day-of-year) to ds50 UTC."""
        ...

    def ds50_to_year_doy(self, ds50_utc: float) -> tuple[int, float]:
        """Convert ds50 UTC to (year, day-of-year)."""
        ...

    def utc_to_ut1(self, ds50_utc: float) -> float:
        """Convert ds50 UTC to ds50 UT1 using loaded time constants.

        If no timing constants are loaded, the input is returned unchanged.
        Units: days since 1950-01-01 00:00:00 (UTC in, UT1 out).

        Example:
            ```python
            from saal import TimeInterface

            ti = TimeInterface()
            ut1 = ti.utc_to_ut1(2192.0)
            print(f"{ut1:.1f}")
            ```

            Output:
            ```bash
            2192.0
            ```
        """
        ...

    def utc_to_tai(self, ds50_utc: float) -> float:
        """Convert ds50 UTC to ds50 TAI using loaded time constants."""
        ...

    def tai_to_utc(self, ds50_tai: float) -> float:
        """Convert ds50 TAI to ds50 UTC using loaded time constants."""
        ...

    def utc_to_tt(self, ds50_utc: float) -> float:
        """Convert ds50 UTC to ds50 TT using loaded time constants."""
        ...

    def tai_to_ut1(self, ds50_tai: float) -> float:
        """Convert ds50 TAI to ds50 UT1 using loaded time constants."""
        ...

    def load_constants(self, path: str) -> None:
        """Load timing constants from a file."""
        ...

    def load_time_constants(self, path: str) -> None:
        """Load timing constants from a file."""
        ...

    def get_fk4_greenwich_angle(self, ds50_ut1: float) -> float:
        """Compute Greenwich right ascension using the FK4 catalog.

        Units: returns radians; input is days since 1950-01-01 00:00:00 UT1.

        Example:
            ```python
            import math
            from saal import TimeInterface

            ti = TimeInterface()
            ang = ti.get_fk4_greenwich_angle(2192.0)
            print(math.isfinite(ang))
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    def get_fk5_greenwich_angle(self, ds50_ut1: float) -> float:
        """Compute Greenwich right ascension using the FK5 catalog.

        Units: returns radians; input is days since 1950-01-01 00:00:00 UT1.

        Example:
            ```python
            import math
            from saal import TimeInterface

            ti = TimeInterface()
            ang = ti.get_fk5_greenwich_angle(2192.0)
            print(math.isfinite(ang))
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    @property
    def constants_loaded(self) -> bool:
        """Return whether timing constants are loaded."""
        ...

    def time_constants_loaded(self) -> bool:
        """Return whether timing constants are loaded.

        Example:
            ```python
            from saal import TimeInterface

            ti = TimeInterface()
            print(ti.time_constants_loaded())
            ```

            Output:
            ```bash
            False
            ```
        """
        ...

class SGP4Interface:
    """Access SGP4 propagation helpers."""

    SGP4_EPHEM_ECI: int
    SGP4_EPHEM_J2K: int
    SGP4_TIMETYPE_MSE: int
    SGP4_TIMETYPE_DS50UTC: int
    DYN_SS_BASIC: int
    GP_ERR_NONE: int
    GP_ERR_BADFK: int
    GP_ERR_ANEGATIVE: int
    GP_ERR_ATOOLARGE: int
    GP_ERR_EHYPERPOLIC: int
    GP_ERR_ENEGATIVE: int
    GP_ERR_MATOOLARGE: int
    GP_ERR_E2TOOLARGE: int

    def __init__(self) -> None: ...
    @property
    def info(self) -> str: ...
    def load(self, sat_key: int) -> None: ...
    def remove(self, sat_key: int) -> None: ...
    def clear(self) -> None: ...
    def get_count(self) -> int: ...
    def get_position_velocity_lla(
        self, sat_key: int, ds50_utc: float
    ) -> tuple[float, list[float], list[float], list[float]]: ...
    def get_position_velocity(self, sat_key: int, ds50_utc: float) -> tuple[list[float], list[float]]: ...
    def get_lla(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_position(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_full_state(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_equinoctial(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_ephemeris(
        self,
        sat_key: int,
        start: float,
        stop: float,
        step: float,
        frame: int,
    ) -> list[float]: ...
    def array_to_ephemeris(
        self,
        xa_tle: list[float],
        start: float,
        stop: float,
        step: float,
        frame: int,
    ) -> list[float]: ...
    def fit_xp_array(
        self,
        epoch: float,
        posvel: list[float],
        ballistic_coefficient: Optional[float],
        srp_coefficient: Optional[float],
    ) -> list[float]: ...
    def fit_sgp4_array(
        self,
        epoch: float,
        posvel: list[float],
        b_star: Optional[float],
    ) -> list[float]: ...
    def get_positions_velocities(self, sat_keys: list[int], ds50_utc: float) -> list[float]: ...
    def set_license_directory(self, lic_file_path: str) -> None: ...
    def get_license_directory(self) -> str: ...
    def reepoch_tle(self, sat_key: int, re_epoch_ds50_utc: float) -> tuple[str, str]: ...

class ParsedB3:
    """Parsed representation of a B3 observation."""

    def __init__(self) -> None: ...

    classification: str
    norad_id: int
    sensor_number: int
    epoch: float
    elevation: Optional[float]
    declination: Optional[float]
    azimuth: Optional[float]
    right_ascension: Optional[float]
    range: Optional[float]
    range_rate: Optional[float]
    year_of_equinox: Optional[int]
    elevation_rate: Optional[float]
    azimuth_rate: Optional[float]
    range_acceleration: Optional[float]
    observation_type: int
    track_position: int
    association_status: int
    site_tag: int
    spadoc_tag: int
    position: Optional[list[float]]

    def get_line(self) -> str: ...

class ObsInterface:
    """Access observation parsing helpers."""

    EQUINOX_OBSTIME: int
    EQUINOX_OBSYEAR: int
    EQUINOX_J2K: int
    EQUINOX_B1950: int
    OBSFORM_B3: int
    OBSFORM_TTY: int
    OBSFORM_CSV: int
    OBSFORM_RF: int
    BADOBSKEY: int
    DUPOBSKEY: int
    OBS_KEYMODE_NODUP: int
    OBS_KEYMODE_DMA: int

    def __init__(self) -> None: ...

    @property
    def info(self) -> str: ...
    def parse_line(self, line: str) -> ParsedB3: ...

class ParsedSensor:
    """Parsed representation of a sensor."""

    key: int
    number: int
    minimum_range: Optional[float]
    maximum_range: Optional[float]
    range_rate_limit: Optional[float]
    apply_range_limits: bool
    mobile: bool
    latitude: Optional[float]
    longitude: Optional[float]
    altitude: Optional[float]
    astronomical_latitude: float
    astronomical_longitude: float
    azimuth_noise: Optional[float]
    elevation_noise: Optional[float]
    range_noise: Optional[float]
    range_rate_noise: Optional[float]
    azimuth_rate_noise: Optional[float]
    elevation_rate_noise: Optional[float]
    description: Optional[str]

class SensorInterface:
    """Access sensor parsing and storage helpers."""

    SEN_KEYMODE_NODUP: int
    SEN_KEYMODE_DMA: int
    BADSENKEY: int
    DUPSENKEY: int
    SENLOC_TYPE_ECR: int
    SENLOC_TYPE_EFG: int
    SENLOC_TYPE_LLH: int
    SENLOC_TYPE_ECI: int

    def __init__(self) -> None: ...

    @property
    def info(self) -> str: ...
    def parse_key(self, sen_key: int) -> ParsedSensor: ...
    def parse_all(self) -> list[ParsedSensor]: ...
    def prune_missing_locations(self) -> None: ...
    def get_astronomical_ll(self, sen_key: int) -> list[float]: ...
    def get_lla(self, sen_key: int) -> Optional[list[float]]: ...
    def get_keys(self, order: int) -> list[int]: ...
    def load_card(self, card: str) -> None: ...
    def remove(self, sen_key: int) -> None: ...
    def get_count(self) -> int: ...
    def load_file(self, file_path: str) -> None: ...
    def clear(self) -> None: ...
    def get_arrays(self, sen_key: int) -> tuple[list[float], str]: ...

class ParsedTLE:
    """Parsed representation of a TLE."""

    def __init__(self) -> None: ...

    epoch: float
    norad_id: int
    inclination: float
    raan: float
    eccentricity: float
    argument_of_perigee: float
    mean_anomaly: float
    mean_motion: float
    ephemeris_type: int
    element_set_number: int
    revolution_number: int
    designator: Optional[str]
    classification: str
    mean_motion_1st_derivative: Optional[float]
    mean_motion_2nd_derivative: Optional[float]
    b_star: Optional[float]
    ballistic_coefficient: Optional[float]
    srp_coefficient: Optional[float]

    def get_lines(self, remove_nulls: bool = False) -> tuple[str, str]: ...

class TLEInterface:
    """Access TLE parsing and storage helpers."""

    TLETYPE_SGP: int
    TLETYPE_SGP4: int
    TLETYPE_XP: int
    TLETYPE_SP: int

    def __init__(self) -> None: ...

    @property
    def info(self) -> str: ...
    def fix_blank_exponent_sign(self, line_1: str) -> str: ...
    def add_check_sums(self, line_1: str, line_2: str) -> tuple[str, str]: ...
    def lines_to_arrays(self, line_1: str, line_2: str) -> tuple[list[float], str]: ...
    def arrays_to_lines(self, xa_tle: list[float], xs_tle: str) -> tuple[str, str]: ...
    def get_check_sums(self, line_1: str, line_2: str) -> tuple[int, int]: ...
    def load_lines(self, line_1: str, line_2: str) -> int: ...
    def load_arrays(self, xa_tle: list[float], xs_tle: str) -> int: ...
    def load_file(self, file_path: str) -> int: ...
    def clear(self) -> None: ...
    def remove(self, sat_key: int) -> None: ...
    def get_count(self) -> int: ...
    def get_keys(self, order: int) -> list[int]: ...
    def get_lines(self, sat_key: int) -> tuple[str, str]: ...
    def get_arrays(self, sat_key: int) -> tuple[list[float], str]: ...
    def parse_lines(self, line_1: str, line_2: str) -> ParsedTLE: ...

__all__ = [
    "MainInterface",
    "AstroInterface",
    "EnvironmentInterface",
    "SGP4Interface",
    "ObsInterface",
    "ParsedB3",
    "SensorInterface",
    "ParsedSensor",
    "TimeInterface",
    "TLEInterface",
    "ParsedTLE",
]
