use super::{HorizonState, TopocentricElements};
use crate::bodies::Observatory;
use crate::time::Epoch;
use saal::astro;

#[derive(Debug, Clone, PartialEq)]
pub struct TopocentricState {
    pub epoch: Epoch,
    pub elements: TopocentricElements,
}

impl Copy for TopocentricState {}

impl From<(&HorizonState, &Observatory)> for TopocentricState {
    fn from(tuple: (&HorizonState, &Observatory)) -> Self {
        let (state, observer) = tuple;
        let theta = observer.get_theta(state.epoch);
        let sen_pos = observer.get_state_at_epoch(state.epoch).position;
        let lat = observer.latitude;
        let teme = astro::horizon_to_teme(theta, lat, &sen_pos.into(), &state.elements.into()).unwrap();

        let topo = astro::teme_to_topo(theta, lat, &sen_pos.into(), &teme).unwrap();
        let mut elements = TopocentricElements::new(topo[astro::XA_TOPO_RA], topo[astro::XA_TOPO_DEC]);

        elements.range = state.elements.range;
        match state.elements.range_rate {
            Some(_) => elements.range_rate = Some(topo[astro::XA_TOPO_RANGEDOT]),
            None => elements.range_rate = None,
        }
        match state.elements.azimuth_rate {
            Some(_) => elements.right_ascension_rate = Some(topo[astro::XA_TOPO_RADOT]),
            None => elements.right_ascension_rate = None,
        }
        match state.elements.elevation_rate {
            Some(_) => elements.declination_rate = Some(topo[astro::XA_TOPO_DECDOT]),
            None => elements.declination_rate = None,
        }

        Self {
            epoch: state.epoch,
            elements,
        }
    }
}

impl TopocentricState {
    pub fn new(epoch: Epoch, elements: TopocentricElements) -> Self {
        Self { epoch, elements }
    }
}
