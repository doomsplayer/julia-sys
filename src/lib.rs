#![feature(untagged_unions)]

extern crate libc;

pub mod sys;
pub use sys::*;

use std::process::Command;
use std::path::PathBuf;
use std::str;

pub fn julia_home() -> PathBuf {
    let julia_home = Command::new("julia")
        .arg("-e")
        .arg(format!("print(JULIA_HOME)"))
        .output()
        .unwrap_or_else(|e| panic!("julia not found: {}", e));

    PathBuf::from(&str::from_utf8(&julia_home.stdout).unwrap()).to_path_buf()
}
