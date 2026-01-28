# flake8: noqa
from keplemon.time import Epoch, TimeSpan
from keplemon.elements import HorizonState, CartesianVector, TopocentricElements
from keplemon.enums import ReferenceFrame
from keplemon.estimation import Observation, ObservationAssociation

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

class CrossTagEvidence:
    """Evidence from a single observation collection for cross-tag detection."""

    epoch: Epoch
    """UTC epoch of the observation collection"""

    sensor_id: str
    """ID of the sensor that made the observations"""

    orphan_count: int
    """Number of orphan observations (observations not matched to any satellite)"""

    approved_satellite_matched: bool
    """Whether the approved satellite candidate was matched to observations"""

    uct_was_visible: bool
    """Whether the UCT satellite was within the sensor's field of view"""

    conclusion: str
    """Conclusion for this collection: 'REAL_UCT', 'CROSS_TAG', or 'INCONCLUSIVE'"""

    approved_associations: list[ObservationAssociation]
    """List of associations between observations and the approved satellite"""

    orphan_observations: list[Observation]
    """List of orphan observations"""

    def __init__(
        self,
        epoch: Epoch,
        sensor_id: str,
        orphan_count: int,
        approved_satellite_matched: bool,
        uct_was_visible: bool,
        conclusion: str,
        approved_associations: list[ObservationAssociation],
        orphan_observations: list[Observation],
    ) -> None: ...

class CrossTagReport:
    """
    Report containing cross-tag detection analysis results.

    Args:
        uct_id: ID of the UCT satellite being analyzed
        is_likely_crosstag: Whether the UCT is likely a cross-tag (misidentification)
        approved_candidate_id: ID of the approved satellite candidate (if cross-tag detected)
        confidence: Confidence level (0.0 to 1.0)
        evidence: List of evidence from individual observation collections
        reason: Human-readable explanation of the conclusion
        total_collections_analyzed: Total number of observation collections analyzed
        collections_with_orphans: Number of collections with orphan observations
        collections_without_orphans: Number of collections without orphan observations
    """

    uct_id: str
    """ID of the UCT satellite being analyzed"""

    is_likely_crosstag: bool
    """Whether the UCT is likely a cross-tag (misidentification)"""

    approved_candidate_id: str | None
    """ID of the approved satellite candidate (if cross-tag detected)"""

    confidence: float
    """Confidence level (0.0 to 1.0)"""

    evidence: list[CrossTagEvidence]
    """List of evidence from individual observation collections"""

    reason: str
    """Human-readable explanation of the conclusion"""

    total_collections_analyzed: int
    """Total number of observation collections analyzed"""

    collections_with_orphans: int
    """Number of collections with orphan observations"""

    collections_without_orphans: int
    """Number of collections without orphan observations"""

    def __init__(
        self,
        uct_id: str,
        is_likely_crosstag: bool,
        approved_candidate_id: str | None,
        confidence: float,
        evidence: list[CrossTagEvidence],
        reason: str,
        total_collections_analyzed: int,
        collections_with_orphans: int,
        collections_without_orphans: int,
    ) -> None: ...
