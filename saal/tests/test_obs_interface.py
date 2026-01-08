import threading
from typing import Generator

import pytest

from pysaal import MainInterface, ObsInterface, ParsedB3, TimeInterface

LOCK = threading.RLock()


@pytest.fixture()
def obs() -> Generator[ObsInterface, None, None]:
    with LOCK:
        interface = ObsInterface()
        yield interface


@pytest.fixture()
def time_interface() -> Generator[TimeInterface, None, None]:
    with LOCK:
        interface = TimeInterface()
        yield interface


def base_parsed_b3(equinox: int) -> ParsedB3:
    obs = ParsedB3()
    obs.classification = "U"
    obs.norad_id = 11111
    obs.sensor_number = 500
    obs.epoch = 25934.75
    obs.declination = -20.6075648583427
    obs.right_ascension = 57.6850704027472
    obs.range = 28002.6701345644
    obs.range_rate = None
    obs.azimuth = None
    obs.elevation = None
    obs.elevation_rate = None
    obs.azimuth_rate = None
    obs.year_of_equinox = equinox
    obs.range_acceleration = None
    obs.observation_type = 9
    obs.track_position = 5
    obs.association_status = 1
    obs.site_tag = 11111
    obs.spadoc_tag = 11111
    obs.position = [0.0, 0.0, 0.0]
    return obs


def test_get_dll_info(obs: ObsInterface) -> None:
    assert MainInterface.DLL_VERSION in obs.info


def test_parsed_b3_get_line_year_of_equinox_indicator(obs: ObsInterface) -> None:
    cases = [
        (
            0,
            "U1111150021001180000000K06076 0350444                                     9 5  11111111111",
        ),
        (
            1,
            "U1111150021001180000000K06076 0350444                                     915  11111111111",
        ),
        (
            2,
            "U1111150021001180000000K06076 0350444                                     925  11111111111",
        ),
        (
            3,
            "U1111150021001180000000K06076 0350444                                     935  11111111111",
        ),
    ]

    for equinox, expected in cases:
        parsed = base_parsed_b3(equinox)
        assert parsed.get_line() == expected


def test_parsed_b3_from_line_matches_fields(
    obs: ObsInterface,
    time_interface: TimeInterface,
) -> None:
    b3_card = "U0001151013352142520112J85202 2220398         -01207880+03706326+05814970 9 4  10001100011"
    parsed = obs.parse_line(b3_card)

    days = 352.0 + 14.0 / 24.0 + 25.0 / (60.0 * 24.0) + 20.112 / (60.0 * 60.0 * 24.0)
    right_ascen = (22.0 / 24.0 + 20.0 / (60.0 * 24.0) + 39.8 / (60.0 * 60.0 * 24.0)) * 360.0
    obs_time = time_interface.year_doy_to_ds50(2013, days)

    assert parsed.classification == "U"
    assert parsed.norad_id == 11
    assert parsed.sensor_number == 510
    assert parsed.observation_type == 9
    assert parsed.track_position == 4
    assert parsed.association_status == 1
    assert parsed.site_tag == 11
    assert parsed.spadoc_tag == 11
    assert parsed.epoch == pytest.approx(obs_time, rel=0.0, abs=1.0e-7)
    assert parsed.declination == pytest.approx(-18.5202, rel=0.0, abs=1.0e-7)
    assert parsed.right_ascension == pytest.approx(right_ascen, rel=0.0, abs=1.0e-7)
    assert parsed.range == pytest.approx(0.0, rel=0.0, abs=1.0e-7)
    assert parsed.range_rate is None
    assert parsed.elevation_rate is None
    assert parsed.azimuth_rate is None
    assert parsed.range_acceleration is None
    assert parsed.position
    assert parsed.position[0] == pytest.approx(-1207.88, rel=0.0, abs=1.0e-7)
    assert parsed.position[1] == pytest.approx(3706.326, rel=0.0, abs=1.0e-7)
    assert parsed.position[2] == pytest.approx(5814.97, rel=0.0, abs=1.0e-7)
