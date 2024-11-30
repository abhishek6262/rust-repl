use crate::console::Console;
use crate::enums::EditorMode;
use crate::message::Message;
use crate::playground::Playground;
use crate::prompt::Prompt;
use crate::stub::Stub;

pub struct Repl {
    mode: EditorMode,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            mode: EditorMode::STANDARD,
        }
    }

    fn set_editor_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn run(&mut self) {
        Console::clear();

        Message::print_welcome();

        loop {
            let input = Prompt::input(&self.mode);

            match input.as_str() {
                ".exit" => break,
                ".help" => Message::print_help(),
                ".editor" => self.set_editor_mode(EditorMode::EDITOR),
                _ => {
                    let buffer = match self.mode {
                        EditorMode::STANDARD => Stub::new(&input),
                        EditorMode::EDITOR => {
                            // Reset the editor mode to standard after buffer execution.
                            if let EditorMode::EDITOR = self.mode {
                                self.mode = EditorMode::STANDARD;
                            }

                            input
                        }
                    };

                    Playground::run(&buffer);
                }
            }
        }
    }
}
