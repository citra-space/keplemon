use super::KeplerianElements;
use super::{CartesianState, CartesianVector};
use crate::enums::{KeplerianType, ReferenceFrame, TimeSystem};
use crate::saal::{astro_func_interface, TLEInterface};
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerianState {
    epoch: Epoch,
    elements: KeplerianElements,
    frame: ReferenceFrame,
    keplerian_type: KeplerianType,
}

impl KeplerianState {
    pub fn get_elements(&self) -> KeplerianElements {
        self.elements
    }

    pub fn from_xa_tle(xa_tle: &[f64; TLEInterface::XA_TLE_SIZE]) -> Self {
        let epoch = Epoch::from_days_since_1950(xa_tle[TLEInterface::XA_TLE_EPOCH], TimeSystem::UTC);
        let keplerian_type = KeplerianType::try_from(xa_tle[TLEInterface::XA_TLE_EPHTYPE]).unwrap();
        let eccentricity = xa_tle[TLEInterface::XA_TLE_ECCEN];
        let inclination = xa_tle[TLEInterface::XA_TLE_INCLI];
        let raan = xa_tle[TLEInterface::XA_TLE_NODE];
        let argument_of_perigee = xa_tle[TLEInterface::XA_TLE_OMEGA];
        let mean_anomaly = xa_tle[TLEInterface::XA_TLE_MNANOM];
        let mean_motion = match keplerian_type {
            KeplerianType::MeanKozaiGP => {
                astro_func_interface::kozai_to_brouwer(eccentricity, inclination, xa_tle[TLEInterface::XA_TLE_MNMOTN])
            }
            _ => xa_tle[TLEInterface::XA_TLE_MNMOTN],
        };
        let semi_major_axis = astro_func_interface::mean_motion_to_sma(mean_motion);
        let elements = KeplerianElements::new(
            semi_major_axis,
            eccentricity,
            inclination,
            raan,
            argument_of_perigee,
            mean_anomaly,
        );
        Self {
            epoch,
            elements,
            frame: ReferenceFrame::TEME,
            keplerian_type,
        }
    }
}

#[pymethods]
impl KeplerianState {
    #[new]
    pub fn new(
        epoch: Epoch,
        elements: KeplerianElements,
        frame: ReferenceFrame,
        keplerian_type: KeplerianType,
    ) -> Self {
        Self {
            epoch,
            elements,
            frame,
            keplerian_type,
        }
    }

    pub fn to_cartesian(&self) -> CartesianState {
        let xa_kep = self.elements.get_xa_kep();
        let (pos, vel) = astro_func_interface::keplerian_to_cartesian(&xa_kep);
        CartesianState::new(
            self.epoch,
            CartesianVector::from(pos),
            CartesianVector::from(vel),
            self.frame,
        )
    }

    pub fn to_frame(&self, frame: ReferenceFrame) -> KeplerianState {
        self.to_cartesian().to_frame(frame).to_keplerian()
    }

    #[getter]
    pub fn get_semi_major_axis(&self) -> f64 {
        self.elements.get_semi_major_axis()
    }

    #[getter]
    pub fn get_mean_anomaly(&self) -> f64 {
        self.elements.get_mean_anomaly()
    }

    #[getter]
    pub fn get_eccentricity(&self) -> f64 {
        self.elements.get_eccentricity()
    }

    #[getter]
    pub fn get_inclination(&self) -> f64 {
        self.elements.get_inclination()
    }

    #[getter]
    pub fn get_raan(&self) -> f64 {
        self.elements.get_raan()
    }

    #[getter]
    pub fn get_argument_of_perigee(&self) -> f64 {
        self.elements.get_argument_of_perigee()
    }

    #[getter]
    pub fn get_apoapsis(&self) -> f64 {
        self.elements.get_apoapsis()
    }

    #[getter]
    pub fn get_periapsis(&self) -> f64 {
        self.elements.get_periapsis()
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_mean_motion(&self) -> f64 {
        self.elements.get_mean_motion(self.keplerian_type)
    }

    #[getter]
    pub fn get_frame(&self) -> ReferenceFrame {
        self.frame
    }

    #[getter]
    pub fn get_type(&self) -> KeplerianType {
        self.keplerian_type
    }

    #[setter]
    pub fn set_epoch(&mut self, epoch: Epoch) {
        self.epoch = epoch;
    }

    #[setter]
    pub fn set_type(&mut self, keplerian_type: KeplerianType) {
        self.keplerian_type = keplerian_type;
    }
}

#[cfg(test)]
mod tests {

    use super::KeplerianState;
    use crate::elements::KeplerianElements;
    use crate::enums::{KeplerianType, ReferenceFrame, TimeSystem};
    use crate::time::Epoch;
    use approx::assert_abs_diff_eq;

    fn epoch_1() -> Epoch {
        Epoch::from_days_since_1950(25142.432, TimeSystem::UTC)
    }
    fn geo_state() -> KeplerianState {
        let geo_elements = KeplerianElements::new(42164.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        KeplerianState::new(epoch_1(), geo_elements, ReferenceFrame::TEME, KeplerianType::Osculating)
    }

    #[test]
    fn test_to_cartesian() {
        let osc = geo_state();
        let state = osc.to_cartesian();
        assert_eq!(state.get_frame(), ReferenceFrame::TEME);
        assert_abs_diff_eq!(state.position[0], 42164.0, epsilon = 1e-6);
        assert_abs_diff_eq!(state.position[1], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(state.position[2], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(state.velocity[0], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(state.velocity[1], 3.0746676656429814, epsilon = 1e-6);
        assert_abs_diff_eq!(state.velocity[2], 0.0, epsilon = 1e-6);
        assert_eq!(osc.epoch, state.epoch);
    }
}
