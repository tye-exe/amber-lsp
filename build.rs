use fs_extra;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[cfg(target_os = "linux")]
fn get_install_dir() -> PathBuf {
    PathBuf::from("/etc/amber_lsp")
}

#[cfg(target_os = "windows")]
fn get_install_dir() -> PathBuf {
    PathBuf::from("C:\\Program Files\\amber_lsp")
}

#[cfg(target_os = "macos")]
fn get_install_dir() -> PathBuf {
    PathBuf::from("/usr/local/etc/amber_lsp")
}

fn main() {
    if env::var("PROFILE").unwrap() != "release" {
        return;
    }

    // Copy resources to an install path
    let source = Path::new("resources/");
    let destination = get_install_dir();

    if !destination.exists() {
        fs::create_dir_all(destination.clone()).expect("Failed to create destination directory");
    }

    fs_extra::dir::copy(
        source,
        destination.as_path(),
        &fs_extra::dir::CopyOptions::new().overwrite(true),
    )
    .expect("Failed to copy resources");
}
