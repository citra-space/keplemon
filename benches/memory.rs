use keplemon::bodies::{Observatory, Satellite};
use keplemon::elements::{CartesianState, CartesianVector, Ephemeris, TLE};
use keplemon::enums::{ReferenceFrame, TimeSystem};
use keplemon::events::HorizonAccessReport;
use keplemon::time::{Epoch, TimeSpan};
use rayon::prelude::*;
use std::env;
use std::hint::black_box;

fn rss_bytes() -> u64 {
    unsafe {
        let mut usage: libc::rusage = std::mem::zeroed();
        if libc::getrusage(libc::RUSAGE_SELF, &mut usage) != 0 {
            return 0;
        }
        #[cfg(target_os = "linux")]
        {
            (usage.ru_maxrss as u64) * 1024
        }
        #[cfg(not(target_os = "linux"))]
        {
            usage.ru_maxrss as u64
        }
    }
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == flag)
        .and_then(|idx| args.get(idx + 1))
        .cloned()
}

fn parse_usize(args: &[String], flag: &str, default: usize) -> usize {
    arg_value(args, flag)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
}

fn parse_f64(args: &[String], flag: &str, default: f64) -> f64 {
    arg_value(args, flag)
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(default)
}

fn make_satellite() -> Satellite {
    let line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999";
    let line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";
    let tle = TLE::from_lines("ISS", line_1, Some(line_2)).expect("failed to parse TLE");
    Satellite::from(tle)
}

fn make_satellite_xp() -> Satellite {
    let line_1 = "1 25544U 98067A   25105.51605324 +.00000000  10000-1  20000-1 4 00002";
    let line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.49300070000008";
    let tle = TLE::from_lines("ISS", line_1, Some(line_2)).expect("failed to parse XP TLE");
    Satellite::from(tle)
}

fn make_ephemeris_points(points: usize, step_seconds: f64) -> Ephemeris {
    let start = Epoch::from_iso("2025-01-01T00:00:00.000000Z", TimeSystem::UTC);
    let position = CartesianVector::new(7000.0, 0.0, 0.0);
    let velocity = CartesianVector::new(0.0, 7.5, 1.0);
    let state = CartesianState::new(start, position, velocity, ReferenceFrame::TEME);
    let ephemeris = Ephemeris::new("mem-ephem".to_string(), Some(0), state).expect("ephemeris init");

    let step = TimeSpan::from_seconds(step_seconds);
    for i in 1..points {
        let epoch = start + (step * i as f64);
        let next = CartesianState::new(epoch, position, velocity, ReferenceFrame::TEME);
        ephemeris.add_state(next).expect("ephemeris add");
    }
    ephemeris
}

fn make_observatories(count: usize) -> Vec<Observatory> {
    let mut observatories = Vec::with_capacity(count);
    for idx in 0..count {
        let lat = -60.0 + (idx as f64 % 120.0);
        let lon = -180.0 + ((idx as f64 * 37.0) % 360.0);
        let alt = 100.0 + (idx as f64 % 500.0);
        observatories.push(Observatory::new(lat, lon, alt));
    }
    observatories
}

fn make_satellite_list_xp(count: usize) -> Vec<Satellite> {
    let mut satellites = Vec::with_capacity(count);
    for idx in 0..count {
        let mut sat = make_satellite_xp();
        sat.id = format!("sat-{}", idx);
        satellites.push(sat);
    }
    satellites
}

