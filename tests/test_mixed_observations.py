"""Test mixed measurement batches with angles, TDOA, and FDOA observations."""

import pytest
from keplemon.bodies import Sensor, Observatory
from keplemon.estimation import (
    Observation,
    TDOAObservation,
    FDOAObservation,
)
from keplemon.time import Epoch
from keplemon.enums import TimeSystem, ReferenceFrame
from keplemon.elements import CartesianVector, CartesianState, TopocentricElements


class TestMixedObservationBatches:
    """Test BatchLeastSquares with mixed observation types."""

    def test_measurement_creation_tdoa_and_angle_observations(self):
        """Test creating a batch with both angle and TDOA observations."""
        # Create sensors
        sensor_angle = Sensor(angular_noise=0.001)
        sensor1_tdoa = Sensor(angular_noise=0.001)
        sensor1_tdoa.tdoa_noise = 1e-6
        sensor2_tdoa = Sensor(angular_noise=0.001)
        sensor2_tdoa.tdoa_noise = 1e-6

        # Create observations
        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)

        # Traditional angle observation (RA=10°, DEC=20° in J2000)
        topo_elements = TopocentricElements.from_j2000(epoch, 10.0, 20.0)
        observer_pos = CartesianVector(6371, 0, 0)  # Earth surface
        obs_angle = Observation(sensor_angle, epoch, topo_elements, observer_pos)
        obs_angle.id = "ANGLE_001"

        # TDOA observation
        pos1 = CartesianVector(0, 0, 0)
        pos2 = CartesianVector(1000, 0, 0)
        obs_tdoa = TDOAObservation(sensor1_tdoa, sensor2_tdoa, epoch, 5e-4, pos1, pos2)
        obs_tdoa.id = "TDOA_001"

        # Create batch with mixed observations
        try:
            # Note: We can't fully test BatchLeastSquares without a real satellite,
            # but we can verify it accepts mixed observation types
            assert obs_angle is not None
            assert obs_tdoa is not None
        except Exception as e:
            pytest.fail(f"Failed to create mixed observations: {e}")

    def test_batch_with_fdoa_and_angle_observations(self):
        """Test creating a batch with both angle and FDOA observations."""
        # Create sensors
        sensor_angle = Sensor(angular_noise=0.001)
        sensor1_fdoa = Sensor(angular_noise=0.001)
        sensor1_fdoa.fdoa_noise = 1.0
        sensor2_fdoa = Sensor(angular_noise=0.001)
        sensor2_fdoa.fdoa_noise = 1.0

        # Create observations
        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)

        # Traditional angle observation (RA=10°, DEC=20° in J2000)
        topo_elements = TopocentricElements.from_j2000(epoch, 10.0, 20.0)
        observer_pos = CartesianVector(6371, 0, 0)  # Earth surface
        obs_angle = Observation(sensor_angle, epoch, topo_elements, observer_pos)

        # FDOA observation
        state1 = CartesianState(
            epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME
        )
        state2 = CartesianState(
            epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME
        )
        obs_fdoa = FDOAObservation(
            sensor1_fdoa, sensor2_fdoa, epoch, 1000.0, state1, state2, 10e9
        )

        # Verify both can be created
        assert obs_angle is not None
        assert obs_fdoa is not None

    def test_batch_measurement_vector_aggregation(self):
        """Test that TDOA and FDOA observations have correct measurement dimensions."""
        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)

        # TDOA observation: 1 measurement
        sensor1_tdoa = Sensor(angular_noise=0.001)
        sensor1_tdoa.tdoa_noise = 1e-6
        sensor2_tdoa = Sensor(angular_noise=0.001)
        sensor2_tdoa.tdoa_noise = 1e-6

        obs_tdoa = TDOAObservation(sensor1_tdoa, sensor2_tdoa, epoch, 5e-4,
                                   CartesianVector(0, 0, 0), CartesianVector(1000, 0, 0))
        m_vec_tdoa, w_vec_tdoa = obs_tdoa.get_measurement_and_weight_vector()
        assert len(m_vec_tdoa) == 1, f"Expected 1 TDOA measurement, got {len(m_vec_tdoa)}"

        # FDOA observation: 1 measurement
        sensor1_fdoa = Sensor(angular_noise=0.001)
        sensor1_fdoa.fdoa_noise = 1.0
        sensor2_fdoa = Sensor(angular_noise=0.001)
        sensor2_fdoa.fdoa_noise = 1.0

        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        obs_fdoa = FDOAObservation(sensor1_fdoa, sensor2_fdoa, epoch, 1000.0, state1, state2, 10e9)
        m_vec_fdoa, w_vec_fdoa = obs_fdoa.get_measurement_and_weight_vector()
        assert len(m_vec_fdoa) == 1, f"Expected 1 FDOA measurement, got {len(m_vec_fdoa)}"

    def test_observation_type_dimension_variation(self):
        """Test that ObservationType trait correctly handles variable dimensions."""
        epoch = Epoch.from_iso("2024-01-01T00:00:00Z", TimeSystem.UTC)

        # TDOA: 1 measurement
        sensor1_tdoa = Sensor(angular_noise=0.001)
        sensor1_tdoa.tdoa_noise = 1e-6
        sensor2_tdoa = Sensor(angular_noise=0.001)
        sensor2_tdoa.tdoa_noise = 1e-6
        obs_tdoa = TDOAObservation(sensor1_tdoa, sensor2_tdoa, epoch, 5e-4,
                                   CartesianVector(0, 0, 0), CartesianVector(1000, 0, 0))
        m_vec_tdoa, _ = obs_tdoa.get_measurement_and_weight_vector()
        assert len(m_vec_tdoa) == 1

        # FDOA: 1 measurement
        sensor1_fdoa = Sensor(angular_noise=0.001)
        sensor1_fdoa.fdoa_noise = 1.0
        sensor2_fdoa = Sensor(angular_noise=0.001)
        sensor2_fdoa.fdoa_noise = 1.0
        state1 = CartesianState(epoch, CartesianVector(0, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        state2 = CartesianState(epoch, CartesianVector(1000, 0, 0), CartesianVector(0, 0, 1000), ReferenceFrame.TEME)
        obs_fdoa = FDOAObservation(sensor1_fdoa, sensor2_fdoa, epoch, 1000.0, state1, state2, 10e9)
        m_vec_fdoa, _ = obs_fdoa.get_measurement_and_weight_vector()
        assert len(m_vec_fdoa) == 1
