use super::{CartesianState, CartesianVector, EquinoctialElements, KeplerianElements, KeplerianState};
use crate::bodies::{Observatory, Satellite, Sensor};
use crate::configs::TLE_OBSERVATION_ANGULAR_NOISE;
use crate::elements::TopocentricElements;
use crate::enums::{Classification, KeplerianType, ReferenceFrame};
use crate::estimation::Observation;
use crate::propagation::{ForceProperties, SGP4Output};
use crate::time::{DAYS_TO_MINUTES, Epoch};
use nalgebra::{DMatrix, DVector};
use saal::{GetSetString, astro, sgp4, tle};
use std::str::FromStr;
use std::sync::Arc;

use uuid::Uuid;

const DEFAULT_EPSILONS: [f64; 8] = [1e-6, 1e-6, 1e-6, 1e-6, 1e-8, 1e-6, 1e-4, 1e-4];

#[derive(Debug, PartialEq)]
struct SAALKeyHandle {
    key: i64,
}

impl Drop for SAALKeyHandle {
    fn drop(&mut self) {
        if self.key == 0 {
            return;
        }
        let _ = sgp4::remove(self.key);
        tle::remove(self.key);
    }
}

#[derive(Debug, PartialEq)]
pub struct TLE {
    pub norad_id: i32,
    pub satellite_id: String,
    pub name: Option<String>,
    pub designator: String,
    pub classification: Classification,
    pub keplerian_state: KeplerianState,
    pub force_properties: ForceProperties,
    key: Option<Arc<SAALKeyHandle>>,
}

impl Drop for TLE {
    fn drop(&mut self) {
        self.remove_from_memory();
    }
}

impl Clone for TLE {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            norad_id: self.norad_id,
            satellite_id: self.satellite_id.clone(),
            name: self.name.clone(),
            designator: self.designator.clone(),
            classification: self.classification,
            keplerian_state: self.keplerian_state,
            force_properties: self.force_properties,
        }
    }
}

impl TLE {
    fn wrap_equinoctial_delta(index: usize, delta: f64) -> f64 {
        if index != astro::XA_EQNX_L {
            return delta;
        }
        // Mean longitude wraps at 360 degrees; keep delta in [-180, 180).
        let mut wrapped = delta % 360.0;
        if wrapped >= 180.0 {
            wrapped -= 360.0;
        } else if wrapped < -180.0 {
            wrapped += 360.0;
        }
        wrapped
    }

    pub fn new(
        satellite_id: String,
        norad_id: i32,
        name: Option<String>,
        classification: Classification,
        designator: String,
        keplerian_state: KeplerianState,
        force_properties: ForceProperties,
    ) -> Result<Self, String> {
        let mut tle = Self {
            satellite_id,
            norad_id,
            name,
            classification,
            designator,
            keplerian_state,
            force_properties,
            key: None,
        };
        match tle.load_to_memory() {
            Ok(_) => Ok(tle),
            Err(e) => Err(e),
        }
    }
    pub fn reload(&mut self) -> Result<(), String> {
        sgp4::remove(self.get_key())?;
        sgp4::load(self.get_key())?;
        Ok(())
    }

    pub fn get_key(&self) -> i64 {
        self.key.as_ref().map(|handle| handle.key).unwrap_or(0)
    }

    pub fn get_equinoctial_elements_at_epoch(&self, epoch: Epoch) -> Result<EquinoctialElements, String> {
        match self.get_type() {
            KeplerianType::MeanBrouwerXP => match sgp4::get_equinoctial(self.get_key(), epoch.days_since_1950) {
                Ok(equinoctial_elements) => Ok(EquinoctialElements::from(equinoctial_elements)),
                Err(e) => Err(e),
            },
            _ => match sgp4::get_full_state(self.get_key(), epoch.days_since_1950) {
                Ok(all) => Ok(SGP4Output::from(all).get_mean_elements().into()),
                Err(e) => Err(e),
            },
        }
    }

