from keplemon._keplemon.saal.time_func_interface import load_time_constants  # type: ignore
from keplemon._keplemon import (  # type: ignore
    get_thread_count,
    set_thread_count,
    set_license_file_path,
    get_license_file_path,
    set_jpl_ephemeris_file_path,
)
from pathlib import Path


PACKAGE_DIRECTORY = Path(__file__).parent
ASSETS_DIRECTORY = PACKAGE_DIRECTORY / "assets"

# Set the license file directory to the package directory
set_license_file_path(ASSETS_DIRECTORY.as_posix())

# Load the time constants from the assets directory
TIME_CONSTANTS_PATH = ASSETS_DIRECTORY / "time_constants.dat"
load_time_constants(TIME_CONSTANTS_PATH.as_posix())

# Load the JPL path
JPL_EPHEMERIS_PATH = ASSETS_DIRECTORY / "JPLcon_1950_2050.405"
set_jpl_ephemeris_file_path(JPL_EPHEMERIS_PATH.as_posix())

__all__ = [
    "get_thread_count",
    "set_thread_count",
    "TIME_CONSTANTS_PATH",
    "JPL_EPHEMERIS_PATH",
    "set_license_file_path",
    "PACKAGE_DIRECTORY",
    "ASSETS_DIRECTORY",
    "get_license_file_path",
]
