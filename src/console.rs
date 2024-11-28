pub struct Console;

impl Console {
    pub fn clear() {
        std::process::Command::new("clear").status().unwrap();
    }
}
