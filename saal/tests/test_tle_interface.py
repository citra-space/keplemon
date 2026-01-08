import threading
from typing import Generator

import pytest

from pysaal import MainInterface, ParsedTLE, TLEInterface

LOCK = threading.Lock()

SGP_LINE_1 = "1 11111U 98067A   25363.54791667 +.00012345  10000-1  20000-1 0 0900"
SGP_LINE_2 = "2 11111  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345"
NULL_LINE_1 = "1 11111U          25363.54791667 +.00012345  00000 0  00000 0 0 0900"
FMTD_LINE_1 = "1 11111U UNKNOWN  25363.54791667 +.00012345  00000+0  00000+0 0 09004"
SGP4_LINE_1 = "1 22222C 15058A   25363.54791667 +.00012345  10000-1  20000-1 2 0900"
SGP4_LINE_2 = "2 22222  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345"
XP_LINE_1 = "1 33333S 21001A   25363.54791667 +.00012345  10000-1  20000-1 4 0900"
XP_LINE_2 = "2 33333  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345"
SP_LINE_1 = "1 44444U 67001A   25363.54791667 +.02000000  00000 0  10000-1 6 0900"
SP_LINE_2 = "2 44444  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345"

SGP_NORAD_ID = 11111.0
SGP_B_STAR = 0.02
SGP_MEAN_MOTION_1ST_DERIVATIVE = 0.00012345
SGP_MEAN_MOTION_2ND_DERIVATIVE = 0.01
SGP_BALLISTIC_COEFFICIENT = 0.0
SGP_EPHEMERIS_TYPE = 0.0
SGP_SRP_COEFFICIENT = 0.0

SGP4_NORAD_ID = 22222.0
SGP4_B_STAR = 0.02
SGP4_MEAN_MOTION_1ST_DERIVATIVE = 0.00012345
SGP4_MEAN_MOTION_2ND_DERIVATIVE = 0.01
SGP4_BALLISTIC_COEFFICIENT = 0.0
SGP4_EPHEMERIS_TYPE = 2.0
SGP4_SRP_COEFFICIENT = 0.0

XP_NORAD_ID = 33333.0
XP_MEAN_MOTION_1ST_DERIVATIVE = 0.00012345
XP_BALLISTIC_COEFFICIENT = 0.02
XP_EPHEMERIS_TYPE = 4.0
XP_SRP_COEFFICIENT = 0.01

SP_NORAD_ID = 44444.0
SP_BALLISTIC_COEFFICIENT = 0.02
SP_EPHEMERIS_TYPE = 6.0
SP_SRP_COEFFICIENT = 0.01

SGP_DESIGNATOR = "98067A"
SGP4_DESIGNATOR = "15058A"
XP_DESIGNATOR = "21001A"
SP_DESIGNATOR = "67001A"

EPOCH = 27757.54791667
INCLINATION = 30.0
RAAN = 40.0
ECCENTRICITY = 0.0005
ARGUMENT_OF_PERIGEE = 60.0
MEAN_ANOMALY = 70.0
MEAN_MOTION = 1.2345678

SGP_XS_TLE = "U98067A"
SGP4_XS_TLE = "C15058A"
XP_XS_TLE = "S21001A"
SP_XS_TLE = "U67001A"

XA_TLE_SATNUM = 0
XA_TLE_EPOCH = 1
XA_TLE_NDOT = 2
XA_TLE_NDOTDOT = 3
XA_TLE_BSTAR = 4
XA_TLE_EPHTYPE = 5
XA_TLE_INCLI = 20
XA_TLE_NODE = 21
XA_TLE_ECCEN = 22
XA_TLE_OMEGA = 23
XA_TLE_MNANOM = 24
XA_TLE_MNMOTN = 25
XA_TLE_REVNUM = 26
XA_TLE_ELSETNUM = 30
XA_TLE_BTERM = 32
XA_TLE_AGOMGP = 38


