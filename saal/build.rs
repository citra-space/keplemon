use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

const MANIFEST_VERSION: &str = "9.6";

fn main() {
    let manifest_file = format!("manifest-{MANIFEST_VERSION}.json");

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

    println!("cargo:rerun-if-changed={}", manifest_file);
    println!("cargo:rustc-env=SAAL_MANIFEST_VERSION={}", MANIFEST_VERSION);
    let (assets_dir, assets_source_dir) = resolve_assets_dir(Path::new("assets"), &out_dir_path, &manifest_file);
    if let Some(source_dir) = assets_source_dir.as_ref() {
        emit_rerun_if_changed(source_dir);
    }
    let (lib_dir, lib_source_dir) = resolve_lib_dir(lib_dir, &out_dir_path, &target_os, &target_arch, &manifest_file);
    if let Some(source_dir) = lib_source_dir.as_ref() {
        emit_rerun_if_changed(source_dir);
    }

    // Iterate over each file in the lib/ directory.
    for entry in fs::read_dir(&lib_dir).expect("Failed to read lib directory") {
        let entry = entry.expect("Failed to access entry in lib directory");
        let path = entry.path();

        if path.is_file() {
            // Tell Cargo to rerun the build script if this source file changes.
            if !lib_dir.starts_with(&out_dir_path) {
                println!("cargo:rerun-if-changed={}", path.display());
            }

            // Determine the destination path in the target directory.
            let file_name = path.file_name().expect("Invalid file name");
            let dest_path = target_dir.join(file_name);

            // Exclude system libraries that cause glibc version issues or crashes.
            let filename_str = file_name.to_string_lossy();
            if filename_str.starts_with("libm.so")
                || filename_str.starts_with("libgcc_s.so")
                || filename_str.starts_with("libc.so")
                || filename_str.starts_with("ld-linux")
                || filename_str.starts_with("libstdc++.so")
                || filename_str.starts_with("libpthread.so")
                || filename_str.starts_with("librt.so")
                || filename_str.starts_with("libdl.so")
            {
                continue;
            }

            // Copy the file.
            fs::copy(&path, &dest_path)
                .unwrap_or_else(|_| panic!("Failed to copy {} to {}", path.display(), dest_path.display()));
        }
    }

    if assets_dir.exists() {
        let asset_targets = [target_dir.clone(), target_dir.join("deps")];
        for entry in fs::read_dir(&assets_dir).expect("Failed to read assets directory") {
            let entry = entry.expect("Failed to access entry in assets directory");
            let path = entry.path();
            if path.is_file() {
                if !assets_dir.starts_with(&out_dir_path) {
                    println!("cargo:rerun-if-changed={}", path.display());
                }
                let file_name = path.file_name().expect("Invalid asset file name");
                for dest_dir in &asset_targets {
                    fs::create_dir_all(dest_dir).expect("Failed to create asset destination directory");
                    let dest_path = dest_dir.join(file_name);
                    fs::copy(&path, &dest_path).unwrap_or_else(|_| {
                        panic!("Failed to copy asset {} to {}", path.display(), dest_path.display())
                    });
                }
            }
        }
    }

    // Copy target-specific libraries into the Python package for wheel bundling.
    if env::var("CARGO_FEATURE_PYTHON").is_ok() {
        let python_pkg_dir = Path::new("python").join("pysaal");
        fs::create_dir_all(&python_pkg_dir).expect("Failed to create python/pysaal directory");
        for entry in fs::read_dir(&lib_dir).expect("Failed to read lib directory") {
            let entry = entry.expect("Failed to access entry in lib directory");
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().expect("Invalid file name");
                let dest_path = python_pkg_dir.join(file_name);
                fs::copy(&path, &dest_path)
                    .unwrap_or_else(|_| panic!("Failed to copy {} to {}", path.display(), dest_path.display()));
            }
        }

        if assets_dir.exists() {
            for entry in fs::read_dir(&assets_dir).expect("Failed to read assets directory") {
                let entry = entry.expect("Failed to access entry in assets directory");
                let path = entry.path();
                if path.is_file() {
                    if !assets_dir.starts_with(&out_dir_path) {
                        println!("cargo:rerun-if-changed={}", path.display());
                    }
                    let file_name = path.file_name().expect("Invalid asset file name");
                    let dest_path = python_pkg_dir.join(file_name);
                    fs::copy(&path, &dest_path).unwrap_or_else(|_| {
                        panic!("Failed to copy asset {} to {}", path.display(), dest_path.display())
                    });
                }
            }
        }

        let stubs_dir = Path::new("stubs").join("pysaal");
        if stubs_dir.exists() {
            println!("cargo:rerun-if-changed={}", stubs_dir.display());
            for entry in fs::read_dir(&stubs_dir).expect("Failed to read stubs directory") {
                let entry = entry.expect("Failed to access entry in stubs directory");
                let path = entry.path();
                if path.is_file() {
                    println!("cargo:rerun-if-changed={}", path.display());
                    let file_name = path.file_name().expect("Invalid stub file name");
                    let dest_path = python_pkg_dir.join(file_name);
                    fs::copy(&path, &dest_path).unwrap_or_else(|_| {
                        panic!("Failed to copy stub {} to {}", path.display(), dest_path.display())
                    });
                }
            }
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
    println!("cargo:rustc-link-lib=dylib=sensor");
}

#[derive(Deserialize)]
struct Manifest {
    version: u32,
    #[serde(default)]
    release_version: Option<String>,
    #[serde(default)]
    assets_archive: Option<Archive>,
    #[serde(default)]
    lib_archives: Vec<LibArchive>,
}

#[derive(Deserialize)]
struct Archive {
    url: String,
    sha256: String,
}

#[derive(Deserialize)]
struct LibArchive {
    os: String,
    arch: String,
    url: String,
    sha256: String,
}

fn resolve_assets_dir(source_dir: &Path, out_dir: &Path, manifest_file: &str) -> (PathBuf, Option<PathBuf>) {
    if let Some(path) = asset_directory_override()
        && assets_present_in_dir(&path)
    {
        if path.starts_with(out_dir) {
            return (path, None);
        }
        let out_assets_dir = out_dir.join("assets");
        copy_dir_recursive(&path, &out_assets_dir);
        return (out_assets_dir, Some(path));
    }

    if assets_present_in_dir(source_dir) {
        if source_dir.starts_with(out_dir) {
            return (source_dir.to_path_buf(), None);
        }
        let out_assets_dir = out_dir.join("assets");
        copy_dir_recursive(source_dir, &out_assets_dir);
        return (out_assets_dir, Some(source_dir.to_path_buf()));
    }

    let out_assets_dir = out_dir.join("assets");
    if !Path::new(manifest_file).exists() {
        fs::create_dir_all(&out_assets_dir)
            .unwrap_or_else(|e| panic!("Failed to create assets dir {}: {e}", out_assets_dir.display()));
        return (out_assets_dir, None);
    }

    let manifest = load_manifest(manifest_file).expect("Failed to read manifest");
    if manifest.version != 1 {
        panic!("Unsupported manifest version {}", manifest.version);
    }

    if let Some(archive) = &manifest.assets_archive {
        download_assets_archive(archive, out_dir, manifest.release_version.as_deref());
        if assets_present_in_dir(&out_assets_dir) {
            return (out_assets_dir, None);
        }
        if assets_present_in_dir(out_dir) {
            return (out_dir.to_path_buf(), None);
        }
    }

    fs::create_dir_all(&out_assets_dir)
        .unwrap_or_else(|e| panic!("Failed to create assets dir {}: {e}", out_assets_dir.display()));
    (out_assets_dir, None)
}

fn resolve_lib_dir(
    source_dir: &Path,
    out_dir: &Path,
    target_os: &str,
    target_arch: &str,
    manifest_file: &str,
) -> (PathBuf, Option<PathBuf>) {
    if lib_dir_has_files(source_dir) {
        if source_dir.starts_with(out_dir) {
            return (source_dir.to_path_buf(), None);
        }
        let out_lib_dir = if source_dir.is_absolute() {
            source_dir.to_path_buf()
        } else {
            out_dir.join(source_dir)
        };
        copy_dir_recursive(source_dir, &out_lib_dir);
        return (out_lib_dir, Some(source_dir.to_path_buf()));
    }

    let out_lib_dir = out_dir.join(source_dir);
    ensure_libs_downloaded(&out_lib_dir, out_dir, target_os, target_arch, manifest_file);
    (out_lib_dir, None)
}

fn ensure_libs_downloaded(lib_dir: &Path, out_dir: &Path, target_os: &str, target_arch: &str, manifest_file: &str) {
    if lib_dir_has_files(lib_dir) {
        return;
    }
    if !Path::new(manifest_file).exists() {
        return;
    }

    let manifest = load_manifest(manifest_file).expect("Failed to read manifest");
    if manifest.version != 1 {
        panic!("Unsupported manifest version {}", manifest.version);
    }

    let archive = manifest
        .lib_archives
        .iter()
        .find(|entry| entry.os == target_os && entry.arch == target_arch);

    let Some(archive) = archive else {
        return;
    };

    let expected = parse_sha256(&archive.sha256).unwrap_or_else(|e| panic!("Invalid lib archive sha256: {e}"));
    let url = resolve_url(&archive.url, manifest.release_version.as_deref())
        .unwrap_or_else(|e| panic!("Invalid lib archive url: {e}"));
    let tmp_dir = out_dir.join(".saal_downloads");
    fs::create_dir_all(&tmp_dir).unwrap_or_else(|e| panic!("Failed to create {}: {e}", tmp_dir.display()));
    let archive_path = tmp_dir.join("lib.zip");

    download_asset(&url, &archive_path, &expected).unwrap_or_else(|e| panic!("Failed to download lib archive: {e}"));
    extract_lib_zip_into(&archive_path, lib_dir);
    let _ = fs::remove_file(&archive_path);
}

fn lib_dir_has_files(dir: &Path) -> bool {
    dir_has_files_recursive(dir)
}

fn download_assets_archive(archive: &Archive, dest_root: &Path, release_version: Option<&str>) {
    let expected = parse_sha256(&archive.sha256).unwrap_or_else(|e| panic!("Invalid assets archive sha256: {e}"));
    let url = resolve_url(&archive.url, release_version).unwrap_or_else(|e| panic!("Invalid assets archive url: {e}"));
    let tmp_dir = dest_root.join(".saal_downloads");
    fs::create_dir_all(&tmp_dir).unwrap_or_else(|e| panic!("Failed to create {}: {e}", tmp_dir.display()));
    let archive_path = tmp_dir.join("assets.zip");

    download_asset(&url, &archive_path, &expected).unwrap_or_else(|e| panic!("Failed to download assets archive: {e}"));
    extract_zip_into(&archive_path, dest_root);
    let _ = fs::remove_file(&archive_path);
}

fn asset_directory_override() -> Option<PathBuf> {
    env::var("SAAL_ASSET_DIRECTORY").ok().map(PathBuf::from)
}

fn assets_present_in_dir(dir: &Path) -> bool {
    dir_has_files_recursive(dir)
}

fn dir_has_files_recursive(dir: &Path) -> bool {
    if !dir.exists() {
        return false;
    }
    let mut stack = vec![dir.to_path_buf()];
    while let Some(path) = stack.pop() {
        let entries = match fs::read_dir(&path) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(_) => continue,
            };
            if file_type.is_file() || file_type.is_symlink() {
                return true;
            }
            if file_type.is_dir() {
                stack.push(entry.path());
            }
        }
    }
    false
}

