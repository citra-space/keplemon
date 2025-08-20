from keplemon.elements import TLE, KeplerianState, KeplerianElements
from keplemon.enums import Classification, KeplerianType, ReferenceFrame, TimeSystem
from keplemon.time import Epoch


def test_tle():
    line_1 = "1 25544U 98067A   21275.12345678 +.00001234  00000 0  12345-6 0 0000"
    line_2 = "2 25544  51.6456 123.4567 0001234  12.3456  78.9012 15.1234567800000"
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
