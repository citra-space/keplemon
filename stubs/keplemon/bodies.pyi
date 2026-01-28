# flake8: noqa
from typing import Iterator, Optional

from keplemon.elements import (
    TLE,
    CartesianState,
    Ephemeris,
    KeplerianState,
    GeodeticPosition,
    OrbitPlotData,
    TopocentricElements,
    RelativeState,
    BoreToBodyAngles,
)
from keplemon.catalogs import TLECatalog
from keplemon.time import Epoch, TimeSpan
from keplemon.events import CloseApproach, CloseApproachReport, CrossTagReport, HorizonAccessReport, FieldOfViewReport, ManeuverEvent, ManeuverReport, ProximityReport
from keplemon.propagation import ForceProperties
from keplemon.enums import ReferenceFrame
from keplemon.estimation import CollectionAssociationReport, Observation, ObservationAssociation, ObservationCollection, ObservationResidual

class Satellite:

    id: str
    """Unique identifier for the satellite."""

    norad_id: int
    """Number corresponding to the satellite's NORAD catalog ID.
    """

    force_properties: ForceProperties
    """Force properties of the satellite used for propagation"""

    name: Optional[str]
    """Human-readable name of the satellite"""

    keplerian_state: Optional[KeplerianState]
    """Keplerian state of the satellite at the epoch of the TLE, if available"""

    geodetic_position: Optional[GeodeticPosition]
    """Geodetic position of the satellite at the epoch of the TLE, if available"""

    def __init__(self) -> None: ...
    @classmethod
    def from_tle(cls, tle: TLE) -> Satellite:
        """
        Instantiate a Satellite from a legacy TLE

        Args:
            tle: Two-line element set for the satellite
        """
        ...

    def get_close_approach(
        self,
        other: Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> Optional[CloseApproach]: ...
    def get_proximity_report(
        self,
        other: Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> Optional[ProximityReport]:
        """
        Check if this satellite stays within a distance threshold of another satellite.

        Args:
            other: Satellite to compare against
            start: UTC epoch of the start of the report
            end: UTC epoch of the end of the report
            distance_threshold: Maximum distance threshold in **_kilometers_**

        Returns:
            Proximity report with a single event if satellites stay within threshold,
            or an empty report if the threshold is exceeded at any point
        """
        ...
    def get_maneuver_event(
        self,
        future_sat: Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
        velocity_threshold: float,
    ) -> Optional[ManeuverEvent]:
        """
        Detect a maneuver by comparing this satellite's state with a future state.

        Args:
            future_sat: Satellite with a future epoch state to compare against
            start: UTC epoch of the start of the search window
            end: UTC epoch of the end of the search window
            distance_threshold: Distance threshold for matching in **_kilometers_**
            velocity_threshold: Velocity threshold for maneuver detection in **_meters per second_**

        Returns:
            ManeuverEvent if a maneuver is detected, None otherwise
        """
        ...
    def get_ephemeris(
        self,
        start: Epoch,
        end: Epoch,
        step: TimeSpan,
    ) -> Ephemeris: ...
    def get_state_at_epoch(self, epoch: Epoch) -> CartesianState: ...
    def to_tle(self) -> Optional[TLE]:
        """
        Returns:
            Satellite as a two-line element set or None if no state is loaded

        """
        ...

    def get_relative_state_at_epoch(self, other: Satellite, epoch: Epoch) -> Optional[RelativeState]:
        """
        Calculate the relative state between this satellite and another satellite at a given epoch.

        Args:
            other: Secondary satellite to calculate the relative state against
            epoch: UTC epoch at which the relative state will be calculated
        """
        ...

    def get_body_angles_at_epoch(self, other: Satellite, epoch: Epoch) -> Optional[BoreToBodyAngles]:
        """
        Calculate the bore-to-body angles between this satellite and another satellite at a given epoch.

        Args:
            other: Secondary satellite to calculate the bore-to-body angles against
            epoch: UTC epoch at which the bore-to-body angles will be calculated
        """
        ...

    def get_plot_data(self, start: Epoch, end: Epoch, step: TimeSpan) -> Optional[OrbitPlotData]: ...
    def get_observatory_access_report(
        self,
        observatories: list[Observatory],
        start: Epoch,
        end: Epoch,
        min_el: float,
        min_duration: TimeSpan,
    ) -> Optional[HorizonAccessReport]:
        """
        Calculate horizon access from multiple observatories to this satellite.

        Args:
            observatories: List of observatories to check for horizon access
            start: UTC epoch of the start of the report
            end: UTC epoch of the end of the report
            min_el: Minimum elevation angle in **_degrees_**
            min_duration: Minimum duration of access

        Returns:
            Horizon access report containing accesses from all observatories to the satellite,
            or None if the satellite ephemeris cannot be generated
        """
        ...

    def get_associations(
        self, collections: list[ObservationCollection]
    ) -> list[ObservationAssociation]:
        """
        Find observation associations for this satellite across multiple observation collections.

        Args:
            collections: List of observation collections to search for associations

        Returns:
            List of observation associations where this satellite matches observations
        """
        ...

    def get_rms(self, obs: list[Observation]) -> float:
        """
        Calculate the root mean squared position error between observations and the satellite state.

        Uses interpolated states from cached ephemeris when available for better performance.

        Args:
            obs: List of observations to compare against

        Returns:
            Root mean squared position error in **_kilometers_**

        Raises:
            ValueError: If no valid residuals could be computed
        """
        ...

    def get_residuals(self, obs: list[Observation]) -> list[ObservationResidual]:
        """
        Calculate position residuals between observations and the satellite state.

        Uses interpolated states from cached ephemeris when available for better performance.

        Args:
            obs: List of observations to compare against

        Returns:
            List of observation residuals

        Raises:
            ValueError: If no valid residuals could be computed
        """
        ...

class Constellation:

    count: int
    """Number of satellites in the constellation"""

    name: Optional[str]
    """Human-readable name of the constellation"""

    def __init__(self) -> None: ...
    def get_plot_data(self, start: Epoch, end: Epoch, step: TimeSpan) -> dict[str, OrbitPlotData]: ...
    @classmethod
    def from_tle_catalog(cls, tle_catalog: TLECatalog) -> Constellation:
        """
        Instantiate a Constellation from a TLE catalog

        Args:
            tle_catalog: TLE catalog for the constellation
        """
        ...

    def get_states_at_epoch(self, epoch: Epoch) -> dict[int, CartesianState]:
        """
        Args:
            epoch: UTC epoch at which the states will be calculated

        Returns:
            (satellite_id, state) dictionary for the constellation at the given epoch
        """
        ...

    def get_ephemeris(
        self,
        start: Epoch,
        end: Epoch,
        step: TimeSpan,
    ) -> dict[str, Ephemeris]:
        """
        Args:
            start: UTC epoch of the start of the ephemeris
            end: UTC epoch of the end of the ephemeris
            step: Time step for the ephemeris

        Returns:
            (satellite_id, ephemeris) dictionary for the constellation
        """
        ...

    def get_ca_report_vs_one(
        self,
        other: Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> CloseApproachReport:
        """
        Calculate close approaches between the constellation and a given satellite.

        Args:
            other: Satellite to compare against
            start: UTC epoch of the start of the close approach report
            end: UTC epoch of the end of the close approach report
            distance_threshold: Distance threshold for close approach screening in **_kilometers_**

        Returns:
            Close approach report for the constellation vs. the given satellite
        """
        ...

    def get_ca_report_vs_many(
        self,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> CloseApproachReport:
        """
        Calculate close approaches among satellites in the calling constellation.

        !!! warning
            This is a long-running operation when the constellation is large.

        Args:
            start: UTC epoch of the start of the close approach report
            end: UTC epoch of the end of the close approach report
            distance_threshold: Distance threshold for close approach screening in **_kilometers_**

        Returns:
            Close approach report for the constellation vs. all other satellites
        """
        ...

    def get_proximity_report_vs_one(
        self,
        other: Satellite,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> ProximityReport:
        """
        Check which constellation satellites stay within a distance threshold of a given satellite.

        Args:
            other: Satellite to compare against
            start: UTC epoch of the start of the report
            end: UTC epoch of the end of the report
            distance_threshold: Maximum distance threshold in **_kilometers_**

        Returns:
            Proximity report containing events for satellite pairs that stay within threshold
        """
        ...

    def get_proximity_report_vs_many(
        self,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
    ) -> ProximityReport:
        """
        Check which satellite pairs in the constellation stay within a distance threshold.

        Args:
            start: UTC epoch of the start of the report
            end: UTC epoch of the end of the report
            distance_threshold: Maximum distance threshold in **_kilometers_**

        Returns:
            Proximity report containing events for satellite pairs that stay within threshold
        """
        ...

    def detect_cross_tags(
        self,
        uct: Satellite,
        observations: list[Observation],
        start: Epoch,
        end: Epoch,
        proximity_threshold: float | None = None,
        confidence_threshold: float | None = None,
    ) -> CrossTagReport:
        """
        Detect if a UCT (Uncorrelated Track) is likely a cross-tag (misidentification of an approved satellite).

        This method analyzes observations during proximity events to determine if the UCT and an approved
        satellite are the same object. The logic is:

        - If orphan observations exist during proximity windows: Both objects were likely visible simultaneously,
          suggesting the UCT is a real distinct object.
        - If no orphan observations exist: All observations match either the UCT or approved satellite,
          suggesting they are the same object (cross-tag).

        Args:
            uct: The UCT satellite to analyze
            observations: List of observations to analyze
            start: UTC epoch of the start of the analysis window
            end: UTC epoch of the end of the analysis window
            proximity_threshold: Maximum distance threshold in **_kilometers_** for proximity detection (default: 10.0)
            confidence_threshold: Minimum confidence level (0.0 to 1.0) to declare a cross-tag (default: 0.75)

        Returns:
            Cross-tag report with analysis results and evidence
        """
        ...

    def get_maneuver_events(
        self,
        future_sats: Constellation,
        start: Epoch,
        end: Epoch,
        distance_threshold: float,
        velocity_threshold: float,
    ) -> ManeuverReport:
        """
        Detect maneuvers by comparing current satellite states with future states.

        Matches satellites by ID between the two constellations and detects maneuvers
        where the velocity difference exceeds the threshold.

        Args:
            future_sats: Constellation with future epoch states to compare against
            start: UTC epoch of the start of the search window
            end: UTC epoch of the end of the search window
            distance_threshold: Distance threshold for matching in **_kilometers_**
            velocity_threshold: Velocity threshold for maneuver detection in **_meters per second_**

        Returns:
            ManeuverReport containing all detected maneuvers
        """
        ...

    def __getitem__(self, satellite_id: str) -> Satellite: ...
    def __setitem__(self, satellite_id: str, sat: Satellite) -> None: ...
    def __iter__(self) -> Iterator[str]: ...
    def __contains__(self, key: str) -> bool: ...
    def __len__(self) -> int: ...
    def keys(self) -> list[str]: ...
    def get_horizon_access_report(
        self,
        site: Observatory,
        start: Epoch,
        end: Epoch,
        min_el: float,
        min_duration: TimeSpan,
    ) -> HorizonAccessReport:
        """
        Calculate horizon access to a given observatory.

        Args:
            site: Observatory to check for horizon access
            start: UTC epoch of the start of the report
            end: UTC epoch of the end of the report
            min_el: Minimum elevation angle in **_degrees_**
            min_duration: Minimum duration of access

        Returns:
            Horizon access report for the constellation from the observatory
        """
        ...

    def cache_ephemeris(self, start: Epoch, end: Epoch, step: TimeSpan) -> None:
        """
        Cache ephemeris for all satellites in the constellation.

        This pre-computes and caches ephemeris data for each satellite, enabling
        faster interpolation-based state lookups via interpolate_state_at_epoch.

        Args:
            start: Start epoch for ephemeris caching
            end: End epoch for ephemeris caching
            step: Time step between cached states
        """
        ...

    def get_association_reports(
        self, collections: list[ObservationCollection]
    ) -> list[CollectionAssociationReport]:
        """
        Get association reports for multiple observation collections.

        Args:
            collections: List of observation collections to find associations for

        Returns:
            List of CollectionAssociationReport, one per input collection
        """
        ...

class Sensor:
    """
    Args:
        name: Identifier of the sensor
        angular_noise: Angular noise in **_degrees_**
    """

    id: str
    """Unique identifier for the sensor."""
    name: Optional[str]
    angular_noise: float
    range_noise: Optional[float]
    """Range noise in **_kilometers_**"""

    range_rate_noise: Optional[float]
    """Range rate noise in **_kilometers per second_**"""

    angular_rate_noise: Optional[float]
    """Angular rate noise in **_degrees per second_**"""
    def __init__(self, angular_noise: float) -> None: ...

class Observatory:
    """
    Args:
        latitude: Latitude in **_degrees_**
        longitude: Longitude in **_degrees_**
        altitude: Altitude in **_kilometers_**
    """

    name: str
    id: str
    """Unique identifier for the observatory."""
    latitude: float
    longitude: float
    altitude: float
    sensors: list[Sensor]
    """List of sensors at the observatory"""
    def __init__(
        self,
        latitude: float,
        longitude: float,
        altitude: float,
    ) -> None: ...
    def get_state_at_epoch(self, epoch: Epoch) -> CartesianState:
        """
        Args:
            epoch: UTC epoch of the state

        Returns:
            TEME Cartesian state of the observatory in **_kilometers_** and **_kilometers per second_**
        """
        ...

    @classmethod
    def from_cartesian_state(cls, state: CartesianState) -> Observatory:
        """
        Create an observatory from a Cartesian state.

        Args:
            state: Cartesian state of the observatory

        """
        ...

    def get_theta(self, epoch: Epoch) -> float:
        """
        Calculate the Greenwich angle plus the observatory longitude at a given epoch.

        Args:
            epoch: UTC epoch for the calculation

        Returns:
            Greenwich angle plus the observatory longitude in **_radians_**
        """
        ...

    def get_horizon_access_report(
        self,
        satellite: Satellite,
        start: Epoch,
        end: Epoch,
        min_el: float,
        min_duration: TimeSpan,
    ) -> HorizonAccessReport:
        """
        Calculate horizon access for a satellite from the observatory.

        Args:
            satellite: Satellite to check for horizon access
            start: UTC epoch of the start of the report
            end: UTC epoch of the end of the report
            min_el: Minimum elevation angle in **_degrees_**
            min_duration: Minimum duration of access in **_seconds_**

        Returns:
            Horizon access report for the satellite from the observatory
        """
        ...

    def get_field_of_view_report(
        self,
        epoch: Epoch,
        sensor_direction: TopocentricElements,
        angular_threshold: float,
        sats: Constellation,
        reference_frame: ReferenceFrame,
    ) -> FieldOfViewReport:
        """
        Calculate satellites in the field of view from a given time and direction.

        Args:
            epoch: UTC epoch of the report
            sensor_direction: Topocentric direction the sensor is pointing
            angular_threshold: Angular threshold in **_degrees_**
            sats: Constellation of satellites to check for being in the field of view
            reference_frame: Reference frame of the output direction elements
        """
        ...

    def get_topocentric_to_satellite(
        self,
        epoch: Epoch,
        sat: Satellite,
        reference_frame: ReferenceFrame,
    ) -> TopocentricElements:
        """
        Get the topocentric elements of a satellite as seen from the observatory.
        Args:
            epoch: UTC epoch of the observation
            sat: Satellite to observe
            reference_frame: Reference frame of the output direction elements
        """
        ...
