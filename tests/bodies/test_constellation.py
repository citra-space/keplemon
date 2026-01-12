import pytest

from keplemon.bodies import Constellation
from keplemon.catalogs import TLECatalog
from keplemon.time import Epoch, TimeSpan
from keplemon.enums import TimeSystem


@pytest.fixture()
def ca_3le_catalog():
    return TLECatalog.from_tle_file("tests/2025-04-15-ca.3le")


@pytest.fixture()
def celestrak_tle_catalog():
    return TLECatalog.from_tle_file("tests/2025-04-15-celestrak.tle")


@pytest.fixture()
def space_track_tle_catalog():
    return TLECatalog.from_tle_file("tests/2025-04-15-space-track.tle")


@pytest.fixture()
def space_track_3le_catalog():
    return TLECatalog.from_tle_file("tests/2025-04-15-space-track.3le")


@pytest.fixture()
def celestrak_3le_catalog():
    return TLECatalog.from_tle_file("tests/2025-04-15-celestrak.3le")


@pytest.fixture()
def celestrak_tle_sats(celestrak_tle_catalog: TLECatalog):
    return Constellation.from_tle_catalog(celestrak_tle_catalog)


@pytest.fixture()
def space_track_tle_sats(space_track_tle_catalog: TLECatalog):
    return Constellation.from_tle_catalog(space_track_tle_catalog)


@pytest.fixture()
def space_track_3le_sats(space_track_3le_catalog: TLECatalog):
    return Constellation.from_tle_catalog(space_track_3le_catalog)


@pytest.fixture()
def celestrak_3le_sats(celestrak_3le_catalog: TLECatalog):
    return Constellation.from_tle_catalog(celestrak_3le_catalog)


@pytest.fixture()
def ca_3le_sats(ca_3le_catalog: TLECatalog):
    return Constellation.from_tle_catalog(ca_3le_catalog)


def test_from_tle_catalog(
    celestrak_3le_sats: Constellation,
    space_track_3le_sats: Constellation,
    celestrak_tle_sats: Constellation,
    space_track_tle_sats: Constellation,
):
    assert space_track_3le_sats.count == 27485
    assert celestrak_3le_sats.count == 11304
    assert space_track_tle_sats.count == 27485
    assert celestrak_tle_sats.count == 11305

    assert space_track_3le_sats.name == "tests/2025-04-15-space-track.3le"
    assert celestrak_3le_sats.name == "tests/2025-04-15-celestrak.3le"
    assert space_track_tle_sats.name == "tests/2025-04-15-space-track.tle"
    assert celestrak_tle_sats.name == "tests/2025-04-15-celestrak.tle"


def test_get_ca_report_vs_many(ca_3le_sats: Constellation):
    start = Epoch.from_iso("2025-04-15T00:00:00.000000Z", TimeSystem.UTC)
    end = start + TimeSpan.from_minutes(5)
    report = ca_3le_sats.get_ca_report_vs_many(start, end, 1.0)
    expected = {
        "TANDEM-X": {"TERRASAR-X": 0.049},
        "SHIJIAN-6 05A (SJ-6 05A)": {"STARLINK-5893": 0.672},
        "STARLINK-4043": {"QB50P2": 0.902},
        "TERRASAR-X": {"TANDEM-X": 0.049},
        "STARLINK-5893": {"SHIJIAN-6 05A (SJ-6 05A)": 0.672},
        "QB50P2": {"STARLINK-4043": 0.902},
    }
    assert len(report.close_approaches) == 3
    for ca in report.close_approaches:
        primary_name = ca_3le_sats[ca.primary_id].name
        secondary_name = ca_3le_sats[ca.secondary_id].name
        distance = ca.distance
        assert primary_name in expected
        assert secondary_name in expected[primary_name]
        assert distance == pytest.approx(expected[primary_name][secondary_name], abs=1e-3)
