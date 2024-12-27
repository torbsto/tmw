use crate::APP_NAME;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The settings comprise the static configuration for the available workspaces
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub workspaces: Vec<Workspace>,
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
        file.write(
            "
workspaces:
  - name: Default
    directory:  \"/home/user\"
  - name: project1
    directory:  \"/home/user/dev/project1\"
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
        };

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_invalid_yaml() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        file.write("Not a valid yaml".as_bytes())?;

        let settings = Settings::load(Some(file.path()));
        assert!(settings.is_err());
        Ok(())
    }
}