fn emit_rerun_if_changed(dir: &Path) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("Failed to read {}: {e}", dir.display()));
    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("Failed to read entry in {}: {e}", dir.display()));
        let path = entry.path();
        let file_type = entry
            .file_type()
            .unwrap_or_else(|e| panic!("Failed to read file type for {}: {e}", path.display()));
        if file_type.is_dir() {
            emit_rerun_if_changed(&path);
            continue;
        }
        if file_type.is_file() || file_type.is_symlink() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn copy_dir_recursive(source_dir: &Path, dest_dir: &Path) {
    if source_dir == dest_dir {
        return;
    }
    fs::create_dir_all(dest_dir).unwrap_or_else(|e| panic!("Failed to create {}: {e}", dest_dir.display()));
    let entries = fs::read_dir(source_dir).unwrap_or_else(|e| panic!("Failed to read {}: {e}", source_dir.display()));
    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("Failed to read entry in {}: {e}", source_dir.display()));
        let path = entry.path();
        let file_type = entry
            .file_type()
            .unwrap_or_else(|e| panic!("Failed to read file type for {}: {e}", path.display()));
        let dest_path = dest_dir.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&path, &dest_path);
            continue;
        }
        if file_type.is_file() || file_type.is_symlink() {
            fs::copy(&path, &dest_path)
                .unwrap_or_else(|e| panic!("Failed to copy {} to {}: {e}", path.display(), dest_path.display()));
        }
    }
}

