use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::path::PathBuf;

fn bench_astro_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("astro");

    let ds50_utc = 8431.0;
    let ds50_tt = saal::time::utc_to_tt(ds50_utc);
    let kep = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612];
    let eqnx = [
        0.005756008409,
        0.002453246053,
        0.130405060328,
        -0.503224317374,
        617.8480000,
        2.005848298418,
    ];
    let posvel = [
        -3032.21272487,
        -15025.7763831,
        21806.4954366,
        3.7543500202,
        -0.889562019026,
        -0.114933710268,
    ];
    let j2000_posvel = [-2436.45, 6789.12, 10234.56, -1.234, 5.678, -2.345];
    let teme_pos = [-3032.21272487, -15025.7763831, 21806.4954366];
    let efg_pos = [6524.834, 6862.875, 6448.296];
    let efg_posvel = [6524.834, 6862.875, 6448.296, 1.1, -2.2, 0.3];
    let ecr_posvel = [6524.834, 6862.875, 6448.296, -0.4, 0.5, -0.6];
    let lla = [34.352495, 46.446417, 5085.218731];
    let xa_rae = [1000.0, 1.2, 0.5, 0.1, 0.01, 0.02];
    let teme_posvel = [
        -3032.21272487,
        -15025.7763831,
        21806.4954366,
        3.7543500202,
        -0.889562019026,
        -0.114933710268,
    ];
    let mut cov_eqnx = [[0.0; 6]; 6];
    for (i, row) in cov_eqnx.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    let mut cov_uvw = [[0.0; 6]; 6];
    for (i, row) in cov_uvw.iter_mut().enumerate() {
        row[i] = 2.0;
    }

    let jpl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("JPLcon_1950_2050.405");
    if let Some(path) = jpl_path.to_str() {
        saal::astro::set_jpl_ephemeris_file_path(path);
    }

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::astro::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("sma_to_mean_motion", "42164.17142"), |b| {
        b.iter(|| saal::astro::sma_to_mean_motion(black_box(42164.17142)));
    });
    group.bench_function(BenchmarkId::new("keplerian_to_cartesian", "kep"), |b| {
        b.iter(|| saal::astro::keplerian_to_cartesian(black_box(&kep)));
    });
    group.bench_function(BenchmarkId::new("cartesian_to_keplerian", "posvel"), |b| {
        b.iter(|| saal::astro::cartesian_to_keplerian(black_box(&posvel)));
    });
    group.bench_function(BenchmarkId::new("set_jpl_ephemeris_file_path", "jpl_path"), |b| {
        let path = jpl_path.to_str().expect("JPL path missing");
        b.iter(|| saal::astro::set_jpl_ephemeris_file_path(black_box(path)));
    });
    group.bench_function(BenchmarkId::new("j2000_to_teme", "posvel"), |b| {
        b.iter(|| saal::astro::j2000_to_teme(black_box(ds50_utc), black_box(&j2000_posvel)));
    });
    group.bench_function(BenchmarkId::new("j2000_to_efg", "posvel"), |b| {
        b.iter(|| saal::astro::j2000_to_efg(black_box(ds50_utc), black_box(&j2000_posvel)));
    });
    group.bench_function(BenchmarkId::new("j2000_to_ecr", "posvel"), |b| {
        b.iter(|| saal::astro::j2000_to_ecr(black_box(ds50_utc), black_box(&j2000_posvel)));
    });
    group.bench_function(BenchmarkId::new("teme_to_j2000", "posvel"), |b| {
        b.iter(|| saal::astro::teme_to_j2000(black_box(ds50_utc), black_box(&teme_posvel)));
    });
    group.bench_function(BenchmarkId::new("teme_to_efg", "posvel"), |b| {
        b.iter(|| saal::astro::teme_to_efg(black_box(ds50_utc), black_box(&teme_posvel)));
    });
    group.bench_function(BenchmarkId::new("efg_to_ecr", "posvel"), |b| {
        b.iter(|| saal::astro::efg_to_ecr(black_box(ds50_utc), black_box(&efg_posvel)));
    });
    group.bench_function(BenchmarkId::new("teme_to_ecr", "posvel"), |b| {
        b.iter(|| saal::astro::teme_to_ecr(black_box(ds50_utc), black_box(&teme_posvel)));
    });
    group.bench_function(BenchmarkId::new("ecr_to_efg", "posvel"), |b| {
        b.iter(|| saal::astro::ecr_to_efg(black_box(ds50_utc), black_box(&ecr_posvel)));
    });
    group.bench_function(BenchmarkId::new("efg_to_teme", "posvel"), |b| {
        b.iter(|| saal::astro::efg_to_teme(black_box(ds50_utc), black_box(&efg_posvel)));
    });
    group.bench_function(BenchmarkId::new("ecr_to_teme", "posvel"), |b| {
        b.iter(|| saal::astro::ecr_to_teme(black_box(ds50_utc), black_box(&ecr_posvel)));
    });
    group.bench_function(BenchmarkId::new("ecr_to_j2000", "posvel"), |b| {
        b.iter(|| saal::astro::ecr_to_j2000(black_box(ds50_utc), black_box(&ecr_posvel)));
    });
    group.bench_function(BenchmarkId::new("efg_to_j2000", "posvel"), |b| {
        b.iter(|| saal::astro::efg_to_j2000(black_box(ds50_utc), black_box(&efg_posvel)));
    });
    group.bench_function(BenchmarkId::new("kozai_to_brouwer", "ecc/inc/mm"), |b| {
        b.iter(|| saal::astro::kozai_to_brouwer(black_box(0.011127), black_box(99.4371), black_box(14.20241)));
    });
    group.bench_function(BenchmarkId::new("brouwer_to_kozai", "ecc/inc/mm"), |b| {
        b.iter(|| saal::astro::brouwer_to_kozai(black_box(0.011127), black_box(99.4371), black_box(14.210726)));
    });
    group.bench_function(BenchmarkId::new("mean_motion_to_sma", "1.0027"), |b| {
        b.iter(|| saal::astro::mean_motion_to_sma(black_box(1.0027382962)));
    });
    group.bench_function(BenchmarkId::new("lla_to_teme", "lla"), |b| {
        b.iter(|| saal::astro::lla_to_teme(black_box(ds50_utc), black_box(&lla)));
    });
    group.bench_function(BenchmarkId::new("topo_equinox_to_date", "eqnx=2000"), |b| {
        b.iter(|| {
            saal::astro::topo_meme_to_teme(
                black_box(saal::obs::EQUINOX_J2K),
                black_box(ds50_utc),
                black_box(1.0),
                black_box(0.5),
            )
        });
    });
    group.bench_function(BenchmarkId::new("topo_date_to_equinox", "eqnx=2000"), |b| {
        b.iter(|| {
            saal::astro::topo_teme_to_meme(
                black_box(saal::obs::EQUINOX_J2K),
                black_box(ds50_utc),
                black_box(1.0),
                black_box(0.5),
            )
        });
    });
    group.bench_function(BenchmarkId::new("topo_date_to_epoch", "out=+1d"), |b| {
        b.iter(|| {
            saal::astro::topo_meme_to_teme(
                black_box(saal::obs::EQUINOX_J2K),
                black_box(ds50_utc + 1.0),
                black_box(1.0),
                black_box(0.5),
            )
        });
    });
    group.bench_function(BenchmarkId::new("topo_epoch_to_date", "out=+1d"), |b| {
        b.iter(|| {
            saal::astro::topo_teme_to_meme(
                black_box(saal::obs::EQUINOX_J2K),
                black_box(ds50_utc + 1.0),
                black_box(1.0),
                black_box(0.5),
            )
        });
    });
    group.bench_function(BenchmarkId::new("osculating_to_mean", "osc"), |b| {
        let osc = [7200.0, 0.006257, 54.935, 234.764, 165.472, 217.612];
        b.iter(|| saal::astro::osculating_to_mean(black_box(&osc)));
    });
    group.bench_function(BenchmarkId::new("equinoctial_to_keplerian", "eqnx"), |b| {
        b.iter(|| saal::astro::equinoctial_to_keplerian(black_box(&eqnx)));
    });
    group.bench_function(BenchmarkId::new("keplerian_to_equinoctial", "kep"), |b| {
        b.iter(|| saal::astro::keplerian_to_equinoctial(black_box(&kep)));
    });
    group.bench_function(BenchmarkId::new("covariance_equinoctial_to_uvw", "cov"), |b| {
        b.iter(|| saal::astro::covariance_equinoctial_to_uvw(black_box(&teme_posvel), black_box(&cov_eqnx)));
    });
    group.bench_function(BenchmarkId::new("covariance_uvw_to_teme", "cov"), |b| {
        b.iter(|| saal::astro::covariance_uvw_to_teme(black_box(&teme_posvel), black_box(&cov_uvw)));
    });
    group.bench_function(BenchmarkId::new("gst_ra_dec_to_az_el", "gst=1.23"), |b| {
        b.iter(|| saal::astro::gst_ra_dec_to_az_el(black_box(1.23), black_box(&lla), black_box(1.0), black_box(0.5)));
    });
    group.bench_function(BenchmarkId::new("time_ra_dec_to_az_el", "ds50=8431"), |b| {
        b.iter(|| {
            saal::astro::time_ra_dec_to_az_el(black_box(ds50_utc), black_box(&lla), black_box(1.0), black_box(0.5))
        });
    });
    group.bench_function(BenchmarkId::new("horizon_to_teme", "rae"), |b| {
        b.iter(|| {
            saal::astro::horizon_to_teme(black_box(1.2), black_box(0.5), black_box(&teme_pos), black_box(&xa_rae))
                .expect("horizon_to_teme failed")
        });
    });
    group.bench_function(BenchmarkId::new("gst_teme_to_lla", "gst=1.23"), |b| {
        b.iter(|| saal::astro::gst_teme_to_lla(black_box(1.23), black_box(&teme_pos)));
    });
    group.bench_function(BenchmarkId::new("time_teme_to_lla", "ds50=8431"), |b| {
        b.iter(|| saal::astro::time_teme_to_lla(black_box(ds50_utc), black_box(&teme_pos)));
    });
    group.bench_function(BenchmarkId::new("efg_to_lla", "efg_pos"), |b| {
        b.iter(|| saal::astro::efg_to_lla(black_box(&efg_pos)).expect("efg_to_lla failed"));
    });
    group.bench_function(BenchmarkId::new("teme_to_topo", "topo"), |b| {
        b.iter(|| {
            saal::astro::teme_to_topo(
                black_box(1.2),
                black_box(0.5),
                black_box(&teme_pos),
                black_box(&teme_posvel),
            )
            .expect("teme_to_topo failed")
        });
    });
    group.bench_function(BenchmarkId::new("get_jpl_sun_and_moon_position", "ds50=8431"), |b| {
        b.iter(|| saal::astro::get_jpl_sun_and_moon_position(black_box(ds50_utc)));
    });
    group.bench_function(BenchmarkId::new("point_is_sunlit", "ds50_tt"), |b| {
        b.iter(|| saal::astro::point_is_sunlit(black_box(ds50_tt), black_box(&teme_pos)));
    });
    group.bench_function(BenchmarkId::new("get_earth_obstruction_angles", "teme"), |b| {
        b.iter(|| saal::astro::get_earth_obstruction_angles(black_box(&teme_pos), black_box(&teme_pos)));
    });

    group.finish();
}

criterion_group!(benches, bench_astro_wrappers);
criterion_main!(benches);
