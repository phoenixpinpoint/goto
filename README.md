# goto

A simple terminal program to navigate to locations on a system using shortcuts.

## Installation

### Linux/macOS

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

### Windows

1. Run the install script:

   **PowerShell:**
   ```powershell
   .\install.ps1
   ```
   
   To install the debug version instead:
   ```powershell
   .\install.ps1 -Debug
   ```

   **Command Prompt:**
   ```cmd
   install.bat
   ```
   
   To install the debug version instead:
   ```cmd
   install.bat debug
   ```
   
   This will build the project and install the binary and wrapper scripts to `%USERPROFILE%\.local\bin`.

2. Add `%USERPROFILE%\.local\bin` to your PATH if it's not already there. You can do this by running PowerShell as Administrator:
   ```powershell
   [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.local\bin", "User")
   ```

### Shell Configuration

Configure your shell to use the goto wrapper:

   **For zsh** (add to `~/.zshrc`):
   ```bash
   source /path/to/goto/scripts/goto.zsh
   ```

   **For bash** (add to `~/.bashrc`):
   ```bash
   source /path/to/goto/scripts/goto.bash
   ```

   **For PowerShell** (if you used the install script, add to your PowerShell profile):
   ```powershell
   . $env:USERPROFILE\.local\bin\goto-wrapper.ps1
   ```
   
   Or if you're using the script directly from the repository:
   ```powershell
   . C:\path\to\goto\scripts\goto.ps1
   ```
   
   To find your PowerShell profile location, run:
   ```powershell
   $PROFILE
   ```

   **For Windows Command Prompt (CMD)**:
   
   If you used the install script, `goto.bat` is already installed and ready to use. Otherwise, copy both `goto.exe` and `scripts\goto.bat` to a directory in your PATH.

### Restart Your Shell

Restart your shell or reload the configuration:

**Linux/macOS:**
```bash
source ~/.zshrc  # or ~/.bashrc for bash
```

**PowerShell:**
```powershell
. $PROFILE
```

## Usage

### Add a shortcut

Save the current directory:
```bash
goto -a shortcut_name
```

Or save a specific path:
```bash
goto -a shortcut_name=/path/to/directory
```

You can use `~` in paths:
```bash
goto -a home=~
goto -a projects=~/code/myprojects
```

### List shortcuts
```bash
goto -l
```

### Navigate to a shortcut
```bash
goto shortcut_name
```

### Show help
```bash
goto -h
```

### Configuration

Shortcuts are stored in `~/.shortcuts` as simple key=value pairs:
```
home=/Users/username
work=/Users/username/code
projects=~/code/myproject
```

You can use `~` in paths and it will be expanded automatically.