fn load_manifest(manifest_file: &str) -> Result<Manifest, String> {
    let content = fs::read_to_string(manifest_file).map_err(|e| format!("failed to read {manifest_file}: {e}"))?;
    serde_json::from_str(&content).map_err(|e| format!("failed to parse {manifest_file}: {e}"))
}

fn resolve_url(template: &str, release_version: Option<&str>) -> Result<String, String> {
    if template.contains("{version}") || template.contains("{release_version}") {
        let Some(version) = release_version else {
            return Err("release_version missing in manifest".to_string());
        };
        Ok(template
            .replace("{version}", version)
            .replace("{release_version}", version))
    } else {
        Ok(template.to_string())
    }
}

fn extract_zip_into(zip_path: &Path, dest_root: &Path) {
    let file = File::open(zip_path).unwrap_or_else(|e| panic!("Failed to open {}: {e}", zip_path.display()));
    let mut archive = ZipArchive::new(file).unwrap_or_else(|e| panic!("Invalid zip {}: {e}", zip_path.display()));

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).expect("Failed to read zip entry");
        let Some(entry_path) = entry.enclosed_name() else {
            continue;
        };
        let out_path = dest_root.join(entry_path);
        if entry.is_dir() {
            fs::create_dir_all(&out_path).unwrap_or_else(|e| panic!("Failed to create {}: {e}", out_path.display()));
            continue;
        }

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|e| panic!("Failed to create {}: {e}", parent.display()));
        }
        let mut out_file =
            File::create(&out_path).unwrap_or_else(|e| panic!("Failed to create {}: {e}", out_path.display()));
        std::io::copy(&mut entry, &mut out_file)
            .unwrap_or_else(|e| panic!("Failed to write {}: {e}", out_path.display()));
    }
}

