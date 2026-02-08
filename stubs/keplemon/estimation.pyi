# flake8: noqa
from typing import Optional

from keplemon.elements import TopocentricElements, CartesianVector
from keplemon.time import Epoch
from keplemon.bodies import Satellite, Sensor, Constellation
from keplemon.enums import KeplerianType, AssociationConfidence

class ObservationCollection:
    """
    A collection of simultaneous observations from the same sensor position.

    Args:
        obs: List of observations with the same epoch and observer position
    """

    sensor_position: CartesianVector
    """TEME position of the sensor in **_kilometers_**"""

    sensor_direction: CartesianVector
    """Unit vector pointing in the average direction of all observations"""

    field_of_view: float
    """Estimated field of view in **_degrees_** based on observation spread"""

    observations: list["Observation"]
    """List of observations in this collection"""

    epoch: "Epoch"
    """Common epoch of all observations in the collection"""

    def __init__(self, obs: list["Observation"]) -> None: ...
    @staticmethod
    def get_list(obs: list["Observation"]) -> list["ObservationCollection"]:
        """
        Group observations by epoch and observer position into collections.

        Args:
            obs: List of observations with potentially different epochs and positions

        Returns:
            List of ObservationCollection instances, one for each unique (epoch, position) pair
        """
        ...

    def get_visibility(self, satellite: "Satellite") -> bool:
        """
        Check if a satellite is within the field of view of this collection.

        Args:
            satellite: Satellite to check visibility for

        Returns:
            True if the satellite is within the field of view
        """
        ...

    def get_association(self, satellite: "Satellite") -> Optional["ObservationAssociation"]:
        """
        Get the best observation association for a satellite within this collection.

        Args:
            satellite: Satellite to find an association for

        Returns:
            Best matching ObservationAssociation, or None if not visible or no match
        """
        ...

    def get_association_report(self, satellites: "Constellation") -> "CollectionAssociationReport":
        """
        Get optimal observation-satellite associations for a constellation.

        This method finds the best combination of observation-satellite pairs,
        prioritizing high confidence associations, then medium, then low.
        Each satellite can only associate with one observation and vice versa.

        Args:
            satellites: Constellation of satellites to find associations for

        Returns:
            CollectionAssociationReport containing associations, orphans, and moving satellite IDs
        """
        ...

class Covariance:
    sigmas: list[float]
    """"""

class CollectionAssociationReport:
    """Report of observation associations for an ObservationCollection."""

    orphan_observations: list["Observation"]
    """Observations that could not be associated with any satellite"""

    associations: list["ObservationAssociation"]
    """List of observation-satellite associations found"""

    moving_satellite_ids: list[str]
    """Satellite IDs with low or medium confidence associations (potential movers)"""

class ObservationAssociation:
    observation_id: str
    satellite_id: str
    residual: "ObservationResidual"
    confidence: "AssociationConfidence"

class Observation:
    """
    Args:
        sensor: Sensor that made the observation
        epoch: Time of the observation
        observed_teme_topocentric: Topocentric elements of the satellite at the time of observation
        observer_teme_position: Position of the observer in TEME coordinates
    """

    id: str
    """Unique identifier for the observation"""

    sensor: Sensor
    """Sensor which produced the observation"""

    epoch: Epoch
    """Time the measurement was observed"""

    range: Optional[float]
    """Observed range from the sensor to the satellite in **_kilometers_**"""

    range_rate: Optional[float]
    """Observed range rate from the sensor to the satellite in **_kilometers per second_**"""

    right_ascension: float
    """Observed TEME right ascension in **_degrees_**"""

    declination: float
    """Observed TEME declination in **_degrees_**"""

    right_ascension_rate: Optional[float]
    """Observed right ascension rate in **_degrees per second_**"""

    declination_rate: Optional[float]
    """Observed declination rate in **_degrees per second_**"""

    observed_satellite_id: Optional[str]
    """Tagged satellite ID of the observation"""

    def __init__(
        self,
        sensor: Sensor,
        epoch: Epoch,
        observed_teme_topocentric: TopocentricElements,
        observer_teme_position: CartesianVector,
    ) -> None: ...
    @staticmethod
    def from_saal_files(sensor_file: str, observation_file: str) -> list["Observation"]: ...
    def get_residual(self, sat: Satellite) -> Optional[ObservationResidual]:
        """
        Calculate the residual of the observation with respect to a given satellite state.

        !!! note
            If an error occurs during propagation of the satellite state, this method will return None.

        Args:
            sat: Expected satellite state

        Returns:
            Calculated residual
        """
        ...

    def get_associations(self, sats: Constellation) -> list[ObservationAssociation]:
        """
        Calculate the associations of the observation with respect to a given constellation of satellites.

        Args:
            sats: Constellation of satellites to compare against

        Returns:
            List of possible observation associations
        """
        ...

