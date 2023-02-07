use crate::{
    args::Options,
    commands::{KERNEL_NAME, KERNEL_PATH},
    tools::{cargo, ToolError},
};

pub fn clean_kernel(options: &Options) -> Result<(), ToolError> {
    cargo::clean(KERNEL_NAME, KERNEL_PATH, options)
}
