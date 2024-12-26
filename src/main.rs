use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "tmw";

/// The settings comprise the static configuration for the available workspaces
#[derive(Default, Debug, Serialize, Deserialize)]
struct Settings {
    workspaces: Vec<Workspace>,
    /// Name of the project variable
    env_file_name: Option<String>,
}

/// A workspace is equivalent to a tmux session
#[derive(Default, Debug, Serialize, Deserialize)]
struct Workspace {
    /// Name of the workspace when displayed
    name: String,
    /// Path to workspace directory
    directory: PathBuf,
    /// Path to file with project specific env variables.
    /// Defaults to `$XDG_CONFIG_HOME/tmw/$PROJECT.env`.
    env_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let settings: Settings = confy::load(APP_NAME, "config")?;

    let runner = TmwRunner {
        settings: &settings,
    };

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
}
#[derive(Debug, Subcommand)]
enum CliCommand {
    /// List all available workspaces
    List {
        /// Exclude the active workspace
        #[clap(short, long)]
        exclude_active: bool,
    },
    /// Switch to the tmux session of the selected workspace
    Select { workspace: String },
    /// Capture the current content of the selected workspace
    Preview { workspace: String },
}

struct TmwRunner<'s> {
    settings: &'s Settings,
}

impl TmwRunner<'_> {
    fn switch_workspace(&self, name: &str) -> Result<()> {
        let workspace = &self
            .settings
            .workspaces
            .iter()
            .find(|ws| ws.name == name)
            .context(format!("Project {:?} is unknown", name))?;

        // ensure that the session provided session already exists
        let result = Command::new("tmux")
            .args(["has-session", "-t", name])
            .output()?;

        if !result.status.success() {
            let dir = workspace
                .directory
                .to_str()
                .context("Invalid workspace directory")?;

            Command::new("tmux")
                .args(["new-session", "-d", "-s", name, "-c", dir])
                .validated_output()?;
        }

        Command::new("tmux")
            .args(["switch-client", "-t", name])
            .validated_output()?;

        Ok(())
    }

    fn preview_workspace(&self, name: &str) -> Result<()> {
        // find session id because it is required for capture-pane
        // this uses tmux filter `-f` to compare the names
        let output = Command::new("tmux")
            .args([
                "ls",
                "-F",
                "#{session_id}",
                "-f",
                &format!("#{{==:#{{session_name}},{}}}", name),
            ])
            .validated_output()?
            .stdout;

        let id = String::from_utf8(output)?;

        if id.is_empty() {
            println!("Workspace {} is not started", name);
            return Ok(());
        }

        Command::new("tmux")
            .args(["capture-pane", "-ep", "-t", id.trim()])
            // Pipe immediately to stdout of this process
            .stdout(Stdio::inherit())
            .validated_output()?;

        Ok(())
    }

    fn list_workspaces(&self, exclude_active: bool) -> Result<()> {
        let condition = match exclude_active {
            true => {
                let output = Command::new("tmux")
                    .args(["display-message", "-p", "#S"])
                    .validated_output()?
                    .stdout;

                Some(String::from_utf8(output)?)
            }
            false => None,
        };

        let names: Vec<_> = self
            .settings
            .workspaces
            .iter()
            .map(|ws| ws.name.as_str())
            .filter(|ws| condition.as_ref().map(|c| c != ws).unwrap_or(true))
            .collect();

        println!("{}", names.join("\n"));

        Ok(())
    }
}

trait ValidatedCommand {
    fn validated_output(&mut self) -> Result<Output>;
}

impl ValidatedCommand for Command {
    fn validated_output(&mut self) -> Result<Output> {
        let output = self.output()?;
        if !output.status.success() {
            let error = String::from_utf8(output.stderr).context("Unexpected tmux output")?;
            Err(anyhow!(error))
        } else {
            Ok(output)
        }
    }
}
