mod runner;
mod settings;

use crate::runner::TmwRunner;
use crate::settings::Settings;
use anyhow::Result;
use clap::{Parser, Subcommand};

pub const APP_NAME: &str = "tmw";

fn main() -> Result<()> {
    let args = Cli::parse();

    let settings = Settings::load(args.config_path)?;
    let runner = TmwRunner::new(&settings);

    match args.command {
        CliCommand::List { exclude_active } => runner.list_workspaces(exclude_active),
        CliCommand::Select { workspace } => runner.switch_workspace(&workspace),
        CliCommand::Preview { workspace } => runner.preview_workspace(&workspace),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,

    #[clap(long)]
    /// Overwrite location of config, defaults to `$XDG_CONFIG_HOME/tmw/config.yml`
    config_path: Option<String>,
}
#[derive(Debug, Subcommand)]
enum CliCommand {
    /// List all available workspaces
    List {
        /// Exclude the active workspace
        #[clap(long)]
        exclude_active: bool,
    },
    /// Switch to the tmux session of the selected workspace
    Select {
        /// Name of the workspace to select
        workspace: String,
    },
    /// Capture the current content of the selected workspace
    Preview {
        /// Name of the workspace to preview
        workspace: String,
    },
}
