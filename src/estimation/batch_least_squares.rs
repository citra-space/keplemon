use super::{Covariance, Observation, ObservationResidual};
use crate::bodies::Satellite;
use crate::configs;
use crate::elements::EquinoctialElements;
use crate::enums::{CovarianceType, KeplerianType};
use crate::time::Epoch;
use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;
use saal::astro;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

pub const DEFAULT_MAX_ITERATIONS: usize = 20;
// SAAL's SGP4/astro calls are not thread-safe under rayon on Linux.
static SAAL_BLS_LOCK: Mutex<()> = Mutex::new(());

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
    measurement_vector: Option<DVector<f64>>,
    weight_vector: Option<DVector<f64>>,
    measurement_sizes: Option<Vec<usize>>,
    predicted_buffers: Option<Vec<Vec<f64>>>,
    predicted_measurements: Option<DVector<f64>>,
    jacobian: Option<DMatrix<f64>>,
    eccentricity_constraint_weight: Option<f64>,
}

impl BatchLeastSquares {
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
            measurement_vector: None,
            weight_vector: None,
            measurement_sizes: None,
            predicted_buffers: None,
            predicted_measurements: None,
            jacobian: None,
            eccentricity_constraint_weight: None,
        }
    }

    fn iterate(&mut self) -> Result<(), String> {
        self.iteration_count += 1;
        self.get_delta_x()?;

        self.current_estimate =
            self.current_estimate
                .new_with_delta_x(self.delta_x.as_ref().unwrap(), self.use_drag, self.use_srp)?;

        Ok(())
    }

    pub fn get_output_type(&self) -> KeplerianType {
        self.output_keplerian_type
    }

    pub fn set_output_type(&mut self, output_keplerian_type: KeplerianType) {
        self.output_keplerian_type = output_keplerian_type;
        self.reset();
    }

    fn trace_enabled() -> bool {
        std::env::var("KEPLEMON_BLS_TRACE").is_ok()
    }

    pub fn get_converged(&self) -> bool {
        self.converged
    }

    pub fn get_current_estimate(&self) -> Satellite {
        self.current_estimate.clone()
    }

    pub fn get_iteration_count(&self) -> usize {
        self.iteration_count
    }

    pub fn solve(&mut self) -> Result<(), String> {
        self.iteration_count = 0;
        self.converged = false;
        self.delta_x = None;
        self.weighted_rms = None;
        self.measurement_vector = None;
        self.weight_vector = None;
        self.measurement_sizes = None;
        self.predicted_buffers = None;
        self.predicted_measurements = None;
        self.jacobian = None;
        let last_epoch = self.obs.iter().map(|o| o.get_epoch()).max().unwrap();
        self.current_estimate = self.current_estimate.clone_at_epoch(last_epoch)?;

        for _ in 0..self.max_iterations {
            self.iterate()?;
            if self.converged {
                break;
            }
        }
        Ok(())
    }

    pub fn get_weighted_rms(&self) -> Option<f64> {
        self.weighted_rms
    }

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

    pub fn set_a_priori(&mut self, a_priori: &Satellite) {
        self.a_priori = a_priori.clone();
        self.reset();
    }

    pub fn set_observations(&mut self, obs: Vec<Observation>) {
        self.obs = obs;
        self.reset();
    }

    pub fn get_observations(&self) -> Vec<Observation> {
        self.obs.clone()
    }

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

    pub fn set_max_iterations(&mut self, max_iterations: usize) {
        self.max_iterations = max_iterations;
    }

    pub fn get_max_iterations(&self) -> usize {
        self.max_iterations
    }

    pub fn set_estimate_drag(&mut self, use_drag: bool) {
        self.use_drag = use_drag;
        self.reset();
    }

    pub fn get_estimate_drag(&self) -> bool {
        self.use_drag
    }

    pub fn set_estimate_srp(&mut self, use_srp: bool) {
        self.use_srp = use_srp;
        self.reset();
    }

    fn reset(&mut self) {
        self.current_estimate = Satellite::new();
        self.current_estimate.norad_id = self.a_priori.norad_id;
        self.current_estimate.name = self.a_priori.name.clone();
        self.iteration_count = 0;
        self.converged = false;
        self.delta_x = None;
        self.weighted_rms = None;
        self.measurement_vector = None;
        self.weight_vector = None;
        self.measurement_sizes = None;
        self.predicted_buffers = None;
        self.predicted_measurements = None;
        self.jacobian = None;

        let mut force_properties = self.a_priori.get_force_properties();

        // Seed SRP if not already set
        if self.get_estimate_srp() && force_properties.srp_coefficient == 0.0 {
            force_properties.srp_coefficient = configs::DEFAULT_SRP_TERM;
            force_properties.srp_area = 1.0;
            force_properties.mass = 1.0;
        }

        // Seed drag if not already set
        if self.get_estimate_drag() && force_properties.drag_coefficient == 0.0 {
            force_properties.drag_coefficient = configs::DEFAULT_DRAG_TERM;
            force_properties.drag_area = 1.0;
            force_properties.mass = 1.0;
        }
        self.current_estimate.set_force_properties(force_properties);

        // Seed orbit state
        let mut kep_state = self.a_priori.get_keplerian_state().unwrap();
        kep_state.keplerian_type = self.output_keplerian_type;
        self.current_estimate.set_keplerian_state(kep_state).unwrap();

        // Disable SRP estimation if output type is incompatible
        if self.use_srp
            && (self.output_keplerian_type == KeplerianType::MeanBrouwerGP
                || self.output_keplerian_type == KeplerianType::MeanKozaiGP)
        {
            self.use_srp = false;
        }
    }

    pub fn get_estimate_srp(&self) -> bool {
        self.use_srp
    }

    pub fn get_eccentricity_constraint_weight(&self) -> Option<f64> {
        self.eccentricity_constraint_weight
    }

    pub fn set_eccentricity_constraint_weight(&mut self, weight: Option<f64>) {
        self.eccentricity_constraint_weight = weight;
    }

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
    fn timing_enabled() -> bool {
        std::env::var("KEPLEMON_BLS_TIMING").is_ok()
    }

    fn apply_eccentricity_constraint(&self, n: &mut DMatrix<f64>, b: &mut DVector<f64>) -> Result<(), String> {
        let weight = match self.eccentricity_constraint_weight {
            Some(w) if w > 0.0 => w,
            _ => return Ok(()),
        };
        let current_state = self
            .current_estimate
            .get_keplerian_state()
            .ok_or("Missing current keplerian state")?;
        let target_sat = self.a_priori.clone_at_epoch(current_state.epoch)?;
        let target_state = target_sat
            .get_keplerian_state()
            .ok_or("Missing a priori keplerian state")?;

        let current_eq: EquinoctialElements = current_state.elements.into();
        let target_eq: EquinoctialElements = target_state.elements.into();

        let r_af = target_eq[astro::XA_EQNX_AF] - current_eq[astro::XA_EQNX_AF];
        let r_ag = target_eq[astro::XA_EQNX_AG] - current_eq[astro::XA_EQNX_AG];

        // Equinoctial delta_x ordering is [a_f, a_g, chi, psi, L, n]
        if n.nrows() >= 2 && n.ncols() >= 2 && b.len() >= 2 {
            n[(0, 0)] += weight;
            b[0] += weight * r_af;
            n[(1, 1)] += weight;
            b[1] += weight * r_ag;
        }
        Ok(())
    }

    fn get_measurements_and_weights(&mut self) {
        let needs_refresh =
            self.measurement_vector.is_none() || self.weight_vector.is_none() || self.measurement_sizes.is_none();
        if needs_refresh {
            let mut measurement_vec = Vec::new();
            let mut weight_diag = Vec::new();
            let mut measurement_sizes = Vec::with_capacity(self.obs.len());
            self.obs.iter().for_each(|ob| {
                let (m_vec, w_vec) = ob.get_measurement_and_weight_vector();
                measurement_sizes.push(m_vec.len());
                measurement_vec.extend(m_vec);
                weight_diag.extend(w_vec);
            });
            self.measurement_vector = Some(DVector::from_vec(measurement_vec));
            self.weight_vector = Some(DVector::from_vec(weight_diag));
            self.measurement_sizes = Some(measurement_sizes);
        }
    }

    fn get_predicted_measurements(&mut self) -> Result<(), String> {
        let m = match self.measurement_vector.as_ref() {
            Some(measurements) => measurements.len(),
            None => return Err("Missing cached measurements".to_string()),
        };
        let measurement_sizes = match self.measurement_sizes.as_ref() {
            Some(sizes) => sizes,
            None => return Err("Missing cached measurement sizes".to_string()),
        };
        let needs_buffer_refresh = self
            .predicted_buffers
            .as_ref()
            .map(|buffers| buffers.len() != self.obs.len())
            .unwrap_or(true);
        if needs_buffer_refresh {
            let mut buffers = Vec::with_capacity(self.obs.len());
            for &dim in measurement_sizes.iter() {
                let mut buf = Vec::with_capacity(dim);
                buf.clear();
                buffers.push(buf);
            }
            self.predicted_buffers = Some(buffers);
        }
        let needs_refresh = self
            .predicted_measurements
            .as_ref()
            .map(|v| v.len() != m)
            .unwrap_or(true);
        if needs_refresh {
            self.predicted_measurements = Some(DVector::zeros(m));
        }

        let predicted_buffers = self.predicted_buffers.as_mut().unwrap();
        let trace = Self::trace_enabled();
        let current_estimate = &self.current_estimate;
        let obs = &self.obs;
        let results: Vec<Result<(), String>> = predicted_buffers
            .par_iter_mut()
            .zip(obs.par_iter())
            .enumerate()
            .map(|(idx, (buf, ob))| {
                if trace {
                    eprintln!(
                        "[tid={:?}] BLS predicted start idx={} ob_id={} sat_id={}",
                        thread::current().id(),
                        idx,
                        ob.id,
                        current_estimate.id
                    );
                }
                let result = {
                    let _guard = SAAL_BLS_LOCK.lock().expect("saal bls lock poisoned");
                    ob.fill_predicted_vector(current_estimate, buf)
                };
                if trace {
                    eprintln!(
                        "[tid={:?}] BLS predicted end idx={} ob_id={} sat_id={} ok={}",
                        thread::current().id(),
                        idx,
                        ob.id,
                        current_estimate.id,
                        result.is_ok()
                    );
                }
                result
            })
            .collect();
        for result in results {
            result?;
        }

        let predicted_measurements = self.predicted_measurements.as_mut().unwrap();
        let mut offset = 0;
        for buf in predicted_buffers.iter() {
            let end = offset + buf.len();
            if end > m {
                return Err("Predicted measurement length mismatch".to_string());
            }
            predicted_measurements.as_mut_slice()[offset..end].copy_from_slice(buf);
            offset = end;
        }
        Ok(())
    }

    fn get_jacobians(&mut self) -> Result<(), String> {
        let m = match self.measurement_vector.as_ref() {
            Some(measurements) => measurements.len(),
            None => return Err("Missing cached measurements".to_string()),
        };
        let mut n = 6;
        if self.use_drag {
            n += 1;
        }
        if self.use_srp {
            n += 1;
        }
        let needs_refresh = self
            .jacobian
            .as_ref()
            .map(|matrix| matrix.nrows() != m || matrix.ncols() != n)
            .unwrap_or(true);
        if needs_refresh {
            self.jacobian = Some(DMatrix::zeros(m, n));
        } else if let Some(matrix) = self.jacobian.as_mut() {
            matrix.as_mut_slice().fill(0.0);
        }

        let predicted_measurements = match self.predicted_measurements.as_ref() {
            Some(predicted) => predicted,
            None => return Err("Missing cached predicted measurements".to_string()),
        };
        let measurement_sizes = match self.measurement_sizes.as_ref() {
            Some(sizes) => sizes,
            None => return Err("Missing cached measurement sizes".to_string()),
        };
        let perturbed_sats = self
            .current_estimate
            .build_perturbed_satellites(self.use_drag, self.use_srp)?;
        if perturbed_sats.len() != n {
            return Err("Perturbed satellite count mismatch".to_string());
        }
        let mut offsets = Vec::with_capacity(measurement_sizes.len());
        let mut offset = 0;
        for &dim in measurement_sizes.iter() {
            offsets.push(offset);
            offset += dim;
        }
        if offset != predicted_measurements.len() {
            return Err("Predicted measurement length mismatch".to_string());
        }

        let jacobian = self.jacobian.as_mut().unwrap();
        let mut columns = vec![vec![0.0; m]; n];
        let trace = Self::trace_enabled();
        let results: Vec<Result<(), String>> = columns
            .par_iter_mut()
            .zip(perturbed_sats.par_iter())
            .enumerate()
            .map(|(col_idx, (col, (sat, epsilon)))| {
                if trace {
                    eprintln!(
                        "[tid={:?}] BLS jacobian start col={} sat_id={} epsilon={}",
                        thread::current().id(),
                        col_idx,
                        sat.id,
                        epsilon
                    );
                }
                for (idx, ob) in self.obs.iter().enumerate() {
                    let dim = measurement_sizes[idx];
                    let start = offsets[idx];
                    let end = start + dim;
                    let h_ref = &predicted_measurements.as_slice()[start..end];
                    let h_p_result = {
                        let _guard = SAAL_BLS_LOCK.lock().expect("saal bls lock poisoned");
                        ob.get_predicted_vector(sat)
                    };
                    let h_p = match h_p_result {
                        Ok(value) => value,
                        Err(err) => {
                            if trace {
                                eprintln!(
                                    "[tid={:?}] BLS jacobian err col={} ob_id={} sat_id={}",
                                    thread::current().id(),
                                    col_idx,
                                    ob.id,
                                    sat.id
                                );
                            }
                            return Err(err);
                        }
                    };
                    for i in 0..dim {
                        col[start + i] = (h_p[i] - h_ref[i]) / epsilon;
                    }
                }
                if trace {
                    eprintln!(
                        "[tid={:?}] BLS jacobian end col={} sat_id={}",
                        thread::current().id(),
                        col_idx,
                        sat.id
                    );
                }
                Ok(())
            })
            .collect();

        for result in results {
            result?;
        }

        for col_idx in 0..n {
            let col = &columns[col_idx];
            for i in 0..m {
                jacobian[(i, col_idx)] = col[i];
            }
        }
        Ok(())
    }

    fn get_delta_x(&mut self) -> Result<(), String> {
        let timing = Self::timing_enabled();
        let iter_start = if timing { Some(Instant::now()) } else { None };
        self.get_measurements_and_weights();

        let pred_start = if timing { Some(Instant::now()) } else { None };
        self.get_predicted_measurements()?;
        if let (true, Some(start)) = (timing, pred_start) {
            eprintln!("BLS timing: predicted_measurements {:.3?}", start.elapsed());
        }

        let jac_start = if timing { Some(Instant::now()) } else { None };
        self.get_jacobians()?;
        if let (true, Some(start)) = (timing, jac_start) {
            eprintln!("BLS timing: jacobians {:.3?}", start.elapsed());
        }

        let y = match self.measurement_vector.as_ref() {
            Some(measurements) => measurements,
            None => return Err("Missing cached measurements".to_string()),
        };
        let w = match self.weight_vector.as_ref() {
            Some(weights) => weights,
            None => return Err("Missing cached weights".to_string()),
        };
        let y_hat = match self.predicted_measurements.as_ref() {
            Some(predicted) => predicted,
            None => return Err("Missing cached predicted measurements".to_string()),
        };
        let mut r = (y - y_hat).clone_owned();

        let measurement_sizes = match self.measurement_sizes.as_ref() {
            Some(sizes) => sizes,
            None => return Err("Missing cached measurement sizes".to_string()),
        };
        wrap_ra_residuals(&mut r, measurement_sizes);

        let h = match self.jacobian.as_ref() {
            Some(matrix) => matrix,
            None => return Err("Missing cached jacobians".to_string()),
        };
        let normal_start = if timing { Some(Instant::now()) } else { None };
        let (mut n, mut b, wrss) = compute_normal_equations(h, w, &r);
        self.apply_eccentricity_constraint(&mut n, &mut b)?;
        if let (true, Some(start)) = (timing, normal_start) {
            eprintln!("BLS timing: normal_equations {:.3?}", start.elapsed());
        }

        // Compute weighted RMS for convergence testing
        let m: f64 = r.len() as f64;
        let current_weighted_rms = (wrss / m).sqrt();
        if self.weighted_rms.is_some() && (current_weighted_rms - self.weighted_rms.unwrap()).abs() < 1e-3 {
            self.converged = true;
        }

        self.weighted_rms = Some(current_weighted_rms);

        let solve_start = if timing { Some(Instant::now()) } else { None };
        self.delta_x = n.cholesky().map(|cholesky| cholesky.solve(&b));
        if let (true, Some(start)) = (timing, solve_start) {
            eprintln!("BLS timing: solve {:.3?}", start.elapsed());
        }
        match self.delta_x {
            Some(_) => Ok(()),
            None => Err("Unable to compute delta_x".to_string()),
        }
        .map(|_| {
            if let (true, Some(start)) = (timing, iter_start) {
                eprintln!("BLS timing: iteration_total {:.3?}", start.elapsed());
            }
        })
    }
}

