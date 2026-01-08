import pytest

from pysaal import AstroInterface, MainInterface


def test_get_dll_info() -> None:
    interface = AstroInterface()
    assert MainInterface.DLL_VERSION in interface.info


def test_keplerian_to_equinoctial() -> None:
    interface = AstroInterface()
    kep = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612]
    eqnx = interface.keplerian_to_equinoctial(kep)

    assert eqnx[0] == pytest.approx(0.005756008409, abs=1.0e-12)
    assert eqnx[1] == pytest.approx(0.002453246053, abs=1.0e-12)
    assert eqnx[2] == pytest.approx(0.130405060328, abs=1.0e-12)
    assert eqnx[3] == pytest.approx(-0.503224317374, abs=1.0e-12)
    assert eqnx[4] == pytest.approx(617.8480000000, abs=1.0e-12)
    assert eqnx[5] == pytest.approx(2.005848298418, abs=1.0e-12)


def test_equinoctial_to_keplerian() -> None:
    interface = AstroInterface()
    eqnx = [
        0.005756008409,
        0.002453246053,
        0.130405060328,
        -0.503224317374,
        617.8480000,
        2.005848298418,
    ]
    kep = interface.equinoctial_to_keplerian(eqnx)

    assert kep[0] == pytest.approx(26558.4820, abs=1.0e-4)
    assert kep[1] == pytest.approx(0.0062570000, abs=1.0e-7)
    assert kep[2] == pytest.approx(54.9350000, abs=1.0e-7)
    assert kep[3] == pytest.approx(234.7640000, abs=1.0e-7)
    assert kep[4] == pytest.approx(165.4720000, abs=1.0e-7)
    assert kep[5] == pytest.approx(217.6120000, abs=1.0e-7)


def test_keplerian_to_cartesian() -> None:
    interface = AstroInterface()
    kep = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612]
    posvel = interface.keplerian_to_cartesian(kep)

    assert posvel[0] == pytest.approx(-3032.21272487, abs=1.0e-7)
    assert posvel[1] == pytest.approx(-15025.7763831, abs=1.0e-7)
    assert posvel[2] == pytest.approx(21806.4954366, abs=1.0e-7)
    assert posvel[3] == pytest.approx(3.754350020203, abs=1.0e-7)
    assert posvel[4] == pytest.approx(-0.889562019024, abs=1.0e-7)
    assert posvel[5] == pytest.approx(-0.114933710268, abs=1.0e-7)


def test_cartesian_to_keplerian() -> None:
    interface = AstroInterface()
    posvel = [
        -3032.21272487,
        -15025.7763831,
        21806.4954366,
        3.7543500202,
        -0.889562019026,
        -0.114933710268,
    ]
    kep = interface.cartesian_to_keplerian(posvel)

    assert kep[0] == pytest.approx(26558.4820, abs=1.0e-4)
    assert kep[1] == pytest.approx(0.0062570000, abs=1.0e-7)
    assert kep[2] == pytest.approx(54.9350000, abs=1.0e-7)
    assert kep[3] == pytest.approx(234.7640000, abs=1.0e-7)
    assert kep[4] == pytest.approx(165.4720000, abs=1.0e-7)
    assert kep[5] == pytest.approx(217.6120000, abs=1.0e-7)


def test_mean_motion_conversions() -> None:
    interface = AstroInterface()
    mean_motion = 1.0027382962
    semi_major_axis = interface.mean_motion_to_sma(mean_motion)
    assert semi_major_axis == pytest.approx(42164.171420, abs=1.0e-6)

    mean_motion_back = interface.sma_to_mean_motion(42164.17142)
    assert mean_motion_back == pytest.approx(mean_motion, abs=1.0e-7)


def test_kozai_brouwer_conversions() -> None:
    interface = AstroInterface()
    kozai = 14.2024103100000
    brouwer = 14.2107268431215
    ecc = 1.112700000000000e-02
    inc = 99.4371000000000

    to_brouwer = interface.kozai_to_brouwer(ecc, inc, kozai)
    assert to_brouwer == pytest.approx(brouwer, abs=1.0e-7)

    to_kozai = interface.brouwer_to_kozai(ecc, inc, brouwer)
    assert to_kozai == pytest.approx(kozai, abs=1.0e-7)


def test_osculating_to_mean() -> None:
    interface = AstroInterface()
    osc = [7200.0, 0.006257, 54.935, 234.764, 165.472, 217.612]
    mean = interface.osculating_to_mean(osc)

    assert mean[0] == pytest.approx(7206.06814087, abs=1.0e-7)
    assert mean[1] == pytest.approx(0.00646986051778, abs=1.0e-7)
    assert mean[2] == pytest.approx(54.9518948032, abs=1.0e-7)
    assert mean[3] == pytest.approx(232.98566416, abs=1.0e-7)
    assert mean[4] == pytest.approx(165.473342418, abs=1.0e-7)
    assert mean[5] == pytest.approx(219.392452222, abs=1.0e-7)
