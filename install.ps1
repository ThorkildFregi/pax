$ErrorActionPreference = "Stop"

Write-Host "Pax Language Installer for Windows" -ForegroundColor Cyan
Write-Host "---------------------------"

$repo = "ThorkildFregi/pax"
$assetName = "pax-windows-x64.exe"
$installDir = "$env:USERPROFILE\.pax"
$binPath = "$installDir\pax.exe"

$releaseApi = "https://api.github.com/repos/$repo/releases/latest"
Write-Host "Checking for latest release..."
$releaseInfo = Invoke-RestMethod -Uri $releaseApi

$asset = $releaseInfo.assets | Where-Object { $_.name -like "*$assetName*" } | Select-Object -First 1

if (-not $asset) {
    Write-Error "Could not find a Windows binary in the latest release."
    exit
}

$downloadUrl = $asset.browser_download_url

if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
    Write-Host "Created installation directory at $installDir"
}

Write-Host "Downloading Pax..."
Invoke-WebRequest -Uri $downloadUrl -OutFile $binPath

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    Write-Host "Adding Pax to User PATH..."
    $newPath = "$currentPath;$installDir"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = $newPath
}

Write-Host "---------------------------" -ForegroundColor Cyan
Write-Host "Pax has been installed with success!" -ForegroundColor Green
Write-Host "Please restart your terminal and type 'pax --version' to test."