import threading
from typing import Generator

import pytest

from pysaal import MainInterface, SGP4Interface, TLEInterface

SGP4_LINE_1 = "1 22222C 15058A   25363.54791667 +.00012345  10000-1  20000-1 2 0900"
SGP4_LINE_2 = "2 22222  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345"
XP_LINE_1 = "1 33333U 15058A   25363.54791667 +.00012345  10000-1  20000-1 4  900"
XP_LINE_2 = "2 33333  30.0000  40.0000 0005000  60.0000  70.0000  8.2345678012345"

EPOCH = 27757.54791667

SGP4_LATITUDE = 22.536547343263198
SGP4_LONGITUDE = 238.66278387347936
SGP4_ALTITUDE = 30319.722365834336
XP_LATITUDE = 22.580834873791684
XP_LONGITUDE = 238.65305913125454
XP_ALTITUDE = 3977.969361992566

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

XA_SGP4OUT_POSX = 2
XA_SGP4OUT_POSY = 3
XA_SGP4OUT_POSZ = 4
XA_SGP4OUT_VELX = 5
XA_SGP4OUT_VELY = 6
XA_SGP4OUT_VELZ = 7
XA_TLE_INCLI = 20
XA_TLE_NODE = 21
XA_TLE_ECCEN = 22
XA_TLE_OMEGA = 23
XA_TLE_MNANOM = 24
XA_TLE_MNMOTN = 25

SGP4_MEAN_MOTION = 1.2345678
SGP4_MEAN_INCLINATION = 30.0
SGP4_MEAN_ECCENTRICITY = 0.0005
SGP4_MEAN_RAAN = 40.0
SGP4_MEAN_MA = 70.0
SGP4_MEAN_ARG_PERIGEE = 60.0

XP_MEAN_MOTION = 8.2345678
XP_MEAN_INCLINATION = 30.0
XP_MEAN_ECCENTRICITY = 0.0005
XP_MEAN_RAAN = 40.0
XP_MEAN_MA = 70.0
XP_MEAN_ARG_PERIGEE = 60.0

LOCK = threading.RLock()


@pytest.fixture()
def tle() -> Generator[TLEInterface, None, None]:
    with LOCK:
        ti = TLEInterface()
        yield ti
        ti.clear()


@pytest.fixture()
def sgp4() -> Generator[SGP4Interface, None, None]:
    with LOCK:
        si = SGP4Interface()
        yield si
        si.clear()


def test_get_full_state(tle: TLEInterface, sgp4: SGP4Interface) -> None:
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)

    sgp4.load(sgp4_key)
    sgp4.load(xp_key)

    sgp4_all = sgp4.get_full_state(sgp4_key, EPOCH)
    xp_all = sgp4.get_full_state(xp_key, EPOCH)

    assert sgp4_all[XA_SGP4OUT_POSX] == pytest.approx(SGP4_X, abs=1.0e-9)
    assert sgp4_all[XA_SGP4OUT_POSY] == pytest.approx(SGP4_Y, abs=1.0e-9)
    assert sgp4_all[XA_SGP4OUT_POSZ] == pytest.approx(SGP4_Z, abs=1.0e-9)
    assert sgp4_all[XA_SGP4OUT_VELX] == pytest.approx(SGP4_VX, abs=1.0e-9)
    assert sgp4_all[XA_SGP4OUT_VELY] == pytest.approx(SGP4_VY, abs=1.0e-9)
    assert sgp4_all[XA_SGP4OUT_VELZ] == pytest.approx(SGP4_VZ, abs=1.0e-9)

    assert xp_all[XA_SGP4OUT_POSX] == pytest.approx(XP_X, abs=1.0e-9)
    assert xp_all[XA_SGP4OUT_POSY] == pytest.approx(XP_Y, abs=1.0e-9)
    assert xp_all[XA_SGP4OUT_POSZ] == pytest.approx(XP_Z, abs=1.0e-9)
    assert xp_all[XA_SGP4OUT_VELX] == pytest.approx(XP_VX, abs=1.0e-9)
    assert xp_all[XA_SGP4OUT_VELY] == pytest.approx(XP_VY, abs=1.0e-9)
    assert xp_all[XA_SGP4OUT_VELZ] == pytest.approx(XP_VZ, abs=1.0e-9)


