use super::{Covariance, Observation, ObservationResidual, ObservationType, TDOAObservation, FDOAObservation};
use crate::bodies::Satellite;
use crate::configs;
use crate::enums::{CovarianceType, KeplerianType};
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use pyo3::prelude::*;

pub const DEFAULT_MAX_ITERATIONS: usize = 20;

// Note: We cannot derive Debug, Clone, or PartialEq for BatchLeastSquares because
// the obs field contains Vec<Box<dyn ObservationType>>, and trait objects do not
// implement Clone or PartialEq by default. If debugging information is needed,
// consider implementing a custom Debug trait.
#[pyclass]
pub struct BatchLeastSquares {
    obs: Vec<Box<dyn ObservationType>>,
    a_priori: Satellite,
    use_drag: bool,
    use_srp: bool,
    delta_x: Option<DVector<f64>>,
    max_iterations: usize,
    current_estimate: Satellite,
    iteration_count: usize,
    weighted_rms: Option<f64>,
    converged: bool,
    output_keplerian_type: KeplerianType,
}

#[pymethods]
impl BatchLeastSquares {
    #[new]
    pub fn new(obs: Vec<Observation>, a_priori: &Satellite) -> Self {
        let output_keplerian_type = a_priori.get_keplerian_state().unwrap().get_type();
        let a_priori = a_priori.clone();
        let current_estimate = a_priori.clone();
        // Convert Observations to boxed trait objects
        let boxed_obs: Vec<Box<dyn ObservationType>> = obs
            .into_iter()
            .map(|o| Box::new(o) as Box<dyn ObservationType>)
            .collect();
        Self {
            obs: boxed_obs,
            a_priori,
            use_drag: false,
            use_srp: false,
            delta_x: None,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            current_estimate,
            iteration_count: 0,
            weighted_rms: None,
            converged: false,
            output_keplerian_type,
        }
    }

    #[staticmethod]
    pub fn from_mixed_observations(
        angle_obs: Vec<Observation>,
        tdoa_obs: Vec<TDOAObservation>,
        fdoa_obs: Vec<FDOAObservation>,
        a_priori: &Satellite,
    ) -> Self {
        let output_keplerian_type = a_priori.get_keplerian_state().unwrap().get_type();
        let a_priori = a_priori.clone();
        let current_estimate = a_priori.clone();

        // Convert all observation types to boxed trait objects
        let mut boxed_obs: Vec<Box<dyn ObservationType>> = Vec::new();

        // Add angle observations
        for obs in angle_obs {
            boxed_obs.push(Box::new(obs) as Box<dyn ObservationType>);
        }

        // Add TDOA observations
        for obs in tdoa_obs {
            boxed_obs.push(Box::new(obs) as Box<dyn ObservationType>);
        }

        // Add FDOA observations
        for obs in fdoa_obs {
            boxed_obs.push(Box::new(obs) as Box<dyn ObservationType>);
        }

        Self {
            obs: boxed_obs,
            a_priori,
            use_drag: false,
            use_srp: false,
            delta_x: None,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            current_estimate,
            iteration_count: 0,
            weighted_rms: None,
            converged: false,
            output_keplerian_type,
        }
    }

