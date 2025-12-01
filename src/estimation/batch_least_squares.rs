use super::{Covariance, Observation, ObservationResidual};
use crate::bodies::Satellite;
use crate::configs;
use crate::enums::{CovarianceType, KeplerianType};
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use pyo3::prelude::*;

pub const DEFAULT_MAX_ITERATIONS: usize = 20;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct BatchLeastSquares {
    obs: Vec<Observation>,
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

// helper functions (not exposed to Python)
impl BatchLeastSquares {
    /// Wrap Right Ascension residuals to [-180°, 180°] for shortest angular distance
    /// Measurement vector is [RA1, DEC1, RA2, DEC2, ...], so every even index is RA
    fn wrap_ra_residuals(residuals: &mut DVector<f64>) {
        for i in (0..residuals.len()).step_by(2) {
            if residuals[i] > 180.0 {
                residuals[i] -= 360.0;
            } else if residuals[i] < -180.0 {
                residuals[i] += 360.0;
            }
        }
    }

    /// Compute normal equations (H^T * W * H and H^T * W * r) using memory-efficient
    /// element-wise operations for diagonal weight matrix W
    /// Returns: (normal_matrix, rhs_vector, weighted_rss)
    fn compute_normal_equations(
        h: &DMatrix<f64>,
        w: &DVector<f64>,
        r: &DVector<f64>,
    ) -> (DMatrix<f64>, DVector<f64>, f64) {
        let n_cols = h.ncols();
        let mut n = DMatrix::zeros(n_cols, n_cols);
        let mut b = DVector::zeros(n_cols);
        let mut wrss = 0.0;

        // H^T * W * H = sum_i(w_i * h_i * h_i^T) where h_i is the i-th row of H
        // H^T * W * r = sum_i(w_i * h_i * r_i)
        for (h_row, (&weight, &residual)) in h.row_iter().zip(w.iter().zip(r.iter())) {
            let wr = weight * residual;

            // Accumulate b = H^T * W * r
            for (j, &h_ij) in h_row.iter().enumerate() {
                b[j] += h_ij * wr;
            }

            // Accumulate n = H^T * W * H (symmetric, so only compute upper triangle)
            for j in 0..n_cols {
                let wh_j = weight * h_row[j];
                for k in j..n_cols {
                    n[(j, k)] += wh_j * h_row[k];
                }
            }

            // Accumulate weighted residual sum of squares
            wrss += weight * residual * residual;
        }

        // Fill lower triangle of symmetric matrix
        for j in 0..n_cols {
            for k in 0..j {
                n[(j, k)] = n[(k, j)];
            }
        }

        (n, b, wrss)
    }
}

#[pymethods]
impl BatchLeastSquares {
    #[new]
    pub fn new(obs: Vec<Observation>, a_priori: &Satellite) -> Self {
        let output_keplerian_type = a_priori.get_keplerian_state().unwrap().get_type();
        let a_priori = a_priori.clone();
        let current_estimate = a_priori.clone();
        Self {
            obs,
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
        let mut range_errors: Vec<f64> = Vec::new();
        for ob in self.obs.iter() {
            match ob.get_residual(&self.current_estimate) {
                Some(residual) => range_errors.push(residual.get_range()),
                None => return None,
            }
        }
        let r = DVector::from_vec(range_errors);
        let m = r.len() as f64;
        let rss = (r.transpose() * &r)[(0, 0)];
        Some((rss / m).sqrt())
    }

    #[setter]
    pub fn set_a_priori(&mut self, a_priori: &Satellite) {
        self.a_priori = a_priori.clone();
        self.reset();
    }

    #[setter]
    pub fn set_observations(&mut self, obs: Vec<Observation>) {
        self.obs = obs;
        self.reset();
    }

    #[getter]
    pub fn get_observations(&self) -> Vec<Observation> {
        self.obs.clone()
    }

    #[getter]
    pub fn get_residuals(&self) -> Vec<(Epoch, ObservationResidual)> {
        let mut residuals: Vec<(Epoch, ObservationResidual)> = Vec::new();
        for ob in self.obs.iter() {
            match ob.get_residual(&self.current_estimate) {
                Some(residual) => {
                    residuals.push((ob.get_epoch(), residual));
                }
                None => return Vec::new(),
            }
        }
        residuals
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
    fn get_measurements_and_weights(&self) -> (DVector<f64>, DVector<f64>) {
        let mut measurement_vec = Vec::new();
        let mut weight_diag = Vec::new();
        self.obs.iter().for_each(|ob| {
            let (m_vec, w_vec) = ob.get_measurement_and_weight_vector();
            measurement_vec.extend(m_vec);
            weight_diag.extend(w_vec);
        });
        let measurement_vector = DVector::from_vec(measurement_vec);
        let weight_vector = DVector::from_vec(weight_diag);
        (measurement_vector, weight_vector)
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
            let ob_jacobian = self.current_estimate.get_jacobian(ob, self.use_drag, self.use_srp)?;
            let dim = ob_jacobian.nrows();
            jacobian.view_mut((row, 0), (dim, n)).copy_from(&ob_jacobian);
            row += dim;
        }
        Ok(jacobian)
    }

    fn get_delta_x(&mut self) -> Result<(), String> {
        let (y, w) = self.get_measurements_and_weights();
        let y_hat = self.get_predicted_measurements()?;
        let mut r = (&y - &y_hat).clone_owned();
        
        Self::wrap_ra_residuals(&mut r);
        
        let h = self.get_jacobians()?;
        let (n, b, wrss) = Self::compute_normal_equations(&h, &w, &r);

        // Compute weighted RMS for convergence testing
        let m: f64 = r.len() as f64;
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
}
