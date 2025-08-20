# flake8: noqa: F401

def mean_motion_to_sma(mean_motion: float) -> float:
    """
    Convert mean motion to semi-major axis.

    Args:
        mean_motion: Mean motion in revolutions/day.

    Returns:
        Semi-major axis in kilometers.
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
