use crate::{
    args::Options,
    output::Output,
    tools::{cargo, ToolError},
};

const KERNEL_PATH: &'static str = "kernel";
const KERNEL_NAME: &'static str = "kernel";

pub fn build_kernel(options: &Options, output: &Output) -> Result<(), ToolError> {
    output.log_building(KERNEL_NAME);
    cargo::build(options.path().join(KERNEL_PATH))
}
