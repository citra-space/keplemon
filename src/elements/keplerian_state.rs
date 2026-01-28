use super::CartesianState;
use super::KeplerianElements;
use crate::enums::{KeplerianType, ReferenceFrame, TimeSystem};
use crate::time::{DAYS_TO_MINUTES, Epoch, TimeSpan};
use saal::{astro, tle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerianState {
    pub epoch: Epoch,
    pub elements: KeplerianElements,
    frame: ReferenceFrame,
    pub keplerian_type: KeplerianType,
}

impl From<&KeplerianState> for CartesianState {
    fn from(kep_state: &KeplerianState) -> Self {
        let xa_kep = kep_state.elements.get_xa_kep();
        let posvel = astro::keplerian_to_cartesian(&xa_kep);
        CartesianState::from((kep_state.epoch, posvel, kep_state.frame))
    }
}

impl From<KeplerianState> for CartesianState {
    fn from(kep_state: KeplerianState) -> Self {
        CartesianState::from(&kep_state)
    }
}

impl From<&[f64; tle::XA_TLE_SIZE]> for KeplerianState {
    fn from(xa_tle: &[f64; tle::XA_TLE_SIZE]) -> Self {
        let epoch = Epoch::from_days_since_1950(xa_tle[tle::XA_TLE_EPOCH], TimeSystem::UTC);
        let keplerian_type = KeplerianType::try_from(xa_tle[tle::XA_TLE_EPHTYPE]).unwrap();
        let eccentricity = xa_tle[tle::XA_TLE_ECCEN];
        let inclination = xa_tle[tle::XA_TLE_INCLI];
        let raan = xa_tle[tle::XA_TLE_NODE];
        let argument_of_perigee = xa_tle[tle::XA_TLE_OMEGA];
        let mean_anomaly = xa_tle[tle::XA_TLE_MNANOM];
        let mean_motion = match keplerian_type {
            KeplerianType::MeanKozaiGP => {
                astro::kozai_to_brouwer(eccentricity, inclination, xa_tle[tle::XA_TLE_MNMOTN])
            }
            _ => xa_tle[tle::XA_TLE_MNMOTN],
        };
        let semi_major_axis = astro::mean_motion_to_sma(mean_motion);
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

impl KeplerianState {
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

    pub fn to_frame(&self, frame: ReferenceFrame) -> KeplerianState {
        let cartesian = CartesianState::from(self);
        cartesian.to_frame(frame).into()
    }

    pub fn get_semi_major_axis(&self) -> f64 {
        self.elements.semi_major_axis
    }

    pub fn get_mean_anomaly(&self) -> f64 {
        self.elements.mean_anomaly
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.elements.eccentricity
    }

    pub fn get_inclination(&self) -> f64 {
        self.elements.inclination
    }

    pub fn get_raan(&self) -> f64 {
        self.elements.raan
    }

    pub fn get_argument_of_perigee(&self) -> f64 {
        self.elements.argument_of_perigee
    }

    pub fn get_apoapsis(&self) -> f64 {
        self.elements.get_apoapsis()
    }

    pub fn get_periapsis(&self) -> f64 {
        self.elements.get_periapsis()
    }

    pub fn get_mean_motion(&self) -> f64 {
        self.elements.get_mean_motion(self.keplerian_type)
    }

    pub fn get_frame(&self) -> ReferenceFrame {
        self.frame
    }

    pub fn get_type(&self) -> KeplerianType {
        self.keplerian_type
    }

    pub fn get_period(&self) -> TimeSpan {
        TimeSpan::from_minutes(DAYS_TO_MINUTES / self.get_mean_motion())
    }
}

#[cfg(test)]
mod tests {

    use super::KeplerianState;
    use crate::elements::{CartesianState, KeplerianElements};
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
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let osc = geo_state();
        let state: CartesianState = osc.into();
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
