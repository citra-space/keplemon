# flake8: noqa
from __future__ import annotations
from keplemon.time import Epoch
from keplemon.enums import Classification, KeplerianType, ReferenceFrame
from keplemon.events import CloseApproach

class OrbitPlotData:
    satellite_id: str
    epochs: list[str]
    semi_major_axes: list[float]
    eccentricities: list[float]
    inclinations: list[float]
    raans: list[float]
    radii: list[float]
    apogee_radii: list[float]
    perigee_radii: list[float]
    latitudes: list[float]
    longitudes: list[float]
    altitudes: list[float]

class GeodeticPosition:
    """
    Args:
        latitude: Latitude in **_degrees_**
        longitude: Longitude in **_degrees_**
        altitude: Altitude in **_kilometers_**
    """

    latitude: float
    """Latitude in **_degrees_**"""

    longitude: float
    """Longitude in **_degrees_**"""

    altitude: float
    """Altitude in **_kilometers_**"""

    def __init__(self, latitude: float, longitude: float, altitude: float) -> None: ...

class HorizonElements:
    """
    Args:
        range: Range in **_kilometers_**
        az: Azimuth in **_degrees_**
        el: Elevation in **_degrees_**
        range_rate: Range rate in **_kilometers per second_**
        az_rate: Azimuth rate in **_degrees per second_**
        el_rate: Elevation rate in **_degrees per second_**
    """

    range: float | None
    azimuth: float
    elevation: float
    range_rate: float | None
    azimuth_rate: float | None
    elevation_rate: float | None

    def __init__(
        self,
        azimuth: float,
        elevation: float,
    ) -> None: ...

class HorizonState:
    """
    Args:
        epoch: UTC epoch of the state
        elements: HorizonElements of the state
    """

    epoch: Epoch
    """UTC epoch of the state"""

    elements: HorizonElements
    """Horizon elements of the state"""

    range: float | None
    """Range in **_kilometers_**"""

    azimuth: float
    """Azimuth in **_degrees_**"""

    elevation: float
    """Elevation in **_degrees_**"""

    range_rate: float | None
    """Range rate in **_kilometers per second_**"""

    azimuth_rate: float | None
    """Azimuth rate in **_degrees per second_**"""

    elevation_rate: float | None
    """Elevation rate in **_degrees per second_**"""

    def __init__(self, epoch: Epoch, elements: HorizonElements) -> None: ...

class KeplerianElements:
    """
    Args:
        semi_major_axis: Average distance from the central body in **_kilometers_**
        eccentricity: Eccentricity of the orbit
        inclination: Inclination of the orbit in **_degrees_**
        raan: Right Ascension of Ascending Node in **_degrees_**
        argument_of_perigee: Argument of Perigee in **_degrees_**
        mean_anomaly: Mean Anomaly in **_degrees_**
    """

    semi_major_axis: float
    eccentricity: float
    inclination: float
    raan: float
    argument_of_perigee: float
    mean_anomaly: float

    def __init__(
        self,
        semi_major_axis: float,
        eccentricity: float,
        inclination: float,
        raan: float,
        argument_of_perigee: float,
        mean_anomaly: float,
    ) -> None: ...

class TLE:

    norad_id: int
    """NORAD catalog ID of the satellite"""

    satellite_id: str
    """"""

    name: str
    """"""

    inclination: float
    """Inclination of the orbit in **_degrees_**"""

    eccentricity: float
    """"""

    raan: float
    """Right Ascension of Ascending Node in **_degrees_**"""

    argument_of_perigee: float
    """Argument of Perigee in **_degrees_**"""

    mean_anomaly: float
    """Mean Anomaly in **_degrees_**"""

    mean_motion: float
    """Mean motion in **_revolutions per day_**"""

    type: KeplerianType
    """"""

    b_star: float
    """"""

    mean_motion_dot: float
    """"""

    mean_motion_dot_dot: float
    """"""

    agom: float
    """"""

    b_term: float
    """"""

    epoch: Epoch
    """UTC epoch of the state"""

    classification: Classification
    """"""

    designator: str
    """8-character identifier of the satellite"""

    cartesian_state: CartesianState
    """TEME cartesian state of the TLE at epoch"""

    @classmethod
    def from_lines(cls, line_1: str, line_2: str, line_3: str | None = None) -> TLE:
        """
        Create a TLE object using strings in 2 or 3 line format
        """
        ...

    @property
    def lines(self) -> tuple[str, str]:
        """
        !!! note
            If the TLE was created in the 3LE format, only lines 2 and 3 will be returned.  The name must be accessed
            using the `name` property.

        Returns:
            Tuple of strings in 2 line format
        """
        ...

    def get_state_at_epoch(self, epoch: Epoch) -> CartesianState: ...

