# flake8: noqa
from keplemon.time import Epoch, TimeSpan
from keplemon.elements import HorizonState, CartesianVector, TopocentricElements
from keplemon.enums import ReferenceFrame, UCTObservability, UCTValidity
from keplemon.estimation import ObservationAssociation

class FieldOfViewCandidate:
    satellite_id: str
    """ID of the candidate satellite"""

    direction: TopocentricElements
    """Measured direction to the candidate satellite in the sensor's topocentric frame"""

class FieldOfViewReport:
    epoch: Epoch
    """UTC epoch of the field of view report"""

    sensor_position: CartesianVector
    """TEME position of the sensor in the observatory's topocentric frame in **_kilometers_**"""

    sensor_direction: TopocentricElements
    """Direction of the sensor in the observatory's topocentric frame"""

    fov_angle: float
    """Field of view angle of the sensor in **_degrees_**"""

    candidates: list[FieldOfViewCandidate]
    """List of candidate satellites within the field of view"""

    reference_frame: ReferenceFrame
    """Reference frame of the output direction elements"""

class CloseApproach:
    epoch: Epoch
    """UTC epoch of the close approach"""

    primary_id: str
    """Satellite ID of the primary body in the close approach"""

    secondary_id: str
    """Satellite ID of the secondary body in the close approach"""

    distance: float
    """Distance between the two bodies in **_kilometers_**"""

class CloseApproachReport:
    """
    Args:
        start: CA screening start time
        end: CA screening end time
        distance_threshold: Distance threshold for CA screening in **_kilometers_**
    """

    close_approaches: list[CloseApproach]
    """List of close approaches found during the screening"""

    distance_threshold: float
    def __init__(self, start: Epoch, end: Epoch, distance_threshold: float) -> None: ...

class HorizonAccess:

    satellite_id: str
    """ID of the satellite for which the access is calculated"""

    observatory_id: str
    """ID of the observatory for which the access is calculated"""

    start: HorizonState
    """State of the satellite at the start of the access period"""

    end: HorizonState
    """State of the satellite at the end of the access period"""

class HorizonAccessReport:
    """
    Args:
        start: UTC epoch of the start of the access report
        end: UTC epoch of the end of the access report
        min_elevation: Minimum elevation angle for access in **_degrees_**
        min_duration: Minimum duration of access
    """

    accesses: list[HorizonAccess]
    """List of horizon accesses found during the screening"""

    elevation_threshold: float
    """Minimum elevation angle for access in **_degrees_**"""

    start: Epoch
    """UTC epoch of the start of the access report"""

    end: Epoch
    """UTC epoch of the end of the access report"""

    duration_threshold: TimeSpan
    """Minimum duration of a valid access"""

    def __init__(
        self,
        start: Epoch,
        end: Epoch,
        min_elevation: float,
        min_duration: TimeSpan,
    ) -> None: ...

class ProximityEvent:
    """Represents a time period where two satellites remain within a distance threshold."""

    primary_id: str
    """Satellite ID of the primary body"""

    secondary_id: str
    """Satellite ID of the secondary body"""

    start_epoch: Epoch
    """UTC epoch of the start of the proximity event"""

    end_epoch: Epoch
    """UTC epoch of the end of the proximity event"""

    minimum_distance: float
    """Minimum distance between the two bodies during the event in **_kilometers_**"""

    maximum_distance: float
    """Maximum distance between the two bodies during the event in **_kilometers_**"""

class ProximityReport:
    """
    Args:
        start: Proximity screening start time
        end: Proximity screening end time
        distance_threshold: Distance threshold for proximity screening in **_kilometers_**
    """

    events: list[ProximityEvent]
    """List of proximity events found during the screening"""

    distance_threshold: float
    """Distance threshold for proximity screening in **_kilometers_**"""

    start: Epoch
    """UTC epoch of the start of the proximity report"""

    end: Epoch
    """UTC epoch of the end of the proximity report"""

    def __init__(self, start: Epoch, end: Epoch, distance_threshold: float) -> None: ...

class ManeuverEvent:
    """Represents a detected maneuver for a satellite."""

    satellite_id: str
    """Satellite ID of the maneuvering body"""

    epoch: Epoch
    """UTC epoch of the detected maneuver"""

    delta_v: CartesianVector
    """Delta-V vector in RIC (radial, in-track, cross-track) frame in **_meters per second_**"""

class ManeuverReport:
    """
    Args:
        start: Maneuver detection start time
        end: Maneuver detection end time
        distance_threshold: Distance threshold for matching in **_kilometers_**
        velocity_threshold: Velocity threshold for maneuver detection in **_meters per second_**
    """

    maneuvers: list[ManeuverEvent]
    """List of detected maneuvers"""

    distance_threshold: float
    """Distance threshold for matching in **_kilometers_**"""

    velocity_threshold: float
    """Velocity threshold for maneuver detection in **_meters per second_**"""

    start: Epoch
    """UTC epoch of the start of the maneuver report"""

    end: Epoch
    """UTC epoch of the end of the maneuver report"""

    def __init__(
        self, start: Epoch, end: Epoch, distance_threshold: float, velocity_threshold: float
    ) -> None: ...

class UCTValidityReport:
    """
    Report containing UCT validity analysis results.

    This report provides information about a UCT's validity based on:
    - Observation associations with the UCT
    - Proximity events with approved satellites (possible cross-tags)
    - Close approaches with approved satellites (possible maneuver origins)
    - Observability status during the analysis window
    """

    satellite_id: str
    """ID of the UCT satellite being analyzed"""

    associations: list[ObservationAssociation]
    """List of observation associations where the UCT matched orphan observations"""

    possible_cross_tags: list[ProximityEvent]
    """List of proximity events with approved satellites that could indicate cross-tagging"""

    possible_origins: list[CloseApproach]
    """List of close approaches with approved satellites that could indicate maneuver origins"""

    observability: UCTObservability
    """Observability status of the UCT during the analysis window"""

    validity: UCTValidity
    """Validity assessment based on the analysis results"""
