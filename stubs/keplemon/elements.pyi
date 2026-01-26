# flake8: noqa
from __future__ import annotations
from typing import Optional

from keplemon.time import Epoch
from keplemon.enums import Classification, KeplerianType, ReferenceFrame
from keplemon.estimation import Observation
from keplemon.events import CloseApproach
from keplemon.bodies import Observatory

class RelativeState:
    epoch: Epoch
    position: CartesianVector
    velocity: CartesianVector
    origin_satellite_id: str
    secondary_satellite_id: str

class BoreToBodyAngles:
    earth_angle: float
    sun_angle: float
    moon_angle: float

class OrbitPlotState:
    epoch: Epoch
    latitude: float
    longitude: float
    altitude: float
    semi_major_axis: float
    eccentricity: float
    inclination: float
    raan: float
    radius: float
    apogee_radius: float
    perigee_radius: float

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

    range: Optional[float]
    azimuth: float
    elevation: float
    range_rate: Optional[float]
    azimuth_rate: Optional[float]
    elevation_rate: Optional[float]

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

    range: Optional[float]
    """Range in **_kilometers_**"""

    azimuth: float
    """Azimuth in **_degrees_**"""

    elevation: float
    """Elevation in **_degrees_**"""

    range_rate: Optional[float]
    """Range rate in **_kilometers per second_**"""

    azimuth_rate: Optional[float]
    """Azimuth rate in **_degrees per second_**"""

    elevation_rate: Optional[float]
    """Elevation rate in **_degrees per second_**"""

    def __init__(self, epoch: Epoch, elements: HorizonElements) -> None: ...
    @classmethod
    def from_topocentric_state(cls, state: TopocentricState, observer: Observatory) -> HorizonState: ...

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

class EquinoctialElements:
    """
    Args:
        a_f: Equinoctial element a_f
        a_g: Equinoctial element a_g
        chi: Equinoctial element chi
        psi: Equinoctial element psi
        mean_longitude: Mean longitude in **_degrees_**
        mean_motion: Mean motion in **_revolutions per day_**
    """

    a_f: float
    a_g: float
    chi: float
    psi: float
    mean_longitude: float
    mean_motion: float

    def __init__(
        self,
        a_f: float,
        a_g: float,
        chi: float,
        psi: float,
        mean_longitude: float,
        mean_motion: float,
    ) -> None: ...

    def to_keplerian(self) -> KeplerianElements: ...

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

    semi_major_axis: float
    """Average distance from the central body in **_kilometers_**
    
    !!! note
        This is always calculated using Brouwer mean motion and will differ slightly from Kozai-computed SMA.
    """

    apoapsis: float
    """Apoapsis radius in **_kilometers_**"""

    periapsis: float
    """Periapsis radius in **_kilometers_**"""

    @classmethod
    def from_lines(cls, line_1: str, line_2: str, line_3: Optional[str] = None) -> TLE:
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

    def get_state_at_epoch(self, epoch: Epoch) -> CartesianState:
        """
        Get a Cartesian state at a specific epoch by propagating the TLE.

        Args:
            epoch: The epoch at which to get the state

        Returns:
            CartesianState in TEME frame at the specified epoch
        """
        ...

    def get_keplerian_state_at_epoch(self, epoch: Epoch) -> KeplerianState:
        """
        Get a Keplerian state at a specific epoch by propagating the TLE.

        Returns mean elements appropriate to the TLE type (Kozai for SGP4, Brouwer for XP).

        Args:
            epoch: The epoch at which to get the state

        Returns:
            KeplerianState in TEME frame at the specified epoch
        """
        ...

    def get_observation_at_epoch(self, epoch: Epoch) -> Observation:
        """
        Get an observation at a specific epoch.

        Creates an observation from the TLE's propagated state at the given epoch,
        useful for batch least squares fitting of multiple TLEs.

        Args:
            epoch: The epoch at which to generate the observation

        Returns:
            Observation derived from this TLE at the given epoch
        """
        ...

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
        right_ascension: TEME right ascension in **_degrees_**
        declination: TEME declination in **_degrees_**
    """

    range: Optional[float]
    """Range in **_kilometers_**"""

    right_ascension: float
    declination: float
    range_rate: Optional[float]
    """Range rate in **_kilometers per second_**"""

    right_ascension_rate: Optional[float]
    """Right ascension rate in **_degrees per second**"""

    declination_rate: Optional[float]
    """Declination rate in **_degrees per second**"""

    def __init__(self, right_ascension: float, declination: float) -> None: ...
    @classmethod
    def from_j2000(cls, epoch: Epoch, ra: float, dec: float) -> TopocentricElements:
        """
        Args:
            epoch: UTC epoch of the angles
            ra: J2000 right ascension in **_degrees_**
            dec: J2000 declination in **_degrees_**
        """
        ...

class TopocentricState:
    """
    Args:
        epoch: UTC epoch of the state
        elements: TopocentricElements of the state
    """

    epoch: Epoch
    """UTC epoch of the state"""

    elements: TopocentricElements
    """Topocentric elements of the state"""

    range: Optional[float]
    """Range in **_kilometers_**"""

    right_ascension: float
    """TEME right ascension in **_degrees_**"""

    declination: float
    """TEME declination in **_degrees_**"""

    range_rate: Optional[float]
    """Range rate in **_kilometers per second_**"""

    right_ascension_rate: Optional[float]
    """Right ascension rate in **_degrees per second**"""

    declination_rate: Optional[float]
    """Declination rate in **_degrees per second**"""

    def __init__(self, epoch: Epoch, elements: TopocentricElements) -> None: ...
    @classmethod
    def from_horizon_state(cls, horizon_state: HorizonState, observer: Observatory) -> TopocentricState:
        """
        Args:
            horizon_state: HorizonState of the target
            observer: Position of the observer
        """
        ...
