use super::{TimeComponents, TimeSpan};
use crate::configs::ZERO_TOLERANCE;
use crate::enums::TimeSystem;
use saal::time;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Epoch {
    pub days_since_1950: f64,
    time_system: TimeSystem,
}

impl Hash for Epoch {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_iso().hash(state);
    }
}

impl Sub for Epoch {
    type Output = TimeSpan;

    fn sub(self, other_epoch: Self) -> TimeSpan {
        let norm_ds50 = other_epoch.to_system(self.time_system).unwrap().days_since_1950;
        TimeSpan::from_days(self.days_since_1950 - norm_ds50)
    }
}

impl Sub<TimeSpan> for Epoch {
    type Output = Epoch;

    fn sub(self, rhs: TimeSpan) -> Epoch {
        Epoch {
            days_since_1950: self.days_since_1950 - rhs.in_days(),
            time_system: self.time_system,
        }
    }
}

impl Add<TimeSpan> for Epoch {
    type Output = Epoch;

    fn add(self, rhs: TimeSpan) -> Epoch {
        Epoch {
            days_since_1950: self.days_since_1950 + rhs.in_days(),
            time_system: self.time_system,
        }
    }
}

impl AddAssign<TimeSpan> for Epoch {
    fn add_assign(&mut self, rhs: TimeSpan) {
        self.days_since_1950 += rhs.in_days();
    }
}

impl PartialOrd for Epoch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Epoch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if (self.days_since_1950 - other.days_since_1950).abs() < ZERO_TOLERANCE {
            std::cmp::Ordering::Equal
        } else if self.days_since_1950 < other.days_since_1950 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
impl Eq for Epoch {}

impl Epoch {}

impl Epoch {
    pub fn get_time_system(&self) -> TimeSystem {
        self.time_system
    }

    pub fn get_gst(&self) -> f64 {
        time::get_fk5_greenwich_angle(self.to_system(TimeSystem::UT1).unwrap().days_since_1950)
    }

    pub fn from_days_since_1950(days_since_1950: f64, time_system: TimeSystem) -> Self {
        Self {
            days_since_1950,
            time_system,
        }
    }

    pub fn from_iso(iso: &str, time_system: TimeSystem) -> Self {
        let components = TimeComponents::from_iso(iso);
        Self::from_time_components(&components, time_system)
    }

    pub fn from_dtg(dtg: &str, time_system: TimeSystem) -> Self {
        let days_since_1950 = time::dtg_to_ds50(dtg);
        Self {
            days_since_1950,
            time_system,
        }
    }

    pub fn from_time_components(components: &TimeComponents, time_system: TimeSystem) -> Self {
        let days_since_1950 = time::ymd_components_to_ds50(
            components.year,
            components.month,
            components.day,
            components.hour,
            components.minute,
            components.second,
        );
        Self {
            days_since_1950,
            time_system,
        }
    }

    pub fn to_dtg_20(&self) -> String {
        time::ds50_to_dtg20(self.days_since_1950)
    }

    pub fn to_dtg_19(&self) -> String {
        time::ds50_to_dtg19(self.days_since_1950)
    }

    pub fn to_dtg_17(&self) -> String {
        time::ds50_to_dtg17(self.days_since_1950)
    }

    pub fn to_dtg_15(&self) -> String {
        time::ds50_to_dtg15(self.days_since_1950)
    }

    pub fn to_time_components(&self) -> TimeComponents {
        let components = time::ds50_to_ymd_components(self.days_since_1950);
        TimeComponents {
            year: components.0,
            month: components.1,
            day: components.2,
            hour: components.3,
            minute: components.4,
            second: components.5,
        }
    }

    pub fn get_day_of_year(&self) -> f64 {
        time::ds50_to_year_doy(self.days_since_1950).1
    }

    pub fn to_fk4_greenwich_angle(&self) -> f64 {
        time::get_fk4_greenwich_angle(self.to_system(TimeSystem::UT1).unwrap().days_since_1950)
    }

    pub fn to_fk5_greenwich_angle(&self) -> f64 {
        time::get_fk5_greenwich_angle(self.to_system(TimeSystem::UT1).unwrap().days_since_1950)
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.days_since_1950 > other.days_since_1950
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.days_since_1950 < other.days_since_1950
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.days_since_1950 != other.days_since_1950
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.days_since_1950 >= other.days_since_1950
    }

    fn __le__(&self, other: &Self) -> bool {
        self.days_since_1950 <= other.days_since_1950
    }

    pub fn to_iso(&self) -> String {
        self.to_time_components().to_iso()
    }

