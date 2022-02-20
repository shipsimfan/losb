use crate::command::Command;

pub fn display_help() {
    println!("Build utility for Lance OS\n");

    println!("\x1B[1mUsage:\x1B[0m");
    println!("    {} [command]\n", std::env::args().next().unwrap());

    println!("\x1B[1mCommands:\x1B[0m");
    println!("    {}\t Builds everything", Command::Build);
    println!(
        "    {}\t Builds everything then produces a hard drive image",
        Command::BuildImage
    );
    println!(
        "    {}\t Builds everything then produces a disk image",
        Command::BuildISO
    );
    println!(
        "    {}\t Cleans all built objects including sysroot",
        Command::Clean
    );
    println!(
        "    {}\t Cleans all built userspace objects including sysroot",
        Command::CleanUser
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
    println!("Default Command - {}", crate::config::DEFAULT_COMMAND);
}
