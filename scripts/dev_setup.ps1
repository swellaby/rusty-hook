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
    Write-Host 'Rust not found...'
    Write-Host "Checking for pre-requisite C++ build tools..."

    # Check for C++ build tool pre-requisites using vswhere since the PS Module and Com approach are
    # too fragile in proxy environments
    # https://devblogs.microsoft.com/cppblog/finding-the-visual-c-compiler-tools-in-visual-studio-2017/
    if (-NOT (Get-Command 'vswhere' -ErrorAction SilentlyContinue))
    {
        Write-Host "Installing vswhere tool to scan for C++ build tools..."
        cinst -y vswhere
    }

    $product = vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if (!$product)
    {
        Write-Host "C++ build tools not found. Starting install now. This might take a while..."
        cinst -y --timeout 14400 --force visualstudio2017-workload-vctools
        Write-Host "C++ build tools install finished. You should restart your machine to complete the install, and then re-run this script. Script will exit in 15 seconds..."
        Start-Sleep -Seconds 15
        Exit
    }
    # Set-PSRepository -Name PSGallery -InstallationPolicy Trusted
    # Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force
    # Install-Module VSSetup -Scope CurrentUser
    # $installationPaths = Get-VSSetupInstance | Select -ExpandProperty InstallationPath
    # foreach ($installationPath in $installationPaths)
    # {
    #     if ((Get-ChildItem -Recurse -Path "$installationPath" -Filter cl.exe | Select-Object -First 1))
    #     {
    #         $buildToolsInstalled = $true
    #         break
    #     }
    # }

    # if (!$buildToolsInstalled)
    # {
    #     Write-Host "C++ build tools not found. Starting install now. This might take a while..."
    #     cinst -y --timeout 14400 --force visualstudio2017-workload-vctools
    #     Write-Host "C++ build tools install finished. You should restart your machine to complete the install, and then re-run this script. Script will exit in 15 seconds..."
    #     Start-Sleep -Seconds 15
    #     Exit
    # }

    Write-Host "Pre-requisite C++ build tools detected..."
    Write-Host "Installing Rust now..."

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
cd (Get-Item $PSScriptRoot).Parent.FullName
rusty-hook init
cargo test --bin rusty-hook