class SphericalVector:
    """
    !!! note
        The range units can be disregarded if this class is not being used for astrodynamic transforms.

    Args:
        range: distance from the origin in **_kilometers_**
        right_ascension: right ascension in **_degrees_**
        declination: declination in **_degrees_**
    """

    range: float
    right_ascension: float
    declination: float
    def __init__(self, range: float, right_ascension: float, declination: float) -> None: ...
    def to_cartesian(self) -> CartesianVector:
        """
        Returns:
            Cartesian vector in **_kilometers_**.
        """
        ...

class CartesianVector:
    """
    Args:
        x: x coordinate
        y: y coordinate
        z: z coordinate
    """

    x: float
    y: float
    z: float
    magnitude: float
    """"""

    def __init__(self, x: float, y: float, z: float) -> None: ...
    def distance(self, other: CartesianVector) -> float:
        """
        !!! note
            Take care to use consistent units with this function.

        Returns:
            Distance between two Cartesian vectors
        """
        ...

    def to_spherical(self) -> SphericalVector:
        """
        Returns:
            Spherical representation of the point
        """
        ...

    def __add__(self, other: CartesianVector) -> CartesianVector: ...
    def __sub__(self, other: CartesianVector) -> CartesianVector: ...
    def angle(self, other: CartesianVector) -> float:
        """
        Returns:
            Angle between two Cartesian vectors in **_radians_**.
        """
        ...

class CartesianState:
    """State represented as x, y, z and vx, vy, vz in a given reference frame.

    Args:
        epoch: UTC epoch of the state
        position: Cartesian position vector in kilometers
        velocity: Cartesian velocity vector in kilometers per second
        frame: reference frame of the state
    """

    position: CartesianVector
    """Position of the state in kilometers"""

    velocity: CartesianVector
    """Velocity of the state in kilometers per second"""

    epoch: Epoch
    """UTC epoch of the state"""

    frame: ReferenceFrame
    """Current reference frame of the state"""

    def __init__(
        self, epoch: Epoch, position: CartesianVector, velocity: CartesianVector, frame: ReferenceFrame
    ) -> None: ...
    def to_keplerian(self) -> KeplerianState:
        """Convert the Cartesian state to osculating Keplerian elements"""
        ...

    def to_frame(self, frame: ReferenceFrame) -> CartesianState:
        """

        Args:
            frame: reference frame of the output state

        Returns:
            CartesianState: Cartesian state in the new frame"""
        ...

class KeplerianState:
    """Orbit represented as Keplerian elements in a given reference frame.

    Args:
        epoch: UTC epoch of the state
        elements: Keplerian elements of the state
        frame: reference frame of the state
        keplerian_type: type of the Keplerian elements
    """

    epoch: Epoch
    frame: ReferenceFrame
    type: KeplerianType
    semi_major_axis: float
    """Average distance from the central body in **_kilometers_**"""

    eccentricity: float
    """Eccentricity of the orbit"""

    inclination: float
    """Inclination of the orbit in **_degrees_**"""

    raan: float
    """Right Ascension of Ascending Node in **_degrees_**"""

    argument_of_perigee: float
    """Argument of Perigee in **_degrees_**"""

    mean_anomaly: float
    """Mean Anomaly in **_degrees_**"""

    mean_motion: float
    """Mean motion in **_revolutions per day_**"""

    apoapsis: float
    """Furthest point from the central body in **_kilometers_**"""

    periapsis: float
    """Closest point to the central body in **_kilometers_**"""

    def __init__(
        self,
        epoch: Epoch,
        elements: KeplerianElements,
        frame: ReferenceFrame,
        keplerian_type: KeplerianType,
    ) -> None: ...
    def to_cartesian(self) -> CartesianState:
        """
        Returns:
            Cartesian state in **_kilometers_** and **_kilometers per second_**.
        """
        ...

    def to_frame(self, frame: ReferenceFrame) -> KeplerianState:
        """
        Args:
            frame: reference frame of the output state

        Returns:
            Keplerian state in the new frame"""
        ...

class Ephemeris:
    def get_close_approach(
        self,
        other: Ephemeris,
        distance_threshold: float,
    ) -> CloseApproach: ...

class TopocentricElements:
    """
    Args:
        ra: TEME right ascension in **_degrees_**
        dec: TEME declination in **_degrees_**
    """

    range: float | None
    """Range in **_kilometers_**"""

    right_ascension: float
    declination: float
    range_rate: float | None
    """Range rate in **_kilometers per second_**"""

    right_ascension_rate: float | None
    """Right ascension rate in **_degrees per second**"""

    declination_rate: float | None
    """Declination rate in **_degrees per second**"""

    def __init__(self, ra: float, dec: float) -> None: ...
    @classmethod
    def from_j2000(cls, epoch: Epoch, ra: float, dec: float) -> TopocentricElements:
        """
        Args:
            epoch: UTC epoch of the angles
            ra: J2000 right ascension in **_degrees_**
            dec: J2000 declination in **_degrees_**
        """
        ...
