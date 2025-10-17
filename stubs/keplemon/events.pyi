# flake8: noqa
from keplemon.time import Epoch, TimeSpan
from keplemon.elements import HorizonState, CartesianVector, TopocentricElements
from keplemon.enums import ReferenceFrame

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