class ObservationResidual:
    range: float
    """Euclidean distance between the observed and expected state in **_kilometers_**"""

    radial: float
    """Radial distance between the observed and expected state in **_kilometers_**"""

    in_track: float
    """In-track distance between the observed and expected state in **_kilometers_**"""

    cross_track: float
    """Cross-track distance between the observed and expected state in **_kilometers_**"""

    velocity: float
    """Velocity magnitude difference between the observed and expected state in **_kilometers per second_**"""

    radial_velocity: float
    """Radial velocity difference between the observed and expected state in **_kilometers per second_**"""

    in_track_velocity: float
    """In-track velocity difference between the observed and expected state in **_kilometers per second_**"""

    cross_track_velocity: float
    """Cross-track velocity difference between the observed and expected state in **_kilometers per second_**"""

    time: float
    """Time difference between the observed and expected state in **_seconds_**"""

    beta: float
    """Out-of-plane difference between the observed and expected state in **_degrees_**"""

    height: float
    """Height difference between the observed and expected state in **_kilometers_**"""

class BatchLeastSquares:
    """
    Args:
        obs: List of observations to be used in the estimation
        a_priori: A priori satellite state
    """

    converged: bool
    """Indicates if the solution meets the tolerance criteria"""

    max_iterations: int
    """Maximum number of iterations to perform when solving if the tolerance is not met"""

    iteration_count: int
    """Number of iterations performed to reach the solution"""

    current_estimate: Satellite
    """Current estimate of the satellite state after iterating or solving"""

    rms: Optional[float]
    """Root mean square of the residuals in **_kilometers_**"""

    weighted_rms: Optional[float]
    """Unitless weighted root mean square of the residuals"""

    estimate_srp: bool
    """Flag to indicate if solar radiation pressure should be estimated
    
    !!! warning
        This currently has unexpected behavior if solving for output_types other than XP
    """

    estimate_drag: bool
    """Flag to indicate if atmospheric drag should be estimated
    
    !!! warning
        This currently has unexpected behavior if solving for output_types other than XP
    """

    a_priori: Satellite
    """A priori satellite state used to initialize the estimation"""

    observations: list[Observation]
    """List of observations used in the estimation"""

    residuals: list[tuple[Epoch, ObservationResidual]]
    """List of residuals for each observation compared to the current estimate"""

    covariance: Optional[Covariance]
    """UVW covariance matrix of the current estimate in **_kilometers_** and **_kilometers per second_**"""

    output_type: KeplerianType
    """Type of Keplerian elements to be used in the output state"""

    eccentricity_constraint_weight: Optional[float]
    """Tikhonov weight that keeps equinoctial a_f/a_g (eccentricity) near the a priori state"""

    estimate_maneuver: bool
    """Enable maneuver estimation mode. When True, disables drag/SRP estimation and
    searches for a maneuver epoch in the last half orbital period."""

    maneuver_epoch: Optional[Epoch]
    """Detected maneuver epoch after solve() if estimate_maneuver was True.
    None if no significant maneuver was detected (delta-V < 0.05 m/s)."""

    delta_v: Optional[CartesianVector]
    """Delta-V in RIC frame (radial, in-track, cross-track) in **_km/s_** after solve()
    if estimate_maneuver was True. None if no significant maneuver was detected."""

    allow_radial_delta_v: bool
    """When False, heavily penalizes radial delta-V component during maneuver epoch search,
    forcing the solution toward pure in-track maneuvers. Default True (no penalty)."""

    def __init__(
        self,
        obs: list[Observation],
        a_priori: Satellite,
    ) -> None: ...
    def solve(self) -> None:
        """Iterate until the solution converges or the maximum number of iterations is reached."""
        ...

    def iterate(self) -> None:
        """Perform a single iteration of the estimation process."""
        ...

    def reset(self) -> None:
        """Reset the estimation process to the initial state."""
        ...