fn usage() {
    eprintln!("Usage:");
    eprintln!("  cargo bench --bench memory -- --case satellite --count 10000");
    eprintln!("  cargo bench --bench memory -- --case ephemeris --points 100000 --step-seconds 60");
    eprintln!("  cargo bench --bench memory -- --case access --observatories 10 --hours 4 --step-minutes 1");
    eprintln!("  cargo bench --bench memory -- --case horizon-report --satellites 30000 --hours 4 --step-minutes 1");
    eprintln!("Notes:");
    eprintln!("  ru_maxrss is a max RSS reading; run one case per process for clean deltas.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let case = arg_value(&args, "--case").unwrap_or_else(|| "satellite".to_string());

    match case.as_str() {
        "satellite" => {
            let count = parse_usize(&args, "--count", 10_000).max(1);
            let before_peak = rss_bytes();
            let satellites: Vec<Satellite> = (0..count).map(|_| make_satellite()).collect();
            let after_peak = rss_bytes();
            let peak_delta = after_peak.saturating_sub(before_peak);
            black_box(&satellites);
            println!("case: satellite");
            println!("satellites: {}", count);
            println!("peak_rss_bytes: {}", after_peak);
            println!("peak_delta_bytes: {}", peak_delta);
            println!("per_satellite_bytes: {}", peak_delta / count as u64);
        }
        "ephemeris" => {
            let points = parse_usize(&args, "--points", 100_000).max(1);
            let step_seconds = parse_f64(&args, "--step-seconds", 60.0);
            let before_peak = rss_bytes();
            let ephemeris = make_ephemeris_points(points, step_seconds);
            let after_peak = rss_bytes();
            let peak_delta = after_peak.saturating_sub(before_peak);
            black_box(&ephemeris);
            println!("case: ephemeris");
            println!("points: {}", points);
            println!("step_seconds: {}", step_seconds);
            println!("peak_rss_bytes: {}", after_peak);
            println!("peak_delta_bytes: {}", peak_delta);
            println!("per_point_bytes: {}", peak_delta / points as u64);
        }
        "access" => {
            let observatories = parse_usize(&args, "--observatories", 10).max(1);
            let hours = parse_f64(&args, "--hours", 4.0);
            let step_minutes = parse_f64(&args, "--step-minutes", 1.0);
            let mut sat = make_satellite();
            let obs = make_observatories(observatories);
            let start = Epoch::from_iso("2025-04-18T04:00:00.000000Z", TimeSystem::UTC);
            let end = start + TimeSpan::from_hours(hours);
            let min_elevation = 10.0;
            let min_duration = TimeSpan::from_minutes(step_minutes);
            let before_peak = rss_bytes();
            let report = sat.get_observatory_access_report(obs, start, end, min_elevation, min_duration);
            let after_peak = rss_bytes();
            let peak_delta = after_peak.saturating_sub(before_peak);
            black_box(&report);
            drop(report);
            println!("case: access");
            println!("observatories: {}", observatories);
            println!("hours: {}", hours);
            println!("step_minutes: {}", step_minutes);
            println!("peak_rss_bytes: {}", after_peak);
            println!("peak_delta_bytes: {}", peak_delta);
        }
        "horizon-report" => {
            let satellites_count = parse_usize(&args, "--satellites", 30_000).max(1);
            let hours = parse_f64(&args, "--hours", 4.0);
            let step_minutes = parse_f64(&args, "--step-minutes", 1.0);
            let site = Observatory::new(34.0, -118.0, 100.0);
            let start = Epoch::from_iso("2025-04-18T04:00:00.000000Z", TimeSystem::UTC);
            let end = start + TimeSpan::from_hours(hours);
            let min_elevation = 10.0;
            let min_duration = TimeSpan::from_minutes(step_minutes);
            let mut satellites = make_satellite_list_xp(satellites_count);
            let before_peak = rss_bytes();
            let site_ephem = site.get_ephemeris(start, end, min_duration);
            let sat_ephem_list: Vec<Ephemeris> = satellites
                .par_iter_mut()
                .filter_map(|sat| sat.get_ephemeris(start, end, min_duration))
                .collect();
            let after_ephem_peak = rss_bytes();
            let ephem_peak_delta = after_ephem_peak.saturating_sub(before_peak);

            let mut report = HorizonAccessReport::new(start, end, min_elevation, min_duration);
            let num = sat_ephem_list.len();
            let accesses = (0..num)
                .into_par_iter()
                .filter_map(|i| {
                    let sat_ephem = &sat_ephem_list[i];
                    sat_ephem.get_horizon_accesses(&site_ephem, min_elevation, min_duration)
                })
                .collect::<Vec<_>>();
            report.set_accesses(accesses.into_iter().flatten().collect());
            let after_access_peak = rss_bytes();
            let access_peak_delta = after_access_peak.saturating_sub(after_ephem_peak);
            let total_peak_delta = after_access_peak.saturating_sub(before_peak);
            black_box(&report);
            drop(report);
            println!("case: horizon-report");
            println!("satellites: {}", satellites_count);
            println!("hours: {}", hours);
            println!("step_minutes: {}", step_minutes);
            println!("ephem_peak_rss_bytes: {}", after_ephem_peak);
            println!("ephem_peak_delta_bytes: {}", ephem_peak_delta);
            println!("access_peak_rss_bytes: {}", after_access_peak);
            println!("access_peak_delta_bytes: {}", access_peak_delta);
            println!("total_peak_delta_bytes: {}", total_peak_delta);
        }
        _ => usage(),
    }
}
