use super::{CartesianState, HorizonElements, TopocentricState};
use crate::bodies::Observatory;
use crate::time::Epoch;
use saal::astro;

#[derive(Debug, Clone, PartialEq)]
pub struct HorizonState {
    pub epoch: Epoch,
    pub elements: HorizonElements,
}

impl Copy for HorizonState {}

impl HorizonState {
    pub fn new(epoch: Epoch, elements: HorizonElements) -> Self {
        Self { epoch, elements }
    }
}

impl From<(&TopocentricState, &Observatory)> for HorizonState {
    fn from(tuple: (&TopocentricState, &Observatory)) -> Self {
        let (state, observer) = tuple;
        let lla = [observer.latitude, observer.longitude, observer.altitude];
        let az_el = astro::time_ra_dec_to_az_el(
            state.epoch.days_since_1950,
            &lla,
            state.elements.right_ascension,
            state.elements.declination,
        );
        let mut elements = HorizonElements::new(az_el[0], az_el[1]);
        elements.range = state.elements.range;

        Self {
            epoch: state.epoch,
            elements,
        }
    }
}
impl From<(CartesianState, CartesianState)> for HorizonState {
    fn from(tuple: (CartesianState, CartesianState)) -> Self {
        let (sensor_teme, target_teme) = tuple;
        let theta_g = sensor_teme.epoch.to_fk5_greenwich_angle();

        let lla = astro::gst_teme_to_lla(theta_g, &sensor_teme.position.into());
        let topo = astro::teme_to_topo(
            theta_g + lla[1].to_radians(),
            lla[0],
            &sensor_teme.position.into(),
            &target_teme.into(),
        )
        .unwrap();
        let elements = HorizonElements {
            azimuth: topo[astro::XA_TOPO_AZ],
            elevation: topo[astro::XA_TOPO_EL],
            range: Some(topo[astro::XA_TOPO_RANGE]),
            range_rate: Some(topo[astro::XA_TOPO_RANGEDOT]),
            azimuth_rate: Some(topo[astro::XA_TOPO_AZDOT]),
            elevation_rate: Some(topo[astro::XA_TOPO_ELDOT]),
        };
        Self {
            epoch: sensor_teme.epoch,
            elements,
        }
    }
}
