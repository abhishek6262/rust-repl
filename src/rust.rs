use crate::playground::Playground;
use std::process::Command;

pub struct Rust;

impl Rust {
    pub fn get_version() -> String {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .expect("Failed to get Rust version");

        std::str::from_utf8(&output.stdout)
            .unwrap()
            .trim()
            .to_string()
    }

    pub fn run(buffer: &str) {
        Playground::run(&buffer);
    }
}
