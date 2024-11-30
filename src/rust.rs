use std::path::Path;
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

    pub fn compile(file_path: &Path, output_file_path: &Path) {
        Command::new("rustc")
            .arg(file_path)
            .args(["-o", output_file_path.to_str().unwrap()])
            .status()
            .unwrap();
    }
}
