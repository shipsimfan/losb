pub struct Output;

impl Output {
    pub fn new() -> Self {
        Output
    }

    pub fn log_building(&self, name: &str) {
        println!("Building {} . . .", name);
    }

    pub fn log_installing(&self, name: &str) {
        println!("Installing {} . . .", name);
    }

    pub fn log_installing_file(&self, file_name: &str) {
        println!("  Instaliing \"{}\" . . .", file_name);
    }

    pub fn log_error(&self, error: &dyn std::error::Error) {
        eprintln!("Error: {}", error);
    }
}
