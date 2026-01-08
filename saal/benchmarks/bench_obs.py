from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import ObsInterface, ParsedB3

B3_CARD = "U0001151013352142520112J85202 2220398         -01207880+03706326+05814970 9 4  10001100011"


@pytest.fixture(scope="module")
def obs_iface() -> ObsInterface:
    return ObsInterface()


@pytest.fixture(scope="module")
def parsed_b3() -> ParsedB3:
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
    obs.year_of_equinox = 0
    obs.range_acceleration = None
    obs.observation_type = 9
    obs.track_position = 5
    obs.association_status = 1
    obs.site_tag = 11111
    obs.spadoc_tag = 11111
    obs.position = [0.0, 0.0, 0.0]
    return obs


def test_bench_obs_info(benchmark: BenchmarkFixture, obs_iface: ObsInterface) -> None:
    benchmark(lambda: obs_iface.info)


def test_bench_obs_parse_line(benchmark: BenchmarkFixture, obs_iface: ObsInterface) -> None:
    benchmark(obs_iface.parse_line, B3_CARD)


def test_bench_obs_get_line(benchmark: BenchmarkFixture, parsed_b3: ParsedB3) -> None:
    benchmark(parsed_b3.get_line)
