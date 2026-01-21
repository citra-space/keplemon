use super::ProximityEvent;
use crate::time::Epoch;

pub struct ProximityReport {
    start: Epoch,
    end: Epoch,
    distance_threshold: f64,
    events: Vec<ProximityEvent>,
}

impl ProximityReport {
    pub fn new(start: Epoch, end: Epoch, distance_threshold: f64) -> Self {
        Self {
            start,
            end,
            distance_threshold,
            events: Vec::new(),
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

    pub fn get_events(&self) -> Vec<ProximityEvent> {
        self.events.clone()
    }

    pub fn set_events(&mut self, events: Vec<ProximityEvent>) {
        self.events = events;
    }
}
