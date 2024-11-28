use std::io::Write;

pub struct Prompt;

impl Prompt {
    pub fn input() -> String {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        buffer.trim_end().to_string()
    }
}
