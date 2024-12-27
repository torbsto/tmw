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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_setup() -> Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        file.write_all(
            "
tmux:
  socket_name: test
workspaces:
  - name: Default
    directory:  /home
  - name: project1
    directory: /etc
        "
            .as_bytes(),
        )?;

        Command::new("tmux")
            .args([
                "-L", "test", "new", "-AD", "-s", "Default", "-t", "Default", "-c", "/home",
            ])
            .output()?;

        Ok(file)
    }

    #[test]
    fn test_list() -> Result<()> {
        let file = create_setup()?;
        Command::cargo_bin(APP_NAME)?
            .args(["--config-path", file.as_ref().to_str().unwrap(), "list"])
            .assert()
            .success()
            .stdout("Default\nproject1\n");
        Ok(())
    }

    #[test]
    fn test_select() -> Result<()> {
        let file = create_setup()?;
        Command::cargo_bin(APP_NAME)?
            .args([
                "--config-path",
                file.as_ref().to_str().unwrap(),
                "select",
                "project1",
            ])
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn test_preview() -> Result<()> {
        let file = create_setup()?;
        Command::cargo_bin(APP_NAME)?
            .args([
                "--config-path",
                file.as_ref().to_str().unwrap(),
                "select",
                "project1",
            ])
            .assert()
            .success();

        Command::cargo_bin(APP_NAME)?
            .args([
                "--config-path",
                file.as_ref().to_str().unwrap(),
                "select",
                "Default",
            ])
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn test_preview_not_started() -> Result<()> {
        let file = create_setup()?;
        Command::cargo_bin(APP_NAME)?
            .args([
                "--config-path",
                file.as_ref().to_str().unwrap(),
                "preview",
                "project1",
            ])
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn test_invalid_select() -> Result<()> {
        let file = create_setup()?;
        Command::cargo_bin(APP_NAME)?
            .args([
                "--config-path",
                file.as_ref().to_str().unwrap(),
                "select",
                "not-known",
            ])
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn test_invalid_preview() -> Result<()> {
        let file = create_setup()?;
        Command::cargo_bin(APP_NAME)?
            .args([
                "--config-path",
                file.as_ref().to_str().unwrap(),
                "select",
                "not-known",
            ])
            .assert()
            .failure();

        Ok(())
    }
}
