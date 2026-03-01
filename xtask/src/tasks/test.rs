#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct TestArgs {
    /// Only test code in documentation.
    #[arg(long)]
    pub(super) doc_only: bool,

    /// Skip testing code in documentation.
    #[arg(long)]
    pub(super) skip_doc: bool,

    /// The package to run test for (`cargo -p <PACKAGE> test`).
    #[arg(long, short)]
    pub(super) package: Option<String>,
}

pub fn run_test(args: TestArgs) -> Result<()> {
    let cargo = super::cargo();

    if !args.doc_only {
        let mut clippy_command = Command::new(&cargo);
        clippy_command.arg("test");

        if let Some(package) = args.package.as_ref() {
            clippy_command.args(["--package", package]);
        } else {
            clippy_command.arg("--workspace");
        }

        clippy_command.arg("--all-targets").arg("--all-features");

        eprintln!(
            "running: {} {}",
            cargo.display(),
            clippy_command
                .get_args()
                .map(|arg| arg.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let exit_status = clippy_command
            .spawn()
            .context("failed to spawn child process")?
            .wait()
            .context("failed to wait for child process")?;

        if !exit_status.success() {
            bail!("cargo test failed: {}", exit_status);
        }
    }

    if !args.skip_doc {
        let mut clippy_command = Command::new(&cargo);
        clippy_command.arg("test");

        if let Some(package) = args.package.as_ref() {
            clippy_command.args(["--package", package]);
        } else {
            clippy_command.arg("--workspace");
        }

        clippy_command.arg("--doc");

        eprintln!(
            "running: {} {}",
            cargo.display(),
            clippy_command
                .get_args()
                .map(|arg| arg.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let exit_status = clippy_command
            .spawn()
            .context("failed to spawn child process")?
            .wait()
            .context("failed to wait for child process")?;

        if !exit_status.success() {
            bail!("cargo test docs failed: {}", exit_status);
        }
    }

    Ok(())
}
