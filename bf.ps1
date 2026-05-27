#Requires -Version 5.1
<#
.SYNOPSIS
  Download BrainForge from GitHub and run `brainforge init` in the current folder.

.EXAMPLE
  # Short URL (run inside your project folder):
  irm https://mglucas0123.github.io/BrainForge/bf.ps1 | iex

.EXAMPLE
  powershell -ExecutionPolicy Bypass -File .\bf.ps1
#>
[CmdletBinding()]
param(
    [string]$Version = "latest",
    [string]$Branch = "main",
    [string]$Repo = "mglucas0123/BrainForge",
    [switch]$Force,
    [switch]$NoMenu,
    [ValidateSet('cursor', 'copilot', 'antigravity', 'all')]
    [string]$Adapter,
    [switch]$SkipInit
)

$ErrorActionPreference = "Stop"
$script:CleanupDirs = [System.Collections.Generic.List[string]]::new()

if ($env:BRAINFORGE_VERSION) { $Version = $env:BRAINFORGE_VERSION }
if ($env:BRAINFORGE_BRANCH) { $Branch = $env:BRAINFORGE_BRANCH }
if ($env:BRAINFORGE_NO_MENU -eq "1") { $NoMenu = $true }
if ($env:BRAINFORGE_ADAPTER) { $Adapter = $env:BRAINFORGE_ADAPTER }

function Write-Info([string]$Message) {
    Write-Host "[brainforge] $Message" -ForegroundColor Cyan
}

function Write-Warn([string]$Message) {
    Write-Host "[brainforge] $Message" -ForegroundColor Yellow
}

function Get-LatestReleaseTag {
    param([string]$Repository)

    try {
        $release = Invoke-RestMethod `
            -Uri "https://api.github.com/repos/$Repository/releases/latest" `
            -Headers @{ "User-Agent" = "brainforge-install" }
        return $release.tag_name
    } catch {
        Write-Warn "Release nao encontrada ($Repository). Usando branch '$Branch' para o kit."
        return $null
    }
}

function Get-ReleaseAssetUrl {
    param(
        [string]$Repository,
        [string]$Tag,
        [string]$AssetName
    )

    $release = Invoke-RestMethod `
        -Uri "https://api.github.com/repos/$Repository/releases/tags/$Tag" `
        -Headers @{ "User-Agent" = "brainforge-install" }

    $asset = $release.assets | Where-Object { $_.name -eq $AssetName } | Select-Object -First 1
    if (-not $asset) {
        return $null
    }
    return $asset.browser_download_url
}

function Save-HttpFile {
    param(
        [string]$Url,
        [string]$Destination
    )
    Write-Info "Baixando: $Url"
    Invoke-WebRequest -Uri $Url -OutFile $Destination -UseBasicParsing
}

function Test-KitLayout {
    param([string]$KitRoot)
    Test-Path (Join-Path $KitRoot "core\BRAINFORGE.md")
}

