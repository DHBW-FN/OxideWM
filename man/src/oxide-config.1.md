% OXIDE-CONFIG(1) oxide-config 0.1.0
% Philipp Kalinowski
% February 2023

# NAME

oxide-config - config

# FILES

When starting, Oxide has two paths it searches for config files.

**~/.config/oxide/config.yml**
: Home config file

**/etc/oxide/config.yml**
: System config file

# DESCRIPTION

Define the behaviour of oxidewm. You can change style, layout, keybindings and more.
The config file is written using YAML syntax.
If the home config file is not existing, default values will be used but commands like `exec` and `exec_always` will not be working.

# KEYBINDING

## KEYS

The keys need at least one MODIFIER and one normal key such as 't'

## MODIFIER

**M**
: Meta key

**A**
: ALT key

**C**
: CONTROL key

**S**
: SHIFT key

# COMMANDS

Commands consist of a command and optional arguments.

## COMMAND

Move **args** [MOVEMENT]
: Move window

Focus **args** [MOVEMENT]
: Move Focus

Quit
: Quit the window manager

Kill
: Kill the currently focused window

Restart
: Reloads the config and restarts components

Layout **args** [LAYOUT]
: Change the current layout

GoToWorkspace **args** [WORKSPACE_ARGS]
: Change the current workspace

MoveToWorkspace **args** [WORKSPACE_ARGS]
: Move the used window to a different workspace

MoveToWorkspaceAndFollow **args** [WORKSPACE_ARGS]
: Move the focused window to and select a different workspace

Exec **args** <COMMAND>
: Execute a given command

Fullscreen
: Toggle fullscreen mode for the focused window

## ARGS

Command arguments are necessary for the movement, the layout or to controll workspaces

## MOVEMENT

Left
: Moves to the left

right
: Moves to the right

## LAYOUT

VerticalStriped
: windows vertically next to each other

HorizontalStriped
: windows horizontally underneath each other

None
: if no argument is provided, the next layout is chosen

## WORKSPACE_ARGS

Next
: next initialized workspace with a higher index than the current workspace

Previous
: next initialized workspace with a lower index than the current workspace

Next_free
: next available workspace with a higher index than the current workspace which is not initialized

Index
: workspace with the given index

## ITERATIONS

iter
: iterates over given number in order to change

# DEFAULT KEYBINDINGS

Here is a short overview of the default keybindings

Meta+Shift+e
: quits the window manager

Meta+Shift+r
: restarts the window manager

Meta+Shift+q
: kills the current window

h/l
: direction keys (left/right)

Meta+<direction>
: changes the focus to the <direction> window

Meta+Shift+<direction>
: moves the window to the <direction>

Meta+f
: changes the current window to fullscreen

Meta+u
: switches to the next layout

Meta+i
: changes the layout to vertical

Meta+Shift+i
: changes to layout to horizontal

Right/Left;
: workspace navigation keys (next/previous)

Meta+<workspace direction>
: changes to the <workspace direction> workspace

Meta+n
: opens a new workspace

Control+Meta+<workspace direction>
: moves a window to the <workspace direction> workspace

Control+Meta+n
: opens a new workspace and moves the window to it

Meta+Shift+<workspace direction>
: moves the window to the <workspace direction> workspace and follows it

Meta+Shift+n
: creates a new workspace, moves the window to it and follows

Control+Meta+Down
: quits the workspace

Meta+t
: opens a new firefox window

1/2/3/4/5/6/7/8/9;
: workspace numbers

Meta+<workspace number>
: switches to workspace <workspace number>

Control+Meta+<workspace number>
: moves window to workspace <workspace number>

Meta+Shift+<workspace number>
: moves window to workspace <workspace number> and follows it

# BORDERS

border_witdh
: sets the border witdh of windows in pixels

border_color
: sets the border color and has to be entered in hexadecimal

border_focus_color
: sets the border color for focused nbdows and has to be entered in hexadecimal

gap
: gap between windows in pixels

# EXECTUE

exc
: onetime execution when the window manager starts

exec_always
: is executed during start of the window manager and also at each restart

# EXAMPLES

TODO: SAMPLE CONFIG

## KEYBINDINGS

```yaml
cmds:
  - keys: ["M", "t"]
    commands:
      - command: Exec
        args: "firefox"
```

In this example pressing the meta key and 't', a new firefox window is opened.

## ITERATIONS

```yaml
iter_cmds:
  - iter: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    command:
      keys: ["M", "C", "$VAR"]
      commands:
        - command: GoToWorkspace
          args: "$VAR"
```

In this example using the ALT and CONTROLL key paired with a number from one to nine, the user can go to the desired workspace.
`$VAR` is a reference for the entered iterator.

# Bugs

Please open an issue <https://github.com/DHBW-FN/OxideWM/issues> .

# COPYRIGHT

Copyright © 2023 Philipp Kalinowski GPLv3+\: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: You are free to change and redistribute it. There is NO WARRANTY to the extent permitted by law.

# FURTHER DOCUMENTATION

Access the full Oxide documentation under **https://oxide.readthedocs.io/**.

# SEE ALSO

**oxide(1)**, **oxide-msg(1)**, **oxide-bar(1)**