    pub fn get_stm(&self, epoch: Epoch, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        let mut n = 6;
        if use_drag {
            n += 1;
        }
        if use_srp {
            n += 1;
        }
        let mut stm: DMatrix<f64> = DMatrix::zeros(n, n);

        let state_0 = self.get_keplerian_state();
        let elements_0 = self.get_equinoctial_elements_at_epoch(self.get_epoch())?;
        let forces_0 = self.get_force_properties();
        let tle_0 = self.clone();
        let reference_elements = tle_0.get_equinoctial_elements_at_epoch(epoch)?;

        // Perturb orbital elements
        for i in 0..6 {
            let mut xa_eqnx = elements_0;
            let epsilon = DEFAULT_EPSILONS[i];
            xa_eqnx[i] += epsilon;
            let perturbed_elements = KeplerianElements::from(&xa_eqnx);
            let perturbed_state = KeplerianState::new(
                self.get_epoch(),
                perturbed_elements,
                ReferenceFrame::TEME,
                self.get_type(),
            );
            let tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                perturbed_state,
                forces_0,
            )
            .unwrap();

            let perturbed_els = tle.get_equinoctial_elements_at_epoch(epoch)?;
            for j in 0..6 {
                let delta = perturbed_els[j] - reference_elements[j];
                let delta = Self::wrap_equinoctial_delta(j, delta);
                stm[(j, i)] = delta / epsilon;
            }
        }

        let mut current_col = 6;
        // Perturb drag
        if use_drag {
            let mut perturbed_forces = forces_0;
            let epsilon = DEFAULT_EPSILONS[6];
            perturbed_forces.drag_coefficient = forces_0.drag_coefficient + epsilon;
            let tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                state_0,
                perturbed_forces,
            )
            .unwrap();

