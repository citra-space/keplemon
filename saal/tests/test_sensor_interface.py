import threading
from typing import Generator

import pytest

from pysaal import MainInterface, SensorInterface

LOCK = threading.RLock()

SENSOR_CARD = "211  3381724 -25333969 -1521161 -5083089  3530462  U SOCORRO CAM1              S"
NOISE_CARD = "211 5   0.0003 0.0003 0.0000 0.0000  -0.0005 -0.0003  0.0000  0.0000  0.0000  BS"

XA_SEN_GEN_SENNUM = 0
XA_SEN_GRN_POS1 = 11
XA_SEN_GRN_POS2 = 12
XA_SEN_GRN_POS3 = 13
XA_SEN_GRN_ASTROLAT = 14
XA_SEN_GRN_ASTROLON = 15
XA_SEN_GRN_ECITIME = 16
XA_SEN_GEN_RNGLIMFLG = 6
XA_SEN_GEN_SMSEN = 7
XA_SEN_GEN_MINRNG = 3
XA_SEN_GEN_MAXRNG = 4
XA_SEN_GEN_RRLIM = 5
XA_SEN_GRN_LOCTYPE = 10
XA_SEN_GEN_AZSIGMA = 110
XA_SEN_GEN_ELSIGMA = 111
XA_SEN_GEN_ARSIGMA = 114
XA_SEN_GEN_ERSIGMA = 115
XA_SEN_GEN_RGSIGMA = 112
XA_SEN_GEN_RRSIGMA = 113
XA_SEN_GEN_AZBIAS = 116
XA_SEN_GEN_ELBIAS = 117
XA_SEN_GEN_RGBIAS = 118
XA_SEN_GEN_RRBIAS = 119
XA_SEN_GEN_TIMEBIAS = 120


@pytest.fixture()
def sensor() -> Generator[SensorInterface, None, None]:
    with LOCK:
        interface = SensorInterface()
        yield interface
        interface.clear()


def test_get_dll_info(sensor: SensorInterface) -> None:
    assert MainInterface.DLL_VERSION in sensor.info


def test_load_file(sensor: SensorInterface) -> None:
    sensor.load_file("tests/data/sensors.dat")
    assert sensor.get_count() == 108
    sensors = sensor.parse_all()
    sensor.clear()
    assert sensor.get_count() == 0
    assert sensors[0].number == 211
    assert sensors[0].description == "SOCORRO CAM1"


def test_get_arrays(sensor: SensorInterface) -> None:
    sensor.load_card(SENSOR_CARD)
    sensor.load_card(NOISE_CARD)
    keys = sensor.get_keys(2)
    key = keys[-1]
    xa_sen, xs_sen = sensor.get_arrays(key)
    sensor.clear()

    assert xa_sen[XA_SEN_GEN_SENNUM] == pytest.approx(211.0)
    assert xa_sen[XA_SEN_GRN_POS1] == pytest.approx(-1521.161)
    assert xa_sen[XA_SEN_GRN_POS2] == pytest.approx(-5083.089)
    assert xa_sen[XA_SEN_GRN_POS3] == pytest.approx(3530.462)
    assert xa_sen[XA_SEN_GRN_ASTROLAT] == pytest.approx(33.81724)
    assert xa_sen[XA_SEN_GRN_ASTROLON] == pytest.approx(-253.33969)
    assert xa_sen[XA_SEN_GRN_ECITIME] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_RNGLIMFLG] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_SMSEN] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_MINRNG] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_MAXRNG] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_RRLIM] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GRN_LOCTYPE] == pytest.approx(1.0)
    assert xa_sen[XA_SEN_GEN_AZSIGMA] == pytest.approx(0.0003)
    assert xa_sen[XA_SEN_GEN_ELSIGMA] == pytest.approx(0.0003)
    assert xa_sen[XA_SEN_GEN_ARSIGMA] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_ERSIGMA] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_RGSIGMA] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_RRSIGMA] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_AZBIAS] == pytest.approx(-0.0005)
    assert xa_sen[XA_SEN_GEN_ELBIAS] == pytest.approx(-0.0003)
    assert xa_sen[XA_SEN_GEN_RGBIAS] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_RRBIAS] == pytest.approx(0.0)
    assert xa_sen[XA_SEN_GEN_TIMEBIAS] == pytest.approx(0.0)
    assert xs_sen.strip() == "U33SOCORRO CAM1"
