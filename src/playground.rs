use crate::rust::Rust;
use std::process::Output;
use std::{path::PathBuf, process::Command};
use tempfile::{Builder, NamedTempFile};

pub struct Playground;

impl Playground {
    pub fn run(buffer: &str) {
        let playground_file = Self::get_playground_file(buffer);
        let executable_file_path = Self::get_executable_file_path(&playground_file);

        let output = Command::new(executable_file_path).output().unwrap();

        Self::display_output(&output);
    }

    fn display_output(output: &Output) {
        let content = if output.status.success() {
            String::from_utf8_lossy(&output.stdout)
        } else {
            String::from_utf8_lossy(&output.stderr)
        };

        println!("{}", content);
    }

    fn get_playground_file(buffer: &str) -> NamedTempFile {
        let file_buffer = format!("fn main() {{\n{}\n}}", buffer); // Add buffer to the stub code
        let playground_file = Builder::new().prefix("rust-repl-").tempfile().unwrap();

        std::fs::write(&playground_file, file_buffer).unwrap();

        playground_file
    }

    fn get_executable_file_path(playground_file: &NamedTempFile) -> PathBuf {
        let playground_file_path = playground_file.path();
        let executable_file_path =
            PathBuf::from(format!("{}-out", playground_file_path.to_str().unwrap()));

        Rust::compile(&playground_file_path, &executable_file_path);

        executable_file_path
    }
}
