"""Tests for TDOA (Time Difference of Arrival) observations."""
import pytest
from keplemon.bodies import Sensor, Satellite
from keplemon.estimation import Observation, TDOAObservation, BatchLeastSquares
from keplemon.time import Epoch
from keplemon.enums import TimeSystem, KeplerianType
from keplemon.elements import TopocentricElements, CartesianVector, TLE

# Speed of light in km/s
SPEED_OF_LIGHT = 299792.458


class TestTDOAObservationBasics:
    """Test basic TDOA observation creation and accessors."""

    def test_tdoa_creation(self):
        """Test creating a TDOA observation."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor1.tdoa_noise = 1e-6

        sensor2 = Sensor(angular_noise=0.001)
        sensor2.tdoa_noise = 1e-6

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        pos1 = CartesianVector(-1000, -2000, -3000)  # km
        pos2 = CartesianVector(1000, 2000, 3000)

        tdoa = TDOAObservation(
            sensor1, sensor2, epoch,
            time_difference=1e-4,
            observer_1_teme_position=pos1,
            observer_2_teme_position=pos2
        )

        assert tdoa is not None
        assert tdoa.epoch == epoch
        assert tdoa.time_difference == pytest.approx(1e-4)
        assert tdoa.sensor_1.tdoa_noise == pytest.approx(1e-6)
        assert tdoa.sensor_2.tdoa_noise == pytest.approx(1e-6)

    def test_tdoa_id_management(self):
        """Test TDOA observation ID management."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor2 = Sensor(angular_noise=0.001)
        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        pos1 = CartesianVector(0, 0, 0)
        pos2 = CartesianVector(1000, 0, 0)

        tdoa = TDOAObservation(sensor1, sensor2, epoch, 1e-4, pos1, pos2)

        # Check that ID is auto-generated
        assert tdoa.id is not None

        # Test setting custom ID via property assignment
        custom_id = "TDOA_OBS_001"
        tdoa.id = custom_id
        assert tdoa.id == custom_id

    def test_tdoa_satellite_id(self):
        """Test TDOA observation satellite ID setting."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor2 = Sensor(angular_noise=0.001)
        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        pos1 = CartesianVector(0, 0, 0)
        pos2 = CartesianVector(1000, 0, 0)

        tdoa = TDOAObservation(sensor1, sensor2, epoch, 1e-4, pos1, pos2)

        assert tdoa.observed_satellite_id is None
        tdoa.observed_satellite_id = 25544  # ISS
        assert tdoa.observed_satellite_id == 25544


class TestTDOAMeasurementModel:
    """Test TDOA measurement model (geometric calculation)."""

    def test_tdoa_measurement_and_weight(self):
        """Test TDOA measurement and weight vector generation."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor1.tdoa_noise = 1e-6

        sensor2 = Sensor(angular_noise=0.001)
        sensor2.tdoa_noise = 1e-6

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        pos1 = CartesianVector(0, 0, 0)
        pos2 = CartesianVector(1000, 0, 0)
        time_diff = 5e-4  # 0.5 milliseconds

        tdoa = TDOAObservation(sensor1, sensor2, epoch, time_diff, pos1, pos2)

        m_vec, w_vec = tdoa.get_measurement_and_weight_vector()

        # Should have 1 measurement
        assert len(m_vec) == 1
        assert len(w_vec) == 1

        # Measurement should be the time difference
        assert m_vec[0] == pytest.approx(time_diff)

        # Weight should be 1/sigma^2
        expected_weight = 1.0 / (1e-6 ** 2)
        assert w_vec[0] == pytest.approx(expected_weight)

    def test_tdoa_weight_without_noise(self):
        """Test TDOA measurement with zero weight when noise not set."""
        sensor1 = Sensor(angular_noise=0.001)
        # Note: NOT setting tdoa_noise

        sensor2 = Sensor(angular_noise=0.001)

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        pos1 = CartesianVector(0, 0, 0)
        pos2 = CartesianVector(1000, 0, 0)

        tdoa = TDOAObservation(sensor1, sensor2, epoch, 1e-4, pos1, pos2)

        m_vec, w_vec = tdoa.get_measurement_and_weight_vector()

        # Should have 1 measurement but zero weight
        assert len(m_vec) == 1
        assert len(w_vec) == 1
        assert w_vec[0] == pytest.approx(0.0)
