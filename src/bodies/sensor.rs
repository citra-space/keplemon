use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    pub id: String,
    pub name: Option<String>,
    pub angular_noise: f64,
    pub range_noise: Option<f64>,
    pub range_rate_noise: Option<f64>,
    pub angular_rate_noise: Option<f64>,
}

impl Sensor {
    pub fn new(angular_noise: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            angular_noise,
            range_noise: None,
            range_rate_noise: None,
            angular_rate_noise: None,
        }
    }
}
