use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Determine the target OS and architecture.
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");

    // Set lib_dir based on OS and architecture.
    let lib_dir = match target_os.as_str() {
        "macos" => match target_arch.as_str() {
            "aarch64" => Path::new("lib/mac/arm"),
            "x86_64" => Path::new("lib/mac/x86"),
            other => panic!("Unsupported macOS architecture: {}", other),
        },
        "linux" => match target_arch.as_str() {
            "aarch64" => Path::new("lib/linux/arm"),
            "x86_64" => Path::new("lib/linux/x86"),
            other => panic!("Unsupported Linux architecture: {}", other),
        },
        "windows" => Path::new("lib/windows"),
        other => panic!("Unsupported OS: {}", other),
    };

    // Get the OUT_DIR provided by Cargo.
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = PathBuf::from(&out_dir);

    // The target directory is typically three levels up from OUT_DIR.
    // (This works for most Cargo setups, though it isn’t officially documented.)
    let target_dir = out_dir_path
        .ancestors()
        .nth(3)
        .expect("Couldn't determine target directory")
        .to_path_buf();

    // Iterate over each file in the lib/ directory.
    for entry in fs::read_dir(lib_dir).expect("Failed to read lib directory") {
        let entry = entry.expect("Failed to access entry in lib directory");
        let path = entry.path();

        if path.is_file() {
            // Tell Cargo to rerun the build script if this file changes.
            println!("cargo:rerun-if-changed={}", path.display());

            // Determine the destination path in the target directory.
            let file_name = path.file_name().expect("Invalid file name");
            let dest_path = target_dir.join(file_name);

            // Copy the file.
            fs::copy(&path, &dest_path)
                .unwrap_or_else(|_| panic!("Failed to copy {} to {}", path.display(), dest_path.display()));
        }
    }

    // Tell Cargo to add the target directory to the linker search path.
    println!("cargo:rustc-link-search=native={}", target_dir.display());
    if target_os == "linux" {
        println!("cargo:rustc-link-arg=-Wl,--disable-new-dtags,-rpath,$ORIGIN");
    } else if target_os == "macos" {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
    }

    // instruct the linker to link against the desired library.
    println!("cargo:rustc-link-lib=dylib=dllmain");
    println!("cargo:rustc-link-lib=dylib=envconst");
    println!("cargo:rustc-link-lib=dylib=timefunc");
    println!("cargo:rustc-link-lib=dylib=astrofunc");
    println!("cargo:rustc-link-lib=dylib=sgp4prop");
    println!("cargo:rustc-link-lib=dylib=tle");
    println!("cargo:rustc-link-lib=dylib=extephem");
    println!("cargo:rustc-link-lib=dylib=satstate");
    println!("cargo:rustc-link-lib=dylib=obs");
}
