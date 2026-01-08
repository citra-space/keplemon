from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import MainInterface


@pytest.fixture(scope="module")
def main_iface() -> MainInterface:
    return MainInterface()


def test_bench_main_info(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    benchmark(lambda: main_iface.info)


def test_bench_main_get_key_mode(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    benchmark(lambda: main_iface.key_mode)


def test_bench_main_set_duplicate_key_mode(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    def run() -> None:
        main_iface.duplicate_key_mode = 1
        main_iface.duplicate_key_mode = 0
        main_iface.reset_key_mode()

    benchmark(run)


def test_bench_main_set_key_mode(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    def run() -> None:
        main_iface.key_mode = 0
        main_iface.key_mode = 1
        main_iface.reset_key_mode()

    benchmark(run)
