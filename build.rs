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

    // Data files to always copy
    let data_extensions: &[&str] = &["GEO", "dat", "405", "txt"];

    // On macOS/Windows, we need to copy native libraries too.
    // On Linux, maturin bundles them into keplemon.libs/ automatically.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let native_lib_ext: Option<&str> = match target_os.as_str() {
        "macos" => Some("dylib"),
        "windows" => Some("dll"),
        _ => None, // Linux: maturin handles bundling
    };

    for entry in fs::read_dir(&target_dir).expect("Failed to read target directory") {
        let entry = entry.expect("Failed to access entry in target directory");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Copy data files on all platforms
        let is_data_file = data_extensions.contains(&ext);

        // Copy native libs on macOS/Windows only
        let is_native_lib = native_lib_ext.is_some_and(|lib_ext| ext == lib_ext);

        if !is_data_file && !is_native_lib {
            continue;
        }

        let filename = path.file_name().expect("Invalid target file name");
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
    println!("cargo:rerun-if-changed=kernels/sgp4_init.cu");
    println!("cargo:rerun-if-changed=kernels/sgp4_batch.cu");
    println!("cargo:rerun-if-changed=kernels/sgp4_types.cuh");
    println!("cargo:rerun-if-changed=kernels/sgp4_constants.cuh");

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
        fs::write(format!("{}/sgp4_init.ptx", out_dir), stub_ptx).expect("Failed to write stub sgp4_init.ptx");
        fs::write(format!("{}/sgp4_batch.ptx", out_dir), stub_ptx).expect("Failed to write stub sgp4_batch.ptx");

        return;
    }

    let nvcc_cmd = if nvcc.exists() { nvcc.to_str().unwrap() } else { "nvcc" };

    // Compile initialization kernel
    compile_kernel(nvcc_cmd, "kernels/sgp4_init.cu", &format!("{}/sgp4_init.ptx", out_dir));

    // Compile batch propagation kernel
    compile_kernel(
        nvcc_cmd,
        "kernels/sgp4_batch.cu",
        &format!("{}/sgp4_batch.ptx", out_dir),
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