    fn iterate(&mut self) -> PyResult<()> {
        self.iteration_count += 1;
        match self.get_delta_x() {
            Ok(_) => {}
            Err(e) => return Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
        self.current_estimate =
            match self
                .current_estimate
                .new_with_delta_x(self.delta_x.as_ref().unwrap(), self.use_drag, self.use_srp)
            {
                Ok(new_estimate) => new_estimate,
                Err(e) => {
                    return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Unable to solve orbit state. {}",
                        e
                    )))
                }
            };

        Ok(())
    }

    #[getter]
    fn get_output_type(&self) -> KeplerianType {
        self.output_keplerian_type
    }

    #[setter]
    fn set_output_type(&mut self, output_keplerian_type: KeplerianType) {
        self.output_keplerian_type = output_keplerian_type;
        self.reset();
    }

    #[getter]
    fn get_converged(&self) -> bool {
        self.converged
    }

    #[getter]
    fn get_current_estimate(&self) -> Satellite {
        self.current_estimate.clone()
    }

    #[getter]
    fn get_iteration_count(&self) -> usize {
        self.iteration_count
    }

    pub fn solve(&mut self) -> PyResult<()> {
        self.iteration_count = 0;
        self.converged = false;
        self.delta_x = None;
        self.weighted_rms = None;
        let last_epoch = self.obs.iter().map(|o| o.get_epoch()).max().unwrap();
        self.current_estimate = match self.current_estimate.clone_at_epoch(last_epoch) {
            Ok(satellite) => satellite,
            Err(e) => return Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        };
        for _ in 0..self.max_iterations {
            match self.iterate() {
                Ok(_) => {
                    if self.converged {
                        break;
                    }
                }
                Err(e) => return Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
            }
        }
        Ok(())
    }

    #[getter]
    pub fn get_weighted_rms(&self) -> Option<f64> {
        self.weighted_rms
    }

    #[getter]
    pub fn get_rms(&self) -> Option<f64> {
        // Compute RMS from residuals across all observation types
        let residuals = self.get_residuals_internal();
        if residuals.is_empty() {
            return None;
        }

        let mut sum_sq = 0.0;
        let mut count = 0;

        for (_, residual) in &residuals {
            // Compute magnitude of residual depending on available components
            // For TDOA: use range field
            // For FDOA: use radial_velocity field
            // For angle observations: use RIC position magnitude
            let range = residual.get_range();
            let radial = residual.get_radial();
            let in_track = residual.get_in_track();
            let cross_track = residual.get_cross_track();
            let radial_velocity = residual.get_radial_velocity();

            // Compute magnitude: sqrt(all non-zero components squared)
            // For angle obs: sqrt(radial^2 + in_track^2 + cross_track^2)
            // For TDOA: sqrt(range^2) = |range|
            // For FDOA: sqrt(radial_velocity^2) = |radial_velocity|
            let magnitude_sq = range * range
                + radial * radial
                + in_track * in_track
                + cross_track * cross_track
                + radial_velocity * radial_velocity;

            sum_sq += magnitude_sq;
            count += 1;
        }

        if count == 0 {
            return None;
        }

        Some((sum_sq / count as f64).sqrt())
    }

    #[setter]
    pub fn set_a_priori(&mut self, a_priori: &Satellite) {
        self.a_priori = a_priori.clone();
        self.reset();
    }

    #[setter]
    pub fn set_observations(&mut self, obs: Vec<Observation>) {
        // Convert Observations to boxed trait objects
        self.obs = obs
            .into_iter()
            .map(|o| Box::new(o) as Box<dyn ObservationType>)
            .collect();
        self.reset();
    }

    #[getter]
    pub fn get_residuals(&self) -> Vec<(Epoch, ObservationResidual)> {
        // Compute residuals for all observation types
        self.get_residuals_internal()
    }

    #[setter]
    pub fn set_max_iterations(&mut self, max_iterations: usize) {
        self.max_iterations = max_iterations;
    }

    #[getter]
    pub fn get_max_iterations(&self) -> usize {
        self.max_iterations
    }

    #[setter]
    pub fn set_estimate_drag(&mut self, use_drag: bool) {
        self.use_drag = use_drag;
        self.reset();
    }

    #[getter]
    pub fn get_estimate_drag(&self) -> bool {
        self.use_drag
    }

    #[setter]
    pub fn set_estimate_srp(&mut self, use_srp: bool) {
        self.use_srp = use_srp;
        self.reset();
    }

    fn reset(&mut self) {
        self.current_estimate = Satellite::new();
        self.current_estimate.set_norad_id(self.a_priori.get_norad_id());
        self.current_estimate.set_name(self.a_priori.get_name());
        self.iteration_count = 0;
        self.converged = false;
        self.delta_x = None;
        self.weighted_rms = None;

        let mut force_properties = self.a_priori.get_force_properties();

        // Seed SRP if not already set
        if self.get_estimate_srp() && force_properties.get_srp_coefficient() == 0.0 {
            force_properties.set_srp_coefficient(configs::DEFAULT_SRP_TERM);
            force_properties.set_srp_area(1.0);
            force_properties.set_mass(1.0);
        }

        // Seed drag if not already set
        if self.get_estimate_drag() && force_properties.get_drag_coefficient() == 0.0 {
            println!("Seeding drag force properties");
            force_properties.set_drag_coefficient(configs::DEFAULT_DRAG_TERM);
            force_properties.set_drag_area(1.0);
            force_properties.set_mass(1.0);
        }
        self.current_estimate.set_force_properties(force_properties);

        // Seed orbit state
        let mut kep_state = self.a_priori.get_keplerian_state().unwrap();
        kep_state.set_type(self.output_keplerian_type);
        self.current_estimate.set_keplerian_state(kep_state).unwrap();

        // Disable SRP estimation if output type is incompatible
        if self.use_srp
            && (self.output_keplerian_type == KeplerianType::MeanBrouwerGP
                || self.output_keplerian_type == KeplerianType::MeanKozaiGP)
        {
            self.use_srp = false;
        }
    }

    #[getter]
    pub fn get_estimate_srp(&self) -> bool {
        self.use_srp
    }

    #[getter]
    pub fn get_covariance(&self) -> Option<Covariance> {
        let residuals = self.get_residuals();
        let mut residual_matrix = DMatrix::zeros(residuals.len(), 6);
        for (i, (_, residual)) in residuals.iter().enumerate() {
            for j in 0..6 {
                residual_matrix[(i, j)] = match j {
                    0 => residual.get_radial(),
                    1 => residual.get_in_track(),
                    2 => residual.get_cross_track(),
                    3 => residual.get_radial_velocity(),
                    4 => residual.get_in_track_velocity(),
                    5 => residual.get_cross_track_velocity(),
                    _ => unreachable!(),
                };
            }
        }
        match residual_matrix.is_empty() {
            true => None,
            false => {
                let covariance_matrix =
                    (residual_matrix.transpose() * &residual_matrix) / (residual_matrix.nrows() as f64);
                let covariance_type = CovarianceType::Relative;
                Some(Covariance::from((covariance_matrix, covariance_type)))
            }
        }
    }
}

