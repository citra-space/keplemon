from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import SensorInterface

SENSOR_CARD = "211  3381724 -25333969 -1521161 -5083089  3530462  U SOCORRO CAM1              S"
NOISE_CARD = "211 5   0.0003 0.0003 0.0000 0.0000  -0.0005 -0.0003  0.0000  0.0000  0.0000  BS"


@pytest.fixture(scope="module")
def sensor_iface() -> SensorInterface:
    return SensorInterface()


def test_bench_sensor_info(benchmark: BenchmarkFixture, sensor_iface: SensorInterface) -> None:
    benchmark(lambda: sensor_iface.info)


def test_bench_sensor_load_card(benchmark: BenchmarkFixture, sensor_iface: SensorInterface) -> None:
    def run() -> None:
        sensor_iface.load_card(SENSOR_CARD)
        sensor_iface.load_card(NOISE_CARD)
        sensor_iface.clear()

    benchmark(run)


def test_bench_sensor_get_arrays(benchmark: BenchmarkFixture, sensor_iface: SensorInterface) -> None:
    def run() -> None:
        sensor_iface.load_card(SENSOR_CARD)
        sensor_iface.load_card(NOISE_CARD)
        keys = sensor_iface.get_keys(2)
        sensor_iface.get_arrays(keys[-1])
        sensor_iface.clear()

    benchmark(run)
