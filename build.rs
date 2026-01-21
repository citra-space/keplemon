use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

fn target_dir(out_dir: &Path) -> PathBuf {
    out_dir
        .ancestors()
        .nth(3)
        .expect("Couldn't determine target directory")
        .to_path_buf()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

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
