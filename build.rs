use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(feature = "cuda")]
use std::process::Command;

fn target_dir(out_dir: &Path) -> PathBuf {
    out_dir
        .ancestors()
        .nth(3)
        .expect("Couldn't determine target directory")
        .to_path_buf()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // CUDA kernel compilation
    #[cfg(feature = "cuda")]
    compile_cuda_kernels();

    // Python wheel build
    if env::var("CARGO_FEATURE_PYTHON").is_err() {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let target_dir = target_dir(&out_dir);

    let python_pkg_dir = Path::new("python").join("keplemon");
    fs::create_dir_all(&python_pkg_dir).expect("Failed to create python/keplemon directory");

    for entry in fs::read_dir(&target_dir).expect("Failed to read target directory") {
        let entry = entry.expect("Failed to access entry in target directory");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let filename = path.file_name().expect("Invalid target file name");
        if filename == "Cargo.lock" || filename == ".cargo-lock" || filename == "libkeplemon.d" {
            continue;
        }

        let dest_path = python_pkg_dir.join(filename);
        fs::copy(&path, &dest_path)
            .unwrap_or_else(|_| panic!("Failed to copy {} to {}", path.display(), dest_path.display()));
    }

    let stubs_dir = Path::new("stubs").join("keplemon");
    if stubs_dir.is_dir() {
        for entry in fs::read_dir(&stubs_dir).expect("Failed to read stubs/keplemon directory") {
            let entry = entry.expect("Failed to access entry in stubs/keplemon");
            let path = entry.path();
            if path.extension() != Some(OsStr::new("pyi")) {
                continue;
            }
            println!("cargo:rerun-if-changed={}", path.display());
            let filename = path.file_name().expect("Invalid stub file name");
            let dest_path = python_pkg_dir.join(filename);
            fs::copy(&path, &dest_path)
                .unwrap_or_else(|_| panic!("Failed to copy stub {} to {}", path.display(), dest_path.display()));
        }
    }
}

#[cfg(feature = "cuda")]
fn compile_cuda_kernels() {
    // TLE propagator files (SGP4/SDP4)
    println!("cargo:rerun-if-changed=kernels/tle_propagator_init.cu");
    println!("cargo:rerun-if-changed=kernels/tle_propagator_batch.cu");
    println!("cargo:rerun-if-changed=kernels/tle_propagator_types.cuh");
    println!("cargo:rerun-if-changed=kernels/tle_propagator_constants.cuh");

    // SGP4 module (near-earth)
    println!("cargo:rerun-if-changed=kernels/sgp4_propagate.cuh");
    println!("cargo:rerun-if-changed=kernels/sgp4_init.cuh");

    // SDP4 module (deep-space)
    println!("cargo:rerun-if-changed=kernels/sdp4_propagate.cuh");
    println!("cargo:rerun-if-changed=kernels/sdp4_init.cuh");
    println!("cargo:rerun-if-changed=kernels/sdp4_deepspace.cuh");

    // GEO numerical propagator files
    println!("cargo:rerun-if-changed=kernels/geo_numerical.cu");
    println!("cargo:rerun-if-changed=kernels/geo_numerical.cuh");
    println!("cargo:rerun-if-changed=kernels/geo_types.cuh");
    println!("cargo:rerun-if-changed=kernels/geo_constants.cuh");

    // SDP4 interpolated propagator files
    println!("cargo:rerun-if-changed=kernels/sdp4_interpolated.cu");
    println!("cargo:rerun-if-changed=kernels/sdp4_interpolated.cuh");
    println!("cargo:rerun-if-changed=kernels/sdp4_interpolated_types.cuh");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    // Find nvcc - check CUDA_PATH or common locations
    let cuda_path = env::var("CUDA_PATH")
        .or_else(|_| env::var("CUDA_HOME"))
        .unwrap_or_else(|_| "/usr/local/cuda".to_string());

    let nvcc = PathBuf::from(&cuda_path).join("bin").join("nvcc");

    // Check if nvcc exists
    if !nvcc.exists() && !Command::new("nvcc").arg("--version").output().is_ok() {
        println!(
            "cargo:warning=nvcc not found. CUDA kernels will not be compiled. \
             CUDA features will be unavailable at runtime. \
             To enable CUDA: install CUDA Toolkit or set CUDA_PATH environment variable. \
             Looked in: {}",
            nvcc.display()
        );
        println!("cargo:warning=Skipping CUDA kernel compilation");

        // Create empty stub PTX files so include_str! doesn't fail
        let stub_ptx = "// CUDA kernels not compiled - nvcc not available\n";
        fs::write(format!("{}/tle_propagator_init.ptx", out_dir), stub_ptx)
            .expect("Failed to write stub tle_propagator_init.ptx");
        fs::write(format!("{}/tle_propagator_batch.ptx", out_dir), stub_ptx)
            .expect("Failed to write stub tle_propagator_batch.ptx");
        fs::write(format!("{}/geo_numerical.ptx", out_dir), stub_ptx).expect("Failed to write stub geo_numerical.ptx");
        fs::write(format!("{}/sdp4_interpolated.ptx", out_dir), stub_ptx)
            .expect("Failed to write stub sdp4_interpolated.ptx");

        return;
    }

    let nvcc_cmd = if nvcc.exists() { nvcc.to_str().unwrap() } else { "nvcc" };

    // Compile TLE propagator initialization kernel (SGP4/SDP4)
    compile_kernel(
        nvcc_cmd,
        "kernels/tle_propagator_init.cu",
        &format!("{}/tle_propagator_init.ptx", out_dir),
    );

    // Compile TLE propagator batch propagation kernel (SGP4/SDP4)
    compile_kernel(
        nvcc_cmd,
        "kernels/tle_propagator_batch.cu",
        &format!("{}/tle_propagator_batch.ptx", out_dir),
    );

    // Compile GEO numerical propagation kernel
    compile_kernel(
        nvcc_cmd,
        "kernels/geo_numerical.cu",
        &format!("{}/geo_numerical.ptx", out_dir),
    );

    // Compile SDP4 interpolated propagation kernel
    compile_kernel(
        nvcc_cmd,
        "kernels/sdp4_interpolated.cu",
        &format!("{}/sdp4_interpolated.ptx", out_dir),
    );

    println!("cargo:info=CUDA kernels compiled successfully");
}

#[cfg(feature = "cuda")]
fn compile_kernel(nvcc: &str, input: &str, output: &str) {
    let status = Command::new(nvcc)
        .args(&[
            "-ptx",            // Compile to PTX
            "-O3",             // Optimization level 3
            "--use_fast_math", // Use fast math operations
            "-arch=sm_50",     // Target compute capability 5.0+ (Maxwell and newer)
            "--std=c++14",     // C++14 standard
            "-I",
            "kernels", // Include directory for headers
            "-o",
            output, // Output PTX file
            input,  // Input CUDA source
        ])
        .status()
        .unwrap_or_else(|e| panic!("Failed to execute nvcc: {}", e));

    if !status.success() {
        panic!("nvcc compilation failed for {}", input);
    }

    println!("cargo:info=Compiled {} to {}", input, output);
}
