use std::sync::atomic::{AtomicBool, Ordering};

pub struct Output {
    first: AtomicBool,
}

const BASE_SIZE: usize = 12;

impl Output {
    pub fn new() -> Self {
        Output {
            first: AtomicBool::new(true),
        }
    }

    pub fn log_building(&self, name: &str) {
        self.log_custom("Building", name, true, true);
    }

    pub fn log_installing(&self, name: &str) {
        self.log_custom("Installing", name, true, true);
    }

    pub fn log_installing_file(&self, file_name: &str) {
        self.log_custom("Installing", file_name, false, true);
    }

    pub fn log_cleaning(&self, name: &str) {
        self.log_custom("Cleaning", name, true, true);
    }

    pub fn log_finished(&self, verb: &str, name: &str) {
        self.log_custom("Finished", &format!("{} {}", verb, name), false, false);
    }

    pub fn log_custom(&self, verb: &str, content: &str, header: bool, dots: bool) {
        if header && !self.first.swap(false, Ordering::AcqRel) {
            println!();
        }

        for _ in 0..BASE_SIZE - verb.len() {
            print!(" ");
        }

        let dots = if dots { ". . ." } else { "" };
        let color = if header { "34" } else { "32" };

        println!("\x1B[1;{color}m{verb}\x1B[0m {content}{dots}");
    }

    pub fn log_error(&self, error: &dyn std::error::Error) {
        eprintln!("\x1B[1;31mError\x1B[37m: {error}\x1B[0m");
    }
}
