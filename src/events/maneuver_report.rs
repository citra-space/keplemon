use super::ManeuverEvent;
use crate::time::Epoch;

pub struct ManeuverReport {
    start: Epoch,
    end: Epoch,
    distance_threshold: f64,
    velocity_threshold: f64,
    maneuvers: Vec<ManeuverEvent>,
}

impl ManeuverReport {
    pub fn new(start: Epoch, end: Epoch, distance_threshold: f64, velocity_threshold: f64) -> Self {
        Self {
            start,
            end,
            distance_threshold,
            velocity_threshold,
            maneuvers: Vec::new(),
        }
    }

    pub fn get_start(&self) -> Epoch {
        self.start
    }

    pub fn get_end(&self) -> Epoch {
        self.end
    }

    pub fn get_distance_threshold(&self) -> f64 {
        self.distance_threshold
    }

    pub fn get_velocity_threshold(&self) -> f64 {
        self.velocity_threshold
    }

    pub fn get_maneuvers(&self) -> Vec<ManeuverEvent> {
        self.maneuvers.clone()
    }

    pub fn set_maneuvers(&mut self, maneuvers: Vec<ManeuverEvent>) {
        self.maneuvers = maneuvers;
    }
}
