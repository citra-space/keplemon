from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import EnvironmentInterface


@pytest.fixture(scope="module")
def environment_iface() -> EnvironmentInterface:
    return EnvironmentInterface()


def test_bench_environment_info(
    benchmark: BenchmarkFixture, environment_iface: EnvironmentInterface
) -> None:
    benchmark(lambda: environment_iface.info)


def test_bench_environment_earth_radius(
    benchmark: BenchmarkFixture, environment_iface: EnvironmentInterface
) -> None:
    benchmark(lambda: environment_iface.earth_radius)


def test_bench_environment_fundamental_catalog(
    benchmark: BenchmarkFixture, environment_iface: EnvironmentInterface
) -> None:
    benchmark(lambda: environment_iface.fundamental_catalog)


def test_bench_environment_set_fundamental_catalog(
    benchmark: BenchmarkFixture, environment_iface: EnvironmentInterface
) -> None:
    def run() -> None:
        environment_iface.fundamental_catalog = 4
        environment_iface.fundamental_catalog = 5

    benchmark(run)
