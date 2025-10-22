import pytest

from keplemon.time import TimeSpan, Epoch, TimeComponents
from keplemon.enums import TimeSystem


def test_time_components():
    components = TimeComponents(2025, 4, 2, 4, 2, 42.420)
    assert components.year == 2025
    assert components.month == 4
    assert components.day == 2
    assert components.hour == 4
    assert components.minute == 2
    assert components.second == 42.420
    assert components.to_iso() == "2025-04-02T04:02:42.420"


def test_epoch():
    EXPECTED_DTG_20 = "2025/092 0402 42.420"
    EXPECTED_UTC_ISO = "2025-04-02T04:02:42.420"
    EXPECTED_TAI_ISO = "2025-04-02T04:03:19.420"
    EXPECTED_UT1_ISO = "2025-04-02T04:02:42.457"
    EXPECTED_UTC_DS50 = 27486.168546527777
    from_iso = Epoch.from_iso(EXPECTED_UTC_ISO, TimeSystem.UTC)
    time_components = from_iso.to_time_components()
    assert time_components.year == 2025
    assert time_components.month == 4
    assert time_components.day == 2
    assert time_components.hour == 4
    assert time_components.minute == 2
    assert time_components.second == pytest.approx(42.42)
    assert from_iso.to_dtg_20() == EXPECTED_DTG_20
    assert from_iso.days_since_1950 == EXPECTED_UTC_DS50

    from_ds50 = Epoch.from_days_since_1950(EXPECTED_UTC_DS50, TimeSystem.UTC)
    assert from_ds50.to_dtg_20() == EXPECTED_DTG_20
    assert from_ds50.to_iso() == EXPECTED_UTC_ISO
    assert from_ds50.days_since_1950 == EXPECTED_UTC_DS50
    assert from_ds50.to_time_components() == time_components
    assert from_ds50.to_dtg_20() == EXPECTED_DTG_20
    assert from_ds50.time_system == TimeSystem.UTC

    tai = from_ds50.to_system(TimeSystem.TAI)
    assert tai.time_system == TimeSystem.TAI
    assert tai.to_iso() == EXPECTED_TAI_ISO

    ut1 = from_ds50.to_system(TimeSystem.UT1)
    assert ut1.time_system == TimeSystem.UT1
    assert ut1.to_iso() == EXPECTED_UT1_ISO

    one_hour = TimeSpan.from_hours(1)
    plus_one_hour = from_ds50 + one_hour
    epoch_diff = plus_one_hour - from_ds50
    minus_one_hour = plus_one_hour - one_hour

    assert epoch_diff.in_hours() == pytest.approx(1.0)
    assert minus_one_hour.days_since_1950 == from_ds50.days_since_1950


class TestEpoch:

    def test_to_datetime(self):
        expected_iso = "2025-04-02T04:02:42.420000"
        epoch = Epoch.from_iso(expected_iso, TimeSystem.UTC)
        dt = epoch.to_datetime()
        assert dt.isoformat() == expected_iso


def test_time_span():

    EXPECTED_DAYS = 0.0625
    EXPECTED_HOURS = 1.5
    EXPECTED_MINUTES = 90.0
    EXPECTED_SECONDS = 5400.0

    from_hours = TimeSpan.from_hours(EXPECTED_HOURS)
    assert from_hours.in_days() == EXPECTED_DAYS
    assert from_hours.in_hours() == EXPECTED_HOURS
    assert from_hours.in_minutes() == EXPECTED_MINUTES
    assert from_hours.in_seconds() == EXPECTED_SECONDS

    from_minutes = TimeSpan.from_minutes(EXPECTED_MINUTES)
    assert from_minutes.in_days() == EXPECTED_DAYS
    assert from_minutes.in_hours() == EXPECTED_HOURS
    assert from_minutes.in_minutes() == EXPECTED_MINUTES
    assert from_minutes.in_seconds() == EXPECTED_SECONDS

    from_seconds = TimeSpan.from_seconds(EXPECTED_SECONDS)
    assert from_seconds.in_days() == EXPECTED_DAYS
    assert from_seconds.in_hours() == EXPECTED_HOURS
    assert from_seconds.in_minutes() == EXPECTED_MINUTES
    assert from_seconds.in_seconds() == EXPECTED_SECONDS

    from_days = TimeSpan.from_days(EXPECTED_DAYS)
    assert from_days.in_days() == EXPECTED_DAYS
    assert from_days.in_hours() == EXPECTED_HOURS
    assert from_days.in_minutes() == EXPECTED_MINUTES
    assert from_days.in_seconds() == EXPECTED_SECONDS
