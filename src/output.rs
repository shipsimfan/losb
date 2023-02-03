pub struct Output;

impl Output {
    pub fn new() -> Self {
        Output
    }

    pub fn log_building(&self, name: &'static str) {
        println!("Building {} . . .", name);
    }

    pub fn log_error(&self, error: &dyn std::error::Error) {
        eprintln!("Error: {}", error);
    }
}
