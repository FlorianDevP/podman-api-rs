#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct FmtArgs {
    /// Automatically apply fmt suggestions (`fmt` without `--check`).
    #[arg(long)]
    pub(super) fix: bool,

    /// The package to run fmt against (`cargo -p <PACKAGE> fmt`).
    #[arg(long, short)]
    pub(super) package: Option<String>,
}

pub fn run_fmt(args: FmtArgs) -> Result<()> {
    let cargo = super::cargo();

    let mut fmt_command = Command::new(&cargo);
    fmt_command.arg("fmt");

    if let Some(package) = args.package.as_ref() {
        fmt_command.args(["--package", package]);
    } else {
        fmt_command.arg("--all");
    }

    fmt_command.arg("--");

    if !args.fix {
        fmt_command.arg("--check");
    }

    eprintln!(
        "running: {} {}",
        cargo.display(),
        fmt_command
            .get_args()
            .map(|arg| arg.to_str().unwrap())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let exit_status = fmt_command
        .spawn()
        .context("failed to spawn child process")?
        .wait()
        .context("failed to wait for child process")?;

    if !exit_status.success() {
        bail!("cargo fmt failed: {}", exit_status);
    }

    Ok(())
}
