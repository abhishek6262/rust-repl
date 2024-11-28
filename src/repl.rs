use crate::console::Console;
use crate::prompt::Prompt;
use crate::rust::Rust;

pub struct Repl;

impl Repl {
    pub fn run() {
        Console::clear();

        Self::print_welcome_message();

        loop {
            let input = Prompt::input();

            if input.eq(".exit") {
                break;
            } else if input.eq(".help") {
                Self::print_help_message();
            } else {
                Rust::run(&input);
            }
        }
    }

    fn print_help_message() {
        println!("The REPL has some special commands, all starting with a dot. They are
        .help: shows the dot commands help
        .editor: enables editor mode, to write multiline Rust code with ease. Once you are in this mode, enter ctrl-D to run the code you wrote.
        .exit: exits the repl");
    }

    fn print_welcome_message() {
        println!("Welcome to {}", Rust::get_version());
        println!("Type \".help\" for more information.");
    }
}
