use crate::args::Options;

pub const KERNEL_PATH: &'static str = "kernel";
pub const KERNEL_NAME: &'static str = "kernel";
pub const KERNEL_SYSROOT_PATH: &'static str = "kernel.elf";

const KERNEL_RELEASE_PATH: &'static str = "kernel/target/x86_64-los/release/init";
const KERNEL_DEBUG_PATH: &'static str = "kernel/target/x86_64-los/debug/init";

pub fn kernel_path(options: &Options) -> &'static str {
    if options.is_release() {
        KERNEL_RELEASE_PATH
    } else {
        KERNEL_DEBUG_PATH
    }
}
