#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use anyhow::{Context as _, Result};
use clap::Parser;

use super::clippy::*;
use super::fmt::*;

#[derive(Debug, Clone, Default, Parser)]
pub struct LintArgs {
    /// Automatically apply lint suggestions and fix formatting.
    #[arg(long)]
    pub(super) fix: bool,

    /// Automatically apply lint suggestions (`clippy --fix`).
    #[arg(long)]
    pub(super) clippy_fix: bool,

    /// The package to run fmt and Clippy against.
    #[arg(long, short)]
    pub(super) package: Option<String>,
}

impl From<LintArgs> for FmtArgs {
    fn from(value: LintArgs) -> Self {
        FmtArgs {
            fix: value.fix,
            package: value.package,
        }
    }
}

impl From<LintArgs> for ClippyArgs {
    fn from(value: LintArgs) -> Self {
        ClippyArgs {
            fix: value.clippy_fix || value.fix,
            package: value.package,
        }
    }
}

pub fn run_lint(args: LintArgs) -> Result<()> {
    run_fmt(args.clone().into()).context("failed to run lint")?;

    run_clippy(args.into()).context("failed to run lint")
}
