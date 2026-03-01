#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct DocArgs {
    /// Also generate documentation for dependencies.
    #[arg(long)]
    pub(super) deps: bool,
}

pub fn run_doc(args: DocArgs) -> Result<()> {
    let cargo = super::cargo();

    let mut doc_command = Command::new(&cargo);
    doc_command.arg("doc");

    if !args.deps {
        doc_command.arg("--no-deps");
    }

    eprintln!(
        "running: {} {}",
        cargo.display(),
        doc_command
            .get_args()
            .map(|arg| arg.to_str().unwrap())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let exit_status = doc_command
        .spawn()
        .context("failed to spawn child process")?
        .wait()
        .context("failed to wait for child process")?;

    if !exit_status.success() {
        bail!("cargo doc failed: {}", exit_status);
    }

    Ok(())
}
