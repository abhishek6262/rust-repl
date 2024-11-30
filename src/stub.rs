pub struct Stub;

impl Stub {
    pub fn new(buffer: &str) -> String {
        format!("fn main() {{\n{}\n}}", buffer)
    }
}
