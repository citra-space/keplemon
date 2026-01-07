use super::CloseApproach;
use crate::time::Epoch;

pub struct CloseApproachReport {
    start: Epoch,
    end: Epoch,
    distance_threshold: f64,
    close_approaches: Vec<CloseApproach>,
}

impl CloseApproachReport {
    pub fn new(start: Epoch, end: Epoch, distance_threshold: f64) -> Self {
        Self {
            start,
            end,
            distance_threshold,
            close_approaches: Vec::new(),
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

    pub fn get_close_approaches(&self) -> Vec<CloseApproach> {
        self.close_approaches.clone()
    }

    pub fn set_close_approaches(&mut self, close_approaches: Vec<CloseApproach>) {
        self.close_approaches = close_approaches;
    }
}