    pub fn to_system(&self, time_system: TimeSystem) -> Result<Self, String> {
        let days_since_1950 = match self.time_system {
            TimeSystem::TAI => match time_system {
                TimeSystem::UTC => time::tai_to_utc(self.days_since_1950),
                TimeSystem::UT1 => time::tai_to_ut1(self.days_since_1950),
                TimeSystem::TAI => self.days_since_1950,
                _ => -1.0,
            },
            TimeSystem::UTC => match time_system {
                TimeSystem::TAI => time::utc_to_tai(self.days_since_1950),
                TimeSystem::UT1 => time::utc_to_ut1(self.days_since_1950),
                TimeSystem::TT => time::utc_to_tt(self.days_since_1950),
                TimeSystem::UTC => self.days_since_1950,
            },
            TimeSystem::TT => -1.0,
            TimeSystem::UT1 => -1.0,
        };

        match days_since_1950 {
            -1.0 => Err(format!(
                "Conversion from {:?} to {:?} not implemented",
                self.time_system, time_system
            )),
            _ => Ok(Self {
                days_since_1950,
                time_system,
            }),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Epoch;
    use crate::enums::TimeSystem;
    use crate::time::{TimeComponents, TimeSpan};
    use approx::assert_abs_diff_eq;

    fn oct_31_1989_044242() -> Epoch {
        let components = TimeComponents {
            year: 1989,
            month: 10,
            day: 31,
            hour: 4,
            minute: 42,
            second: 42.0,
        };
        Epoch::from_time_components(&components, TimeSystem::UTC)
    }

    fn dec_20_2012_000000() -> Epoch {
        let components = TimeComponents {
            year: 2012,
            month: 12,
            day: 20,
            hour: 0,
            minute: 0,
            second: 0.0,
        };
        Epoch::from_time_components(&components, TimeSystem::UTC)
    }

    #[test]
    fn test_to_dtg_20() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        assert_eq!(dec_20_2012_000000().to_dtg_20(), "2012/355 0000 00.000");
        assert_eq!(oct_31_1989_044242().to_dtg_20(), "1989/304 0442 42.000");
    }

    #[test]
    fn test_to_dtg_19() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        assert_eq!(dec_20_2012_000000().to_dtg_19(), "2012Dec20000000.000");
        assert_eq!(oct_31_1989_044242().to_dtg_19(), "1989Oct31044242.000");
    }

    #[test]
    fn test_to_dtg_17() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        assert_eq!(dec_20_2012_000000().to_dtg_17(), "2012/355.00000000");
        assert_eq!(oct_31_1989_044242().to_dtg_17(), "1989/304.19631944");
    }

    #[test]
    fn test_to_dtg_15() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        assert_eq!(dec_20_2012_000000().to_dtg_15(), "12355000000.000");
        assert_eq!(oct_31_1989_044242().to_dtg_15(), "89304044242.000");
    }

    #[test]
    fn test_to_fk4_greenwich_angle() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        assert_abs_diff_eq!(
            dec_20_2012_000000().to_fk4_greenwich_angle(),
            1.5529973858442574,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            oct_31_1989_044242().to_fk4_greenwich_angle(),
            1.9222952766001384,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_to_fk5_greenwich_angle() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        assert_abs_diff_eq!(
            dec_20_2012_000000().to_fk5_greenwich_angle(),
            1.553003824819477,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            oct_31_1989_044242().to_fk5_greenwich_angle(),
            1.92230028431527,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_to_time_components() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let time_components = dec_20_2012_000000().to_time_components();
        assert_eq!(time_components.year, 2012);
        assert_eq!(time_components.month, 12);
        assert_eq!(time_components.day, 20);
        assert_eq!(time_components.hour, 0);
        assert_eq!(time_components.minute, 0);
        assert_eq!(time_components.second, 0.0);
    }

    #[test]
    fn test_to_system() {
        let _guard = crate::test_lock::GLOBAL_TEST_LOCK.lock().unwrap();
        let utc = dec_20_2012_000000();
        let tai = utc.to_system(TimeSystem::TAI).unwrap();
        let ut1 = utc.to_system(TimeSystem::UT1).unwrap();
        let tt = utc.to_system(TimeSystem::TT).unwrap();

        let utc_minus_tai = TimeSpan::from_days(utc.days_since_1950 - tai.days_since_1950);
        let utc_minus_ut1 = TimeSpan::from_days(utc.days_since_1950 - ut1.days_since_1950);
        let utc_minus_tt = TimeSpan::from_days(utc.days_since_1950 - tt.days_since_1950);
        assert_abs_diff_eq!(utc_minus_tai.in_seconds(), -35.0, epsilon = 1e-6);
        assert_abs_diff_eq!(utc_minus_ut1.in_seconds(), -0.2846799208782613, epsilon = 1e-6);
        assert_abs_diff_eq!(utc_minus_tt.in_seconds(), -67.184, epsilon = 1e-6);
    }
}
