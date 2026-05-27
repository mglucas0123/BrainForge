# Install/update rtk.exe in brainforge/tools/rtk (único local — não espelha em .cursor).
# Usage:
#   powershell -ExecutionPolicy Bypass -File .\brainforge\tools\rtk\install-rtk-local.ps1
#   powershell -ExecutionPolicy Bypass -File .\brainforge\tools\rtk\install-rtk-local.ps1 -Version 0.39.0 -Force
param(
    [string]$Version = "latest",
    [switch]$Force
)

$ErrorActionPreference = "Stop"

function Get-AssetDownload {
    param(
        [string]$VersionTag,
        [string]$AssetName
    )

    if ($VersionTag -eq "latest") {
        $apiUrl = "https://api.github.com/repos/rtk-ai/rtk/releases/latest"
        $release = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "brainforge-rtk-install" }
        $asset = $release.assets | Where-Object { $_.name -eq $AssetName } | Select-Object -First 1
        if (-not $asset) {
            throw "Asset '$AssetName' not found in latest release."
        }

        return [pscustomobject]@{
            Tag = $release.tag_name
            Url = $asset.browser_download_url
        }
    }

    $tag = if ($VersionTag.StartsWith("v")) { $VersionTag } else { "v$VersionTag" }
    $url = "https://github.com/rtk-ai/rtk/releases/download/$tag/$AssetName"
    return [pscustomobject]@{
        Tag = $tag
        Url = $url
    }
}

$assetName = "rtk-x86_64-pc-windows-msvc.zip"
$download = Get-AssetDownload -VersionTag $Version -AssetName $assetName

$zipPath = Join-Path $env:TEMP ("rtk-local-" + [guid]::NewGuid().ToString("N") + ".zip")
$extractDir = Join-Path $env:TEMP ("rtk-local-" + [guid]::NewGuid().ToString("N"))
$targetPath = Join-Path $PSScriptRoot "rtk.exe"

Write-Host "RTK release: $($download.Tag)"
Write-Host "Download: $($download.Url)"

New-Item -ItemType Directory -Path $extractDir -Force | Out-Null

try {
    Invoke-WebRequest -Uri $download.Url -OutFile $zipPath
    Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force

    $rtkBinary = Get-ChildItem -Path $extractDir -Recurse -File -Filter "rtk.exe" | Select-Object -First 1
    if (-not $rtkBinary) {
        throw "rtk.exe not found after zip extraction."
    }

    if ((Test-Path $targetPath) -and -not $Force) {
        throw "Already exists '$targetPath'. Use -Force to overwrite."
    }

    Copy-Item -LiteralPath $rtkBinary.FullName -Destination $targetPath -Force
    Unblock-File -Path $targetPath -ErrorAction SilentlyContinue

    $versionOut = & $targetPath --version 2>&1
    Write-Host "Installed: $versionOut"
    Write-Host "Path: .\brainforge\tools\rtk\rtk.exe"
} finally {
    if (Test-Path $zipPath) {
        Remove-Item -LiteralPath $zipPath -Force -ErrorAction SilentlyContinue
    }
    if (Test-Path $extractDir) {
        Remove-Item -LiteralPath $extractDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
