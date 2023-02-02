mod args;
mod commands;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let options = args::parse_args()?;

    // Execute command
    match options.command() {
        _ => panic!("TODO: Implement"),
    }
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