def test_get_position(tle: TLEInterface, sgp4: SGP4Interface) -> None:
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)

    sgp4.load(sgp4_key)
    sgp4.load(xp_key)

    sgp4_pos = sgp4.get_position(sgp4_key, EPOCH)
    xp_pos = sgp4.get_position(xp_key, EPOCH)

    assert sgp4_pos[0] == pytest.approx(SGP4_X, abs=1.0e-9)
    assert sgp4_pos[1] == pytest.approx(SGP4_Y, abs=1.0e-9)
    assert sgp4_pos[2] == pytest.approx(SGP4_Z, abs=1.0e-9)
    assert xp_pos[0] == pytest.approx(XP_X, abs=1.0e-9)
    assert xp_pos[1] == pytest.approx(XP_Y, abs=1.0e-9)
    assert xp_pos[2] == pytest.approx(XP_Z, abs=1.0e-9)


def test_get_position_velocity(tle: TLEInterface, sgp4: SGP4Interface) -> None:
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)

    sgp4.load(sgp4_key)
    sgp4.load(xp_key)

    sgp4_pos, sgp4_vel = sgp4.get_position_velocity(sgp4_key, EPOCH)
    xp_pos, xp_vel = sgp4.get_position_velocity(xp_key, EPOCH)

    assert sgp4_pos[0] == pytest.approx(SGP4_X, abs=1.0e-9)
    assert sgp4_pos[1] == pytest.approx(SGP4_Y, abs=1.0e-9)
    assert sgp4_pos[2] == pytest.approx(SGP4_Z, abs=1.0e-9)
    assert sgp4_vel[0] == pytest.approx(SGP4_VX, abs=1.0e-9)
    assert sgp4_vel[1] == pytest.approx(SGP4_VY, abs=1.0e-9)
    assert sgp4_vel[2] == pytest.approx(SGP4_VZ, abs=1.0e-9)

    assert xp_pos[0] == pytest.approx(XP_X, abs=1.0e-9)
    assert xp_pos[1] == pytest.approx(XP_Y, abs=1.0e-9)
    assert xp_pos[2] == pytest.approx(XP_Z, abs=1.0e-9)
    assert xp_vel[0] == pytest.approx(XP_VX, abs=1.0e-9)
    assert xp_vel[1] == pytest.approx(XP_VY, abs=1.0e-9)
    assert xp_vel[2] == pytest.approx(XP_VZ, abs=1.0e-9)


def test_get_lla(tle: TLEInterface, sgp4: SGP4Interface) -> None:
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)

    sgp4.load(sgp4_key)
    sgp4.load(xp_key)

    sgp4_lla = sgp4.get_lla(sgp4_key, EPOCH)
    xp_lla = sgp4.get_lla(xp_key, EPOCH)

    assert sgp4_lla[0] == pytest.approx(SGP4_LATITUDE, abs=1.0e-9)
    assert sgp4_lla[1] == pytest.approx(SGP4_LONGITUDE, abs=1.0e-9)
    assert sgp4_lla[2] == pytest.approx(SGP4_ALTITUDE, abs=1.0e-9)

    assert xp_lla[0] == pytest.approx(XP_LATITUDE, abs=1.0e-9)
    assert xp_lla[1] == pytest.approx(XP_LONGITUDE, abs=1.0e-9)
    assert xp_lla[2] == pytest.approx(XP_ALTITUDE, abs=1.0e-9)


def test_get_dll_info_contains_version(sgp4: SGP4Interface) -> None:
    assert MainInterface.DLL_VERSION in sgp4.info


def test_get_ephemeris(tle: TLEInterface, sgp4: SGP4Interface) -> None:
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)
    sgp4.load(sgp4_key)
    sgp4.load(xp_key)
    start = EPOCH - 1.0
    stop = EPOCH
    step = 5.0
    frame = 1

    sgp4_ephem_by_key = sgp4.get_ephemeris(sgp4_key, start, stop, step, frame)
    xp_ephem_by_key = sgp4.get_ephemeris(xp_key, start, stop, step, frame)
    sgp4_xa, _ = tle.get_arrays(sgp4_key)
    xp_xa, _ = tle.get_arrays(xp_key)
    sgp4_ephem = sgp4.array_to_ephemeris(sgp4_xa, start, stop, step, frame)
    xp_ephem = sgp4.array_to_ephemeris(xp_xa, start, stop, step, frame)

    assert sgp4_ephem_by_key == sgp4_ephem
    assert xp_ephem_by_key == xp_ephem
    assert sgp4_ephem[-6] == pytest.approx(SGP4_X, abs=1.0e-9)
    assert sgp4_ephem[-5] == pytest.approx(SGP4_Y, abs=1.0e-9)
    assert sgp4_ephem[-4] == pytest.approx(SGP4_Z, abs=1.0e-9)
    assert sgp4_ephem[-3] == pytest.approx(SGP4_VX, abs=1.0e-9)
    assert sgp4_ephem[-2] == pytest.approx(SGP4_VY, abs=1.0e-9)
    assert sgp4_ephem[-1] == pytest.approx(SGP4_VZ, abs=1.0e-9)
    assert xp_ephem[-6] == pytest.approx(XP_X, abs=1.0e-9)
    assert xp_ephem[-5] == pytest.approx(XP_Y, abs=1.0e-9)
    assert xp_ephem[-4] == pytest.approx(XP_Z, abs=1.0e-9)
    assert xp_ephem[-3] == pytest.approx(XP_VX, abs=1.0e-9)
    assert xp_ephem[-2] == pytest.approx(XP_VY, abs=1.0e-9)
    assert xp_ephem[-1] == pytest.approx(XP_VZ, abs=1.0e-9)
    assert len(sgp4_ephem) == 2023
    assert len(xp_ephem) == 2023


