use super::{CartesianState, EquinoctialElements, KeplerianElements, KeplerianState};
use crate::bodies::Satellite;
use crate::enums::{Classification, KeplerianType, ReferenceFrame};
use crate::estimation::Observation;
use crate::propagation::{ForceProperties, SGP4Output};
use crate::saal::{sgp4_prop_interface, tle_interface, GetSetString};
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use pyo3::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

const DEFAULT_EPSILONS: [f64; 8] = [1e-6, 1e-6, 1e-6, 1e-6, 1e-8, 1e-6, 1e-4, 1e-4];

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct TLE {
    key: i64,
    norad_id: i32,
    satellite_id: String,
    name: Option<String>,
    designator: String,
    classification: Classification,
    keplerian_state: KeplerianState,
    force_properties: ForceProperties,
}

impl Drop for TLE {
    fn drop(&mut self) {
        self.remove_from_memory();
    }
}

impl Clone for TLE {
    fn clone(&self) -> Self {
        let mut tle = Self {
            key: 0,
            norad_id: self.norad_id,
            satellite_id: self.satellite_id.clone(),
            name: self.name.clone(),
            designator: self.designator.clone(),
            classification: self.classification,
            keplerian_state: self.keplerian_state,
            force_properties: self.force_properties,
        };
        tle.load_to_memory().unwrap();
        tle
    }
}