@pytest.fixture()
def tle() -> Generator[TLEInterface, None, None]:
    with LOCK:
        ti = TLEInterface()
        yield ti
        ti.clear()


def test_get_dll_info(tle: TLEInterface) -> None:
    assert MainInterface.DLL_VERSION in tle.info


def test_get_lines(tle: TLEInterface) -> None:
    sgp_key = tle.load_lines(SGP_LINE_1, SGP_LINE_2)
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)
    sp_key = tle.load_lines(SP_LINE_1, SP_LINE_2)

    line_1, line_2 = tle.get_lines(sgp_key)
    line_1_sgp4, line_2_sgp4 = tle.get_lines(sgp4_key)
    line_1_xp, line_2_xp = tle.get_lines(xp_key)
    line_1_sp, line_2_sp = tle.get_lines(sp_key)

    assert line_1_sgp4 == SGP4_LINE_1
    assert line_2_sgp4 == SGP4_LINE_2
    assert line_1_xp == XP_LINE_1
    assert line_2_xp == XP_LINE_2
    assert line_1_sp == SP_LINE_1
    assert line_2_sp == SP_LINE_2
    assert line_1 == SGP_LINE_1
    assert line_2 == SGP_LINE_2


def test_get_arrays(tle: TLEInterface) -> None:
    sgp_key = tle.load_lines(SGP_LINE_1, SGP_LINE_2)
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)
    sp_key = tle.load_lines(SP_LINE_1, SP_LINE_2)

    expected_xa_tle, expected_xs_tle = tle.lines_to_arrays(SGP_LINE_1, SGP_LINE_2)
    expected_xa_sgp4, expected_xs_sgp4 = tle.lines_to_arrays(SGP4_LINE_1, SGP4_LINE_2)
    expected_xa_xp, expected_xs_xp = tle.lines_to_arrays(XP_LINE_1, XP_LINE_2)
    expected_xa_sp, expected_xs_sp = tle.lines_to_arrays(SP_LINE_1, SP_LINE_2)

    xa_tle, xs_tle = tle.get_arrays(sgp_key)
    xa_sgp4, xs_sgp4 = tle.get_arrays(sgp4_key)
    xa_xp, xs_xp = tle.get_arrays(xp_key)
    xa_sp, xs_sp = tle.get_arrays(sp_key)

    assert xs_tle == expected_xs_tle
    assert xa_tle == expected_xa_tle
    assert xs_sgp4 == expected_xs_sgp4
    assert xa_sgp4 == expected_xa_sgp4
    assert xs_xp == expected_xs_xp
    assert xa_xp == expected_xa_xp
    assert xs_sp == expected_xs_sp
    assert xa_sp == expected_xa_sp


def test_get_keys(tle: TLEInterface) -> None:
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    sgp_key = tle.load_lines(SGP_LINE_1, SGP_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)
    sp_key = tle.load_lines(SP_LINE_1, SP_LINE_2)
    final_count = tle.get_count()
    assert final_count == 4

    keys_load = tle.get_keys(2)
    assert keys_load == [sgp4_key, sgp_key, xp_key, sp_key]

    keys_desc = tle.get_keys(1)
    assert keys_desc == [sp_key, xp_key, sgp4_key, sgp_key]


def test_remove_nulls(tle: TLEInterface) -> None:
    parsed = tle.parse_lines(NULL_LINE_1, SGP_LINE_2)
    line_1, _line_2 = parsed.get_lines(True)
    assert line_1 == FMTD_LINE_1


