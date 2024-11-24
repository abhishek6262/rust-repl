use std::process::{Command, Output};
use tempfile::{Builder, NamedTempFile};

pub struct Playground {
    playground_file: NamedTempFile,
    executable_path: String,
}

impl Playground {
    pub fn new() -> Self {
        let playground_file = Builder::new().prefix("rs-repl-").tempfile().unwrap();
        let executable_path = format!(
            "{}-out",
            playground_file.path().to_str().unwrap().to_string()
        );

        Self {
            playground_file,
            executable_path,
        }
    }

    pub fn run(&self, buffer: &str) -> Result<String, String> {
        self.initiate(&buffer);
        self.format();
        self.compile();

        let output = self.execute();

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    fn initiate(&self, buffer: &str) {
        std::fs::write(
            &self.playground_file,
            "fn main() { %temp% }".replace("%temp%", &buffer),
        )
        .unwrap();
    }

    fn format(&self) {
        Command::new("rustfmt")
            .arg(&self.playground_file.path())
            .output()
            .unwrap();
    }

    fn compile(&self) {
        Command::new("rustc")
            .arg(&self.playground_file.path())
            .args(["-o", self.executable_path.as_str()])
            .status()
            .unwrap();
    }

    fn execute(&self) -> Output {
        Command::new(&self.executable_path).output().unwrap()
    }
}
