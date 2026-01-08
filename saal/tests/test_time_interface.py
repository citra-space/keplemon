import pytest

from pysaal import MainInterface, TimeInterface


def test_get_dll_info() -> None:
    ti = TimeInterface()
    assert MainInterface.DLL_VERSION in ti.info


def test_ymd_components_to_ds50() -> None:
    ti = TimeInterface()
    ds50 = ti.ymd_components_to_ds50(1956, 1, 1, 0, 0, 0.0)
    assert ds50 == 2192.0


def test_ds50_to_ymd_components() -> None:
    ti = TimeInterface()
    components = ti.ds50_to_ymd_components(2192.0)
    assert components == (1956, 1, 1, 0, 0, 0.0)


def test_dtg_to_ds50() -> None:
    ti = TimeInterface()
    ds50 = ti.dtg_to_ds50("1956/001 0000 00.000")
    assert ds50 == 2192.0


def test_ds50_to_dtg_formats() -> None:
    ti = TimeInterface()
    assert ti.ds50_to_dtg20(2192.0) == "1956/001 0000 00.000"
    assert ti.ds50_to_dtg19(2192.0) == "1956Jan01000000.000"
    assert ti.ds50_to_dtg17(2192.0) == "1956/001.00000000"
    assert ti.ds50_to_dtg15(2192.0) == "56001000000.000"


def test_year_doy_conversions() -> None:
    ti = TimeInterface()
    ds50 = ti.year_doy_to_ds50(1956, 1.0)
    assert ds50 == 2192.0
    year, doy = ti.ds50_to_year_doy(2192.0)
    assert (year, doy) == (1956, 1.0)


def test_constants_loaded() -> None:
    ti = TimeInterface()
    assert ti.constants_loaded


def test_conversions() -> None:
    ti = TimeInterface()
    utc = ti.ymd_components_to_ds50(1973, 1, 30, 0, 0, 0.0)
    tai = 8431.000138888889
    ut1 = 8431.00000830081
    tt = 8431.00051138889

    assert ti.utc_to_tai(utc) == pytest.approx(tai, abs=1.0e-10)
    assert ti.tai_to_utc(tai) == pytest.approx(utc, abs=1.0e-10)
    assert ti.utc_to_ut1(utc) == pytest.approx(ut1, abs=1.0e-10)
    assert ti.utc_to_tt(utc) == pytest.approx(tt, abs=1.0e-10)
    assert ti.tai_to_ut1(tai) == pytest.approx(ut1, abs=1.0e-10)


def test_greenwich_angles() -> None:
    ti = TimeInterface()
    utc = ti.ymd_components_to_ds50(1973, 1, 2, 0, 0, 0.0)
    ut1 = ti.utc_to_ut1(utc)
    fk4 = ti.get_fk4_greenwich_angle(ut1)
    fk5 = ti.get_fk5_greenwich_angle(ut1)

    assert fk4 == pytest.approx(1.7712987335192203, abs=1.0e-7)
    assert fk5 == pytest.approx(1.7713027012394775, abs=1.0e-7)