def test_parsed_tles_to_lines(tle: TLEInterface) -> None:
    parsed_sgp = ParsedTLE()
    parsed_sgp.norad_id = int(SGP_NORAD_ID)
    parsed_sgp.designator = SGP_DESIGNATOR
    parsed_sgp.ephemeris_type = 0
    parsed_sgp.classification = "U"
    parsed_sgp.epoch = EPOCH
    parsed_sgp.inclination = INCLINATION
    parsed_sgp.raan = RAAN
    parsed_sgp.eccentricity = ECCENTRICITY
    parsed_sgp.argument_of_perigee = ARGUMENT_OF_PERIGEE
    parsed_sgp.mean_anomaly = MEAN_ANOMALY
    parsed_sgp.mean_motion = MEAN_MOTION
    parsed_sgp.b_star = SGP_B_STAR
    parsed_sgp.mean_motion_1st_derivative = SGP_MEAN_MOTION_1ST_DERIVATIVE
    parsed_sgp.mean_motion_2nd_derivative = SGP_MEAN_MOTION_2ND_DERIVATIVE
    parsed_sgp.ballistic_coefficient = 1.1
    parsed_sgp.srp_coefficient = 2.2
    parsed_sgp.element_set_number = 900
    parsed_sgp.revolution_number = 12345

    parsed_sgp4 = ParsedTLE()
    parsed_sgp4.norad_id = int(SGP4_NORAD_ID)
    parsed_sgp4.designator = SGP4_DESIGNATOR
    parsed_sgp4.ephemeris_type = 2
    parsed_sgp4.classification = "C"
    parsed_sgp4.epoch = EPOCH
    parsed_sgp4.inclination = INCLINATION
    parsed_sgp4.raan = RAAN
    parsed_sgp4.eccentricity = ECCENTRICITY
    parsed_sgp4.argument_of_perigee = ARGUMENT_OF_PERIGEE
    parsed_sgp4.mean_anomaly = MEAN_ANOMALY
    parsed_sgp4.mean_motion = MEAN_MOTION
    parsed_sgp4.b_star = SGP4_B_STAR
    parsed_sgp4.mean_motion_1st_derivative = SGP4_MEAN_MOTION_1ST_DERIVATIVE
    parsed_sgp4.mean_motion_2nd_derivative = SGP4_MEAN_MOTION_2ND_DERIVATIVE
    parsed_sgp4.ballistic_coefficient = 1.1
    parsed_sgp4.srp_coefficient = 2.2
    parsed_sgp4.element_set_number = 900
    parsed_sgp4.revolution_number = 12345

    parsed_xp = ParsedTLE()
    parsed_xp.norad_id = int(XP_NORAD_ID)
    parsed_xp.designator = XP_DESIGNATOR
    parsed_xp.ephemeris_type = 4
    parsed_xp.classification = "S"
    parsed_xp.epoch = EPOCH
    parsed_xp.inclination = INCLINATION
    parsed_xp.raan = RAAN
    parsed_xp.eccentricity = ECCENTRICITY
    parsed_xp.argument_of_perigee = ARGUMENT_OF_PERIGEE
    parsed_xp.mean_anomaly = MEAN_ANOMALY
    parsed_xp.mean_motion = MEAN_MOTION
    parsed_xp.mean_motion_1st_derivative = XP_MEAN_MOTION_1ST_DERIVATIVE
    parsed_xp.ballistic_coefficient = XP_BALLISTIC_COEFFICIENT
    parsed_xp.srp_coefficient = XP_SRP_COEFFICIENT
    parsed_xp.b_star = 1.1
    parsed_xp.mean_motion_2nd_derivative = 2.2
    parsed_xp.element_set_number = 900
    parsed_xp.revolution_number = 12345

    parsed_sp = ParsedTLE()
    parsed_sp.norad_id = int(SP_NORAD_ID)
    parsed_sp.designator = SP_DESIGNATOR
    parsed_sp.ephemeris_type = 6
    parsed_sp.classification = "U"
    parsed_sp.epoch = EPOCH
    parsed_sp.inclination = INCLINATION
    parsed_sp.raan = RAAN
    parsed_sp.eccentricity = ECCENTRICITY
    parsed_sp.argument_of_perigee = ARGUMENT_OF_PERIGEE
    parsed_sp.mean_anomaly = MEAN_ANOMALY
    parsed_sp.mean_motion = MEAN_MOTION
    parsed_sp.ballistic_coefficient = SP_BALLISTIC_COEFFICIENT
    parsed_sp.srp_coefficient = SP_SRP_COEFFICIENT
    parsed_sp.b_star = 1.1
    parsed_sp.mean_motion_1st_derivative = 2.2
    parsed_sp.mean_motion_2nd_derivative = 3.3
    parsed_sp.element_set_number = 900
    parsed_sp.revolution_number = 12345

    sgp_line_1, sgp_line_2 = parsed_sgp.get_lines(False)
    sgp4_line_1, sgp4_line_2 = parsed_sgp4.get_lines(False)
    xp_line_1, xp_line_2 = parsed_xp.get_lines(False)
    sp_line_1, sp_line_2 = parsed_sp.get_lines(False)

    assert sgp_line_1 == SGP_LINE_1
    assert sgp_line_2 == SGP_LINE_2
    assert parsed_sgp.ballistic_coefficient is None
    assert parsed_sgp.srp_coefficient is None
    assert sgp4_line_1 == SGP4_LINE_1
    assert sgp4_line_2 == SGP4_LINE_2
    assert parsed_sgp4.ballistic_coefficient is None
    assert parsed_sgp4.srp_coefficient is None
    assert xp_line_1 == XP_LINE_1
    assert xp_line_2 == XP_LINE_2
    assert parsed_xp.b_star is None
    assert parsed_xp.mean_motion_2nd_derivative is None
    assert sp_line_1 == SP_LINE_1
    assert sp_line_2 == SP_LINE_2
    assert parsed_sp.b_star is None
    assert parsed_sp.mean_motion_1st_derivative is None
    assert parsed_sp.mean_motion_2nd_derivative is None


