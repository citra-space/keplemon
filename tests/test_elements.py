import pytest

from keplemon.elements import (
    TLE,
    KeplerianState,
    KeplerianElements,
    HorizonElements,
    HorizonState,
    TopocentricState,
    TopocentricElements,
)
from keplemon.enums import Classification, KeplerianType, ReferenceFrame, TimeSystem
from keplemon.time import Epoch
from keplemon.bodies import Observatory


@pytest.fixture()
def observatory():
    return Observatory(latitude=38.926021, longitude=-104.826633, altitude=1.94)


@pytest.fixture()
def angles_only_topocentric_state():
    epoch = Epoch.from_iso("2025-11-17T00:28:37.486761Z", TimeSystem.UTC)
    elements = TopocentricElements.from_j2000(epoch, 330.625950, -6.337688)
    return TopocentricState(epoch=epoch, elements=elements)


@pytest.fixture()
def angles_only_horizon_state(angles_only_topocentric_state: TopocentricState):
    epoch = angles_only_topocentric_state.epoch
    elements = HorizonElements(163.02889043353866, 43.44139004143117)
    return HorizonState(epoch=epoch, elements=elements)


class TestHorizonState:

    def test_from_topocentric_state(
        self,
        observatory: Observatory,
        angles_only_topocentric_state: TopocentricState,
        angles_only_horizon_state: HorizonState,
    ):

        horizon = HorizonState.from_topocentric_state(angles_only_topocentric_state, observatory)
        assert horizon.elevation == pytest.approx(angles_only_horizon_state.elevation, abs=1e-6)
        assert horizon.azimuth == pytest.approx(angles_only_horizon_state.azimuth, abs=1e-6)


class TestTopocentricState:

    def test_from_horizon_state(
        self,
        observatory: Observatory,
        angles_only_horizon_state: HorizonState,
        angles_only_topocentric_state: TopocentricState,
    ):
        topocentric = TopocentricState.from_horizon_state(angles_only_horizon_state, observatory)
        assert topocentric.right_ascension == pytest.approx(angles_only_topocentric_state.right_ascension, abs=1e-6)
        assert topocentric.declination == pytest.approx(angles_only_topocentric_state.declination, abs=1e-6)


def test_tle():
    line_1 = "1 25544U 98067A   21275.12345678 +.00001234  00000+0  12345-6 0 00006"
    line_2 = "2 25544  51.6456 123.4567 0001234  12.3456  78.9012 15.12345678000007"
    tle = TLE.from_lines(line_1, line_2)

    assert tle.norad_id == 25544
    assert tle.designator == "98067A"
    assert tle.classification == Classification.Unclassified
    assert tle.type == KeplerianType.MeanKozaiGP
    assert tle.lines == (line_1, line_2)


def test_keplerian_state():
    elements = KeplerianElements(
        semi_major_axis=7000.0,
        eccentricity=0.001,
        inclination=0.5,
        raan=1.0,
        argument_of_perigee=0.2,
        mean_anomaly=0.1,
    )
    state = KeplerianState(
        epoch=Epoch.from_iso("2025-04-02T04:02:42.420", TimeSystem.UTC),
        elements=elements,
        frame=ReferenceFrame.J2000,
        keplerian_type=KeplerianType.Osculating,
    )
    assert state.frame == ReferenceFrame.J2000
