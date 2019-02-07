$CurrentIdentity = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
if (-NOT $CurrentIdentity.IsInRole([Security.Principal.WindowsBuiltInRole] 'Administrator'))
{
    Write-Host 'This script must be run with Administrative permissions on Windows. Attempting to re-start with required permissions...'
    $args = "-NoExit -File `"$PSCommandPath`" "
    Start-Process -FilePath powershell.exe -WorkingDirectory $PSScriptRoot -Verb RunAs -ArgumentList $args
    Exit
}

if (-NOT (Get-Command 'choco' -ErrorAction SilentlyContinue))
{
    Write-Host 'Chocolatey NuGet not found. Installing now...'
    Set-ExecutionPolicy Bypass -Scope Process -Force
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
    RefreshEnv
}

if (-NOT (Get-Command 'rustc' -ErrorAction SilentlyContinue))
{
    Write-Host 'Rust not found. Installing now...'
    cinst -y --timeout 14400 --force visualstudio2017-workload-vctools
    $rustupExecutable = "$($env:TEMP)\rustup-init.exe"

    if (-NOT (Test-Path $rustupExecutable))
    {
        Invoke-WebRequest -Uri 'https://win.rustup.rs' -Method 'GET' -OutFile $rustupExecutable
    }

    & $rustupExecutable -y

    $cargoBinPath = "$($env:USERPROFILE)\.cargo\bin"
    if (-NOT ([Regex]::escape($env:Path) -like "*$([Regex]::escape($cargoBinPath))*"))
    {
        [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$cargoBinPath;", [System.EnvironmentVariableTarget]::User)
    }

    RefreshEnv
}

if (-NOT (Get-Command 'cargo-clippy' -ErrorAction SilentlyContinue))
{
    Write-Host 'Clippy not found. Installing now...'
    rustup component add clippy
    RefreshEnv
}

if (-NOT (Get-Command 'rusty-hook' -ErrorAction SilentlyContinue))
{
    Write-Host 'rusty-hook not found. Installing now...'
    cargo install --path (Get-Item $PSScriptRoot).Parent.FullName
    RefreshEnv
}

Write-Host "Environment successfully configured!"
