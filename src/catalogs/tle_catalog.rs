use crate::bodies::Satellite;
use crate::configs::{ATMOSPHERE_BOUNDARY_RADIUS, DEFAULT_STEP_MINUTES};
use crate::elements::{OrbitPlotData, OrbitPlotState, TLE};
use crate::enums::KeplerianType;
use crate::estimation::{BatchLeastSquares, Observation};
use crate::time::TimeSpan;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TLECatalog {
    pub name: Option<String>,
    map: HashMap<String, TLE>,
}

impl TLECatalog {
    pub fn new() -> Self {
        TLECatalog {
            name: None,
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, tle: TLE) {
        self.map.insert(tle.satellite_id.clone(), tle);
    }

    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    pub fn get(&self, satellite_id: String) -> Option<TLE> {
        self.map.get(&satellite_id).cloned()
    }

    /// Get a TLE by its NORAD catalog number (e.g., 25544 for ISS)
    pub fn get_by_norad_id(&self, norad_id: i32) -> Option<TLE> {
        self.map.values().find(|tle| tle.norad_id == norad_id).cloned()
    }

    pub fn remove(&mut self, satellite_id: String) {
        self.map.remove(&satellite_id);
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn get_count(&self) -> usize {
        self.map.len()
    }

    /// Returns an iterator over all TLEs in the catalog
    pub fn values(&self) -> impl Iterator<Item = &TLE> {
        self.map.values()
    }

    /// Returns all TLEs as a Vec
    pub fn list(&self) -> Vec<TLE> {
        self.map.values().cloned().collect()
    }

    pub fn from_tle_file(file_path: &str) -> Result<TLECatalog, String> {
        let mut catalog = TLECatalog::default();
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        let num_chunks = match lines[1][0..1].parse::<u8>() {
            Ok(1) => 3,
            Ok(2) => 2,
            _ => return Err(format!("Invalid TLE format in {}", file_path)),
        };
        for chunk in lines.chunks(num_chunks) {
            let tle = match num_chunks {
                3 => TLE::from_lines(&chunk[0], &chunk[1], Some(&chunk[2])),
                2 => TLE::from_lines(&chunk[0], &chunk[1], None),
                _ => {
                    return Err(format!("Invalid TLE line count of {} in {}", num_chunks, file_path));
                }
            };
            catalog.add(tle?);
        }
        catalog.name = Some(file_path.to_string());
        Ok(catalog)
    }

    pub fn get_plot_data(&self) -> OrbitPlotData {
        let mut plot_data = OrbitPlotData::new(self.name.clone().unwrap_or_else(|| "TLE Catalog".to_string()));
        for tle in self.map.values() {
            plot_data.add_state(OrbitPlotState::from(&tle.get_keplerian_state()));
        }
        plot_data
    }

    pub fn fit_best_tle(&self, srp_coefficient: Option<f64>, drag_coefficient: Option<f64>) -> Result<TLE, String> {
        let mut tles = self.list();
        if tles.is_empty() {
            return Err("TLE catalog is empty".to_string());
        }

        // Sort by epoch descending for backward stepping through time
        tles.sort_by(|a, b| {
            b.get_epoch()
                .days_since_1950
                .partial_cmp(&a.get_epoch().days_since_1950)
                .unwrap()
        });

        // Use the most recent TLE as a priori (first in descending order)
        let a_priori_tle = &tles[0];

        // Generate observations from each TLE, stepping backward until the previous TLE or max_days
        let mut obs: Vec<Observation> = Vec::new();
        let step = TimeSpan::from_minutes(DEFAULT_STEP_MINUTES);

        for (i, tle) in tles.iter().enumerate() {
            let start_epoch = tle.get_epoch();

            let max_end = start_epoch - TimeSpan::from_minutes(tle.get_period());
            let end_epoch = if i + 1 < tles.len() {
                let prev_tle_epoch = tles[i + 1].get_epoch();
                if prev_tle_epoch > max_end {
                    prev_tle_epoch
                } else {
                    max_end
                }
            } else {
                max_end
            };

            let mut current_epoch = start_epoch;
            while current_epoch >= end_epoch {
                if let Ok(observation) = tle.get_observation_at_epoch(current_epoch) {
                    obs.push(observation);
                }
                current_epoch = current_epoch - step;
            }
        }
        log::debug!(
            "Generated {} observations between {} and {} using {} TLEs",
            obs.len(),
            obs.last().unwrap().get_epoch().to_iso(),
            obs.first().unwrap().get_epoch().to_iso(),
            tles.len()
        );

        let mut a_priori_satellite = Satellite::from(a_priori_tle.clone());
        let mut force_properties = a_priori_satellite.get_force_properties();
        let mut use_drag = a_priori_satellite.get_periapsis().unwrap() < ATMOSPHERE_BOUNDARY_RADIUS;
        let mut use_srp = a_priori_satellite.get_apoapsis().unwrap() > ATMOSPHERE_BOUNDARY_RADIUS;
        if let Some(srp) = srp_coefficient {
            force_properties.srp_coefficient = srp;
            use_srp = false;
        }
        if let Some(dc) = drag_coefficient {
            force_properties.drag_coefficient = dc;
            use_drag = false;
        }
        a_priori_satellite.set_force_properties(force_properties);
        let mut bls = BatchLeastSquares::new(obs, &a_priori_satellite);

        bls.set_output_type(KeplerianType::MeanBrouwerXP);
        bls.set_estimate_drag(use_drag);
        bls.set_estimate_srp(use_srp);
        bls.solve()?;
        Ok(bls.get_current_estimate().into())
    }
}
