from __future__ import annotations

from importlib import resources

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import TimeInterface


@pytest.fixture(scope="module")
def time_iface() -> TimeInterface:
    return TimeInterface()


@pytest.fixture(scope="module")
def time_constants_path() -> str:
    return str(resources.files("saal").joinpath("assets", "time_constants.dat"))


def test_bench_time_info(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(lambda: time_iface.info)


def test_bench_time_ymd_components_to_ds50(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.ymd_components_to_ds50, 1973, 1, 1, 0, 0, 0.0)


def test_bench_time_utc_to_ut1(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.utc_to_ut1, 8431.0)


def test_bench_time_utc_to_tai(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.utc_to_tai, 8431.0)


def test_bench_time_tai_to_utc(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.tai_to_utc, 8431.0)


def test_bench_time_utc_to_tt(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.utc_to_tt, 8431.0)


def test_bench_time_tai_to_ut1(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.tai_to_ut1, 8431.0)


def test_bench_time_ds50_to_ymd_components(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.ds50_to_ymd_components, 8431.0)


def test_bench_time_dtg_to_ds50(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.dtg_to_ds50, "1973/001 0000 00.000")


def test_bench_time_ds50_to_dtg20(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.ds50_to_dtg20, 8431.0)


def test_bench_time_ds50_to_dtg19(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.ds50_to_dtg19, 8431.0)


def test_bench_time_ds50_to_dtg17(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.ds50_to_dtg17, 8431.0)


def test_bench_time_ds50_to_dtg15(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.ds50_to_dtg15, 8431.0)


def test_bench_time_year_doy_to_ds50(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.year_doy_to_ds50, 1973, 1.0)


def test_bench_time_ds50_to_year_doy(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.ds50_to_year_doy, 8431.0)


def test_bench_time_load_constants(
    benchmark: BenchmarkFixture, time_iface: TimeInterface, time_constants_path: str
) -> None:
    benchmark(time_iface.load_constants, time_constants_path)


def test_bench_time_load_time_constants(
    benchmark: BenchmarkFixture, time_iface: TimeInterface, time_constants_path: str
) -> None:
    benchmark(time_iface.load_time_constants, time_constants_path)


def test_bench_time_get_fk4_greenwich_angle(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.get_fk4_greenwich_angle, 8431.0)


def test_bench_time_get_fk5_greenwich_angle(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.get_fk5_greenwich_angle, 8431.0)


def test_bench_time_constants_loaded(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(lambda: time_iface.constants_loaded)


def test_bench_time_time_constants_loaded(benchmark: BenchmarkFixture, time_iface: TimeInterface) -> None:
    benchmark(time_iface.time_constants_loaded)