fn extract_lib_zip_into(zip_path: &Path, lib_dir: &Path) {
    let leaf = lib_dir.file_name().and_then(|name| name.to_str()).unwrap_or("");
    let top_level = zip_uniform_top_level(zip_path);
    let dest_root = match top_level.as_deref() {
        Some(name) if name == leaf => lib_dir.parent().unwrap_or_else(|| Path::new(".")),
        _ => lib_dir,
    };
    extract_zip_into(zip_path, dest_root);
}

fn zip_uniform_top_level(zip_path: &Path) -> Option<String> {
    let file = File::open(zip_path).ok()?;
    let mut archive = ZipArchive::new(file).ok()?;
    let mut top_level: Option<String> = None;

    for i in 0..archive.len() {
        let entry = archive.by_index(i).ok()?;
        let path = entry.enclosed_name()?;
        let mut components = path.components();
        let first = components.next()?.as_os_str().to_str()?;
        let second = components.next();
        if second.is_none() && entry.is_file() {
            return None;
        }

        match &top_level {
            None => top_level = Some(first.to_string()),
            Some(existing) if existing != first => return None,
            Some(_) => {}
        }
    }

    top_level
}

fn parse_sha256(value: &str) -> Result<String, String> {
    let mut parts = value.splitn(2, ':');
    let algo = parts.next().unwrap_or("");
    let hash = parts.next().unwrap_or("");
    if algo != "sha256" || hash.len() != 64 {
        return Err(format!("invalid sha256 value: {value}"));
    }
    Ok(hash.to_ascii_lowercase())
}

fn download_asset(url: &str, path: &Path, expected: &str) -> Result<(), String> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| format!("failed to download {url}: {e}"))?;
    let mut reader = response.into_reader();
    let tmp_path = path.with_extension("download");
    let mut file = File::create(&tmp_path).map_err(|e| format!("failed to create {}: {e}", tmp_path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    loop {
        let read = reader
            .read(&mut buf)
            .map_err(|e| format!("failed to read {url}: {e}"))?;
        if read == 0 {
            break;
        }
        file.write_all(&buf[..read])
            .map_err(|e| format!("failed to write {}: {e}", tmp_path.display()))?;
        hasher.update(&buf[..read]);
    }

    let actual = hex::encode(hasher.finalize());
    if actual != expected {
        let _ = fs::remove_file(&tmp_path);
        return Err(format!("sha256 mismatch for {url}: expected {expected}, got {actual}"));
    }

    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("failed to remove {}: {e}", path.display()))?;
    }
    fs::rename(&tmp_path, path).map_err(|e| format!("failed to move {}: {e}", tmp_path.display()))?;
    Ok(())
}
