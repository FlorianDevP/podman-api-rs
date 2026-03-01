#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use anyhow::{Context as _, Result};
use clap::Parser;

use super::codegen::*;
use super::doc::*;
use super::lint::*;
use super::test::*;

#[derive(Debug, Clone, Default, Parser)]
pub struct AllArgs {
    /// The package to run All tasks against.
    #[arg(long, short)]
    package: Option<String>,
}

pub fn run_all(args: AllArgs) -> Result<()> {
    run_codegen(CodegenArgs::default()).context("failed to run codegen")?;

    run_lint(LintArgs {
        package: args.package.clone(),
        clippy_fix: true,
        ..Default::default()
    })
    .context("failed to run lint")?;

    run_test(TestArgs {
        package: args.package,
        ..Default::default()
    })
    .context("failed to run test")?;

    run_doc(DocArgs::default()).context("failed to run doc")
}