impl BatchLeastSquares {
    fn get_measurements_and_weights(&self) -> (DVector<f64>, DMatrix<f64>) {
        let mut measurement_vec = Vec::new();
        let mut weight_diag = Vec::new();
        self.obs.iter().for_each(|ob| {
            let (m_vec, w_vec) = ob.get_measurement_and_weight_vector();
            measurement_vec.extend(m_vec);
            weight_diag.extend(w_vec);
        });
        let measurement_vector = DVector::from_vec(measurement_vec);
        let weight_matrix = DMatrix::from_diagonal(&DVector::from_vec(weight_diag));
        (measurement_vector, weight_matrix)
    }

    fn get_predicted_measurements(&self) -> Result<DVector<f64>, String> {
        let mut predicted_measurements = Vec::new();
        for ob in self.obs.iter() {
            match ob.get_predicted_vector(&self.current_estimate) {
                Ok(predicted) => predicted_measurements.extend(predicted),
                Err(e) => Err(e)?,
            }
        }
        Ok(DVector::from_vec(predicted_measurements))
    }

    fn get_jacobians(&self) -> Result<DMatrix<f64>, String> {
        let m = self.get_predicted_measurements()?.len();
        let mut n = 6;
        let mut row = 0;
        if self.use_drag {
            n += 1;
        }
        if self.use_srp {
            n += 1;
        }
        let mut jacobian = DMatrix::zeros(m, n);
        for ob in self.obs.iter() {
            // Dereference the Box to get a reference to the trait object
            let ob_jacobian = self.current_estimate.get_jacobian(&**ob, self.use_drag, self.use_srp)?;
            let dim = ob_jacobian.nrows();
            jacobian.view_mut((row, 0), (dim, n)).copy_from(&ob_jacobian);
            row += dim;
        }
        Ok(jacobian)
    }

    fn get_delta_x(&mut self) -> Result<(), String> {
        let (y, w) = self.get_measurements_and_weights();
        let y_hat = self.get_predicted_measurements()?;
        let r = &y - &y_hat;
        let h = self.get_jacobians()?;
        let h_transpose_w = &h.transpose() * &w;
        let n = &h_transpose_w * &h;
        let b = &h_transpose_w * &r;

        // Compute weighted RMS for convergence testing and noise balancing
        let m = r.len() as f64;
        let wrss = (r.transpose() * &w * &r)[(0, 0)];
        let current_weighted_rms = (wrss / m).sqrt();
        if self.weighted_rms.is_some() && (current_weighted_rms - self.weighted_rms.unwrap()).abs() < 1e-3 {
            self.converged = true;
        }

        self.weighted_rms = Some(current_weighted_rms);

        self.delta_x = n.lu().solve(&b);
        match self.delta_x {
            Some(_) => Ok(()),
            None => Err("Unable to compute delta_x".to_string()),
        }
    }

    fn get_residuals_internal(&self) -> Vec<(Epoch, ObservationResidual)> {
        // Compute residuals for all observation types (angle, TDOA, FDOA)
        let mut residuals = Vec::new();
        for ob in self.obs.iter() {
            if let Some(residual) = ob.get_residual(&self.current_estimate) {
                residuals.push((ob.get_epoch(), residual));
            }
        }
        residuals
    }
}
