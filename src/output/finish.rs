use std::io::{stdout, Write};

pub struct Finish {
    dots: bool,
    newline: bool,
}

impl Finish {
    pub fn dots_carriage_return() -> Finish {
        Finish {
            dots: true,
            newline: false,
        }
    }

    pub fn newline() -> Finish {
        Finish {
            dots: false,
            newline: true,
        }
    }

    pub fn dots_newline() -> Finish {
        Finish {
            dots: true,
            newline: true,
        }
    }

    pub fn execute(self) {
        if self.dots {
            print!(". . .");
        }

        print!("            ");

        if self.newline {
            println!();
        } else {
            print!("\r");
            stdout().lock().flush().unwrap();
        }
    }
}
