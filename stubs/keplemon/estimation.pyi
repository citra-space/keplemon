# flake8: noqa
from keplemon.elements import TopocentricElements, CartesianVector, CartesianState
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

class TDOAObservation:
    """
    Time Difference of Arrival (TDOA) observation for differential timing measurements.

    Represents the time difference in signal arrival at two sensor locations, which corresponds
    to a range difference between the satellite and the two sensors.

    Args:
        sensor_1: First sensor that received the signal
        sensor_2: Second sensor that received the signal
        epoch: Time of the observation
        time_difference: Time difference between arrival at sensor_2 and sensor_1 in **_seconds_**
        observer_1_teme_position: Position of sensor 1 in TEME coordinates in **_kilometers_**
        observer_2_teme_position: Position of sensor 2 in TEME coordinates in **_kilometers_**
    """

    id: str
    """Unique identifier for the observation"""

    sensor_1: Sensor
    """First sensor which produced the observation"""

    sensor_2: Sensor
    """Second sensor which produced the observation"""

    epoch: Epoch
    """Time the measurement was observed"""

    time_difference: float
    """Time difference between sensor_2 and sensor_1 arrival in **_seconds_**"""

    observer_1_teme_position: CartesianVector
    """Position of sensor 1 in TEME coordinates in **_kilometers_**"""

    observer_2_teme_position: CartesianVector
    """Position of sensor 2 in TEME coordinates in **_kilometers_**"""

    observed_satellite_id: int | None
    """Tagged satellite ID of the observation"""

    def __init__(
        self,
        sensor_1: Sensor,
        sensor_2: Sensor,
        epoch: Epoch,
        time_difference: float,
        observer_1_teme_position: CartesianVector,
        observer_2_teme_position: CartesianVector,
    ) -> None: ...
    def get_measurement_and_weight_vector(self) -> tuple[list[float], list[float]]:
        """
        Get the measurement value and weight for this TDOA observation.

        Returns:
            Tuple of (measurement_vector, weight_vector) where weight = 1/sigma^2
        """
        ...
    def get_predicted_vector(self, satellite: Satellite) -> list[float]:
        """
        Get the predicted TDOA measurement for a given satellite state.

        Args:
            satellite: Satellite state to compute prediction for

        Returns:
            Predicted TDOA value in **_seconds_**
        """
        ...
    def get_residual(self, sat: Satellite) -> ObservationResidual | None:
        """
        Calculate the residual of the observation with respect to a given satellite state.

        Returns the residual as a range difference in the `range` field of ObservationResidual.

        Args:
            sat: Expected satellite state

        Returns:
            Calculated residual with range field populated
        """
        ...

class FDOAObservation:
    """
    Frequency Difference of Arrival (FDOA) observation for differential Doppler measurements.

    Represents the difference in Doppler shift observed at two sensor locations, which corresponds
    to a range rate difference between the satellite and the two sensors.

    Args:
        sensor_1: First sensor that received the signal
        sensor_2: Second sensor that received the signal
        epoch: Time of the observation
        frequency_difference: Frequency difference between sensor_2 and sensor_1 in **_Hz_**
        observer_1_teme_state: State (position and velocity) of sensor 1 in TEME coordinates
        observer_2_teme_state: State (position and velocity) of sensor 2 in TEME coordinates
        transmit_frequency: Transmitted signal frequency in **_Hz_**
    """

    id: str
    """Unique identifier for the observation"""

    sensor_1: Sensor
    """First sensor which produced the observation"""

    sensor_2: Sensor
    """Second sensor which produced the observation"""

    epoch: Epoch
    """Time the measurement was observed"""

    frequency_difference: float
    """Frequency difference between sensor_2 and sensor_1 in **_Hz_**"""

    observer_1_teme_state: CartesianState
    """State (position and velocity) of sensor 1 in TEME coordinates"""

    observer_2_teme_state: CartesianState
    """State (position and velocity) of sensor 2 in TEME coordinates"""

    transmit_frequency: float
    """Transmitted signal frequency in **_Hz_**"""

    observed_satellite_id: int | None
    """Tagged satellite ID of the observation"""

    def __init__(
        self,
        sensor_1: Sensor,
        sensor_2: Sensor,
        epoch: Epoch,
        frequency_difference: float,
        observer_1_teme_state: CartesianState,
        observer_2_teme_state: CartesianState,
        transmit_frequency: float,
    ) -> None: ...
    def get_measurement_and_weight_vector(self) -> tuple[list[float], list[float]]:
        """
        Get the measurement value and weight for this FDOA observation.

        Returns:
            Tuple of (measurement_vector, weight_vector) where weight = 1/sigma^2
        """
        ...
    def get_predicted_vector(self, satellite: Satellite) -> list[float]:
        """
        Get the predicted FDOA measurement for a given satellite state.

        Args:
            satellite: Satellite state to compute prediction for

        Returns:
            Predicted FDOA value in **_Hz_**
        """
        ...
    def get_residual(self, sat: Satellite) -> ObservationResidual | None:
        """
        Calculate the residual of the observation with respect to a given satellite state.

        Returns the residual as a range rate difference in the `radial_velocity` field of ObservationResidual.

        Args:
            sat: Expected satellite state

        Returns:
            Calculated residual with radial_velocity field populated
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

    @staticmethod
    def with_range_only(range: float) -> ObservationResidual:
        """
        Create an ObservationResidual with only the range component populated.

        Used internally for TDOA observations where only range difference is meaningful.

        Args:
            range: Range difference in **_kilometers_**

        Returns:
            ObservationResidual with range field set, all other fields zero
        """
        ...

    @staticmethod
    def with_radial_velocity_only(radial_velocity: float) -> ObservationResidual:
        """
        Create an ObservationResidual with only the radial velocity component populated.

        Used internally for FDOA observations where only range rate difference is meaningful.

        Args:
            radial_velocity: Range rate difference in **_kilometers per second_**

        Returns:
            ObservationResidual with radial_velocity field set, all other fields zero
        """
        ...

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

    @staticmethod
    def from_mixed_observations(
        angle_obs: list[Observation],
        tdoa_obs: list[TDOAObservation],
        fdoa_obs: list[FDOAObservation],
        a_priori: Satellite,
    ) -> BatchLeastSquares:
        """
        Create a BatchLeastSquares solver with mixed observation types.

        Combines angle observations (RA/DEC), TDOA (time difference of arrival),
        and FDOA (frequency difference of arrival) observations into a single batch
        for orbit determination.

        Args:
            angle_obs: List of angle (RA/DEC) observations
            tdoa_obs: List of TDOA observations
            fdoa_obs: List of FDOA observations
            a_priori: A priori satellite state

        Returns:
            BatchLeastSquares solver configured with all observation types
        """
        ...

    def solve(self) -> None:
        """Iterate until the solution converges or the maximum number of iterations is reached."""
        ...

    def iterate(self) -> None:
        """Perform a single iteration of the estimation process."""
        ...

    def reset(self) -> None:
        """Reset the estimation process to the initial state."""
        ...