def test_arrays_to_parsed_tles(tle: TLEInterface) -> None:
    parsed_sgp = tle.parse_lines(SGP_LINE_1, SGP_LINE_2)
    parsed_sgp4 = tle.parse_lines(SGP4_LINE_1, SGP4_LINE_2)
    parsed_xp = tle.parse_lines(XP_LINE_1, XP_LINE_2)
    parsed_sp = tle.parse_lines(SP_LINE_1, SP_LINE_2)

    assert parsed_sgp.norad_id == int(SGP_NORAD_ID)
    assert parsed_sgp.designator == SGP_DESIGNATOR
    assert parsed_sgp.b_star == pytest.approx(SGP_B_STAR, abs=1.0e-10)
    assert parsed_sgp.mean_motion_1st_derivative == pytest.approx(SGP_MEAN_MOTION_1ST_DERIVATIVE, abs=1.0e-10)
    assert parsed_sgp.mean_motion_2nd_derivative == pytest.approx(SGP_MEAN_MOTION_2ND_DERIVATIVE, abs=1.0e-10)
    assert parsed_sgp.ballistic_coefficient is None
    assert parsed_sgp.ephemeris_type == 0
    assert parsed_sgp.srp_coefficient is None
    assert parsed_sgp.classification == "U"
    assert parsed_sgp.epoch == EPOCH
    assert parsed_sgp.inclination == INCLINATION
    assert parsed_sgp.raan == RAAN
    assert parsed_sgp.eccentricity == ECCENTRICITY
    assert parsed_sgp.argument_of_perigee == ARGUMENT_OF_PERIGEE
    assert parsed_sgp.mean_anomaly == MEAN_ANOMALY
    assert parsed_sgp.mean_motion == MEAN_MOTION

    assert parsed_sgp4.norad_id == int(SGP4_NORAD_ID)
    assert parsed_sgp4.designator == SGP4_DESIGNATOR
    assert parsed_sgp4.b_star == pytest.approx(SGP4_B_STAR, abs=1.0e-10)
    assert parsed_sgp4.mean_motion_1st_derivative == pytest.approx(SGP4_MEAN_MOTION_1ST_DERIVATIVE, abs=1.0e-10)
    assert parsed_sgp4.mean_motion_2nd_derivative == pytest.approx(SGP4_MEAN_MOTION_2ND_DERIVATIVE, abs=1.0e-10)
    assert parsed_sgp4.ballistic_coefficient is None
    assert parsed_sgp4.ephemeris_type == 2
    assert parsed_sgp4.srp_coefficient is None
    assert parsed_sgp4.classification == "C"
    assert parsed_sgp4.epoch == EPOCH
    assert parsed_sgp4.inclination == INCLINATION
    assert parsed_sgp4.raan == RAAN
    assert parsed_sgp4.eccentricity == ECCENTRICITY
    assert parsed_sgp4.argument_of_perigee == ARGUMENT_OF_PERIGEE
    assert parsed_sgp4.mean_anomaly == MEAN_ANOMALY
    assert parsed_sgp4.mean_motion == MEAN_MOTION

    assert parsed_xp.norad_id == int(XP_NORAD_ID)
    assert parsed_xp.designator == XP_DESIGNATOR
    assert parsed_xp.b_star is None
    assert parsed_xp.mean_motion_1st_derivative == pytest.approx(XP_MEAN_MOTION_1ST_DERIVATIVE, abs=1.0e-10)
    assert parsed_xp.mean_motion_2nd_derivative is None
    assert parsed_xp.ballistic_coefficient == pytest.approx(XP_BALLISTIC_COEFFICIENT, abs=1.0e-10)
    assert parsed_xp.ephemeris_type == 4
    assert parsed_xp.srp_coefficient == pytest.approx(XP_SRP_COEFFICIENT, abs=1.0e-10)
    assert parsed_xp.classification == "S"
    assert parsed_xp.epoch == EPOCH
    assert parsed_xp.inclination == INCLINATION
    assert parsed_xp.raan == RAAN
    assert parsed_xp.eccentricity == ECCENTRICITY
    assert parsed_xp.argument_of_perigee == ARGUMENT_OF_PERIGEE
    assert parsed_xp.mean_anomaly == MEAN_ANOMALY
    assert parsed_xp.mean_motion == MEAN_MOTION

    assert parsed_sp.norad_id == int(SP_NORAD_ID)
    assert parsed_sp.designator == SP_DESIGNATOR
    assert parsed_sp.b_star is None
    assert parsed_sp.mean_motion_1st_derivative is None
    assert parsed_sp.mean_motion_2nd_derivative is None
    assert parsed_sp.ballistic_coefficient == pytest.approx(SP_BALLISTIC_COEFFICIENT, abs=1.0e-10)
    assert parsed_sp.ephemeris_type == 6
    assert parsed_sp.srp_coefficient == pytest.approx(SP_SRP_COEFFICIENT, abs=1.0e-10)
    assert parsed_sp.classification == "U"
    assert parsed_sp.epoch == EPOCH
    assert parsed_sp.inclination == INCLINATION
    assert parsed_sp.raan == RAAN
    assert parsed_sp.eccentricity == ECCENTRICITY
    assert parsed_sp.argument_of_perigee == ARGUMENT_OF_PERIGEE
    assert parsed_sp.mean_anomaly == MEAN_ANOMALY
    assert parsed_sp.mean_motion == MEAN_MOTION


