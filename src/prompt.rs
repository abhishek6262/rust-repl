use crate::enums::EditorMode;
use crate::stub::Stub;
use inquire::{Editor, Text};

pub struct Prompt;

impl Prompt {
    pub fn input(mode: &EditorMode) -> String {
        match mode {
            EditorMode::EDITOR => {
                let predefined_text = Stub::new(" //");

                Editor::new(":")
                    .with_predefined_text(predefined_text.as_str())
                    .prompt()
                    .unwrap()
            }

            EditorMode::STANDARD => Text::new(":").prompt().unwrap(),
        }
    }
}
