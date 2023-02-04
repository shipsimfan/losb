use crate::{
    args::Options,
    commands::names::KERNEL_NAME,
    output::Output,
    tools::{cargo, ToolError},
};

const KERNEL_PATH: &'static str = "kernel";

pub fn build_kernel(options: &Options, output: &Output) -> Result<(), ToolError> {
    cargo::build(KERNEL_NAME, KERNEL_PATH, options, output)
}