def test_arrays_to_lines(tle: TLEInterface) -> None:
    xa_sgp, xs_sgp = tle.lines_to_arrays(SGP_LINE_1, SGP_LINE_2)
    xa_sgp4, xs_sgp4 = tle.lines_to_arrays(SGP4_LINE_1, SGP4_LINE_2)
    xa_xp, xs_xp = tle.lines_to_arrays(XP_LINE_1, XP_LINE_2)
    xa_sp, xs_sp = tle.lines_to_arrays(SP_LINE_1, SP_LINE_2)

    line1_sgp, line2_sgp = tle.arrays_to_lines(xa_sgp, xs_sgp)
    assert line1_sgp.strip() == SGP_LINE_1
    assert line2_sgp.strip() == SGP_LINE_2

    line1_sgp4, line2_sgp4 = tle.arrays_to_lines(xa_sgp4, xs_sgp4)
    assert line1_sgp4.strip() == SGP4_LINE_1
    assert line2_sgp4.strip() == SGP4_LINE_2

    line1_xp, line2_xp = tle.arrays_to_lines(xa_xp, xs_xp)
    assert line1_xp.strip() == XP_LINE_1
    assert line2_xp.strip() == XP_LINE_2

    line1_sp, line2_sp = tle.arrays_to_lines(xa_sp, xs_sp)
    assert line1_sp.strip() == SP_LINE_1
    assert line2_sp.strip() == SP_LINE_2


