use std::sync::atomic::{AtomicBool, Ordering};

mod color;
mod finish;
mod initial;

pub use color::*;
pub use finish::*;
pub use initial::*;

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

    #[allow(unused)]
    pub fn reset_first(&self) {
        self.first.store(true, Ordering::Release)
    }

    pub fn log_begin(&self, verb: &str, content: &str, initial: Initial) {
        self.log(
            verb,
            content,
            initial,
            Color::Green,
            Finish::dots_carriage_return(),
        );
    }

    pub fn log_complete(&self, verb: &str, content: &str) {
        self.log(verb, content, Initial::None, Color::Blue, Finish::newline());
    }

    pub fn log_warning(&self, content: &str) {
        self.log(
            "Warning",
            content,
            Initial::None,
            Color::Yellow,
            Finish::newline(),
        )
    }

    pub fn log(&self, verb: &str, content: &str, initial: Initial, color: Color, finish: Finish) {
        let first = self.first.swap(false, Ordering::AcqRel);
        initial.execute(first);

        for _ in 0..BASE_SIZE - verb.len() {
            print!(" ");
        }

        print!("\x1B[1m{color}{verb}\x1B[0m {content}");

        finish.execute();
    }

    pub fn log_error(&self, error: &dyn std::error::Error) {
        eprintln!("\n\x1B[1;31mError\x1B[37m: {error}\x1B[0m");
    }
}
