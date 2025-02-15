use crate::APP_NAME;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The settings comprise the static configuration for the available workspaces
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub workspaces: Vec<Workspace>,
    pub tmux: Option<Tmux>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tmux {
    pub socket_name: Option<String>,
}

impl Settings {
    pub fn load(path: Option<impl AsRef<Path>>) -> Result<Self> {
        match path {
            None => confy::load(APP_NAME, "config"),
            Some(path) => confy::load_path(path),
        }
        .context("Could not load settings")
    }
}

/// A workspace is equivalent to a tmux session
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Workspace {
    /// Name of the workspace when displayed
    pub name: String,
    /// Path to workspace directory
    pub directory: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        file.write_all(
            "
workspaces:
  - name: Default
    directory:  /home/user
  - name: project1
    directory:  /home/user/dev/project1
        "
            .as_bytes(),
        )?;

        let actual = Settings::load(Some(file.path()))?;
        let expected = Settings {
            workspaces: vec![
                Workspace {
                    name: "Default".to_owned(),
                    directory: PathBuf::from("/home/user"),
                },
                Workspace {
                    name: "project1".to_owned(),
                    directory: PathBuf::from("/home/user/dev/project1"),
                },
            ],
            tmux: None,
        };

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_read_tmux() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        file.write_all(
            "
tmux:
  socket_name: test
workspaces: []
        "
            .as_bytes(),
        )?;

        let actual = Settings::load(Some(file.path()))?;
        let expected = Settings {
            workspaces: vec![],
            tmux: Some(Tmux {
                socket_name: Some(String::from("test")),
            }),
        };

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_invalid_yaml() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        file.write_all("Not a valid yaml".as_bytes())?;

        let settings = Settings::load(Some(file.path()));
        assert!(settings.is_err());
        Ok(())
    }
}
