// Parameters
pub const DEFAULT_COMMAND: crate::Command = crate::Command::Build;

// Files
pub const TARGET_IMG: &str = "./os.img";

// Directories
pub const SYSROOT_DIR: &str = "./sysroot";

// Programs
pub const EMULATOR: &str = "qemu-system-x86_64";
pub const EMULATOR_FLAGS: [&str; 4] = ["-bios", "OVMF.fd", "-hdd", "os.img"];
pub const EMULATOR_DEBUG_FLAGS: [&str; 3] = ["-S", "-gdb", "tcp::1234"];

pub const DEBUGGER: &str = "gdb";
pub const DEBUGGER_FLAGS: [&str; 4] = [
    "-s",
    "./sysroot/kernel.elf",
    "-ex",
    "target remote localhost:1234",
];

pub const VBOX: &str = "VBoxManage";
pub const VBOX_FLAGS: [&str; 5] = ["convertfromraw", "--format", "VDI", TARGET_IMG, "os.vdi"];
