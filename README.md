# tmw

A tmux workspace utility.

tmw allows you to store workspaces in a configuration file, to list and preview the workspaces, and to select them.
The workspace's session starts in a provided directory and exposes an env file for configuration.

## Usage

See `tmw --help`:

```
❯ tmw --help
Usage: tmw <COMMAND>

Commands:
  list     List all available workspaces
  select   Switch to the tmux session of the selected workspace
  preview  Capture the current content of the selected workspace
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

The intended use is in combination with something like fzf:

```
tmw select $(tmw list --exclude-active | command fzf --prompt="Session> " --preview='tmw preview {}' --border=none)
```

and integration with tmux, for example with a popup configured in your `tmux.conf`:

```
bind-key f display-popup -E 'fish -c tmux-projects'
```
