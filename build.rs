extern crate bindgen;
use std::env;
use std::process::Command;
use std::path::{PathBuf, Path};
use std::str;

fn julia_home() -> PathBuf {
    let julia_home = Command::new("julia")
        .arg("-e")
        .arg(format!("print(JULIA_HOME)"))
        .output()
        .unwrap_or_else(|e| panic!("julia not found: {}", e));

    PathBuf::from(&str::from_utf8(&julia_home.stdout).unwrap())
}

fn lib_path() -> PathBuf {
    let mut julia_home = julia_home();
    if !julia_home.pop() {
        panic!("malformed julia path");
    }
    julia_home.push("lib");
    julia_home
}

fn include_path() -> PathBuf {
    let mut julia_home = julia_home();
    if !julia_home.pop() {
        panic!("malformed julia path");
    }
    julia_home.push("include");
    julia_home.push("julia");
    julia_home
}

fn write_sys() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path().to_string_lossy()))
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("sys.rs"));
}

fn main() {
    // write_sys();
    println!("cargo:rustc-link-search={}", lib_path().to_string_lossy());
    println!("cargo:rustc-link-lib=dylib={}", "julia");
}