use crate::rust::Rust;

pub struct Message;

impl Message {
    pub fn print_help() {
        println!("The REPL has some special commands, all starting with a dot. They are
        .help: shows the dot commands help
        .editor: enables editor mode, to write multiline Rust code with ease. Once you are in this mode, enter ctrl-D to run the code you wrote.
        .exit: exits the repl");
    }

    pub fn print_welcome() {
        println!("Welcome to {}", Rust::get_version());
        println!("Type \".help\" for more information.");
    }
}
