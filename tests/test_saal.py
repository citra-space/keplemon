import pytest

from keplemon.saal import astro_func_interface, SAALObservation, SensorInterface, ObsInterface, MainInterface
from keplemon.time import Epoch
from keplemon.enums import TimeSystem, SAALKeyMode


@pytest.fixture()
def sensor_card():
    return "211  3381724 -25333969 -1521161 -5083089  3530462  U SOCORRO CAM1              S"


@pytest.fixture()
def noise_card():
    return "211 5   0.0003 0.0003 0.0000 0.0000  -0.0005 -0.0003  0.0000  0.0000  0.0000  BS"


class TestMainInterface:
    def test_get_key_mode(self):
        mode = MainInterface.get_key_mode()
        assert mode == SAALKeyMode.DirectMemoryAccess


class TestSensorInterface:

    def test_xa_indices(self):
        assert SensorInterface.XA_SEN_GEN_SENNUM == 0
        assert SensorInterface.XA_SEN_GEN_MINRNG == 3
        assert SensorInterface.XA_SEN_GEN_MAXRNG == 4
        assert SensorInterface.XA_SEN_GEN_RRLIM == 5
        assert SensorInterface.XA_SEN_GEN_RNGLIMFLG == 6
        assert SensorInterface.XA_SEN_GEN_SMSEN == 7
        assert SensorInterface.XA_SEN_GRN_LOCTYPE == 10
        assert SensorInterface.XA_SEN_GRN_POS1 == 11
        assert SensorInterface.XA_SEN_GRN_POS2 == 12
        assert SensorInterface.XA_SEN_GRN_POS3 == 13
        assert SensorInterface.XA_SEN_GRN_ASTROLAT == 14
        assert SensorInterface.XA_SEN_GRN_ASTROLON == 15
        assert SensorInterface.XA_SEN_GRN_ECITIME == 16
        assert SensorInterface.XA_SEN_GEN_ELSIGMA == 111
        assert SensorInterface.XA_SEN_GEN_AZSIGMA == 110
        assert SensorInterface.XA_SEN_GEN_RGSIGMA == 112
        assert SensorInterface.XA_SEN_GEN_RRSIGMA == 113
        assert SensorInterface.XA_SEN_GEN_ARSIGMA == 114
        assert SensorInterface.XA_SEN_GEN_ERSIGMA == 115
        assert SensorInterface.XA_SEN_GEN_AZBIAS == 116
        assert SensorInterface.XA_SEN_GEN_ELBIAS == 117
        assert SensorInterface.XA_SEN_GEN_RGBIAS == 118
        assert SensorInterface.XA_SEN_GEN_RRBIAS == 119
        assert SensorInterface.XA_SEN_GEN_TIMEBIAS == 120
        assert SensorInterface.XA_SEN_SIZE == 128

    def test_load_file(self):
        SensorInterface.load_file("tests/sensors.dat")
        assert SensorInterface.count_loaded() == 108
        SensorInterface.get_loaded_keys()
        sensors = SensorInterface.get_all()
        SensorInterface.remove_all()
        assert SensorInterface.count_loaded() == 0
        assert sensors[0].number == 211
        assert sensors[0].description == "SOCORRO CAM1"

    def test_get_arrays(self, sensor_card, noise_card):
        SensorInterface.load_card(sensor_card)
        SensorInterface.load_card(noise_card)
        key = SensorInterface.get_loaded_keys()[-1]

        xa_sen, xs_sen = SensorInterface.get_arrays(key)
        assert len(xa_sen) == SensorInterface.XA_SEN_SIZE
        assert xa_sen[SensorInterface.XA_SEN_GEN_SENNUM] == 211.0
        assert xa_sen[SensorInterface.XA_SEN_GRN_POS1] == -1521.161
        assert xa_sen[SensorInterface.XA_SEN_GRN_POS2] == -5083.089
        assert xa_sen[SensorInterface.XA_SEN_GRN_POS3] == 3530.462
        assert xa_sen[SensorInterface.XA_SEN_GRN_ASTROLAT] == 33.81724
        assert xa_sen[SensorInterface.XA_SEN_GRN_ASTROLON] == -253.33969
        assert xa_sen[SensorInterface.XA_SEN_GRN_ECITIME] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_RNGLIMFLG] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_SMSEN] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_MINRNG] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_MAXRNG] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_RRLIM] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GRN_LOCTYPE] == 1.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_AZSIGMA] == 0.0003
        assert xa_sen[SensorInterface.XA_SEN_GEN_ELSIGMA] == 0.0003
        assert xa_sen[SensorInterface.XA_SEN_GEN_ARSIGMA] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_ERSIGMA] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_RGSIGMA] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_RRSIGMA] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_AZBIAS] == -0.0005
        assert xa_sen[SensorInterface.XA_SEN_GEN_ELBIAS] == -0.0003
        assert xa_sen[SensorInterface.XA_SEN_GEN_RGBIAS] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_RRBIAS] == 0.0
        assert xa_sen[SensorInterface.XA_SEN_GEN_TIMEBIAS] == 0.0
        assert xs_sen.strip() == "U33SOCORRO CAM1"
        SensorInterface.remove_key(key)
        assert SensorInterface.count_loaded() == 0

    def test_load_card(self, sensor_card, noise_card):
        SensorInterface.load_card(sensor_card)
        SensorInterface.load_card(noise_card)
        assert SensorInterface.count_loaded() == 1
        key = SensorInterface.get_loaded_keys()[-1]
        sensor = SensorInterface.get(key)

        SensorInterface.remove_key(key)
        assert SensorInterface.count_loaded() == 0
        assert sensor.key == key
        assert sensor.number == 211
        assert sensor.apply_range_limits
        assert not sensor.minimum_range
        assert not sensor.maximum_range
        assert not sensor.range_rate_limit
        assert not sensor.mobile
        assert sensor.latitude == pytest.approx(33.817242266703744, abs=1e-4)
        assert sensor.longitude == pytest.approx(253.33970533290127, abs=1e-4)
        assert sensor.altitude == pytest.approx(1.509809294541032, abs=1e-4)
        assert sensor.astronomical_latitude == pytest.approx(33.81724, abs=1e-4)
        assert sensor.astronomical_longitude == pytest.approx(-253.33969, abs=1e-4)
        assert sensor.range_noise is None
        assert sensor.azimuth_noise == pytest.approx(0.0003, abs=1e-6)
        assert sensor.elevation_noise == pytest.approx(0.0003, abs=1e-6)
        assert sensor.angular_noise == pytest.approx(0.00042426406871192857, abs=1e-6)
        assert sensor.range_rate_noise is None
        assert sensor.azimuth_rate_noise is None
        assert sensor.elevation_rate_noise is None


