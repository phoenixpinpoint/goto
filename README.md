# goto

A simple terminal program to navigate to locations on a system using shortcuts.

## Installation

1. Run the install script:
   ```bash
   ./install.sh
   ```
   
   To install the debug version instead:
   ```bash
   ./install.sh --debug
   ```
   
   This will build the project and install the binary to `~/.local/bin`.

2. Make sure `~/.local/bin` is in your PATH. Add to your shell config if needed:
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

3. Source the shell wrapper script in your shell configuration file:

   **For zsh** (add to `~/.zshrc`):
   ```bash
   source /path/to/goto/scripts/goto.zsh
   ```

   **For bash** (add to `~/.bashrc`):
   ```bash
   source /path/to/goto/scripts/goto.bash
   ```

4. Restart your shell or run:
   ```bash
   source ~/.zshrc  # or ~/.bashrc for bash
   ```

## Usage

### Add a shortcut
```bash
goto -a shortcut_name
```
This will save your current directory as `shortcut_name`.

### List shortcuts
```bash
goto -l
```

### Navigate to a shortcut
```bash
goto shortcut_name
```

### Configuration

Shortcuts are stored in `~/.shortcuts` as simple key=value pairs:
```
home=/Users/username
work=/Users/username/code
projects=~/code/myproject
```

You can use `~` in paths and it will be expanded automatically.
