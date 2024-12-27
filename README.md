# tmw

A tmux workspace utility.

tmw allows you to store workspaces in a configuration file, to list and preview the workspaces, and to select them.
The workspace's session starts in a provided directory and exposes an env file for configuration.

## Usage

See `tmw --help`:

```
❯ tmw --help
Usage: tmw [OPTIONS] <COMMAND>

Commands:
  list     List all available workspaces
  select   Switch to the tmux session of the selected workspace
  preview  Capture the current content of the selected workspace
  help     Print this message or the help of the given subcommand(s)

Options:
      --config-path <CONFIG_PATH>  Overwrite location of config, defaults to `$XDG_CONFIG_HOME/tmw/config.yml`
  -h, --help                       Print help
  -V, --version                    Print version
```

The intended use is in combination with something like fzf:

```shell
tmw select $(tmw list --exclude-active | fzf --prompt="Session> " --preview='tmw preview {}' --border=none)
```

and integration with tmux, for example with a popup configured in your `tmux.conf`:

```shell
bind-key f display-popup -E 'fish -c tmux-projects'
```
