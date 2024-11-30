mod console;
mod enums;
mod message;
mod playground;
mod prompt;
mod repl;
mod rust;
mod stub;

use crate::repl::Repl;

fn main() {
    Repl::new().run();
}
