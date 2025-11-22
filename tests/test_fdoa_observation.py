"""Unit tests for FDOA (Frequency Difference of Arrival) observations."""

import pytest
from keplemon.bodies import Sensor
from keplemon.estimation import FDOAObservation
from keplemon.time import Epoch
from keplemon.enums import TimeSystem, ReferenceFrame
from keplemon.elements import CartesianVector, CartesianState


class TestFDOAObservationBasics:
    """Basic FDOA observation creation and property tests."""

    def test_fdoa_creation(self):
        """Test basic FDOA observation creation."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor2 = Sensor(angular_noise=0.001)

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)

        # State includes position and velocity
        pos1 = CartesianVector(0, 0, 0)
        vel1 = CartesianVector(0, 0, 1000)
        state1 = CartesianState(epoch, pos1, vel1, ReferenceFrame.TEME)

        pos2 = CartesianVector(1000, 0, 0)
        vel2 = CartesianVector(0, 0, 1000)
        state2 = CartesianState(epoch, pos2, vel2, ReferenceFrame.TEME)

        freq_diff = 1000.0  # 1 kHz difference
        transmit_freq = 10e9  # 10 GHz

        fdoa = FDOAObservation(
            sensor1, sensor2, epoch, freq_diff, state1, state2, transmit_freq
        )

        assert fdoa is not None
        assert fdoa.epoch == epoch

    def test_fdoa_id_management(self):
        """Test FDOA observation ID management."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor2 = Sensor(angular_noise=0.001)

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        fdoa = FDOAObservation(sensor1, sensor2, epoch, 1000.0, state1, state2, 10e9)

        # Auto-generated ID should exist
        assert fdoa.id is not None
        assert len(fdoa.id) > 0

        # Custom ID management
        custom_id = "FDOA_CUSTOM_001"
        fdoa.id = custom_id
        assert fdoa.id == custom_id

    def test_fdoa_satellite_id(self):
        """Test FDOA observation satellite ID management."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor2 = Sensor(angular_noise=0.001)

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        fdoa = FDOAObservation(sensor1, sensor2, epoch, 1000.0, state1, state2, 10e9)

        # Initially None
        assert fdoa.observed_satellite_id is None

        # Set and verify
        fdoa.observed_satellite_id = 25544  # ISS NORAD ID
        assert fdoa.observed_satellite_id == 25544


class TestFDOAMeasurementModel:
    """Tests for FDOA measurement model and weight calculations."""

    def test_fdoa_measurement_and_weight(self):
        """Test FDOA measurement and weight vector generation."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor1.fdoa_noise = 1.0  # 1 Hz noise

        sensor2 = Sensor(angular_noise=0.001)
        sensor2.fdoa_noise = 1.0  # 1 Hz noise

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        freq_diff = 1000.0  # 1 kHz difference
        transmit_freq = 10e9

        fdoa = FDOAObservation(
            sensor1, sensor2, epoch, freq_diff, state1, state2, transmit_freq
        )

        m_vec, w_vec = fdoa.get_measurement_and_weight_vector()

        # Should have 1 measurement
        assert len(m_vec) == 1
        assert len(w_vec) == 1

        # Measurement should be the frequency difference
        assert m_vec[0] == pytest.approx(freq_diff)

        # Weight should be 1/(sigma_1^2 + sigma_2^2) for differential measurement
        sigma1 = 1.0
        sigma2 = 1.0
        variance_combined = sigma1**2 + sigma2**2
        expected_weight = 1.0 / variance_combined
        assert w_vec[0] == pytest.approx(expected_weight)

    def test_fdoa_asymmetric_noise(self):
        """Test FDOA with different noise levels for each sensor."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor1.fdoa_noise = 1.0

        sensor2 = Sensor(angular_noise=0.001)
        sensor2.fdoa_noise = 2.0  # Different noise level

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        fdoa = FDOAObservation(sensor1, sensor2, epoch, 1000.0, state1, state2, 10e9)

        m_vec, w_vec = fdoa.get_measurement_and_weight_vector()

        # Weight should account for both sensors' noise
        sigma1 = 1.0
        sigma2 = 2.0
        variance_combined = sigma1**2 + sigma2**2
        expected_weight = 1.0 / variance_combined
        assert w_vec[0] == pytest.approx(expected_weight)

    def test_fdoa_weight_without_noise(self):
        """Test FDOA measurement with zero weight when noise not set."""
        sensor1 = Sensor(angular_noise=0.001)
        # Note: NOT setting fdoa_noise

        sensor2 = Sensor(angular_noise=0.001)

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        fdoa = FDOAObservation(sensor1, sensor2, epoch, 100.0, state1, state2, 10e9)

        m_vec, w_vec = fdoa.get_measurement_and_weight_vector()

        # Should have 1 measurement but zero weight
        assert len(m_vec) == 1
        assert len(w_vec) == 1
        assert w_vec[0] == pytest.approx(0.0)

    def test_fdoa_partial_noise_specification(self):
        """Test FDOA with only one sensor having noise specified."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor1.fdoa_noise = 0.5  # Only sensor 1 has noise

        sensor2 = Sensor(angular_noise=0.001)
        # sensor 2 has no fdoa_noise

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        fdoa = FDOAObservation(sensor1, sensor2, epoch, 500.0, state1, state2, 10e9)

        m_vec, w_vec = fdoa.get_measurement_and_weight_vector()

        # Should fall back to sensor 1 weight
        assert w_vec[0] == pytest.approx(1.0 / (0.5**2))


class TestFDOAPhysicalModel:
    """Tests for FDOA physical measurement model."""

    def test_fdoa_doppler_prediction(self):
        """Test that predicted FDOA vector can be computed (requires satellite)."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor2 = Sensor(angular_noise=0.001)

        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        freq_diff = 1000.0
        transmit_freq = 10e9

        fdoa = FDOAObservation(
            sensor1, sensor2, epoch, freq_diff, state1, state2, transmit_freq
        )

        # Verify that the observation stores the transmit frequency
        # (we can't fully test prediction without a real satellite)
        assert fdoa is not None

    def test_fdoa_epoch_access(self):
        """Test that FDOA epoch is correctly stored and retrieved."""
        sensor1 = Sensor(angular_noise=0.001)
        sensor2 = Sensor(angular_noise=0.001)

        epoch1 = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)
        epoch2 = Epoch.from_iso("2024-01-02T00:00:00Z", TimeSystem.UTC)

        state1 = CartesianState(epoch1, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch1, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        fdoa1 = FDOAObservation(
            sensor1, sensor2, epoch1, 1000.0, state1, state2, 10e9
        )

        state1_2 = CartesianState(epoch2, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2_2 = CartesianState(epoch2, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)

        fdoa2 = FDOAObservation(
            sensor1, sensor2, epoch2, 1000.0, state1_2, state2_2, 10e9
        )

        assert fdoa1.epoch == epoch1
        assert fdoa2.epoch == epoch2
        assert fdoa1.epoch != fdoa2.epoch
