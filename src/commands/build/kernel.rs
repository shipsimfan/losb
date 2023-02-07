use crate::{
    args::Options,
    commands::{KERNEL_NAME, KERNEL_PATH},
    tools::{cargo, ToolError},
};

pub fn build_kernel(options: &Options) -> Result<(), ToolError> {
    cargo::build(KERNEL_NAME, KERNEL_PATH, options)
}
