use saal::satellite;

#[derive(Debug, Clone, PartialEq, Copy)]
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

impl From<[f64; satellite::XA_DELTA_SIZE]> for ObservationResidual {
    fn from(delta: [f64; satellite::XA_DELTA_SIZE]) -> Self {
        Self {
            range: delta[satellite::XA_DELTA_POS],
            time: delta[satellite::XA_DELTA_TIME],
            radial: delta[satellite::XA_DELTA_PRADIAL],
            in_track: delta[satellite::XA_DELTA_PINTRCK],
            cross_track: delta[satellite::XA_DELTA_PCRSSTRCK],
            velocity: delta[satellite::XA_DELTA_VEL],
            radial_velocity: delta[satellite::XA_DELTA_VRADIAL],
            in_track_velocity: delta[satellite::XA_DELTA_VINTRCK],
            cross_track_velocity: delta[satellite::XA_DELTA_VCRSSTRCK],
            beta: delta[satellite::XA_DELTA_BETA],
            height: delta[satellite::XA_DELTA_HEIGHT],
            angular_momentum: delta[satellite::XA_DELTA_ANGMOM],
        }
    }
}

impl ObservationResidual {
    pub fn get_range(&self) -> f64 {
        self.range
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_radial(&self) -> f64 {
        self.radial
    }

    pub fn get_in_track(&self) -> f64 {
        self.in_track
    }

    pub fn get_cross_track(&self) -> f64 {
        self.cross_track
    }

    pub fn get_velocity(&self) -> f64 {
        self.velocity
    }

    pub fn get_radial_velocity(&self) -> f64 {
        self.radial_velocity
    }

    pub fn get_in_track_velocity(&self) -> f64 {
        self.in_track_velocity
    }

    pub fn get_cross_track_velocity(&self) -> f64 {
        self.cross_track_velocity
    }

    pub fn get_beta(&self) -> f64 {
        self.beta
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    pub fn get_angular_momentum(&self) -> f64 {
        self.angular_momentum
    }
}
