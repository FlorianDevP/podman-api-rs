#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct ClippyArgs {
    /// Automatically apply lint suggestions (`clippy --fix`).
    #[arg(long)]
    pub(super) fix: bool,

    /// The package to run Clippy against (`cargo -p <PACKAGE> clippy`).
    #[arg(long, short)]
    pub(super) package: Option<String>,
}

pub fn run_clippy(args: ClippyArgs) -> Result<()> {
    let cargo = super::cargo();

    let mut clippy_command = Command::new(&cargo);
    clippy_command.arg("clippy");

    if let Some(package) = args.package.as_ref() {
        clippy_command.args(["--package", package]);
    } else {
        clippy_command.arg("--workspace");
    }

    clippy_command.arg("--all-targets").arg("--all-features");

    if args.fix {
        clippy_command.arg("--fix");
    }

    clippy_command.arg("--");

    // Deny all warnings.
    clippy_command.args(["--deny", "warnings", "-Dclippy::all"]);

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
        bail!("clippy failed: {}", exit_status);
    }

    Ok(())
}
