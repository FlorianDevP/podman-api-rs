mod tasks;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo xtask")]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Runs `codegen`, `lint`, `test` and `doc`.
    All(tasks::all::AllArgs),
    Codegen(tasks::codegen::CodegenArgs),
    /// Runs `cargo fmt`.
    Fmt(tasks::fmt::FmtArgs),
    /// Runs `cargo clippy`.
    Clippy(tasks::clippy::ClippyArgs),
    /// Runs `cargo fmt` and `clippy`.
    Lint(tasks::lint::LintArgs),
    Doc(tasks::doc::DocArgs),
    Test(tasks::test::TestArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        CliCommand::All(args) => tasks::all::run_all(args),
        CliCommand::Codegen(args) => tasks::codegen::run_codegen(args),
        CliCommand::Fmt(args) => tasks::fmt::run_fmt(args),
        CliCommand::Clippy(args) => tasks::clippy::run_clippy(args),
        CliCommand::Lint(args) => tasks::lint::run_lint(args),
        CliCommand::Doc(args) => tasks::doc::run_doc(args),
        CliCommand::Test(args) => tasks::test::run_test(args),
    }
}