def test_lines_to_arrays(tle: TLEInterface) -> None:
    xa_sgp, xs_sgp = tle.lines_to_arrays(SGP_LINE_1, SGP_LINE_2)
    xa_sgp4, xs_sgp4 = tle.lines_to_arrays(SGP4_LINE_1, SGP4_LINE_2)
    xa_xp, xs_xp = tle.lines_to_arrays(XP_LINE_1, XP_LINE_2)
    xa_sp, xs_sp = tle.lines_to_arrays(SP_LINE_1, SP_LINE_2)

    assert xa_sgp[XA_TLE_EPOCH] == EPOCH
    assert xa_sgp[XA_TLE_SATNUM] == SGP_NORAD_ID
    assert xa_sgp[XA_TLE_INCLI] == INCLINATION
    assert xa_sgp[XA_TLE_NODE] == RAAN
    assert xa_sgp[XA_TLE_ECCEN] == ECCENTRICITY
    assert xa_sgp[XA_TLE_OMEGA] == ARGUMENT_OF_PERIGEE
    assert xa_sgp[XA_TLE_MNANOM] == MEAN_ANOMALY
    assert xa_sgp[XA_TLE_MNMOTN] == MEAN_MOTION
    assert xa_sgp[XA_TLE_BSTAR] == pytest.approx(SGP_B_STAR, abs=1.0e-8)
    assert xa_sgp[XA_TLE_EPHTYPE] == SGP_EPHEMERIS_TYPE
    assert xa_sgp[XA_TLE_NDOT] == SGP_MEAN_MOTION_1ST_DERIVATIVE
    assert xa_sgp[XA_TLE_NDOTDOT] == pytest.approx(SGP_MEAN_MOTION_2ND_DERIVATIVE, abs=1.0e-8)
    assert xa_sgp[XA_TLE_BTERM] == SGP_BALLISTIC_COEFFICIENT
    assert xa_sgp[XA_TLE_AGOMGP] == SGP_SRP_COEFFICIENT
    assert xs_sgp.strip() == SGP_XS_TLE

    assert xa_sgp4[XA_TLE_EPOCH] == EPOCH
    assert xa_sgp4[XA_TLE_SATNUM] == SGP4_NORAD_ID
    assert xa_sgp4[XA_TLE_INCLI] == INCLINATION
    assert xa_sgp4[XA_TLE_NODE] == RAAN
    assert xa_sgp4[XA_TLE_ECCEN] == ECCENTRICITY
    assert xa_sgp4[XA_TLE_OMEGA] == ARGUMENT_OF_PERIGEE
    assert xa_sgp4[XA_TLE_MNANOM] == MEAN_ANOMALY
    assert xa_sgp4[XA_TLE_MNMOTN] == MEAN_MOTION
    assert xa_sgp4[XA_TLE_BSTAR] == pytest.approx(SGP4_B_STAR, abs=1.0e-8)
    assert xa_sgp4[XA_TLE_EPHTYPE] == SGP4_EPHEMERIS_TYPE
    assert xa_sgp4[XA_TLE_NDOT] == SGP4_MEAN_MOTION_1ST_DERIVATIVE
    assert xa_sgp4[XA_TLE_NDOTDOT] == pytest.approx(SGP4_MEAN_MOTION_2ND_DERIVATIVE, abs=1.0e-8)
    assert xa_sgp4[XA_TLE_BTERM] == SGP4_BALLISTIC_COEFFICIENT
    assert xa_sgp4[XA_TLE_AGOMGP] == SGP4_SRP_COEFFICIENT
    assert xs_sgp4.strip() == SGP4_XS_TLE

    assert xa_xp[XA_TLE_EPOCH] == EPOCH
    assert xa_xp[XA_TLE_SATNUM] == XP_NORAD_ID
    assert xa_xp[XA_TLE_INCLI] == INCLINATION
    assert xa_xp[XA_TLE_NODE] == RAAN
    assert xa_xp[XA_TLE_ECCEN] == ECCENTRICITY
    assert xa_xp[XA_TLE_OMEGA] == ARGUMENT_OF_PERIGEE
    assert xa_xp[XA_TLE_MNANOM] == MEAN_ANOMALY
    assert xa_xp[XA_TLE_MNMOTN] == MEAN_MOTION
    assert xa_xp[XA_TLE_EPHTYPE] == XP_EPHEMERIS_TYPE
    assert xa_xp[XA_TLE_NDOT] == XP_MEAN_MOTION_1ST_DERIVATIVE
    assert xa_xp[XA_TLE_BTERM] == pytest.approx(XP_BALLISTIC_COEFFICIENT, abs=1.0e-10)
    assert xa_xp[XA_TLE_AGOMGP] == pytest.approx(XP_SRP_COEFFICIENT, abs=1.0e-10)
    assert xs_xp.strip() == XP_XS_TLE

    assert xa_sp[XA_TLE_EPOCH] == EPOCH
    assert xa_sp[XA_TLE_SATNUM] == SP_NORAD_ID
    assert xa_sp[XA_TLE_INCLI] == INCLINATION
    assert xa_sp[XA_TLE_NODE] == RAAN
    assert xa_sp[XA_TLE_ECCEN] == ECCENTRICITY
    assert xa_sp[XA_TLE_OMEGA] == ARGUMENT_OF_PERIGEE
    assert xa_sp[XA_TLE_MNANOM] == MEAN_ANOMALY
    assert xa_sp[XA_TLE_MNMOTN] == MEAN_MOTION
    assert xa_sp[XA_TLE_EPHTYPE] == SP_EPHEMERIS_TYPE
    assert xa_sp[XA_TLE_BTERM] == pytest.approx(SP_BALLISTIC_COEFFICIENT, abs=1.0e-10)
    assert xa_sp[XA_TLE_AGOMGP] == pytest.approx(SP_SRP_COEFFICIENT, abs=1.0e-10)
    assert xs_sp.strip() == SP_XS_TLE


def test_load_lines(tle: TLEInterface) -> None:
    sgp_key = tle.load_lines(SGP_LINE_1, SGP_LINE_2)
    sgp4_key = tle.load_lines(SGP4_LINE_1, SGP4_LINE_2)
    xp_key = tle.load_lines(XP_LINE_1, XP_LINE_2)
    sp_key = tle.load_lines(SP_LINE_1, SP_LINE_2)
    count = tle.get_count()
    assert sgp_key > 0
    assert sgp4_key > 0
    assert xp_key > 0
    assert sp_key > 0
    assert count == 4


def test_tle_file(tle: TLEInterface) -> None:
    result = tle.load_file("tests/data/2025-12-30-celestrak.tle")
    count = tle.get_count()
    assert result >= 0
    assert count == 14001


def test_load_3le_file(tle: TLEInterface) -> None:
    result = tle.load_file("tests/data/2025-12-30-celestrak.3le")
    count = tle.get_count()
    assert result >= 0
    assert count == 14001
