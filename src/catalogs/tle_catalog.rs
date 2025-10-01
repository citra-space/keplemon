use crate::elements::{OrbitPlotData, OrbitPlotState, TLE};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TLECatalog {
    name: Option<String>,
    map: HashMap<String, TLE>,
}

#[pymethods]
impl TLECatalog {
    #[new]
    pub fn new() -> Self {
        TLECatalog {
            name: None,
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, tle: TLE) {
        self.map.insert(tle.get_id(), tle);
    }

    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    pub fn get(&self, satellite_id: String) -> Option<TLE> {
        self.map.get(&satellite_id).cloned()
    }

    pub fn remove(&mut self, satellite_id: String) {
        self.map.remove(&satellite_id);
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    fn __getitem__(&self, satellite_id: String) -> PyResult<TLE> {
        match self.map.get(&satellite_id) {
            Some(tle) => Ok(tle.clone()),
            None => Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Invalid key: {}",
                satellite_id
            ))),
        }
    }

    #[getter]
    pub fn get_count(&self) -> usize {
        self.map.len()
    }

    #[getter]
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    #[staticmethod]
    pub fn from_tle_file(file_path: &str) -> PyResult<TLECatalog> {
        let mut catalog = TLECatalog::default();
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        let num_chunks = match lines[1][0..1].parse::<u8>() {
            Ok(1) => 3,
            Ok(2) => 2,
            _ => panic!("Invalid TLE format"),
        };
        for chunk in lines.chunks(num_chunks) {
            let tle = match num_chunks {
                3 => TLE::from_lines(&chunk[0], &chunk[1], Some(&chunk[2])),
                2 => TLE::from_lines(&chunk[0], &chunk[1], None),
                _ => {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Invalid TLE line count of {} in {}",
                        num_chunks, file_path
                    )));
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
            plot_data.add_state(OrbitPlotState::from_keplerian_state(&tle.get_keplerian_state()));
        }
        plot_data
    }
}
