use std::path::Path;

mod build;
mod image;
mod projects;

#[derive(Debug)]
pub enum Command {
    Build,
    BuildImage,
    BuildISO,
    Clean,
    Debug,
    Help,
    Run,
    VBox,
    Version,
}

fn display_help() {
    println!("Build utility for Lance OS\n");

    println!("\x1B[1mUsage:\x1B[0m");
    println!(
        "    {} [command] [configuration]\n",
        std::env::args().next().unwrap()
    );

    println!("\x1B[1mCommands:\x1B[0m");
    println!("    {}\t Builds the configuration", Command::Build);
    println!(
        "    {}\t Builds the configuration then produces a hard drive image",
        Command::BuildImage
    );
    println!(
        "    {}\t Builds the configuration then produces a disk image",
        Command::BuildISO
    );
    println!(
        "    {}\t Cleans the configuration and the sysroot",
        Command::Clean
    );
    println!(
        "    {}\t Performs {}, then runs qemu connected with gdb (Linux Only)",
        Command::Debug,
        Command::BuildImage
    );
    println!(
        "    {}\t Displays information about this program",
        Command::Help
    );
    println!(
        "    {}\t\t Performs {}, then runs qemu (Linux Only)",
        Command::Run,
        Command::BuildImage
    );
    println!(
        "    {}\t Performs {}, then converts the image into a .vdi",
        Command::VBox,
        Command::BuildImage
    );
    println!(
        "    {}\t Displays the version of this program",
        Command::Version
    );

    println!();
    println!("Default options:");
    println!("    Command - {}", crate::config::DEFAULT_COMMAND);
    println!(
        "    Configuration - {}",
        crate::config::DEFAULT_CONFIGURATION
    );
}

fn display_version() {
    println!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}

fn build(configuration: &str) -> Result<(), Box<dyn std::error::Error>> {
    let projects = projects::Projects::parse_from_file(Path::new(crate::config::PROJECTS_JSON))?;
    projects.build_configuration(configuration)
}

fn clean(configuration: &str) -> Result<(), Box<dyn std::error::Error>> {
    let projects = projects::Projects::parse_from_file(Path::new(crate::config::PROJECTS_JSON))?;
    projects.clean_configuration(configuration)
}

fn build_image(configuration: &str) -> Result<(), Box<dyn std::error::Error>> {
    build(configuration)?;
    image::create_image()
}

fn run(configuration: &str) -> Result<(), Box<dyn std::error::Error>> {
    build_image(configuration)?;

    let mut emulator_command = std::process::Command::new(crate::config::EMULATOR);
    emulator_command.args(crate::config::EMULATOR_FLAGS);
    emulator_command.stdout(std::process::Stdio::inherit());
    emulator_command.stderr(std::process::Stdio::inherit());
    emulator_command.stdin(std::process::Stdio::inherit());
    emulator_command.output()?;

    Ok(())
}

fn debug(configuration: &str) -> Result<(), Box<dyn std::error::Error>> {
    build_image(configuration)?;

    let mut emulator_command = std::process::Command::new(crate::config::EMULATOR);
    emulator_command.args(crate::config::EMULATOR_FLAGS);
    emulator_command.args(crate::config::EMULATOR_DEBUG_FLAGS);
    emulator_command.stdout(std::process::Stdio::inherit());
    emulator_command.stderr(std::process::Stdio::inherit());
    emulator_command.stdin(std::process::Stdio::inherit());
    emulator_command.spawn()?;

    let mut debugger_command = std::process::Command::new(crate::config::DEBUGGER);
    debugger_command.args(crate::config::DEBUGGER_FLAGS);
    debugger_command.stdout(std::process::Stdio::inherit());
    debugger_command.stderr(std::process::Stdio::inherit());
    debugger_command.stdin(std::process::Stdio::inherit());
    debugger_command.output()?;

    Ok(())
}

fn vbox(configuration: &str) -> Result<(), Box<dyn std::error::Error>> {
    build_image(configuration)?;

    let mut vbox_command = std::process::Command::new(crate::config::VBOX);
    vbox_command.args(crate::config::VBOX_FLAGS);
    vbox_command.stdout(std::process::Stdio::inherit());
    vbox_command.stderr(std::process::Stdio::inherit());
    vbox_command.stdin(std::process::Stdio::inherit());
    let output = vbox_command.output()?;

    if !output.status.success() {
        Err(Box::new(crate::error::Error::BuildError(output.status)))
    } else {
        Ok(())
    }
}

impl Command {
    pub fn parse(command: &str) -> Result<Self, crate::Error> {
        match command.to_lowercase().as_str() {
            "build" => Ok(Command::Build),
            "build-image" => Ok(Command::BuildImage),
            "build-iso" => Ok(Command::BuildISO),
            "clean" => Ok(Command::Clean),
            "debug" => Ok(Command::Debug),
            "help" => Ok(Command::Help),
            "run" => Ok(Command::Run),
            "vbox" => Ok(Command::VBox),
            "version" => Ok(Command::Version),
            _ => Err(crate::Error::InvalidCommand(command.to_string())),
        }
    }

    pub fn perform(self, configuration: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Build => build(configuration),
            Command::BuildImage => build_image(configuration),
            Command::BuildISO => Err(Box::new(crate::Error::NotImplemented(self))),
            Command::Clean => clean(configuration),
            Command::Debug => debug(configuration),
            Command::Help => Ok(display_help()),
            Command::Run => run(configuration),
            Command::VBox => vbox(configuration),
            Command::Version => Ok(display_version()),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Build => "build",
                Command::BuildImage => "build-image",
                Command::BuildISO => "build-iso",
                Command::Clean => "clean",
                Command::Debug => "debug",
                Command::Help => "help",
                Command::Run => "run",
                Command::VBox => "vbox",
                Command::Version => "version",
            }
        )
    }
}