class TestObsInterface:
    def test_load_from_b3(self):
        b3_data = "U0132834622001070200572187128 1459398 21295312 0148621 -0316 -1825 00000  4 5  10132801328"
        key = ObsInterface.load_from_b3(b3_data)
        assert ObsInterface.count_loaded() == 1
        ObsInterface.remove_key(key)
        assert ObsInterface.count_loaded() == 0

    def test_saal_observation(self):
        b3_data = "U0132834622001070200572187128 1459398 21295312 0148621 -0316 -1825 00000  4 5  10132801328"
        b3 = SAALObservation(b3_data)
        epoch = Epoch.from_days_since_1950(b3.epoch_ds50utc, TimeSystem.UTC)
        assert epoch.to_iso() == "2022-01-01T07:02:00.572"
        assert epoch.to_dtg_15() == "22001070200.572"

        assert b3.security_character == "U"
        assert b3.satellite_number == 1328
        assert b3.sensor_number == 346
        assert b3.epoch_ds50utc == 26299.293062175926
        assert b3.elevation_or_declination == 18.7128
        assert b3.azimuth_or_right_ascension == 145.9398
        assert b3.slant_range == 2129.531
        assert b3.range_rate == 1.48621
        assert b3.elevation_rate == -0.0316
        assert b3.azimuth_rate == -0.1825
        assert b3.range_acceleration == 0.0
        assert b3.observation_type == "4"
        assert b3.track_position_indicator == 5
        assert b3.association_status == 1
        assert b3.site_tag == 1328
        assert b3.spadoc_tag == 1328
        assert b3.position == [0.0, 0.0, 0.0]


class TestAstroFuncInterface:
    def test_get_jpl_sun_and_moon_position(self):
        epoch = 24000.0  # Example Julian Date
        sun_pos, moon_pos = astro_func_interface.get_jpl_sun_and_moon_position(epoch)

        # Check that the returned positions are tuples of length 3
        assert sun_pos[0] == pytest.approx(-149257535.84066284, abs=1e-3)
        assert moon_pos[0] == pytest.approx(-375197.53303902777, abs=1e-3)

    def test_horizon_to_teme(self):
        theta = 4.01991574771239
        lat = 54.0
        xa_rae = [
            0.430460160479830 * 6378.135,  # rng
            311.60356010055284,  # az
            0.0003630520892354455,  # el
            -2.77471740320679,  # rng_rate
            -0.143557569934800,  # az_rate
            2.461934326381368e-002,  # el_rate
        ]
        sen_pos = [-2398.87840986937, -2891.94814468770, 5136.98500000000]

        teme = astro_func_interface.horizon_to_teme(theta, lat, sen_pos, xa_rae)

        expected_pos = [-3037.43093289, -446.126832813, 6208.50743365]
        expected_vel = [-5.937185805, -3.51389427125, -3.15199314614]

        assert teme[0] == pytest.approx(expected_pos[0], abs=1e-7)
        assert teme[1] == pytest.approx(expected_pos[1], abs=1e-7)
        assert teme[2] == pytest.approx(expected_pos[2], abs=1e-7)
        assert teme[3] == pytest.approx(expected_vel[0], abs=1e-7)
        assert teme[4] == pytest.approx(expected_vel[1], abs=1e-7)
        assert teme[5] == pytest.approx(expected_vel[2], abs=1e-7)
