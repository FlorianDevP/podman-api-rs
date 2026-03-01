#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct CodegenArgs {}

pub fn run_codegen(args: CodegenArgs) -> Result<()> {
    Ok(())
}
