# flake8: noqa
from __future__ import annotations
from pathlib import Path
from keplemon.enums import TimeSystem
from typing import overload, Any, Union
from datetime import datetime

def request_time_constants_update(output_path: Union[str, Path]) -> None:
    """
    Request time constants and EOP data from USNO and save to the specified path.

    Args:
        output_path: Path where the SAAL-formatted time constants will be written
    """
    ...

class TimeSpan:
    @classmethod
    def from_days(cls, days: float) -> TimeSpan:
        """
        Args:
            days: Total duration in days
        """
        ...

    @classmethod
    def from_seconds(cls, seconds: float) -> TimeSpan:
        """
        Args:
            seconds: Total duration in seconds
        """
        ...

    @classmethod
    def from_minutes(cls, minutes: float) -> TimeSpan:
        """
        Args:
            minutes: Total duration in minutes
        """
        ...

    @classmethod
    def from_hours(cls, hours: float) -> TimeSpan:
        """
        Args:
            hours: Total duration in hours
        """
        ...

    def in_days(self) -> float:
        """
        Returns:
            Total duration in days
        """
        ...

    def in_seconds(self) -> float:
        """
        Returns:
            Total duration in seconds
        """
        ...

    def in_minutes(self) -> float:
        """
        Returns:
            Total duration in minutes
        """
        ...

    def in_hours(self) -> float:
        """
        Returns:
            Total duration in hours
        """
        ...

class TimeComponents:
    year: int
    """"""

    month: int
    """"""

    day: int
    """"""

    hour: int
    """"""

    minute: int
    """"""

    second: float
    """"""

    def __init__(
        self,
        year: int,
        month: int,
        day: int,
        hour: int,
        minute: int,
        second: float,
    ) -> None:
        """
        Initialize TimeComponents with the given year, month, day, hour, minute, and second.

        !!! warning
            It is the user's responsibility to input valid month/day combinations and time values.  This class does not
            perform validation.

        Args:
            year: Year (e.g., 2023)
            month: Month (1-12)
            day: Day (1-31)
            hour: Hour (0-23)
            minute: Minute (0-59)
            second: Second (0.0-59.999...)
        """

    def to_iso(self) -> str:
        """
        Returns:
            Epoch in ISO 8601 format (YYYY-MM-DDThh:mm:ss.sss)
        """
        ...

    @classmethod
    def from_iso(cls, iso_str: str) -> TimeComponents:
        """
        Args:
            iso_str: ISO 8601 formatted string (YYYY-MM-DDThh:mm:ss.sss)
        """
        ...

class Epoch:
    days_since_1950: float
    """
    Core floating epoch representation used by the SAAL binaries.
    """

    time_system: TimeSystem
    """"""

    day_of_year: float
    """Decimal day of the year (1-365.999...)"""

    def to_datetime(self) -> datetime:
        """
        Returns:
            Aware datetime object in UTC time system
        """
        ...

    @classmethod
    def from_datetime(cls, dt: datetime) -> Epoch:
        """
        Args:
            dt: Aware or naive datetime object (assumed to be UTC if naive)
        """
        ...

    @classmethod
    def now(cls) -> Epoch:
        """
        Returns:
            Current epoch in UTC time system
        """
        ...

    @classmethod
    def from_days_since_1950(cls, days: float, time_system: TimeSystem) -> Epoch:
        """
        Args:
            days: Days since 1 Jan 1956
            time_system: System used to calculate the number of days since 1 Jan 1956
        """
        ...

    @classmethod
    def from_iso(cls, iso_str: str, time_system: TimeSystem) -> Epoch:
        """
        Args:
            iso_str: ISO 8601 formatted string (YYYY-MM-DDThh:mm:ss.sss)
            time_system: System used to calculate the epoch
        """
        ...

    @classmethod
    def from_time_components(cls, components: TimeComponents, time_system: TimeSystem) -> Epoch:
        """
        Args:
            components: Epoch represented as individual components
            time_system: System used to calculate the epoch
        """
        ...

    @classmethod
    def from_dtg(cls, dtg: str, time_system: TimeSystem) -> Epoch:
        """
        Create an epoch from a standard SAAL DTG format.

        !!! note
            DTG formats include:

            - DTG 20: YYYY/DDD HHMM SS.SSS
            - DTG 19: YYYYMonDDHHMMSS.SSS
            - DTG 17: YYYY/DDD.DDDDDDDD
            - DTG 15: YYDDDHHMMSS.SSS

        Args:
            dtg: DTG formatted string (YYYY/DDD hhmm ss.sss)
            time_system: System used to calculate the epoch
        """
        ...

    def to_iso(self) -> str:
        """
        Returns:
            Epoch in ISO 8601 format (YYYY-MM-DDThh:mm:ss.sss)
        """
        ...

    def to_dtg_20(self) -> str:
        """
        Returns:
            Epoch in DTG 20 format (YYYY/DDD HHMM SS.SSS)
        """
        ...

    def to_dtg_19(self) -> str:
        """
        Returns:
            Epoch in DTG 19 format (YYYYMonDDHHMMSS.SSS)
        """
        ...

    def to_dtg_17(self) -> str:
        """
        Returns:
            Epoch in DTG 17 format (YYYY/DDD.DDDDDDDD)
        """
        ...

    def to_dtg_15(self) -> str:
        """
        Returns:
            Epoch in DTG 15 format (YYDDDHHMMSS.SSS)
        """
        ...

    def to_time_components(self) -> TimeComponents:
        """
        Returns:
            Epoch as individual components
        """
        ...

    def to_fk4_greenwich_angle(self) -> float:
        """
        Returns:
            Greenwich angle in radians using FK4 theory
        """
        ...

    def to_fk5_greenwich_angle(self) -> float:
        """
        Returns:
            Greenwich angle in radians using FK5 theory
        """
        ...

    def to_system(self, time_system: TimeSystem) -> Epoch:
        """
        Convert the epoch to a different time system.

        Args:
            time_system: Desired ouput system
        """
        ...

    def __add__(self, delta: TimeSpan) -> Epoch: ...
    @overload
    def __sub__(self, other: TimeSpan) -> Epoch: ...
    @overload
    def __sub__(self, other: Epoch) -> TimeSpan: ...
    def __gt__(self, other: Epoch) -> bool: ...
    def __lt__(self, other: Epoch) -> bool: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __ge__(self, other: Epoch) -> bool: ...
    def __le__(self, other: Epoch) -> bool: ...
