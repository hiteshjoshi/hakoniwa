use clap::{AppSettings, Parser, Subcommand};

use crate::cli::RunCommand;

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
enum Commands {
    /// Run a COMMAND in a sandbox
    Run(RunCommand),
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct RootCommand {
    #[clap(subcommand)]
    command: Commands,
}

pub fn execute() {
    let cli = RootCommand::parse();
    match &cli.command {
        Commands::Run(cmd) => RunCommand::execute(&cli, cmd),
    }
}