impl TLE {
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
            key: 0,
            satellite_id,
            norad_id,
            name,
            classification,
            designator,
            keplerian_state,
            force_properties,
        };
        match tle.load_to_memory() {
            Ok(_) => Ok(tle),
            Err(e) => Err(e),
        }
    }

    pub fn get_key(&self) -> i64 {
        self.key
    }

    pub fn get_equinoctial_elements_at_epoch(&self, epoch: Epoch) -> EquinoctialElements {
        match self.get_type() {
            KeplerianType::MeanBrouwerXP => {
                match sgp4_prop_interface::get_equinoctial_at_ds50(self.key, epoch.days_since_1950) {
                    Ok(equinoctial_elements) => EquinoctialElements::from(equinoctial_elements),
                    Err(_) => {
                        sgp4_prop_interface::load_key(self.key).unwrap();
                        let els =
                            sgp4_prop_interface::get_equinoctial_at_ds50(self.key, epoch.days_since_1950).unwrap();
                        sgp4_prop_interface::remove_key(self.key).unwrap();
                        EquinoctialElements::from(els)
                    }
                }
            }
            _ => match sgp4_prop_interface::get_all_at_ds50(self.key, epoch.days_since_1950) {
                Ok(all) => SGP4Output::from(all).get_mean_elements().to_equinoctial(),
                Err(_) => {
                    sgp4_prop_interface::load_key(self.key).unwrap();
                    let all = sgp4_prop_interface::get_all_at_ds50(self.key, epoch.days_since_1950).unwrap();
                    sgp4_prop_interface::remove_key(self.key).unwrap();
                    SGP4Output::from(all).get_mean_elements().to_equinoctial()
                }
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
        let elements_0 = self.get_equinoctial_elements_at_epoch(self.get_epoch());
        let forces_0 = self.get_force_properties();
        let tle_0 = self.clone();
        let reference_elements = tle_0.get_equinoctial_elements_at_epoch(epoch);

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
                self.get_id(),
                self.get_norad_id(),
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                perturbed_state,
                forces_0,
            )
            .unwrap();

            let perturbed_els = tle.get_equinoctial_elements_at_epoch(epoch);
            for j in 0..6 {
                stm[(j, i)] = (perturbed_els[j] - reference_elements[j]) / epsilon;
            }
        }

        let mut current_col = 6;
        // Perturb drag
        if use_drag {
            let mut perturbed_forces = forces_0;
            let epsilon = DEFAULT_EPSILONS[6];
            perturbed_forces.set_drag_coefficient(forces_0.get_drag_coefficient() + epsilon);
            let tle = TLE::new(
                self.get_id(),
                self.get_norad_id(),
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                state_0,
                perturbed_forces,
            )
            .unwrap();

            let perturbed_els = tle.get_equinoctial_elements_at_epoch(epoch);
            for j in 0..6 {
                stm[(j, current_col)] = (perturbed_els[j] - reference_elements[j]) / epsilon;
            }
            stm[(current_col, current_col)] = 1.0;
            current_col += 1;
        }

        //Perturb SRP or mean motion dot
        if use_srp {
            let mut perturbed_forces = forces_0;
            let epsilon = DEFAULT_EPSILONS[7];
            if self.get_type() == KeplerianType::MeanBrouwerXP || self.get_type() == KeplerianType::Osculating {
                perturbed_forces.set_srp_coefficient(forces_0.get_srp_term() + epsilon);
            } else {
                perturbed_forces.set_mean_motion_dot(forces_0.get_mean_motion_dot() + epsilon);
            }

            let tle = TLE::new(
                self.get_id(),
                self.get_norad_id(),
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                state_0,
                perturbed_forces,
            )
            .unwrap();

            let perturbed_els = tle.get_equinoctial_elements_at_epoch(epoch);
            for j in 0..6 {
                stm[(j, current_col)] = (perturbed_els[j] - reference_elements[j]) / epsilon;
            }
            stm[(current_col, current_col)] = 1.0;
        }
        Ok(stm)
    }

    pub fn new_with_delta_x(&self, delta_x: &DVector<f64>, use_drag: bool, use_srp: bool) -> Result<TLE, String> {
        let mut new_elements = self.get_equinoctial_elements_at_epoch(self.get_epoch());

        for i in 0..6 {
            new_elements[i] += delta_x[i];
        }
        let mut forces = self.get_force_properties();
        if use_drag {
            forces.set_drag_coefficient(forces.get_drag_coefficient() + delta_x[6]);
        }
        if use_srp {
            forces.set_srp_coefficient(forces.get_srp_coefficient() + delta_x[delta_x.len() - 1]);
        }

        let new_state = KeplerianState::new(
            self.get_epoch(),
            new_elements.to_keplerian(),
            ReferenceFrame::TEME,
            self.get_type(),
        );
        match TLE::new(
            self.get_id(),
            self.get_norad_id(),
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

    pub fn get_jacobian(&self, ob: &Observation, use_drag: bool, use_srp: bool) -> Result<DMatrix<f64>, String> {
        // Build the reference satellite
        let ref_sat = Satellite::from_tle(self.clone());

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
        let ref_elements = self.get_equinoctial_elements_at_epoch(self.get_epoch());

        for j in 0..6 {
            let mut perturbed_elements = ref_elements;
            let epsilon = DEFAULT_EPSILONS[j];
            perturbed_elements[j] += epsilon;
            let perturbed_kep = KeplerianElements::from(&perturbed_elements);
            let perturbed_state = KeplerianState::new(
                ref_state.get_epoch(),
                perturbed_kep,
                ref_state.get_frame(),
                ref_state.get_type(),
            );
            let perturbed_tle = TLE::new(
                self.get_id(),
                self.get_norad_id(),
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                perturbed_state,
                self.force_properties,
            )
            .unwrap();
            let perturbed_sat = Satellite::from_tle(perturbed_tle);
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
            perturbed_forces.set_drag_coefficient(perturbed_forces.get_drag_coefficient() + epsilon);
            let perturbed_tle = TLE::new(
                self.get_id(),
                self.get_norad_id(),
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                ref_state,
                perturbed_forces,
            )
            .unwrap();
            let perturbed_sat = Satellite::from_tle(perturbed_tle);
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
            perturbed_forces.set_srp_coefficient(perturbed_forces.get_srp_coefficient() + epsilon);
            let perturbed_tle = TLE::new(
                self.get_id(),
                self.get_norad_id(),
                self.name.clone(),
                self.classification,
                self.designator.clone(),
                ref_state,
                perturbed_forces,
            )
            .unwrap();
            let perturbed_sat = Satellite::from_tle(perturbed_tle);
            let h_p = ob.get_predicted_vector(&perturbed_sat)?;

            // Compute the j-th column as (h_p - h_ref) / epsilon
            for i in 0..m {
                jac[(i, next_row)] = (h_p[i] - h_ref[i]) / epsilon;
            }
        }

        Ok(jac)
    }

    pub fn get_xa_tle(&self) -> [f64; tle_interface::XA_TLE_SIZE] {
        let mut xa_tle = [0.0; tle_interface::XA_TLE_SIZE];
        xa_tle[tle_interface::XA_TLE_SATNUM] = self.norad_id as f64;
        xa_tle[tle_interface::XA_TLE_EPOCH] = self.get_epoch().days_since_1950;
        xa_tle[tle_interface::XA_TLE_INCLI] = self.get_inclination();
        xa_tle[tle_interface::XA_TLE_NODE] = self.get_raan();
        xa_tle[tle_interface::XA_TLE_ECCEN] = self.get_eccentricity();
        xa_tle[tle_interface::XA_TLE_OMEGA] = self.get_argument_of_perigee();
        xa_tle[tle_interface::XA_TLE_MNANOM] = self.get_mean_anomaly();
        xa_tle[tle_interface::XA_TLE_MNMOTN] = self.get_mean_motion();
        xa_tle[tle_interface::XA_TLE_EPHTYPE] = self.get_type() as i32 as f64;

        match self.get_type() {
            KeplerianType::Osculating => {
                xa_tle[tle_interface::XA_TLE_SP_BTERM] = self.get_b_term();
                xa_tle[tle_interface::XA_TLE_SP_AGOM] = self.get_agom();
            }
            KeplerianType::MeanBrouwerXP => {
                xa_tle[tle_interface::XA_TLE_BTERM] = self.get_b_term();
                xa_tle[tle_interface::XA_TLE_AGOMGP] = self.get_agom();
            }
            _ => {
                xa_tle[tle_interface::XA_TLE_BSTAR] = self.get_b_star();
                xa_tle[tle_interface::XA_TLE_NDOT] = self.get_mean_motion_dot();
                xa_tle[tle_interface::XA_TLE_NDOTDOT] = self.get_mean_motion_dot_dot();
            }
        }
        xa_tle
    }

    pub fn get_xs_tle(&self) -> String {
        let cls_plus_des = self.classification.as_char().to_string() + &self.designator;
        GetSetString::from_string(&cls_plus_des).value()
    }

    pub fn get_force_properties(&self) -> ForceProperties {
        self.force_properties
    }

    pub fn remove_from_memory(&mut self) {
        tle_interface::remove_from_memory(self.key);
        self.key = 0;
    }

    pub fn load_to_memory(&mut self) -> Result<(), String> {
        let xa_tle = self.get_xa_tle();
        let xs_tle = self.get_xs_tle();
        match tle_interface::load_from_arrays(xa_tle, &xs_tle) {
            Ok(key) => {
                self.key = key;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_two_lines(line_1: &str, line_2: &str) -> Result<TLE, String> {
        let (xa_tle, xs_tle) = tle_interface::lines_to_arrays(line_1, line_2).unwrap();
        let cls_char = &xs_tle[tle_interface::XS_TLE_SECCLASS_0_1..tle_interface::XS_TLE_SECCLASS_0_1 + 1];
        let designator = &xs_tle[tle_interface::XS_TLE_SATNAME_1_12..tle_interface::XS_TLE_SATNAME_1_12 + 12];
        let keplerian_state = KeplerianState::from_xa_tle(&xa_tle);
        let force_properties = ForceProperties::from_xa_tle(&xa_tle);
        match Self::new(
            Uuid::new_v4().to_string(),
            xa_tle[tle_interface::XA_TLE_SATNUM] as i32,
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
}

#[pymethods]
impl TLE {
    #[staticmethod]
    #[pyo3(signature = (line_1, line_2, line_3 = None))]
    pub fn from_lines(line_1: &str, line_2: &str, line_3: Option<&str>) -> PyResult<TLE> {
        let tle = match line_3 {
            Some(line_3) => Self::from_three_lines(line_1, line_2, line_3),
            None => Self::from_two_lines(line_1, line_2),
        };
        match tle {
            Ok(tle) => Ok(tle),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
        }
    }

    #[getter]
    pub fn get_lines(&self) -> (String, String) {
        let xa_tle = self.get_xa_tle();
        let xs_tle = self.get_xs_tle();
        tle_interface::arrays_to_lines(xa_tle, &xs_tle).unwrap()
    }

    #[getter]
    pub fn get_inclination(&self) -> f64 {
        self.keplerian_state.get_inclination()
    }

    #[getter]
    pub fn get_raan(&self) -> f64 {
        self.keplerian_state.get_raan()
    }

    #[getter]
    pub fn get_eccentricity(&self) -> f64 {
        self.keplerian_state.get_eccentricity()
    }

    #[getter]
    pub fn get_argument_of_perigee(&self) -> f64 {
        self.keplerian_state.get_argument_of_perigee()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    #[getter]
    pub fn get_mean_anomaly(&self) -> f64 {
        self.keplerian_state.get_mean_anomaly()
    }

    #[getter]
    pub fn get_mean_motion(&self) -> f64 {
        self.keplerian_state.get_mean_motion()
    }

    #[getter]
    pub fn get_type(&self) -> KeplerianType {
        self.keplerian_state.get_type()
    }

    #[getter]
    pub fn get_b_star(&self) -> f64 {
        self.force_properties.get_b_star()
    }

    #[getter]
    pub fn get_mean_motion_dot(&self) -> f64 {
        self.force_properties.get_mean_motion_dot()
    }

    #[getter]
    pub fn get_mean_motion_dot_dot(&self) -> f64 {
        self.force_properties.get_mean_motion_dot_dot()
    }

    #[getter]
    pub fn get_agom(&self) -> f64 {
        self.force_properties.get_srp_term()
    }

    #[getter]
    pub fn get_b_term(&self) -> f64 {
        self.force_properties.get_drag_term()
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.keplerian_state.get_epoch()
    }

    #[getter]
    pub fn get_classification(&self) -> Classification {
        self.classification
    }

    #[getter]
    pub fn get_designator(&self) -> String {
        self.designator.clone()
    }

    #[getter]
    pub fn get_norad_id(&self) -> i32 {
        self.norad_id
    }

    #[getter]
    pub fn get_id(&self) -> String {
        self.satellite_id.clone()
    }

    #[getter]
    fn get_cartesian_state(&self) -> CartesianState {
        self.keplerian_state.to_cartesian()
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::{KeplerianElements, KeplerianState, TLE};
    use crate::enums::{Classification, KeplerianType, ReferenceFrame, TimeSystem};
    use crate::propagation::ForceProperties;
    use crate::saal::astro_func_interface;
    use crate::time::Epoch;
    use approx::assert_abs_diff_eq;
    use uuid::Uuid;

    const SGP_LINE_1: &str = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999";
    const SGP_LINE_2: &str = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";
    const XP_LINE_1: &str = "1 25544U 98067A   20200.51605324 +.00000000  10000-1  20000-1 4 0999";
    const XP_LINE_2: &str = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";

    fn xp_tle_from_lines() -> TLE {
        TLE::from_lines(XP_LINE_1, XP_LINE_2, None).unwrap()
    }

    fn xp_tle_from_fields() -> TLE {
        let elements = KeplerianElements::new(
            astro_func_interface::mean_motion_to_sma(15.49300070),
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
        let elements = KeplerianElements::new(
            astro_func_interface::mean_motion_to_sma(15.49300070),
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
            KeplerianType::MeanKozaiGP,
        );
        let force_properties = ForceProperties::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.00000884, 0.000022898);

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
        assert_eq!(tle.get_b_term(), 0.0);
        assert_eq!(tle.get_type(), KeplerianType::MeanKozaiGP);
        assert_eq!(tle.classification, Classification::Unclassified);
        assert_eq!(tle.designator, "98067A");
    }

    #[test]
    fn test_xp_from_lines() {
        let tle = xp_tle_from_lines();
        assert_eq!(tle.norad_id, 25544);
        assert_eq!(tle.get_inclination(), 51.6443);
        assert_eq!(tle.get_raan(), 93.0);
        assert_eq!(tle.get_eccentricity(), 0.0001400);
        assert_eq!(tle.get_argument_of_perigee(), 84.0);
        assert_eq!(tle.get_mean_anomaly(), 276.0);
        assert_eq!(tle.get_mean_motion(), 15.49300070);
        assert_eq!(tle.get_b_star(), 0.0);
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
        let tle = xp_tle_from_fields();
        let (line_1, line_2) = tle.get_lines();
        assert_eq!(line_1, XP_LINE_1);
        assert_eq!(line_2, XP_LINE_2);
    }

    #[test]
    fn test_sgp_to_lines() {
        let tle = sgp_tle_from_fields();
        let (line_1, line_2) = tle.get_lines();
        assert_eq!(line_1, SGP_LINE_1);
        assert_eq!(line_2, SGP_LINE_2);
    }

    #[test]
    fn test_load_sgp_to_memory() {
        let mut tle = sgp_tle_from_fields();
        tle.load_to_memory().unwrap();
        assert_ne!(tle.get_key(), 0);
        tle.remove_from_memory();
    }

    #[test]
    fn test_load_xp_to_memory() {
        let mut tle = xp_tle_from_fields();
        tle.load_to_memory().unwrap();
        assert_ne!(tle.get_key(), 0);
        tle.remove_from_memory();
    }
}