function Install-KitFromZip {
    param(
        [string]$ZipPath,
        [string]$ProjectRoot
    )

    $extractDir = Join-Path $env:TEMP ("bf-kit-" + [guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Path $extractDir -Force | Out-Null

    try {
        Expand-Archive -Path $ZipPath -DestinationPath $extractDir -Force

        $kitSrc = Get-ChildItem -Path $extractDir -Recurse -Directory -Filter "brainforge" |
            Where-Object { Test-KitLayout $_.FullName } |
            Select-Object -First 1

        if (-not $kitSrc) {
            throw "brainforge/ nao encontrado dentro do zip."
        }

        $dest = Join-Path $ProjectRoot "brainforge"
        if (Test-Path $dest) {
            Remove-Item -LiteralPath $dest -Recurse -Force
        }
        Copy-Item -LiteralPath $kitSrc.FullName -Destination $dest -Recurse -Force
    } finally {
        if (Test-Path $extractDir) {
            Remove-Item -LiteralPath $extractDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

function Find-DevExecutable {
    param([string]$StartDir)

    if ($env:BRAINFORGE_EXE -and (Test-Path -LiteralPath $env:BRAINFORGE_EXE)) {
        return (Resolve-Path -LiteralPath $env:BRAINFORGE_EXE).Path
    }

    $cur = $StartDir
    for ($i = 0; $i -lt 6; $i++) {
        $candidate = Join-Path $cur "target\release\brainforge.exe"
        if (Test-Path -LiteralPath $candidate) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
        $parent = Split-Path $cur -Parent
        if (-not $parent -or $parent -eq $cur) { break }
        $cur = $parent
    }
    return $null
}

function Build-ExecutableFromRepo {
    param(
        [string]$RepoRoot,
        [string]$ExeDest
    )

    $cargoToml = Join-Path $RepoRoot "Cargo.toml"
    if (-not (Test-Path -LiteralPath $cargoToml)) {
        throw "Cargo.toml nao encontrado em $RepoRoot"
    }
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "Rust/cargo nao encontrado no PATH."
    }

    Write-Warn "Compilando brainforge.exe (pode demorar)..."
    Push-Location $RepoRoot
    try {
        & cargo build --release -p brainforge-cli
        if ($LASTEXITCODE -ne 0) {
            throw "cargo build falhou (exit $LASTEXITCODE)"
        }
        $built = Join-Path $RepoRoot "target\release\brainforge.exe"
        if (-not (Test-Path -LiteralPath $built)) {
            throw "Binario nao gerado em target\release\"
        }
        Copy-Item -LiteralPath $built -Destination $ExeDest -Force
        Unblock-File -LiteralPath $ExeDest -ErrorAction SilentlyContinue
    } finally {
        Pop-Location
    }
}

function Install-KitFromRepoArchive {
    param(
        [string]$Repository,
        [string]$Ref,
        [string]$ProjectRoot
    )

    $zipUrl = "https://github.com/$Repository/archive/refs/heads/$Ref.zip"
    if ($Ref -match '^v') {
        $zipUrl = "https://github.com/$Repository/archive/refs/tags/$Ref.zip"
    }

    $zipPath = Join-Path $env:TEMP ("bf-src-" + [guid]::NewGuid().ToString("N") + ".zip")
    $extractDir = Join-Path $env:TEMP ("bf-src-" + [guid]::NewGuid().ToString("N"))

    try {
        Save-HttpFile -Url $zipUrl -Destination $zipPath
        New-Item -ItemType Directory -Path $extractDir -Force | Out-Null
        Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force

        $kitSrc = Get-ChildItem -Path $extractDir -Recurse -Directory -Filter "brainforge" |
            Where-Object { Test-KitLayout $_.FullName } |
            Select-Object -First 1

        if (-not $kitSrc) {
            throw "Pasta brainforge/ nao encontrada no arquivo do GitHub."
        }

        $dest = Join-Path $ProjectRoot "brainforge"
        if (Test-Path $dest) {
            Remove-Item -LiteralPath $dest -Recurse -Force
        }
        Copy-Item -LiteralPath $kitSrc.FullName -Destination $dest -Recurse -Force

        $script:CleanupDirs.Add($extractDir) | Out-Null
        return $kitSrc.Parent.FullName
    } finally {
        if (Test-Path $zipPath) { Remove-Item -LiteralPath $zipPath -Force -ErrorAction SilentlyContinue }
    }
}

function Install-Executable {
    param(
        [string]$Repository,
        [string]$Tag,
        [string]$ProjectRoot
    )

    $exeDest = Join-Path $ProjectRoot "brainforge.exe"
    $url = Get-ReleaseAssetUrl -Repository $Repository -Tag $Tag -AssetName "brainforge.exe"
    if (-not $url) {
        throw "Asset brainforge.exe nao encontrado na release $Tag."
    }

    Save-HttpFile -Url $url -Destination $exeDest
    Unblock-File -LiteralPath $exeDest -ErrorAction SilentlyContinue

    $hashUrl = Get-ReleaseAssetUrl -Repository $Repository -Tag $Tag -AssetName "brainforge.exe.sha256"
    if ($hashUrl) {
        $hashFile = Join-Path $env:TEMP ("bf-hash-" + [guid]::NewGuid().ToString("N") + ".sha256")
        try {
            Save-HttpFile -Url $hashUrl -Destination $hashFile
            $expected = (Get-Content -LiteralPath $hashFile -Raw).Trim().Split()[0]
            $actual = (Get-FileHash -LiteralPath $exeDest -Algorithm SHA256).Hash
            if ($expected -ne $actual) {
                Remove-Item -LiteralPath $exeDest -Force
                throw "SHA256 do brainforge.exe nao confere."
            }
            Write-Info "SHA256 ok."
        } finally {
            if (Test-Path $hashFile) { Remove-Item -LiteralPath $hashFile -Force -ErrorAction SilentlyContinue }
        }
    }

    return $exeDest
}

# ── main ──────────────────────────────────────────────────────────────

$ProjectRoot = (Get-Location).Path
Write-Info "Projeto: $ProjectRoot"

$tag = if ($Version -eq "latest") { Get-LatestReleaseTag -Repository $Repo } else { if ($Version.StartsWith("v")) { $Version } else { "v$Version" } }
$kitRef = if ($tag) { $tag } else { $Branch }

$kitPath = Join-Path $ProjectRoot "brainforge"
$needKit = $Force -or -not (Test-KitLayout $kitPath)
$archiveRepoRoot = $null

if ($needKit) {
    $kitZipName = "brainforge-kit.zip"
    $kitInstalled = $false

    if ($tag) {
        $kitZipUrl = Get-ReleaseAssetUrl -Repository $Repo -Tag $tag -AssetName $kitZipName
        if ($kitZipUrl) {
            $zipPath = Join-Path $env:TEMP ("bf-kitzip-" + [guid]::NewGuid().ToString("N") + ".zip")
            try {
                Save-HttpFile -Url $kitZipUrl -Destination $zipPath
                Install-KitFromZip -ZipPath $zipPath -ProjectRoot $ProjectRoot
                $kitInstalled = $true
                Write-Info "Kit instalado (release $tag)."
            } finally {
                if (Test-Path $zipPath) { Remove-Item -LiteralPath $zipPath -Force -ErrorAction SilentlyContinue }
            }
        }
    }

    if (-not $kitInstalled) {
        Write-Warn "brainforge-kit.zip ausente; baixando fonte ($kitRef)..."
        $archiveRepoRoot = Install-KitFromRepoArchive -Repository $Repo -Ref $kitRef -ProjectRoot $ProjectRoot
        Write-Info "Kit instalado (GitHub archive)."
    }
} else {
    Write-Info "Kit ja existe em brainforge/ (use -Force para substituir)."
}

$exePath = Join-Path $ProjectRoot "brainforge.exe"
$needExe = $Force -or -not (Test-Path -LiteralPath $exePath)

if ($needExe) {
    if ($tag) {
        Install-Executable -Repository $Repo -Tag $tag -ProjectRoot $ProjectRoot
        Write-Info "CLI instalado (release $tag)."
    } else {
        $devExe = Find-DevExecutable -StartDir $ProjectRoot
        if ($devExe) {
            Copy-Item -LiteralPath $devExe -Destination $exePath -Force
            Unblock-File -LiteralPath $exePath -ErrorAction SilentlyContinue
            Write-Info "CLI copiado de build local: $devExe"
        } elseif ($archiveRepoRoot) {
            Build-ExecutableFromRepo -RepoRoot $archiveRepoRoot -ExeDest $exePath
            Write-Info "CLI compilado do fonte baixado."
        } else {
            throw @"
Nenhuma release GitHub com brainforge.exe.
Opcoes:
  1) Publique uma tag v* no repositorio (recomendado)
  2) Defina BRAINFORGE_EXE apontando para um brainforge.exe
  3) Tenha Rust no PATH e deixe o script compilar (baixa o zip do repo)
"@
        }
    }
} else {
    Write-Info "brainforge.exe ja existe (use -Force para substituir)."
}

if ($SkipInit) {
    Write-Info "SkipInit: download concluido."
    exit 0
}

$initArgs = @("init")
if ($NoMenu) {
    $initArgs += @("--adapter", "all", "--no-menu")
} elseif ($Adapter) {
    $initArgs += @("--adapter", $Adapter)
}

Write-Host ""
Write-Info "Executando: brainforge $($initArgs -join ' ')"
Write-Host ""

& $exePath @initArgs
$code = $LASTEXITCODE

foreach ($dir in $script:CleanupDirs) {
    if (Test-Path -LiteralPath $dir) {
        Remove-Item -LiteralPath $dir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

exit $code
