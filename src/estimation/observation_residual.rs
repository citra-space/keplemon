use crate::saal::sat_state_interface;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationResidual {
    range: f64,
    time: f64,
    radial: f64,
    in_track: f64,
    cross_track: f64,
    velocity: f64,
    radial_velocity: f64,
    in_track_velocity: f64,
    cross_track_velocity: f64,
    beta: f64,
    height: f64,
    angular_momentum: f64,
}

impl From<[f64; sat_state_interface::XA_DELTA_SIZE]> for ObservationResidual {
    fn from(delta: [f64; sat_state_interface::XA_DELTA_SIZE]) -> Self {
        Self {
            range: delta[sat_state_interface::XA_DELTA_POS],
            time: delta[sat_state_interface::XA_DELTA_TIME],
            radial: delta[sat_state_interface::XA_DELTA_PRADIAL],
            in_track: delta[sat_state_interface::XA_DELTA_PINTRCK],
            cross_track: delta[sat_state_interface::XA_DELTA_PCRSSTRCK],
            velocity: delta[sat_state_interface::XA_DELTA_VEL],
            radial_velocity: delta[sat_state_interface::XA_DELTA_VRADIAL],
            in_track_velocity: delta[sat_state_interface::XA_DELTA_VINTRCK],
            cross_track_velocity: delta[sat_state_interface::XA_DELTA_VCRSSTRCK],
            beta: delta[sat_state_interface::XA_DELTA_BETA],
            height: delta[sat_state_interface::XA_DELTA_HEIGHT],
            angular_momentum: delta[sat_state_interface::XA_DELTA_ANGMOM],
        }
    }
}

impl ObservationResidual {
    /// Create a residual with only the range component (used for TDOA observations)
    pub fn with_range_only(range: f64) -> Self {
        Self {
            range,
            time: 0.0,
            radial: 0.0,
            in_track: 0.0,
            cross_track: 0.0,
            velocity: 0.0,
            radial_velocity: 0.0,
            in_track_velocity: 0.0,
            cross_track_velocity: 0.0,
            beta: 0.0,
            height: 0.0,
            angular_momentum: 0.0,
        }
    }

    /// Create a residual with only the radial velocity component (used for FDOA observations)
    pub fn with_radial_velocity_only(radial_velocity: f64) -> Self {
        Self {
            range: 0.0,
            time: 0.0,
            radial: 0.0,
            in_track: 0.0,
            cross_track: 0.0,
            velocity: 0.0,
            radial_velocity,
            in_track_velocity: 0.0,
            cross_track_velocity: 0.0,
            beta: 0.0,
            height: 0.0,
            angular_momentum: 0.0,
        }
    }
}

#[pymethods]
impl ObservationResidual {
    #[getter]
    pub fn get_range(&self) -> f64 {
        self.range
    }

    #[getter]
    pub fn get_time(&self) -> f64 {
        self.time
    }

    #[getter]
    pub fn get_radial(&self) -> f64 {
        self.radial
    }

    #[getter]
    pub fn get_in_track(&self) -> f64 {
        self.in_track
    }

    #[getter]
    pub fn get_cross_track(&self) -> f64 {
        self.cross_track
    }

    #[getter]
    pub fn get_velocity(&self) -> f64 {
        self.velocity
    }

    #[getter]
    pub fn get_radial_velocity(&self) -> f64 {
        self.radial_velocity
    }

    #[getter]
    pub fn get_in_track_velocity(&self) -> f64 {
        self.in_track_velocity
    }

    #[getter]
    pub fn get_cross_track_velocity(&self) -> f64 {
        self.cross_track_velocity
    }

    #[getter]
    pub fn get_beta(&self) -> f64 {
        self.beta
    }

    #[getter]
    pub fn get_height(&self) -> f64 {
        self.height
    }

    #[getter]
    pub fn get_angular_momentum(&self) -> f64 {
        self.angular_momentum
    }
}
