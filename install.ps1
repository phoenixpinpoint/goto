# PowerShell install script for goto
# Usage: .\install.ps1 [-Debug]

param(
    [switch]$Debug
)

$ErrorActionPreference = "Stop"

# Determine build type
$buildType = if ($Debug) { "debug" } else { "release" }
$targetDir = "target\$buildType"

Write-Host "Building goto ($buildType)..." -ForegroundColor Cyan

# Build the project
if ($buildType -eq "release") {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed!"
        exit 1
    }
} else {
    cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed!"
        exit 1
    }
}

# Determine installation directory
$installDir = "$env:USERPROFILE\.local\bin"

# Create the directory if it doesn't exist
if (-not (Test-Path $installDir)) {
    Write-Host "Creating directory $installDir..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

Write-Host "Installing goto to $installDir..." -ForegroundColor Cyan

# Copy the executable
Copy-Item "$targetDir\goto.exe" "$installDir\goto.exe" -Force

# Copy the PowerShell wrapper script
Copy-Item "scripts\goto.ps1" "$installDir\goto-wrapper.ps1" -Force

Write-Host ""
Write-Host "goto installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Add $installDir to your PATH if it is not already there"
Write-Host "  2. Add this line to your PowerShell profile:"
Write-Host "     . $installDir\goto-wrapper.ps1" -ForegroundColor Cyan
Write-Host ""
Write-Host "Useful commands:" -ForegroundColor Yellow
Write-Host '  notepad $PROFILE' -ForegroundColor Cyan
Write-Host '  New-Item -Path $PROFILE -ItemType File -Force' -ForegroundColor Cyan
Write-Host ""
