from keplemon.bodies import Constellation, Observatory
from keplemon.catalogs import TLECatalog
from keplemon.elements import TopocentricElements
from keplemon.time import Epoch
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
