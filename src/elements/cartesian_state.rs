use super::{CartesianVector, KeplerianElements, KeplerianState};
use crate::enums::{KeplerianType, ReferenceFrame};
use crate::time::Epoch;
use saal::astro;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CartesianState {
    pub epoch: Epoch,
    pub position: CartesianVector,
    pub velocity: CartesianVector,
    frame: ReferenceFrame,
}

impl CartesianState {
    pub fn new(epoch: Epoch, position: CartesianVector, velocity: CartesianVector, frame: ReferenceFrame) -> Self {
        Self {
            epoch,
            position,
            velocity,
            frame,
        }
    }

    pub fn get_frame(&self) -> ReferenceFrame {
        self.frame
    }

    pub fn to_frame(&self, frame: ReferenceFrame) -> CartesianState {
        match self.frame {
            ReferenceFrame::TEME => match frame {
                ReferenceFrame::TEME => *self,
                ReferenceFrame::J2000 => {
                    let posvel = astro::teme_to_j2000(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::ECR => {
                    let posvel = astro::teme_to_ecr(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::EFG => {
                    let posvel = astro::teme_to_efg(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
            },
            ReferenceFrame::J2000 => match frame {
                ReferenceFrame::TEME => {
                    let posvel = astro::j2000_to_teme(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::J2000 => *self,
                ReferenceFrame::ECR => {
                    let posvel = astro::j2000_to_ecr(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }

                ReferenceFrame::EFG => {
                    let posvel = astro::j2000_to_efg(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
            },
            ReferenceFrame::ECR => match frame {
                ReferenceFrame::TEME => {
                    let posvel = astro::ecr_to_teme(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::J2000 => {
                    let posvel = astro::ecr_to_j2000(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::ECR => *self,
                ReferenceFrame::EFG => {
                    let posvel = astro::ecr_to_efg(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
            },
            ReferenceFrame::EFG => match frame {
                ReferenceFrame::TEME => {
                    let posvel = astro::efg_to_teme(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::J2000 => {
                    let posvel = astro::efg_to_j2000(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::ECR => {
                    let posvel = astro::efg_to_ecr(self.epoch.days_since_1950, &self.into());
                    CartesianState::from((self.epoch, posvel, frame))
                }
                ReferenceFrame::EFG => *self,
            },
        }
    }
}

impl From<(Epoch, [f64; 6], ReferenceFrame)> for CartesianState {
    fn from(data: (Epoch, [f64; 6], ReferenceFrame)) -> Self {
        Self {
            epoch: data.0,
            position: CartesianVector::from([data.1[0], data.1[1], data.1[2]]),
            velocity: CartesianVector::from([data.1[3], data.1[4], data.1[5]]),
            frame: data.2,
        }
    }
}

impl From<CartesianState> for KeplerianState {
    fn from(cartesian: CartesianState) -> Self {
        let kep = KeplerianElements::from(astro::cartesian_to_keplerian(&cartesian.into()));
        KeplerianState::new(cartesian.epoch, kep, cartesian.frame, KeplerianType::Osculating)
    }
}

impl From<CartesianState> for [f64; 6] {
    fn from(state: CartesianState) -> Self {
        [
            state.position.get_x(),
            state.position.get_y(),
            state.position.get_z(),
            state.velocity.get_x(),
            state.velocity.get_y(),
            state.velocity.get_z(),
        ]
    }
}

impl From<&CartesianState> for [f64; 6] {
    fn from(state: &CartesianState) -> Self {
        [
            state.position.get_x(),
            state.position.get_y(),
            state.position.get_z(),
            state.velocity.get_x(),
            state.velocity.get_y(),
            state.velocity.get_z(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::{CartesianState, CartesianVector};
    use crate::elements::KeplerianState;
    use crate::enums::{ReferenceFrame, TimeSystem};
    use crate::time::Epoch;
    use approx::assert_abs_diff_eq;

    fn get_state(frame: ReferenceFrame) -> CartesianState {
        let epoch = Epoch::from_days_since_1950(25142.432, TimeSystem::UTC);
        let position = CartesianVector::new(42164.0, 0.0, 0.0);
        let velocity = CartesianVector::new(0.0, 3.0746676656429814, 0.0);
        CartesianState::new(epoch, position, velocity, frame)
    }

    #[test]
    fn test_to_keplerian() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let state = get_state(ReferenceFrame::TEME);
        let osc = KeplerianState::from(state);
        assert_abs_diff_eq!(osc.get_semi_major_axis(), 42164.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_mean_anomaly(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_eccentricity(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_inclination(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_raan(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_argument_of_perigee(), 0.0, epsilon = 1e-6);
        assert_eq!(osc.get_frame(), ReferenceFrame::TEME);
        assert_eq!(osc.epoch, state.epoch);
    }
}