def test_fit_arrays(sgp4: SGP4Interface) -> None:
    sgp4_posvel = [SGP4_X, SGP4_Y, SGP4_Z, SGP4_VX, SGP4_VY, SGP4_VZ]
    xp_posvel = [XP_X, XP_Y, XP_Z, XP_VX, XP_VY, XP_VZ]

    sgp4_xa = sgp4.fit_sgp4_array(EPOCH, sgp4_posvel, 0.02)
    xp_xa = sgp4.fit_xp_array(EPOCH, xp_posvel, 0.02, 0.01)

    assert sgp4_xa[XA_TLE_INCLI] == pytest.approx(SGP4_MEAN_INCLINATION, abs=1.0e-9)
    assert sgp4_xa[XA_TLE_ECCEN] == pytest.approx(SGP4_MEAN_ECCENTRICITY, abs=1.0e-9)
    assert sgp4_xa[XA_TLE_NODE] == pytest.approx(SGP4_MEAN_RAAN, abs=1.0e-9)
    assert sgp4_xa[XA_TLE_MNANOM] == pytest.approx(SGP4_MEAN_MA, abs=1.0e-9)
    assert sgp4_xa[XA_TLE_MNMOTN] == pytest.approx(SGP4_MEAN_MOTION, abs=1.0e-9)
    assert sgp4_xa[XA_TLE_OMEGA] == pytest.approx(SGP4_MEAN_ARG_PERIGEE, abs=1.0e-9)
    assert xp_xa[XA_TLE_INCLI] == pytest.approx(XP_MEAN_INCLINATION, abs=1.0e-9)
    assert xp_xa[XA_TLE_ECCEN] == pytest.approx(XP_MEAN_ECCENTRICITY, abs=1.0e-9)
    assert xp_xa[XA_TLE_NODE] == pytest.approx(XP_MEAN_RAAN, abs=1.0e-9)
    assert xp_xa[XA_TLE_MNANOM] == pytest.approx(XP_MEAN_MA, abs=1.0e-9)
    assert xp_xa[XA_TLE_MNMOTN] == pytest.approx(XP_MEAN_MOTION, abs=1.0e-9)
    assert xp_xa[XA_TLE_OMEGA] == pytest.approx(XP_MEAN_ARG_PERIGEE, abs=1.0e-9)


def test_get_positions_velocities(tle: TLEInterface, sgp4: SGP4Interface) -> None:
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)
    sgp4.load(sgp4_key)
    sgp4.load(xp_key)

    sgp4_pos, sgp4_vel = sgp4.get_position_velocity(sgp4_key, EPOCH)
    xp_pos, xp_vel = sgp4.get_position_velocity(xp_key, EPOCH)
    all_posvel = sgp4.get_positions_velocities([sgp4_key, xp_key], EPOCH)

    assert all_posvel[0] == pytest.approx(sgp4_pos[0], abs=1.0e-9)
    assert all_posvel[1] == pytest.approx(sgp4_pos[1], abs=1.0e-9)
    assert all_posvel[2] == pytest.approx(sgp4_pos[2], abs=1.0e-9)
    assert all_posvel[3] == pytest.approx(sgp4_vel[0], abs=1.0e-9)
    assert all_posvel[4] == pytest.approx(sgp4_vel[1], abs=1.0e-9)
    assert all_posvel[5] == pytest.approx(sgp4_vel[2], abs=1.0e-9)
    assert all_posvel[6] == pytest.approx(xp_pos[0], abs=1.0e-9)
    assert all_posvel[7] == pytest.approx(xp_pos[1], abs=1.0e-9)
    assert all_posvel[8] == pytest.approx(xp_pos[2], abs=1.0e-9)
    assert all_posvel[9] == pytest.approx(xp_vel[0], abs=1.0e-9)
    assert all_posvel[10] == pytest.approx(xp_vel[1], abs=1.0e-9)
    assert all_posvel[11] == pytest.approx(xp_vel[2], abs=1.0e-9)
