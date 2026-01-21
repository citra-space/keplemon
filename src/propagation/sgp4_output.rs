use crate::elements::{CartesianState, CartesianVector, GeodeticPosition, KeplerianElements};
use crate::enums::{ReferenceFrame, TimeSystem};
use crate::time::Epoch;
use saal::sgp4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SGP4Output {
    cartesian_state: CartesianState,
    mean_elements: KeplerianElements,
    osculating_elements: KeplerianElements,
    geodetic_position: GeodeticPosition,
}

impl SGP4Output {
    pub fn get_cartesian_state(&self) -> CartesianState {
        self.cartesian_state
    }

    pub fn get_mean_elements(&self) -> KeplerianElements {
        self.mean_elements
    }

    pub fn get_osculating_elements(&self) -> KeplerianElements {
        self.osculating_elements
    }

    pub fn get_geodetic_position(&self) -> GeodeticPosition {
        self.geodetic_position
    }
}

impl From<[f64; sgp4::XA_SGP4OUT_SIZE]> for SGP4Output {
    fn from(xa_sgp4out: [f64; sgp4::XA_SGP4OUT_SIZE]) -> Self {
        let epoch = Epoch::from_days_since_1950(xa_sgp4out[sgp4::XA_SGP4OUT_DS50UTC], TimeSystem::UTC);
        let position = CartesianVector::new(
            xa_sgp4out[sgp4::XA_SGP4OUT_POSX],
            xa_sgp4out[sgp4::XA_SGP4OUT_POSY],
            xa_sgp4out[sgp4::XA_SGP4OUT_POSZ],
        );
        let velocity = CartesianVector::new(
            xa_sgp4out[sgp4::XA_SGP4OUT_VELX],
            xa_sgp4out[sgp4::XA_SGP4OUT_VELY],
            xa_sgp4out[sgp4::XA_SGP4OUT_VELZ],
        );

        let cartesian_state = CartesianState::new(epoch, position, velocity, ReferenceFrame::TEME);
        let geodetic_position = GeodeticPosition::new(
            xa_sgp4out[sgp4::XA_SGP4OUT_LAT],
            xa_sgp4out[sgp4::XA_SGP4OUT_LON],
            xa_sgp4out[sgp4::XA_SGP4OUT_HEIGHT],
        );
        let mean_elements = KeplerianElements::new(
            xa_sgp4out[sgp4::XA_SGP4OUT_MN_A],
            xa_sgp4out[sgp4::XA_SGP4OUT_MN_E],
            xa_sgp4out[sgp4::XA_SGP4OUT_MN_INCLI],
            xa_sgp4out[sgp4::XA_SGP4OUT_MN_NODE],
            xa_sgp4out[sgp4::XA_SGP4OUT_MN_OMEGA],
            xa_sgp4out[sgp4::XA_SGP4OUT_MN_MA],
        );
        let osculating_elements = KeplerianElements::new(
            xa_sgp4out[sgp4::XA_SGP4OUT_OSC_A],
            xa_sgp4out[sgp4::XA_SGP4OUT_OSC_E],
            xa_sgp4out[sgp4::XA_SGP4OUT_OSC_INCLI],
            xa_sgp4out[sgp4::XA_SGP4OUT_OSC_NODE],
            xa_sgp4out[sgp4::XA_SGP4OUT_OSC_OMEGA],
            xa_sgp4out[sgp4::XA_SGP4OUT_OSC_MA],
        );
        Self {
            cartesian_state,
            mean_elements,
            osculating_elements,
            geodetic_position,
        }
    }
}
