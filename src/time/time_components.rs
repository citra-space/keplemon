#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeComponents {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: f64,
}

impl TimeComponents {}

impl TimeComponents {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: f64) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn to_iso(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:06.3}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    pub fn from_iso(iso: &str) -> Self {
        let date_time: Vec<&str> = iso.split("T").collect();
        let ymd: Vec<&str> = date_time[0].split("-").collect();
        let z_stripped = date_time[1].replace("Z", "");
        let hms: Vec<&str> = z_stripped.split(":").collect();

        Self {
            year: ymd[0].parse().unwrap(),
            month: ymd[1].parse().unwrap(),
            day: ymd[2].parse().unwrap(),
            hour: hms[0].parse().unwrap(),
            minute: hms[1].parse().unwrap(),
            second: hms[2].parse().unwrap(),
        }
    }
}
