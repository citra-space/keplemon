import pytest
from keplemon.bodies import Satellite, Constellation, Earth
from keplemon.catalogs import TLECatalog
from keplemon.elements import TLE
from keplemon.time import Epoch
from keplemon.enums import TimeSystem


def test_earth():
    assert Earth.get_equatorial_radius() == 6378.135


def test_constellation():
    celestrak_tles = Constellation.from_tle_catalog(TLECatalog.from_tle_file("tests/2025-04-15-celestrak.tle"))

    space_track_tles = Constellation.from_tle_catalog(TLECatalog.from_tle_file("tests/2025-04-15-space-track.tle"))
    space_track_3les = Constellation.from_tle_catalog(TLECatalog.from_tle_file("tests/2025-04-15-space-track.3le"))
    celestrak_3les = Constellation.from_tle_catalog(TLECatalog.from_tle_file("tests/2025-04-15-celestrak.3le"))

    assert space_track_3les.count == 27485
    assert celestrak_3les.count == 11304
    assert space_track_tles.count == 27485
    assert celestrak_tles.count == 11305

    assert space_track_3les.name == "tests/2025-04-15-space-track.3le"
    assert celestrak_3les.name == "tests/2025-04-15-celestrak.3le"
    assert space_track_tles.name == "tests/2025-04-15-space-track.tle"
    assert celestrak_tles.name == "tests/2025-04-15-celestrak.tle"


def test_satellite():
    line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999"
    line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660"
    tle = TLE.from_lines(line_1, line_2)

    sat = Satellite.from_tle(tle)
    assert sat.norad_id == 25544

    line_1 = "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990"
    line_2 = "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950"
    tle = TLE.from_lines(line_1, line_2)
    sat_1 = Satellite.from_tle(tle)

    line_1 = "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990"
    line_2 = "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950"
    tle = TLE.from_lines(line_1, line_2)
    sat_2 = Satellite.from_tle(tle)

    start = Epoch.from_iso("2025-04-15T12:00:00.000000Z", TimeSystem.UTC)
    end = Epoch.from_iso("2025-04-16T12:00:00.000000Z", TimeSystem.UTC)
    ca = sat_1.get_close_approach(sat_2, start, end, 25.0)
    assert ca
    assert ca.epoch.to_iso() == "2025-04-15T12:32:28.531"
    assert ca.distance == pytest.approx(6.088, abs=0.1)
