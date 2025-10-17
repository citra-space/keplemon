import pytest
from keplemon.bodies import Satellite, Constellation, Earth, Observatory
from keplemon.catalogs import TLECatalog
from keplemon.elements import TLE, TopocentricElements
from keplemon.time import Epoch, TimeSpan
from keplemon.enums import TimeSystem, ReferenceFrame


def test_observatory():
    site = Observatory(0, 0, 0)
    sats = Constellation.from_tle_catalog(TLECatalog.from_tle_file("tests/2025-04-15-celestrak.tle"))
    epoch = Epoch.from_iso("2025-04-15T12:00:00.000000Z", TimeSystem.UTC)
    fov_report = site.get_field_of_view_report(epoch, TopocentricElements(0, 0), 10.0, sats, ReferenceFrame.TEME)
    assert len(fov_report.candidates) == 18
    candidate = fov_report.candidates[0]
    topo = site.get_topocentric_to_satellite(epoch, sats[candidate.satellite_id], ReferenceFrame.TEME)
    assert topo.right_ascension == candidate.direction.right_ascension


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
    assert sat_1.geodetic_position is not None
    assert sat_2.geodetic_position is not None
    assert sat_1.geodetic_position.latitude == pytest.approx(0.3938497796549098, abs=0.1)
    assert sat_1.geodetic_position.longitude == pytest.approx(55.074384090833696, abs=0.1)
    assert sat_1.geodetic_position.altitude == pytest.approx(35808.08113476326, abs=0.1)


def test_satellite_observatory_access_report():
    line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999"
    line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660"
    tle = TLE.from_lines("ISS", line_1, line_2)
    satellite = Satellite.from_tle(tle)

    obs1 = Observatory(latitude=34.0, longitude=-118.0, altitude=100.0)
    obs1.name = "LA Observatory"
    obs2 = Observatory(latitude=51.5, longitude=-0.1, altitude=50.0)
    obs2.name = "London Observatory"
    obs3 = Observatory(latitude=-33.9, longitude=18.4, altitude=20.0)
    obs3.name = "Cape Town Observatory"

    observatories = [obs1, obs2, obs3]

    # Define time range (4 hours)
    start = Epoch.from_iso("2025-04-18T04:00:00.000000Z", TimeSystem.UTC)
    end = Epoch.from_iso("2025-04-18T08:00:00.000000Z", TimeSystem.UTC)

    # Set parameters
    min_elevation = 10.0  # degrees
    min_duration = TimeSpan.from_minutes(1.0)

    # Get the observatory access report
    report = satellite.get_observatory_access_report(observatories, start, end, min_elevation, min_duration)

    # Verify report was generated
    assert report is not None
    assert report.start == start
    assert report.end == end
    assert report.elevation_threshold == min_elevation
    assert report.duration_threshold.in_minutes() == pytest.approx(min_duration.in_minutes())

    # Verify we get exactly 3 accesses for this specific TLE and time range
    assert len(report.accesses) == 3

    # Count accesses per observatory
    la_accesses = [a for a in report.accesses if a.observatory_id == obs1.id]
    london_accesses = [a for a in report.accesses if a.observatory_id == obs2.id]
    cape_town_accesses = [a for a in report.accesses if a.observatory_id == obs3.id]

    # Verify specific counts per observatory
    assert len(la_accesses) == 1, f"Expected 1 LA access, got {len(la_accesses)}"
    assert len(london_accesses) == 0, f"Expected 0 London accesses, got {len(london_accesses)}"
    assert len(cape_town_accesses) == 2, f"Expected 2 Cape Town accesses, got {len(cape_town_accesses)}"

    # Verify access properties
    for access in report.accesses:
        assert access.observatory_id is not None
        assert access.satellite_id is not None
        assert access.start is not None
        assert access.end is not None
        # Verify elevation meets or is approximately equal to minimum (within tolerance)
        assert (
            access.start.elevation >= min_elevation or pytest.approx(access.start.elevation, abs=0.1) == min_elevation
        )
        assert access.end.elevation >= min_elevation or pytest.approx(access.end.elevation, abs=0.1) == min_elevation

        # Verify duration meets minimum
        duration = access.end.epoch - access.start.epoch
        assert (
            duration.in_minutes() >= min_duration.in_minutes()
            or pytest.approx(duration.in_minutes(), abs=0.1) == min_duration.in_minutes()
        )
