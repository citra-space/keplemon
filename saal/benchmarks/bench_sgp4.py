from __future__ import annotations

import threading
from collections.abc import Generator
from pathlib import Path

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import SGP4Interface, TLEInterface

SGP4_LINE_1 = "1 22222C 15058A   25363.54791667 +.00012345  10000-1  20000-1 2 0900"
SGP4_LINE_2 = "2 22222  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345"
XP_LINE_1 = "1 33333U 15058A   25363.54791667 +.00012345  10000-1  20000-1 4  900"
XP_LINE_2 = "2 33333  30.0000  40.0000 0005000  60.0000  70.0000  8.2345678012345"
EPOCH = 27757.54791667
SGP4_X = -33722.20240953347
SGP4_Y = 3451.0939430966114
SGP4_Z = 14050.115953447255
SGP4_VX = -0.7534699464839536
SGP4_VY = -3.0289958981453213
SGP4_VZ = -1.0597673984106273
XP_X = -9515.23633738959
XP_Y = 975.4110099884765
XP_Z = 3961.4106652075125
XP_VX = -1.4191743539251394
XP_VY = -5.702875678562367
XP_VZ = -1.9961866743072767

LOCK = threading.RLock()


@pytest.fixture(scope="module")
def tle_iface() -> TLEInterface:
    return TLEInterface()


@pytest.fixture(scope="module")
def sgp4_iface() -> SGP4Interface:
    return SGP4Interface()


@pytest.fixture(scope="module")
def asset_dir() -> str:
    return str(Path(__file__).resolve().parents[1] / "assets")


@pytest.fixture()
def sgp4_keys(tle_iface: TLEInterface, sgp4_iface: SGP4Interface) -> Generator[tuple[int, int], None, None]:
    with LOCK:
        sgp4_key = tle_iface.load_lines(SGP4_LINE_1, SGP4_LINE_2)
        xp_key = tle_iface.load_lines(XP_LINE_1, XP_LINE_2)
        sgp4_iface.load(sgp4_key)
        sgp4_iface.load(xp_key)
    yield sgp4_key, xp_key
    with LOCK:
        sgp4_iface.clear()
        tle_iface.clear()


def test_bench_sgp4_info(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface) -> None:
    benchmark(lambda: sgp4_iface.info)


def test_bench_sgp4_get_position_velocity_lla(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    sgp4_key, _ = sgp4_keys
    benchmark(sgp4_iface.get_position_velocity_lla, sgp4_key, EPOCH)


def test_bench_sgp4_get_position_velocity(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    sgp4_key, _ = sgp4_keys
    benchmark(sgp4_iface.get_position_velocity, sgp4_key, EPOCH)


def test_bench_sgp4_get_position(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    sgp4_key, _ = sgp4_keys
    benchmark(sgp4_iface.get_position, sgp4_key, EPOCH)


def test_bench_sgp4_get_lla(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]) -> None:
    sgp4_key, _ = sgp4_keys
    benchmark(sgp4_iface.get_lla, sgp4_key, EPOCH)


def test_bench_sgp4_get_full_state(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    sgp4_key, _ = sgp4_keys
    benchmark(sgp4_iface.get_full_state, sgp4_key, EPOCH)


def test_bench_sgp4_get_equinoctial(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    _, xp_key = sgp4_keys
    benchmark(sgp4_iface.get_equinoctial, xp_key, EPOCH)


def test_bench_sgp4_get_ephemeris(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    sgp4_key, _ = sgp4_keys
    frame = 1
    start = EPOCH - 1.0
    stop = EPOCH
    step = 5.0
    benchmark(sgp4_iface.get_ephemeris, sgp4_key, start, stop, step, frame)


def test_bench_sgp4_array_to_ephemeris(
    benchmark: BenchmarkFixture,
    sgp4_iface: SGP4Interface,
    tle_iface: TLEInterface,
    sgp4_keys: tuple[int, int],
) -> None:
    sgp4_key, _ = sgp4_keys
    xa_tle, _ = tle_iface.get_arrays(sgp4_key)
    frame = 1
    start = EPOCH - 1.0
    stop = EPOCH
    step = 5.0
    benchmark(sgp4_iface.array_to_ephemeris, xa_tle, start, stop, step, frame)


def test_bench_sgp4_fit_sgp4_array(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface) -> None:
    posvel = [SGP4_X, SGP4_Y, SGP4_Z, SGP4_VX, SGP4_VY, SGP4_VZ]
    benchmark(sgp4_iface.fit_sgp4_array, EPOCH, posvel, 0.02)


def test_bench_sgp4_fit_xp_array(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface) -> None:
    posvel = [XP_X, XP_Y, XP_Z, XP_VX, XP_VY, XP_VZ]
    benchmark(sgp4_iface.fit_xp_array, EPOCH, posvel, 0.02, 0.01)


def test_bench_sgp4_get_positions_velocities(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    sgp4_key, xp_key = sgp4_keys
    benchmark(sgp4_iface.get_positions_velocities, [sgp4_key, xp_key], EPOCH)


def test_bench_sgp4_get_license_directory(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface) -> None:
    benchmark(sgp4_iface.get_license_directory)


def test_bench_sgp4_set_license_directory(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, asset_dir: str
) -> None:
    benchmark(sgp4_iface.set_license_directory, asset_dir)


def test_bench_sgp4_reepoch_tle(
    benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, sgp4_keys: tuple[int, int]
) -> None:
    sgp4_key, _ = sgp4_keys
    benchmark(sgp4_iface.reepoch_tle, sgp4_key, EPOCH)


def test_bench_sgp4_load(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, tle_iface: TLEInterface) -> None:
    def run() -> None:
        with LOCK:
            key = tle_iface.load_lines(SGP4_LINE_1, SGP4_LINE_2)
            sgp4_iface.load(key)
            sgp4_iface.remove(key)
            tle_iface.remove(key)

    benchmark(run)


def test_bench_sgp4_remove(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, tle_iface: TLEInterface) -> None:
    def run() -> None:
        with LOCK:
            key = tle_iface.load_lines(SGP4_LINE_1, SGP4_LINE_2)
            sgp4_iface.load(key)
            sgp4_iface.remove(key)
            tle_iface.remove(key)

    benchmark(run)


def test_bench_sgp4_clear(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, tle_iface: TLEInterface) -> None:
    def run() -> None:
        with LOCK:
            key = tle_iface.load_lines(SGP4_LINE_1, SGP4_LINE_2)
            sgp4_iface.load(key)
            sgp4_iface.clear()
            tle_iface.remove(key)

    benchmark(run)


def test_bench_sgp4_get_count(benchmark: BenchmarkFixture, sgp4_iface: SGP4Interface, tle_iface: TLEInterface) -> None:
    def run() -> int:
        with LOCK:
            key = tle_iface.load_lines(SGP4_LINE_1, SGP4_LINE_2)
            sgp4_iface.load(key)
            count = sgp4_iface.get_count()
            sgp4_iface.clear()
            tle_iface.remove(key)
            return count

    benchmark(run)
