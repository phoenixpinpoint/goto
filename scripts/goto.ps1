# PowerShell wrapper for goto
# Add this to your PowerShell profile ($PROFILE):
# . C:\path\to\goto\scripts\goto.ps1
#
# Or copy goto.exe to a directory in your PATH and this script to the same location,
# then add the script's directory to your PATH

function goto {
    # Determine the path to the goto executable
    # First check if it's in the same directory as this script
    $scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
    $gotoExe = Join-Path $scriptPath "goto.exe"
    
    # If not found in script directory, assume it's in PATH
    if (-not (Test-Path $gotoExe)) {
        $gotoExe = "goto.exe"
    }
    
    # Call the goto program with all arguments and capture output
    $output = & $gotoExe $args 2>&1 | Out-String
    $output = $output.Trim()
    
    # Check if output starts with GOTO:
    if ($output -match '^GOTO:(.+)$') {
        # Extract the path and change directory
        $path = $matches[1]
        Set-Location $path
    }
    else {
        # Just display the output if it's not a directory change
        Write-Output $output
    }
}
