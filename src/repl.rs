use crate::playground::Playground;
use crate::repl::OutputKind::{COMMAND, STATEMENT};
use std::process::Command;

pub enum OutputKind {
    COMMAND(String),
    STATEMENT(String),
}

pub struct Repl {
    playground: Playground,
}

fn get_rust_version() -> String {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        std::str::from_utf8(&output.stdout)
            .unwrap()
            .trim()
            .to_string()
    } else {
        "Failed to get Rust version".to_string()
    }
}

impl Repl {
    pub fn new() -> Self {
        println!("Welcome to {}.", get_rust_version());
        println!("Type \".help\" for more information.");

        Repl {
            playground: Playground::new(),
        }
    }

    pub fn exec(&self, input: &str) -> Result<OutputKind, String> {
        match input {
            ".help" => {
                Ok(STATEMENT("The REPL has some special commands, all starting with a dot. They are
                .help: shows the dot commands help
                .editor: enables editor mode, to write multiline Rust code with ease. Once you are in this mode, enter ctrl-D to run the code you wrote.
                .exit: exits the repl".to_string()))
            }

            ".exit" => Ok(COMMAND("exit".to_string())),

            _ => {
                let output= self.playground.run(&input)?.to_string();

                Ok(STATEMENT(output))
            },
        }
    }
}

impl Drop for Repl {
    fn drop(&mut self) {
        println!("Goodbye!");
    }
}
