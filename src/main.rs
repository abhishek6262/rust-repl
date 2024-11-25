mod playground;
mod repl;

use crate::repl::{OutputKind, Repl};
use std::io::Write;
use std::process::Command;

fn main() {
    // Clear anything in the existing console
    Command::new("clear").status().unwrap();

    let repl = Repl::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        match repl.exec(&buffer.trim_end()) {
            Ok(OutputKind::COMMAND(instruction)) => {
                if instruction.eq("exit") {
                    break;
                }
            }

            Ok(OutputKind::STATEMENT(statement)) => {
                println!("{}", statement);
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
