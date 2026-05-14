$ErrorActionPreference = "Stop"

$Repo = "EzyGang/kaneo-cli"
$BinName = "kaneo-cli"

function Get-Platform {
    try {
        $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
        switch ($arch) {
            "X64"  { return "win32-x64" }
            "Arm64" { Write-Error "Windows ARM64 is not yet supported"; exit 1 }
            default { Write-Error "Unsupported architecture: $arch"; exit 1 }
        }
    } catch {
        if ([Environment]::Is64BitOperatingSystem) {
            return "win32-x64"
        }
        Write-Error "Unsupported platform: 32-bit Windows is not supported"; exit 1
    }
}

function Get-LatestVersion {
    $url = "https://api.github.com/repos/$Repo/releases/latest"
    $release = Invoke-RestMethod -Uri $url -Headers @{ "User-Agent" = "kaneo-installer" }
    return $release.tag_name
}

$Platform = Get-Platform
Write-Host "Detected platform: $Platform" -ForegroundColor Cyan

Write-Host "Fetching latest version..." -ForegroundColor Cyan
$Version = Get-LatestVersion
if (-not $Version) {
    Write-Error "Could not determine latest version"
    exit 1
}
Write-Host "Latest version: $Version" -ForegroundColor Green

$Url = "https://github.com/$Repo/releases/download/$Version/$BinName-$Platform.tar.gz"
Write-Host "Downloading $Url..." -ForegroundColor Cyan

$TmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Path $TmpDir -Force | Out-Null

try {
    $Archive = Join-Path $TmpDir "$BinName.tar.gz"
    Invoke-WebRequest -Uri $Url -OutFile $Archive -UseBasicParsing

    tar -xzf $Archive -C $TmpDir

    $Binary = Join-Path $TmpDir "$BinName.exe"
    if (-not (Test-Path $Binary)) {
        Write-Error "Binary not found in archive"
        exit 1
    }

    $InstallDir = Join-Path $env:LOCALAPPDATA "Programs\kaneo-cli"
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $Dest = Join-Path $InstallDir "$BinName.exe"
    Move-Item -Path $Binary -Destination $Dest -Force
    Write-Host "Installed $BinName to $Dest" -ForegroundColor Green

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
        Write-Host "Added $InstallDir to user PATH" -ForegroundColor Yellow
        Write-Host "Restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
    }

    Write-Host ""
    Write-Host "Run '$BinName --help' to get started." -ForegroundColor Cyan
}
finally {
    Remove-Item -Path $TmpDir -Recurse -Force -ErrorAction SilentlyContinue
}
