# flake8: noqa
from keplemon.time import Epoch, TimeSpan
from keplemon.elements import HorizonState

class CloseApproach:
    epoch: Epoch
    """UTC epoch of the close approach"""

    primary_id: int
    """Satellite ID of the primary body in the close approach"""

    secondary_id: int
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
