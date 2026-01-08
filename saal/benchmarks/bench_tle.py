from __future__ import annotations

from pathlib import Path

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import TLEInterface

SGP_LINE_1 = "1 11111U 98067A   25363.54791667 +.00012345  10000-1  20000-1 0 0900"
SGP_LINE_2 = "2 11111  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345"
NULL_LINE_1 = "1 11111U          25363.54791667 +.00012345  00000 0  00000 0 0 0900"


@pytest.fixture(scope="module")
def tle_iface() -> TLEInterface:
    return TLEInterface()


@pytest.fixture(scope="module")
def sgp_arrays(tle_iface: TLEInterface) -> tuple[list[float], str]:
    xa_tle, xs_tle = tle_iface.lines_to_arrays(SGP_LINE_1, SGP_LINE_2)
    return list(xa_tle), xs_tle


@pytest.fixture(scope="module")
def celestrak_path() -> str:
    return str(Path(__file__).resolve().parents[1] / "tests" / "data" / "2025-12-30-celestrak.tle")


def test_bench_tle_info(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    benchmark(lambda: tle_iface.info)


def test_bench_tle_fix_blank_exponent_sign(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    benchmark(tle_iface.fix_blank_exponent_sign, NULL_LINE_1)


def test_bench_tle_add_check_sums(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    benchmark(tle_iface.add_check_sums, SGP_LINE_1, SGP_LINE_2)


def test_bench_tle_lines_to_arrays(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    benchmark(tle_iface.lines_to_arrays, SGP_LINE_1, SGP_LINE_2)


def test_bench_tle_arrays_to_lines(
    benchmark: BenchmarkFixture, tle_iface: TLEInterface, sgp_arrays: tuple[list[float], str]
) -> None:
    xa_tle, xs_tle = sgp_arrays
    benchmark(tle_iface.arrays_to_lines, xa_tle, xs_tle)


def test_bench_tle_get_check_sums(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    benchmark(tle_iface.get_check_sums, SGP_LINE_1, SGP_LINE_2)


def test_bench_tle_load_lines(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    def run() -> None:
        key = tle_iface.load_lines(SGP_LINE_1, SGP_LINE_2)
        tle_iface.remove(key)

    benchmark(run)


def test_bench_tle_load_arrays(
    benchmark: BenchmarkFixture, tle_iface: TLEInterface, sgp_arrays: tuple[list[float], str]
) -> None:
    xa_tle, xs_tle = sgp_arrays

    def run() -> None:
        key = tle_iface.load_arrays(xa_tle, xs_tle)
        tle_iface.remove(key)

    benchmark(run)


def test_bench_tle_remove_key(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    def run() -> None:
        key = tle_iface.load_lines(SGP_LINE_1, SGP_LINE_2)
        tle_iface.remove(key)

    benchmark(run)


def test_bench_tle_remove_all(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    def run() -> None:
        _ = tle_iface.load_lines(SGP_LINE_1, SGP_LINE_2)
        tle_iface.clear()

    benchmark(run)


def test_bench_tle_get_count(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    def run() -> int:
        key = tle_iface.load_lines(SGP_LINE_1, SGP_LINE_2)
        count = tle_iface.get_count()
        tle_iface.remove(key)
        return count

    benchmark(run)


def test_bench_tle_get_keys(benchmark: BenchmarkFixture, tle_iface: TLEInterface) -> None:
    def run() -> list[int]:
        key = tle_iface.load_lines(SGP_LINE_1, SGP_LINE_2)
        keys = tle_iface.get_keys(0)
        tle_iface.remove(key)
        return keys

    benchmark(run)


def test_bench_tle_load_file(benchmark: BenchmarkFixture, tle_iface: TLEInterface, celestrak_path: str) -> None:
    def run() -> int:
        count = tle_iface.load_file(celestrak_path)
        tle_iface.clear()
        return count

    benchmark(run)
