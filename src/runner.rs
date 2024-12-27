use crate::settings::Settings;
use anyhow::{anyhow, Context};
use std::process::{Command, Output, Stdio};

pub struct TmwRunner<'s> {
    settings: &'s Settings,
}

impl<'s> TmwRunner<'s> {
    pub fn new(settings: &'s Settings) -> Self {
        Self { settings }
    }

    pub fn switch_workspace(&self, name: &str) -> anyhow::Result<()> {
        let workspace = &self
            .settings
            .workspaces
            .iter()
            .find(|ws| ws.name == name)
            .context(format!("Project {} is unknown", name))?;

        // ensure that the session provided session already exists
        let result = self.tmux(vec!["has-session", "-t", name]).output()?;

        if !result.status.success() {
            let dir = workspace
                .directory
                .to_str()
                .context("Invalid workspace directory")?;

            self.tmux(vec!["new-session", "-d", "-s", name, "-c", dir])
                .validated_output()?;
        }

        self.tmux(vec!["switch-client", "-t", name])
            .validated_output()?;

        Ok(())
    }

    pub fn preview_workspace(&self, name: &str) -> anyhow::Result<()> {
        // find the session id by listing all session in the specified format, parsing the result
        // and filter by name
        let output = self
            .tmux(vec!["ls", "-F", "#{session_id},#{session_name}"])
            .validated_output()?
            .stdout;

        let string_output = String::from_utf8(output)?;
        let id = string_output
            .lines()
            .filter_map(|line| line.split_once(','))
            .find(|(_, session_name)| *session_name == name)
            .map(|(id, _)| id);

        match id {
            None => {
                println!("Workspace {} is not running", name);
            }
            Some(session_id) => {
                self.tmux(vec!["capture-pane", "-ep", "-t", session_id.trim()])
                    // Pipe immediately to stdout of this process
                    .stdout(Stdio::inherit())
                    .validated_output()?;
            }
        }

        Ok(())
    }

    pub fn list_workspaces(&self, exclude_active: bool) -> anyhow::Result<()> {
        let condition = match exclude_active {
            true => {
                let output = self
                    .tmux(vec!["display-message", "-p", "#S"])
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

    fn tmux(&self, args: Vec<&str>) -> Command {
        let mut merged_args = Vec::with_capacity(args.len() + 10);

        let socket_args = self
            .settings
            .tmux
            .as_ref()
            .and_then(|settings| settings.socket_name.as_ref().map(|name| vec!["-L", &name]))
            .unwrap_or_default();

        merged_args.extend(socket_args);
        merged_args.extend(args);

        let mut command = Command::new("tmux");
        command.args(merged_args);
        command
    }
}

trait ValidatedCommand {
    fn validated_output(&mut self) -> anyhow::Result<Output>;
}

impl ValidatedCommand for Command {
    fn validated_output(&mut self) -> anyhow::Result<Output> {
        let output = self.output()?;
        if !output.status.success() {
            let error = String::from_utf8(output.stderr).context("Unexpected tmux output")?;
            Err(anyhow!(error))
        } else {
            Ok(output)
        }
    }
}
