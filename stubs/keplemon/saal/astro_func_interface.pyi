# flake8: noqa: F401

def mean_motion_to_sma(mean_motion: float) -> float:
    """
    Convert mean motion to semi-major axis in kilometers.

    Args:
        mean_motion: Mean motion in revolutions/day.
    """
    ...

def sma_to_mean_motion(sma: float) -> float:
    """
    Convert semi-major axis to mean motion in revolutions/day.

    Args:
        sma: Semi-major axis in kilometers.
    """
    ...

def kozai_to_brouwer(e_kozai: float, i_kozai: float, n_kozai: float) -> float:
    """
    Convert Kozai orbital elements to Brouwer orbital elements.

    Args:
        e_kozai: Eccentricity (unitless).
        i_kozai: Inclination in degrees.
        n_kozai: Mean motion in revolutions/day.
    """
    ...

def brouwer_to_kozai(e_brouwer: float, i_brouwer: float, n_brouwer: float) -> float:
    """
    Convert Brouwer orbital elements to Kozai orbital elements.

    Args:
        e_brouwer: Eccentricity (unitless).
        i_brouwer: Inclination in degrees.
        n_brouwer: Mean motion in revolutions/day.
    """
    ...

def ra_dec_to_az_el(
    theta: float,
    lat: float,
    long: float,
    ra: float,
    dec: float,
) -> tuple[float, float]:
    """
    Convert right ascension and declination to azimuth and elevation.

    Args:
        theta: Greenwich angle in radians.
        lat: Sensor latitude in degrees.
        long: Sensor longitude in degrees.
        ra: TEME right ascension in degrees.
        dec: TEME declination in degrees.

    Returns:
        A tuple containing azimuth and elevation in degrees.
    """
    ...

def ra_dec_to_az_el_time(
    ds50_utc: float,
    lat: float,
    long: float,
    ra: float,
    dec: float,
) -> tuple[float, float]:
    """
    Convert right ascension and declination to azimuth and elevation.

    Args:
        ds50_utc: Epoch in DS50 UTC format.
        lat: Sensor latitude in degrees.
        long: Sensor longitude in degrees.
        ra: Right ascension in degrees.
        dec: Declination in degrees.

    Returns:
        A tuple containing azimuth and elevation in degrees.
    """
    ...

def teme_to_topo(
    theta: float,
    lat: float,
    sen_pos: list[float],
    sat_pos: list[float],
    sat_vel: list[float],
) -> list[float]:
    """
    Convert TEME coordinates to topocentric coordinates.

    Args:
        theta: Greenwich angle plus the sensor longitude in radians.
        lat: Latitude in degrees.
        sen_pos: Sensor TEME position in kilometers.
        sat_pos: Satellite TEME position in kilometers.
        sat_vel: Satellite TEME velocity in kilometers/second.

    Returns:
        Topocentric coordinates as a list of floats.
    """
    ...

def topo_date_to_equinox(yr_of_equinox: int, ds50utc: float, ra: float, dec: float) -> tuple[float, float]:
    """
    Convert topocentric right ascension and declination to equinox coordinates for a given year of equinox.

    Args:
        yr_of_equinox: Year of the equinox (using YROFEQNX_ constants).
        ds50utc: Epoch in DS50 UTC format.
        ra: Topocentric right ascension in degrees.
        dec: Topocentric declination in degrees.

    Returns:
        A tuple containing the equinox right ascension and declination in degrees.
    """
    ...

def topo_equinox_to_date(yr_of_equinox: int, ds50utc: float, ra: float, dec: float) -> tuple[float, float]:
    """
    Convert equinox right ascension and declination to topocentric coordinates for a given year of equinox.

    Args:
        yr_of_equinox: Year of the equinox (using YROFEQNX_ constants).
        ds50utc: Epoch in DS50 UTC format.
        ra: Equinox right ascension in degrees.
        dec: Equinox declination in degrees.

    Returns:
        A tuple containing the topocentric right ascension and declination in degrees.
    """
    ...

def theta_teme_to_lla(theta: float, lat: float, long: float, ra: float, dec: float) -> tuple[float, float]:
    """
    Convert TEME coordinates to latitude, longitude, and altitude.

    Args:
        theta: Greenwich angle in radians.
        lat: Sensor latitude in degrees.
        long: Sensor longitude in degrees.
        ra: TEME right ascension in degrees.
        dec: TEME declination in degrees.
    """
    ...

def get_jpl_sun_and_moon_position(ds50_utc: float) -> tuple[list[float], list[float]]:
    """
    Get the JPL ephemeris positions of the Sun and Moon in TEME coordinates.

    Args:
        ds50_utc: Epoch in DS50 UTC format.

    Returns:
        A tuple containing two lists:
            - Sun position in TEME coordinates [x, y, z] in kilometers.
            - Moon position in TEME coordinates [x, y, z] in kilometers.
    """
    ...

def time_teme_to_lla(ds50_utc: float, lat: float, long: float, ra: float, dec: float) -> tuple[float, float]:
    """
    Convert TEME coordinates to latitude, longitude, and altitude.

    Args:
        ds50_utc: Epoch in DS50 UTC format.
        lat: Sensor latitude in degrees.
        long: Sensor longitude in degrees.
        ra: TEME right ascension in degrees.
        dec: TEME declination in degrees.
    """

XA_TOPO_AZ: int
"""Index for topocentric azimuth in degrees."""

XA_TOPO_EL: int
"""Index for topocentric elevation in degrees."""

XA_TOPO_RANGE: int
"""Index for topocentric range in kilometers."""

XA_TOPO_RADOT: int
"""Index for topocentric right ascension dot in degrees/second."""

XA_TOPO_DECDOT: int
"""Index for topocentric declination dot in degrees/second."""

XA_TOPO_AZDOT: int
"""Index for topocentric azimuth dot in degrees/second."""

XA_TOPO_ELDOT: int
"""Index for topocentric elevation dot in degrees/second."""

XA_TOPO_RANGEDOT: int
"""Index for topocentric range dot in kilometers/second."""

XA_TOPO_RA: int
"""Index for topocentric right ascension in degrees."""

XA_TOPO_DEC: int
"""Index for topocentric declination in degrees."""

XA_TOPO_SIZE: int
"""Size of XA_TOPO_ array"""

YROFEQNX_2000: int
"""Year of equinox 2000 constant"""

YROFEQNX_CURR: int
"""Year of current equinox constant"""