            let perturbed_els = tle.get_equinoctial_elements_at_epoch(epoch)?;
            for j in 0..6 {
                let delta = perturbed_els[j] - reference_elements[j];
                let delta = Self::wrap_equinoctial_delta(j, delta);
                stm[(j, current_col)] = delta / epsilon;
            }
            stm[(current_col, current_col)] = 1.0;
            current_col += 1;
        }

        //Perturb SRP or mean motion dot
        if use_srp {
            let mut perturbed_forces = forces_0;
            let epsilon = DEFAULT_EPSILONS[7];
            if self.get_type() == KeplerianType::MeanBrouwerXP || self.get_type() == KeplerianType::Osculating {
                perturbed_forces.srp_coefficient = forces_0.srp_coefficient + epsilon;
            } else {
                perturbed_forces.mean_motion_dot = forces_0.mean_motion_dot + epsilon;
            }

            let tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                state_0,
                perturbed_forces,
            )
            .unwrap();

            let perturbed_els = tle.get_equinoctial_elements_at_epoch(epoch)?;
            for j in 0..6 {
                let delta = perturbed_els[j] - reference_elements[j];
                let delta = Self::wrap_equinoctial_delta(j, delta);
                stm[(j, current_col)] = delta / epsilon;
            }
            stm[(current_col, current_col)] = 1.0;
        }
        Ok(stm)
    }

    pub fn new_with_delta_x(&self, delta_x: &DVector<f64>, use_drag: bool, use_srp: bool) -> Result<TLE, String> {
        let mut new_elements = self.get_equinoctial_elements_at_epoch(self.get_epoch())?;

        for i in 0..6 {
            new_elements[i] += delta_x[i];
        }
        let mut forces = self.force_properties;
        if use_drag {
            forces.drag_coefficient += delta_x[6];
        }
        if use_srp {
            forces.srp_coefficient += delta_x[delta_x.len() - 1];
        }

        let new_state = KeplerianState::new(
            self.get_epoch(),
            new_elements.into(),
            ReferenceFrame::TEME,
            self.get_type(),
        );
        match TLE::new(
            self.satellite_id.clone(),
            self.norad_id,
            self.name.clone(),
            self.classification,
            self.designator.clone(),
            new_state,
            forces,
        ) {
            Ok(tle) => Ok(tle),
            Err(e) => Err(e),
        }
    }

    pub fn get_observation_at_epoch(&self, epoch: Epoch) -> Result<Observation, String> {
        let teme_state = self.get_cartesian_state_at_epoch(epoch)?;
        let efg = teme_state.to_frame(ReferenceFrame::EFG).position;
        let lla = astro::efg_to_lla(&efg.into())?;
        let site = Observatory::new(lla[0], lla[1], 0.0);
        let observer_teme = site.get_state_at_epoch(epoch);
        let lst = epoch.get_gst() + lla[1].to_radians();
        let xa_topo = astro::teme_to_topo(lst, lla[0], &observer_teme.position.into(), &teme_state.into())?;
        let ra = xa_topo[astro::XA_TOPO_RA];
        let dec = xa_topo[astro::XA_TOPO_DEC];
        let topo_els = TopocentricElements::new(ra, dec);

        let sensor = Sensor::new(TLE_OBSERVATION_ANGULAR_NOISE);
        Ok(Observation::new(sensor, epoch, topo_els, observer_teme.position))
    }

    pub fn get_period(&self) -> f64 {
        DAYS_TO_MINUTES / self.get_mean_motion() // in minutes
    }

    pub fn get_jacobian(&self, ob: &Observation, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        // Build the reference satellite
        let ref_sat = Satellite::from(self.clone());

        // Get the predicted measurements for the reference satellite
        let h_ref = ob.get_predicted_vector(&ref_sat)?;
        let m = h_ref.len();
        let mut n = 6;
        if use_drag {
            n += 1;
        }
        if use_srp {
            n += 1;
        }
        let mut jac = DMatrix::<f64>::zeros(m, n);

        // Get the reference keplerian elements as an array
        let ref_state = self.get_keplerian_state();
        let ref_elements = self.get_equinoctial_elements_at_epoch(self.get_epoch())?;

        for j in 0..6 {
            let mut perturbed_elements = ref_elements;
            let epsilon = DEFAULT_EPSILONS[j];
            perturbed_elements[j] += epsilon;
            let perturbed_kep = KeplerianElements::from(&perturbed_elements);
            let perturbed_state = KeplerianState::new(
                ref_state.epoch,
                perturbed_kep,
                ref_state.get_frame(),
                ref_state.get_type(),
            );
            let perturbed_tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                perturbed_state,
                self.force_properties,
            )
            .unwrap();
            let perturbed_sat = Satellite::from(perturbed_tle);
            let h_p = ob.get_predicted_vector(&perturbed_sat)?;

            // Compute the j-th column as (h_p - h_ref) / epsilon
            for i in 0..m {
                jac[(i, j)] = (h_p[i] - h_ref[i]) / epsilon;
            }
        }

        let mut next_row = 6;
        // drag
        if use_drag {
            let mut perturbed_forces = self.force_properties;
            let epsilon = DEFAULT_EPSILONS[6];
            perturbed_forces.drag_coefficient += epsilon;
            let perturbed_tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                ref_state,
                perturbed_forces,
            )
            .unwrap();
            let perturbed_sat = Satellite::from(perturbed_tle);
            let h_p = ob.get_predicted_vector(&perturbed_sat)?;

            // Compute the j-th column as (h_p - h_ref) / epsilon
            for i in 0..m {
                jac[(i, next_row)] = (h_p[i] - h_ref[i]) / epsilon;
            }
            next_row += 1;
        }

        //srp or mean motion dot
        if use_srp {
            let mut perturbed_forces = self.force_properties;
            let epsilon = DEFAULT_EPSILONS[7];
            perturbed_forces.srp_coefficient += epsilon;
            let perturbed_tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                ref_state,
                perturbed_forces,
            )
            .unwrap();
            let perturbed_sat = Satellite::from(perturbed_tle);
            let h_p = ob.get_predicted_vector(&perturbed_sat)?;

            // Compute the j-th column as (h_p - h_ref) / epsilon
            for i in 0..m {
                jac[(i, next_row)] = (h_p[i] - h_ref[i]) / epsilon;
            }
        }

        Ok(jac)
    }

    pub fn build_perturbed_satellites(&self, use_drag: bool, use_srp: bool) -> Result<Vec<(Satellite, f64)>, String> {
        let mut n = 6;
        if use_drag {
            n += 1;
        }
        if use_srp {
            n += 1;
        }
        let mut sats = Vec::with_capacity(n);

        let ref_state = self.get_keplerian_state();
        let ref_elements = self.get_equinoctial_elements_at_epoch(self.get_epoch())?;

        for j in 0..6 {
            let mut perturbed_elements = ref_elements;
            let epsilon = DEFAULT_EPSILONS[j];
            perturbed_elements[j] += epsilon;
            let perturbed_kep = KeplerianElements::from(&perturbed_elements);
            let perturbed_state = KeplerianState::new(
                ref_state.epoch,
                perturbed_kep,
                ref_state.get_frame(),
                ref_state.get_type(),
            );
            let perturbed_tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                perturbed_state,
                self.force_properties,
            )?;
            sats.push((Satellite::from(perturbed_tle), epsilon));
        }

        if use_drag {
            let mut perturbed_forces = self.force_properties;
            let epsilon = DEFAULT_EPSILONS[6];
            perturbed_forces.drag_coefficient += epsilon;
            let perturbed_tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                ref_state,
                perturbed_forces,
            )?;
            sats.push((Satellite::from(perturbed_tle), epsilon));
        }

        if use_srp {
            let mut perturbed_forces = self.force_properties;
            let epsilon = DEFAULT_EPSILONS[7];
            perturbed_forces.srp_coefficient += epsilon;
            let perturbed_tle = TLE::new(
                self.satellite_id.clone(),
                self.norad_id,
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                ref_state,
                perturbed_forces,
            )?;
            sats.push((Satellite::from(perturbed_tle), epsilon));
        }

        Ok(sats)
    }

    pub fn get_xa_tle(&self) -> [f64; tle::XA_TLE_SIZE] {
        let mut xa_tle = [0.0; tle::XA_TLE_SIZE];
        xa_tle[tle::XA_TLE_SATNUM] = self.norad_id as f64;
        xa_tle[tle::XA_TLE_EPOCH] = self.get_epoch().days_since_1950;
        xa_tle[tle::XA_TLE_INCLI] = self.get_inclination();
        xa_tle[tle::XA_TLE_NODE] = self.get_raan();
        xa_tle[tle::XA_TLE_ECCEN] = self.get_eccentricity();
        xa_tle[tle::XA_TLE_OMEGA] = self.get_argument_of_perigee();
        xa_tle[tle::XA_TLE_MNANOM] = self.get_mean_anomaly();
        xa_tle[tle::XA_TLE_MNMOTN] = self.get_mean_motion();
        xa_tle[tle::XA_TLE_EPHTYPE] = self.get_type() as i32 as f64;

        match self.get_type() {
            KeplerianType::Osculating => {
                xa_tle[tle::XA_TLE_SP_BTERM] = self.get_b_term();
                xa_tle[tle::XA_TLE_SP_AGOM] = self.get_agom();
            }
            KeplerianType::MeanBrouwerXP => {
                xa_tle[tle::XA_TLE_BTERM] = self.get_b_term();
                xa_tle[tle::XA_TLE_AGOMGP] = self.get_agom();
            }
            _ => {
                xa_tle[tle::XA_TLE_BSTAR] = self.get_b_star();
                xa_tle[tle::XA_TLE_NDOT] = self.get_mean_motion_dot();
                xa_tle[tle::XA_TLE_NDOTDOT] = self.get_mean_motion_dot_dot();
            }
        }
        xa_tle
    }

    pub fn get_xs_tle(&self) -> String {
        let cls_plus_des = self.classification.as_char().to_string() + &self.designator;
        GetSetString::from(cls_plus_des.as_str()).value()
    }

    pub fn get_force_properties(&self) -> ForceProperties {
        self.force_properties
    }

    pub fn remove_from_memory(&mut self) {
        if self.key.is_some() {
            self.key = None;
        }
    }

    pub fn load_to_memory(&mut self) -> Result<(), String> {
        let xa_tle = self.get_xa_tle();
        let xs_tle = self.get_xs_tle();
        match tle::load_arrays(xa_tle, &xs_tle) {
            Ok(key) => {
                let result = sgp4::load(key);
                result?;
                self.key = Some(Arc::new(SAALKeyHandle { key }));
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_two_lines(line_1: &str, line_2: &str) -> Result<TLE, String> {
        let (xa_tle, xs_tle) = tle::lines_to_arrays(line_1, line_2).unwrap();
        let cls_char = &xs_tle[tle::XS_TLE_SECCLASS_0_1..tle::XS_TLE_SECCLASS_0_1 + 1];
        let designator = &xs_tle[tle::XS_TLE_SATNAME_1_12..tle::XS_TLE_SATNAME_1_12 + 12];
        let keplerian_state = KeplerianState::from(&xa_tle);
        let force_properties = ForceProperties::from(&xa_tle);
        match Self::new(
            Uuid::new_v4().to_string(),
            xa_tle[tle::XA_TLE_SATNUM] as i32,
            None,
            Classification::from_str(cls_char).unwrap(),
            designator.trim().to_string(),
            keplerian_state,
            force_properties,
        ) {
            Ok(tle) => Ok(tle),
            Err(e) => Err(e),
        }
    }

    pub fn from_three_lines(line_1: &str, line_2: &str, line_3: &str) -> Result<TLE, String> {
        let tle = Self::from_two_lines(line_2, line_3);
        match tle {
            Ok(mut tle) => {
                tle.name = match line_1.starts_with("0 ") {
                    true => Some(line_1[2..].trim().to_string()),
                    false => Some(line_1.trim().to_string()),
                };
                Ok(tle)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_keplerian_state(&self) -> KeplerianState {
        self.keplerian_state
    }

    pub fn get_keplerian_state_at_epoch(&self, epoch: Epoch) -> Result<KeplerianState, String> {
        match sgp4::get_full_state(self.get_key(), epoch.days_since_1950) {
            Ok(all) => Ok(KeplerianState::new(
                epoch,
                SGP4Output::from(all).get_mean_elements(),
                ReferenceFrame::TEME,
                self.get_type(),
            )),
            Err(e) => Err(e),
        }
    }

    pub fn get_cartesian_state_at_epoch(&self, epoch: Epoch) -> Result<CartesianState, String> {
        match sgp4::get_position_velocity(self.get_key(), epoch.days_since_1950) {
            Ok((pos, vel)) => {
                let pos = CartesianVector::from(pos);
                let vel = CartesianVector::from(vel);
                Ok(CartesianState::new(epoch, pos, vel, ReferenceFrame::TEME))
            }
            Err(e) => {
                log::debug!(
                    "{} propagating {} TLE to {}",
                    e,
                    self.get_name().unwrap_or("UNKNOWN".to_string()),
                    epoch.to_iso()
                );
                Err(e.trim().to_string())
            }
        }
    }

    pub fn from_lines(line_1: &str, line_2: &str, line_3: Option<&str>) -> Result<TLE, String> {
        let tle = match line_3 {
            Some(line_3) => Self::from_three_lines(line_1, line_2, line_3),
            None => Self::from_two_lines(line_1, line_2),
        }?;
        Ok(tle)
    }

    pub fn get_lines(&self) -> Result<(String, String), String> {
        let xa_tle = self.get_xa_tle();
        let xs_tle = self.get_xs_tle();
        let (mut line_1, mut line_2) = tle::arrays_to_lines(xa_tle, &xs_tle).unwrap();
        tle::fix_blank_exponent_sign(&mut line_1);
        tle::add_check_sums(&mut line_1, &mut line_2)?;
        Ok((line_1, line_2))
    }

    pub fn get_apoapsis(&self) -> f64 {
        self.keplerian_state.get_apoapsis()
    }

    pub fn get_periapsis(&self) -> f64 {
        self.keplerian_state.get_periapsis()
    }

    pub fn get_inclination(&self) -> f64 {
        self.keplerian_state.elements.inclination
    }

    pub fn get_raan(&self) -> f64 {
        self.keplerian_state.elements.raan
    }

    pub fn get_semi_major_axis(&self) -> f64 {
        self.keplerian_state.elements.semi_major_axis
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.keplerian_state.elements.eccentricity
    }

    pub fn get_argument_of_perigee(&self) -> f64 {
        self.keplerian_state.elements.argument_of_perigee
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn get_mean_anomaly(&self) -> f64 {
        self.keplerian_state.elements.mean_anomaly
    }

    pub fn get_mean_motion(&self) -> f64 {
        self.keplerian_state.get_mean_motion()
    }

    pub fn get_type(&self) -> KeplerianType {
        self.keplerian_state.get_type()
    }

    pub fn get_b_star(&self) -> f64 {
        self.force_properties.get_b_star()
    }

    pub fn get_mean_motion_dot(&self) -> f64 {
        self.force_properties.mean_motion_dot
    }

    pub fn get_mean_motion_dot_dot(&self) -> f64 {
        self.force_properties.mean_motion_dot_dot
    }

    pub fn get_agom(&self) -> f64 {
        self.force_properties.get_srp_term()
    }

    pub fn get_b_term(&self) -> f64 {
        self.force_properties.get_drag_term()
    }

    pub fn get_epoch(&self) -> Epoch {
        self.keplerian_state.epoch
    }

    pub fn get_cartesian_state(&self) -> CartesianState {
        self.keplerian_state.into()
    }

    // ========================================================================
    // Batch Propagation Methods (GPU-accelerated when available)
    // ========================================================================

    /// Propagate multiple TLEs to multiple epochs
    ///
    /// This method uses the BatchPropagator with automatic CPU/GPU selection.
    /// GPU acceleration is used when beneficial based on problem size.
    ///
    /// # Arguments
    /// * `tles` - Array of TLEs to propagate
    /// * `epochs` - Array of epochs to propagate to
    ///
    /// # Returns
    /// 2D vector of states: `result[sat_idx][epoch_idx]`
    ///
    /// # Example
    /// ```no_run
    /// use keplemon::elements::TLE;
    /// use keplemon::time::Epoch;
    ///
    /// let tles = vec![/* ... */];
    /// let epochs = vec![/* ... */];
    /// let states = TLE::propagate_batch(&tles, &epochs).unwrap();
    /// ```
    pub fn propagate_batch(tles: &[TLE], epochs: &[Epoch]) -> Result<Vec<Vec<CartesianState>>, String> {
        use crate::propagation::BatchPropagator;

        let propagator = BatchPropagator::new();
        propagator.propagate_batch(tles, epochs)
    }

    /// Propagate a single TLE to multiple epochs
    ///
    /// This method automatically uses GPU if the number of epochs is large enough
    /// to benefit from GPU acceleration.
    ///
    /// # Arguments
    /// * `epochs` - Array of epochs to propagate to
    ///
    /// # Returns
    /// Vector of states, one for each epoch
    ///
    /// # Example
    /// ```no_run
    /// use keplemon::elements::TLE;
    /// use keplemon::time::Epoch;
    ///
    /// # let line1 = "1 25544U 98067A   23001.00000000  .00016717  00000-0  10270-3 0  9005";
    /// # let line2 = "2 25544  51.6400 208.9163 0006317  69.9862  25.2906 15.54225995456456";
    /// let tle = TLE::from_lines(line1, line2, None).unwrap();
    /// # let epochs: Vec<Epoch> = vec![];
    /// let states = tle.propagate_to_epochs(&epochs).unwrap();
    /// ```
    pub fn propagate_to_epochs(&self, epochs: &[Epoch]) -> Result<Vec<CartesianState>, String> {
        use crate::propagation::BatchPropagator;

        if epochs.is_empty() {
            return Ok(Vec::new());
        }

        // For a single satellite, decide threshold based on number of epochs
        // GPU becomes beneficial around 100+ epochs
        let propagator = if epochs.len() >= 100 {
            BatchPropagator::new().set_gpu_threshold(100)
        } else {
            BatchPropagator::new().set_gpu_threshold(usize::MAX) // Force CPU
        };

        let results = propagator.propagate_batch(std::slice::from_ref(self), epochs)?;
        Ok(results.into_iter().next().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::{KeplerianElements, KeplerianState, TLE};
    use crate::enums::{Classification, KeplerianType, ReferenceFrame, TimeSystem};
    use crate::propagation::{ForceProperties, b_star_to_drag_coefficient, drag_coefficient_to_b_star};
    use crate::time::Epoch;
    use approx::assert_abs_diff_eq;
    use saal::astro;
    use uuid::Uuid;

    const SGP_LINE_1: &str = "1 25544U 98067A   20200.51605324 +.00000884  00000+0  22898-4 0 00005";
    const SGP_LINE_2: &str = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.49300070000008";
    const XP_LINE_1: &str = "1 25544U 98067A   20200.51605324 +.00000000  10000-1  20000-1 4 00002";
    const XP_LINE_2: &str = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.49300070000008";

    fn xp_tle_from_lines() -> TLE {
        TLE::from_lines(XP_LINE_1, XP_LINE_2, None).unwrap()
    }

    fn xp_tle_from_fields() -> TLE {
        let elements = KeplerianElements::new(
            astro::mean_motion_to_sma(15.49300070),
            0.0001400,
            51.6443,
            93.0,
            84.0,
            276.0,
        );
        let keplerian_state = KeplerianState::new(
            Epoch::from_days_since_1950(25767.51605324, TimeSystem::UTC),
            elements,
            ReferenceFrame::TEME,
            KeplerianType::MeanBrouwerXP,
        );
        let force_properties = ForceProperties::new(0.01, 1.0, 0.02, 1.0, 1.0, 0.0, 0.0);
        TLE::new(
            Uuid::new_v4().to_string(),
            25544,
            None,
            Classification::Unclassified,
            "98067A".to_string(),
            keplerian_state,
            force_properties,
        )
        .unwrap()
    }

    fn sgp_tle_from_lines() -> TLE {
        TLE::from_lines(SGP_LINE_1, SGP_LINE_2, None).unwrap()
    }

    fn sgp_tle_from_fields() -> TLE {
        let eccentricity = 0.0001400;
        let inclination = 51.6443;
        let mean_motion = 15.49300070;
        let brouwer = astro::kozai_to_brouwer(eccentricity, inclination, mean_motion);

        let elements = KeplerianElements::new(
            astro::mean_motion_to_sma(brouwer),
            eccentricity,
            inclination,
            93.0,
            84.0,
            276.0,
        );
        let keplerian_state = KeplerianState::new(
            Epoch::from_days_since_1950(25767.51605324, TimeSystem::UTC),
            elements,
            ReferenceFrame::TEME,
            KeplerianType::MeanKozaiGP,
        );
        let drag_coefficient = b_star_to_drag_coefficient(0.000022898);
        let force_properties = ForceProperties::new(0.0, 0.0, drag_coefficient, 1.0, 1.0, 0.00000884, 0.0);

        TLE::new(
            Uuid::new_v4().to_string(),
            25544,
            None,
            Classification::Unclassified,
            "98067A".to_string(),
            keplerian_state,
            force_properties,
        )
        .unwrap()
    }

    #[test]
    fn test_sgp_from_lines() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let tle = sgp_tle_from_lines();
        assert_eq!(tle.norad_id, 25544);
        assert_eq!(tle.get_epoch().days_since_1950, 25767.51605324);
        assert_eq!(tle.get_inclination(), 51.6443);
        assert_eq!(tle.get_raan(), 93.0);
        assert_eq!(tle.get_eccentricity(), 0.0001400);
        assert_eq!(tle.get_argument_of_perigee(), 84.0);
        assert_eq!(tle.get_mean_anomaly(), 276.0);
        assert_eq!(tle.get_mean_motion(), 15.49300070);
        assert_eq!(tle.get_b_star(), 0.000022898);
        assert_eq!(tle.get_mean_motion_dot(), 0.00000884);
        assert_eq!(tle.get_mean_motion_dot_dot(), 0.0);
        assert_eq!(tle.get_agom(), 0.0);
        assert_eq!(tle.get_b_term(), b_star_to_drag_coefficient(0.000022898));
        assert_eq!(tle.get_type(), KeplerianType::MeanKozaiGP);
        assert_eq!(tle.classification, Classification::Unclassified);
        assert_eq!(tle.designator, "98067A");
    }

    #[test]
    fn test_xp_from_lines() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let tle = xp_tle_from_lines();
        assert_eq!(tle.norad_id, 25544);
        assert_eq!(tle.get_inclination(), 51.6443);
        assert_eq!(tle.get_raan(), 93.0);
        assert_eq!(tle.get_eccentricity(), 0.0001400);
        assert_eq!(tle.get_argument_of_perigee(), 84.0);
        assert_eq!(tle.get_mean_anomaly(), 276.0);
        assert_abs_diff_eq!(tle.get_mean_motion(), 15.4930007, epsilon = 1e-7);
        assert_abs_diff_eq!(tle.get_b_star(), drag_coefficient_to_b_star(0.02), epsilon = 1e-7);
        assert_eq!(tle.get_mean_motion_dot(), 0.0);
        assert_eq!(tle.get_mean_motion_dot_dot(), 0.0);
        assert_abs_diff_eq!(tle.get_agom(), 0.01);
        assert_abs_diff_eq!(tle.get_b_term(), 0.02);
        assert_eq!(tle.get_type(), KeplerianType::MeanBrouwerXP);
        assert_eq!(tle.classification, Classification::Unclassified);
        assert_eq!(tle.designator, "98067A");
    }

    #[test]
    fn test_xp_to_lines() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let tle = xp_tle_from_fields();
        let (line_1, line_2) = tle.get_lines().unwrap();
        assert_eq!(line_1, XP_LINE_1);
        assert_eq!(line_2, XP_LINE_2);
    }

    #[test]
    fn test_sgp_to_lines() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let tle = sgp_tle_from_fields();
        let (line_1, line_2) = tle.get_lines().unwrap();
        assert_eq!(line_1, SGP_LINE_1);
        assert_eq!(line_2, SGP_LINE_2);
    }
}
