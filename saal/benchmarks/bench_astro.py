from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import AstroInterface

KEP = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612]
EQNX = [
    0.005756008409,
    0.002453246053,
    0.130405060328,
    -0.503224317374,
    617.8480000,
    2.005848298418,
]
POSVEL = [
    -3032.21272487,
    -15025.7763831,
    21806.4954366,
    3.7543500202,
    -0.889562019026,
    -0.114933710268,
]
OSC = [7200.0, 0.006257, 54.935, 234.764, 165.472, 217.612]
TEME_POS = [-3032.21272487, -15025.7763831, 21806.4954366]
EFG_POS = [6524.834, 6862.875, 6448.296]


@pytest.fixture(scope="module")
def astro_iface() -> AstroInterface:
    return AstroInterface()


def test_bench_astro_info(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(lambda: astro_iface.info)


def test_bench_keplerian_to_equinoctial(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.keplerian_to_equinoctial, KEP)


def test_bench_equinoctial_to_keplerian(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.equinoctial_to_keplerian, EQNX)


def test_bench_keplerian_to_cartesian(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.keplerian_to_cartesian, KEP)


def test_bench_cartesian_to_keplerian(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.cartesian_to_keplerian, POSVEL)


def test_bench_mean_motion_to_sma(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.mean_motion_to_sma, 1.0027382962)


def test_bench_sma_to_mean_motion(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.sma_to_mean_motion, 42164.17142)


def test_bench_kozai_to_brouwer(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.kozai_to_brouwer, 0.011127, 99.4371, 14.20241)


def test_bench_brouwer_to_kozai(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.brouwer_to_kozai, 0.011127, 99.4371, 14.210726)


def test_bench_osculating_to_mean(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.osculating_to_mean, OSC)


def test_bench_gst_teme_to_lla(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.gst_teme_to_lla, 1.23, TEME_POS)


def test_bench_efg_to_lla(benchmark: BenchmarkFixture, astro_iface: AstroInterface) -> None:
    benchmark(astro_iface.efg_to_lla, EFG_POS)