/// Wrap Right Ascension residuals to [-180°, 180°] for shortest angular distance
/// Measurement vector is [RA1, DEC1, RA2, DEC2, ...], so every even index is RA
fn wrap_ra_residuals(residuals: &mut DVector<f64>, measurement_sizes: &[usize]) {
    let mut offset = 0;
    for &dim in measurement_sizes.iter() {
        if offset >= residuals.len() {
            break;
        }
        if dim > 0 {
            let ra_idx = offset;
            if residuals[ra_idx] > 180.0 {
                residuals[ra_idx] -= 360.0;
            } else if residuals[ra_idx] < -180.0 {
                residuals[ra_idx] += 360.0;
            }
        }
        offset += dim;
    }
}

/// Compute normal equations (H^T * W * H and H^T * W * r) using memory-efficient
/// element-wise operations for diagonal weight matrix W
/// Returns: (normal_matrix, rhs_vector, weighted_rss)
fn compute_normal_equations(h: &DMatrix<f64>, w: &DVector<f64>, r: &DVector<f64>) -> (DMatrix<f64>, DVector<f64>, f64) {
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

#[cfg(test)]
mod tests {
    use super::BatchLeastSquares;
    use crate::bodies::{Observatory, Satellite, Sensor};
    use crate::elements::{TLE, TopocentricElements};
    use crate::enums::{KeplerianType, TimeSystem};
    use crate::estimation::Observation;
    use crate::time::{Epoch, TimeSpan};
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct TestObservation {
        epoch: String,
        ra: f64,
        dec: f64,
        sensor_latitude: f64,
        sensor_longitude: f64,
        sensor_altitude: f64,
        angular_noise: f64,
    }

    #[test]
    fn test_solve() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let initial_tle = TLE::from_lines(
            "1 99999U          25334.80826079 -.00000092  00000 0  00000 0 0 0000",
            "2 99999   5.1462  74.9949 0001499 136.0805 318.9951  0.9987069300000",
            None,
        )
        .unwrap();
        let initial_sat = Satellite::from(initial_tle);
        let truth_tle = TLE::from_lines(
            "1 99999U          25335.68208465 +.00000000  16437-1  00000+0 4 00000",
            "2 99999   5.2391  74.7607 0001808 154.7302 254.7343  0.99871681000004",
            None,
        )
        .unwrap();
        let truth_sat = Satellite::from(truth_tle);

        let obs_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test-observations.json");
        let obs_contents = fs::read_to_string(obs_path).unwrap();
        let json_obs: Vec<TestObservation> = serde_json::from_str(&obs_contents).unwrap();

        let mut observations: Vec<Observation> = Vec::with_capacity(json_obs.len());
        for json_ob in json_obs {
            let epoch = Epoch::from_iso(&json_ob.epoch, TimeSystem::UTC);
            let site = Observatory::new(
                json_ob.sensor_latitude,
                json_ob.sensor_longitude,
                json_ob.sensor_altitude,
            );
            let els = TopocentricElements::new(json_ob.ra, json_ob.dec);
            let sensor = Sensor::new(json_ob.angular_noise);
            let ob = Observation::new(sensor, epoch, els, site.get_state_at_epoch(epoch).position);
            observations.push(ob);
        }

        let mut bls = BatchLeastSquares::new(observations, &initial_sat);
        bls.set_output_type(KeplerianType::MeanBrouwerXP);
        bls.set_estimate_srp(true);
        bls.solve().unwrap();

        let start = truth_sat.get_keplerian_state().unwrap().epoch;
        let end = start + TimeSpan::from_days(1.0);
        let step = TimeSpan::from_minutes(5.0);
        let mut next_epoch = start;
        let bls_sat = bls.get_current_estimate();
        let mut ranges = Vec::new();
        while next_epoch <= end {
            let state = bls_sat
                .get_relative_state_at_epoch(&truth_sat, next_epoch)
                .expect("Missing relative state");
            ranges.push(state.position.get_magnitude());
            next_epoch += step;
        }

        let rms = bls.get_rms().expect("Missing RMS");
        assert!(rms < 0.2);
        let max_range = ranges.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(max_range < 0.5);
    }

    #[test]
    fn test_b3_range_azimuth_elevation_solve() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let observations = Observation::from_saal_files("tests/sensors.dat", "tests/test-b3-obs.txt").unwrap();
        let input_line_1 = "1  1328U 65032A   22002.76919088 -.00000054  00000-0  46736-4 0    1";
        let input_line_2 = "2  1328  41.1831 135.3209 0257943 126.6976 235.7772 13.3866251677035";
        let tle = TLE::from_lines(input_line_1, input_line_2, None).unwrap();
        let a_priori_sat = Satellite::from(tle);
        let baseline_line_1 = "1  1328U 65032A   22009.34714486  .00000030  00000-0  46736-4 2    19";
        let baseline_line_2 = "2  1328  41.1831 107.1779 0257942 160.9267 256.1604 13.38111639 71239";
        let baseline_tle = TLE::from_lines(baseline_line_1, baseline_line_2, None).unwrap();
        let baseline_sat = Satellite::from(baseline_tle);

        let mut bls = BatchLeastSquares::new(observations, &a_priori_sat);
        bls.solve().unwrap();
        let start = baseline_sat.get_keplerian_state().unwrap().epoch;
        let end = start + TimeSpan::from_days(1.0);
        let step = TimeSpan::from_minutes(5.0);
        let mut next_epoch = start;
        let bls_sat = bls.get_current_estimate();
        let mut ranges = Vec::new();
        while next_epoch <= end {
            let state = bls_sat
                .get_relative_state_at_epoch(&baseline_sat, next_epoch)
                .expect("Missing relative state");
            ranges.push(state.position.get_magnitude());
            next_epoch += step;
        }
        assert!(bls.get_rms().unwrap() < 2.0);
        let max_range = ranges.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(max_range < 2.0);
    }
}
