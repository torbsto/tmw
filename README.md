# tmw

[![Build Status](https://github.com/torbsto/tmw/actions/workflows/check.yml/badge.svg)](https://github.com/torbsto/tmw/actions/workflows/check.yml)

A tmux workspace utility.

## Features

* Configure your existing workspaces
* List workspaces
* Preview the workspaces current tmux pane
* Select the workspace and switch the tmux session

## Usage

tmw is intended to be used with other tools like [fzf](https://github.com/junegunn/fzf)
and [direnv](https://direnv.net/).

For example, the following command opens a fuzzy-search over all configured workspaces,
shows a preview of the current pane for each workspace and switches to the selected workspace:

```shell
tmw select $(tmw list --exclude-active | fzf --prompt="Session> " --preview='tmw preview {}')
```

It can be combined with a tmux pop-up:

```shell
bind-key f display-popup -E '$SHELL -c $TMW_COMMAND'
```
