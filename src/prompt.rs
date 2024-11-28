use inquire::Text;

pub struct Prompt;

impl Prompt {
    pub fn input() -> String {
        Text::new(":").prompt().unwrap().to_string()
    }
}
