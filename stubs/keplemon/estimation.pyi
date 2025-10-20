# flake8: noqa
from keplemon.elements import TopocentricElements, CartesianVector
from keplemon.time import Epoch
from keplemon.bodies import Satellite, Sensor
from keplemon.enums import KeplerianType

class Covariance:
    sigmas: list[float]
    """"""

class Observation:
    """
    Args:
        sensor: Sensor that made the observation
        epoch: Time of the observation
        observed_teme_topo: Topocentric elements of the satellite at the time of observation
        observer_teme_pos: Position of the observer in TEME coordinates
    """

    id: str
    """Unique identifier for the observation"""

    sensor: Sensor
    """Sensor which produced the observation"""

    epoch: Epoch
    """Time the measurement was observed"""

    range: float | None
    """Observed range from the sensor to the satellite in **_kilometers_**"""

    range_rate: float | None
    """Observed range rate from the sensor to the satellite in **_kilometers per second_**"""

    right_ascension: float
    """Observed TEME right ascension in **_degrees_**"""

    declination: float
    """Observed TEME declination in **_degrees_**"""

    right_ascension_rate: float | None
    """Observed right ascension rate in **_degrees per second_**"""

    declination_rate: float | None
    """Observed declination rate in **_degrees per second_**"""

    observed_satellite_id: int | None
    """Tagged satellite ID of the observation"""

    def __init__(
        self,
        sensor: Sensor,
        epoch: Epoch,
        observed_teme_topo: TopocentricElements,
        observer_teme_pos: CartesianVector,
    ) -> None: ...
    def get_residual(self, sat: Satellite) -> ObservationResidual | None:
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

    rms: float | None
    """Root mean square of the residuals in **_kilometers_**"""

    weighted_rms: float | None
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

    covariance: Covariance | None
    """UVW covariance matrix of the current estimate in **_kilometers_** and **_kilometers per second_**"""

    output_type: KeplerianType
    """Type of Keplerian elements to be used in the output state"""

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
