mod console;
mod playground;
mod prompt;
mod repl;
mod rust;

use crate::repl::Repl;

fn main() {
    Repl::run();
}
