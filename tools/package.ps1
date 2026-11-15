param([switch]$SkipBuild)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $projectRoot
try {
    if (-not $SkipBuild) {
        cargo build --release --locked
        if ($LASTEXITCODE -ne 0) { throw 'Release build failed.' }
    }
    $executable = Join-Path $projectRoot 'target\release\little-timber.exe'
    if (-not (Test-Path -LiteralPath $executable)) { throw 'Build the release executable first.' }
    $package = Join-Path $projectRoot 'dist\LittleTimber'
    New-Item -ItemType Directory -Path $package -Force | Out-Null
    Copy-Item -LiteralPath $executable -Destination (Join-Path $package 'LittleTimber.exe') -Force
    Copy-Item -LiteralPath (Join-Path $projectRoot 'README.md') -Destination $package -Force
    $packageDocs = Join-Path $package 'docs'
    New-Item -ItemType Directory -Path $packageDocs -Force | Out-Null
    Get-ChildItem -LiteralPath (Join-Path $projectRoot 'docs') -Filter '*.md' -File |
        Copy-Item -Destination $packageDocs -Force
    $packageArt = Join-Path $package 'assets\woodland'
    New-Item -ItemType Directory -Path $packageArt -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $projectRoot 'assets\woodland\house.png') -Destination $packageArt -Force
    $archive = Join-Path $projectRoot 'dist\LittleTimber-0.1.0-windows-x64.zip'
    Compress-Archive -LiteralPath $package -DestinationPath $archive -Force
    Get-Item -LiteralPath (Join-Path $package 'LittleTimber.exe'),$archive | Select-Object FullName,Length
} finally {
    Pop-Location
}
