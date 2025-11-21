use crate::bodies::Satellite;
use crate::time::Epoch;

/// Trait for different types of observations used in orbit determination
pub trait ObservationType: Send + Sync {
    /// Get measurement values and corresponding weights (1/sigma^2)
    /// Returns (measurement_vector, weight_vector)
    fn get_measurement_and_weight_vector(&self) -> (Vec<f64>, Vec<f64>);

    /// Get predicted measurements for a given satellite state
    fn get_predicted_vector(&self, satellite: &Satellite) -> Result<Vec<f64>, String>;

    /// Get the observation epoch
    fn get_epoch(&self) -> Epoch;

    /// Get the satellite ID if set
    fn get_satellite_id(&self) -> Option<i32>;

    /// Get the dimension (number of measurements) this observation contributes
    fn dimension(&self) -> usize {
        self.get_measurement_and_weight_vector().0.len()
    }
}
