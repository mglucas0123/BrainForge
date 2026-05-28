#Requires -Version 5.1
<#
.SYNOPSIS
  Download BrainForge from GitHub and run `brainforge init` in the current folder.

.EXAMPLE
  # Short URL (run inside your project folder):
  iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)

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
    [string]$Adapter,
    [switch]$SkipInit
)

$ErrorActionPreference = "Stop"
$script:CleanupDirs = [System.Collections.Generic.List[string]]::new()
$script:InteractiveBootstrap = (-not $NoMenu) -and (-not $Adapter) -and (-not $SkipInit)

if ($env:BRAINFORGE_VERSION) { $Version = $env:BRAINFORGE_VERSION }
if ($env:BRAINFORGE_BRANCH) { $Branch = $env:BRAINFORGE_BRANCH }
if ($env:BRAINFORGE_NO_MENU -eq "1") { $NoMenu = $true }
if ($env:BRAINFORGE_ADAPTER) { $Adapter = $env:BRAINFORGE_ADAPTER }

if ([string]::IsNullOrWhiteSpace($Adapter)) {
    $Adapter = $null
} elseif ($Adapter -notin @('cursor', 'copilot', 'antigravity', 'all')) {
    throw "Adapter invalido: '$Adapter'. Use: cursor, copilot, antigravity ou all."
}

function Write-Info([string]$Message) {
    if ($script:InteractiveBootstrap) { return }
    Write-Host "[brainforge] $Message" -ForegroundColor Cyan
}

function Write-Warn([string]$Message) {
    if ($script:InteractiveBootstrap) { return }
    Write-Host "[brainforge] $Message" -ForegroundColor Yellow
}

function Write-BootstrapProgress([string]$Message) {
    if (-not $script:InteractiveBootstrap) {
        Write-Info $Message
        return
    }
    $line = "[brainforge] $Message"
    if ($line.Length -gt 78) { $line = $line.Substring(0, 78) }
    Write-Host ("`r{0,-78}" -f $line) -NoNewline -ForegroundColor DarkGray
}

function Finish-BootstrapProgress() {
    if ($script:InteractiveBootstrap) {
        Write-Host ""
    }
}

function Get-DirectReleaseAssetUrl {
    param(
        [string]$Repository,
        [string]$Tag,
        [string]$AssetName
    )
    return "https://github.com/$Repository/releases/download/$Tag/$AssetName"
}

function Test-ReleaseAssetExists {
    param([string]$Url)
    try {
        Invoke-WebRequest -Uri $Url -Method Head -UseBasicParsing | Out-Null
        return $true
    } catch {
        return $false
    }
}

function Resolve-ReleaseTag {
    param(
        [string]$Repository,
        [string]$Version
    )

    if ($Version -ne "latest") {
        return if ($Version.StartsWith("v")) { $Version } else { "v$Version" }
    }

    try {
        $release = Invoke-RestMethod `
            -Uri "https://api.github.com/repos/$Repository/releases/latest" `
            -Headers @{ "User-Agent" = "brainforge-install" }
        if ($release.tag_name) {
            return $release.tag_name
        }
    } catch {
        Write-Warn "API GitHub indisponivel (rate limit?). Tentando releases diretas..."
    }

    foreach ($candidate in @("v1.0.4", "v1.0.3", "v1.0.2", "v1.0.1", "v1.0.0")) {
        $probe = Get-DirectReleaseAssetUrl -Repository $Repository -Tag $candidate -AssetName "brainforge.exe"
        if (Test-ReleaseAssetExists -Url $probe) {
            Write-Info "Release detectada: $candidate"
            return $candidate
        }
    }

    Write-Warn "Nenhuma release com brainforge.exe; usando branch '$Branch' para o kit."
    return $null
}

function Get-ReleaseAssetUrl {
    param(
        [string]$Repository,
        [string]$Tag,
        [string]$AssetName
    )

    try {
        $release = Invoke-RestMethod `
            -Uri "https://api.github.com/repos/$Repository/releases/tags/$Tag" `
            -Headers @{ "User-Agent" = "brainforge-install" }

        $asset = $release.assets | Where-Object { $_.name -eq $AssetName } | Select-Object -First 1
        if ($asset) {
            return $asset.browser_download_url
        }
    } catch {
        Write-Warn "API assets falhou para $Tag; URL direta."
    }

    $direct = Get-DirectReleaseAssetUrl -Repository $Repository -Tag $Tag -AssetName $AssetName
    if (Test-ReleaseAssetExists -Url $direct) {
        return $direct
    }
    return $null
}

function Save-HttpFile {
    param(
        [string]$Url,
        [string]$Destination
    )
    $name = Split-Path $Url -Leaf
    if ([string]::IsNullOrWhiteSpace($name)) { $name = "arquivo" }
    Write-BootstrapProgress "Baixando $name..."
    Invoke-WebRequest -Uri $Url -OutFile $Destination -UseBasicParsing
}

function Test-KitLayout {
    param([string]$KitRoot)
    Test-Path (Join-Path $KitRoot "core\BRAINFORGE.md")
}

function Get-HostKitPath {
    param([string]$ProjectRoot)
    foreach ($name in @('.brainforge', 'brainforge')) {
        $p = Join-Path $ProjectRoot $name
        if (Test-KitLayout $p) {
            return (Resolve-Path -LiteralPath $p).Path
        }
    }
    return $null
}

function Remove-LegacyHostKitFolder {
    param([string]$ProjectRoot)
    $modern = Join-Path $ProjectRoot '.brainforge'
    $legacy = Join-Path $ProjectRoot 'brainforge'
    if (-not (Test-KitLayout $modern)) { return }
    if (-not (Test-KitLayout $legacy)) { return }
    try {
        $m = (Resolve-Path -LiteralPath $modern).Path
        $l = (Resolve-Path -LiteralPath $legacy).Path
        if ($m -eq $l) { return }
    } catch { }
    Write-Info "Removendo pasta legada brainforge/ (canônico: .brainforge/)."
    Remove-Item -LiteralPath $legacy -Recurse -Force
}

function Find-KitFolder {
    param([string]$SearchRoot)
    Get-ChildItem -Path $SearchRoot -Recurse -Directory -ErrorAction SilentlyContinue |
        Where-Object {
            ($_.Name -eq '.brainforge' -or $_.Name -eq 'brainforge') -and (Test-KitLayout $_.FullName)
        } |
        Select-Object -First 1
}

function Install-KitToHost {
    param(
        [string]$KitSrcPath,
        [string]$ProjectRoot
    )
    $dest = Join-Path $ProjectRoot ".brainforge"
    $legacy = Join-Path $ProjectRoot "brainforge"
    if (Test-Path $dest) { Remove-Item -LiteralPath $dest -Recurse -Force }
    if (Test-Path $legacy) { Remove-Item -LiteralPath $legacy -Recurse -Force }
    Copy-Item -LiteralPath $KitSrcPath -Destination $dest -Recurse -Force
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

        $kitSrc = Find-KitFolder -SearchRoot $extractDir

        if (-not $kitSrc) {
            throw ".brainforge/ nao encontrado dentro do zip."
        }

        Install-KitToHost -KitSrcPath $kitSrc.FullName -ProjectRoot $ProjectRoot
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

        $kitSrc = Find-KitFolder -SearchRoot $extractDir

        if (-not $kitSrc) {
            throw "Pasta .brainforge/ nao encontrada no arquivo do GitHub."
        }

        Install-KitToHost -KitSrcPath $kitSrc.FullName -ProjectRoot $ProjectRoot

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

}

# ── main ──────────────────────────────────────────────────────────────

$ProjectRoot = (Get-Location).Path
if (-not $script:InteractiveBootstrap) {
    Write-Info "Projeto: $ProjectRoot"
}

$tag = Resolve-ReleaseTag -Repository $Repo -Version $Version
$kitRef = if ($tag) { $tag } else { $Branch }

$kitPath = Join-Path $ProjectRoot ".brainforge"
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
                Write-BootstrapProgress "Kit instalado ($tag)."
                Finish-BootstrapProgress
                if (-not $script:InteractiveBootstrap) {
                    Write-Info "Kit instalado (release $tag)."
                }
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
    Write-Info "Kit ja existe em .brainforge/ (use -Force para substituir)."
}

$exePath = Join-Path $ProjectRoot "brainforge.exe"
$needExe = $Force -or -not (Test-Path -LiteralPath $exePath)

if ($needExe) {
    if ($tag) {
        [void](Install-Executable -Repository $Repo -Tag $tag -ProjectRoot $ProjectRoot)
        Write-BootstrapProgress "CLI pronto ($tag)."
        Finish-BootstrapProgress
        if (-not $script:InteractiveBootstrap) {
            Write-Info "CLI instalado (release $tag)."
        }
        if ($env:BRAINFORGE_USE_DEV_EXE -eq "1") {
            $devExe = Find-DevExecutable -StartDir $ProjectRoot
            if ($devExe) {
                Copy-Item -LiteralPath $devExe -Destination $exePath -Force
                Unblock-File -LiteralPath $exePath -ErrorAction SilentlyContinue
                Write-Info "CLI substituido por build local: $devExe"
            }
        }
    } else {
        $devExe = if ($env:BRAINFORGE_USE_DEV_EXE -eq "1") { Find-DevExecutable -StartDir $ProjectRoot } else { $null }
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

# v1.0.0 CLI only discovers `brainforge/`; host kit is `.brainforge/` after bf.ps1.
$hostKit = Get-HostKitPath -ProjectRoot $ProjectRoot
if ($hostKit) {
    $env:BRAINFORGE_KIT = $hostKit
    $initArgs = @("--kit", $hostKit) + $initArgs
}

if ($script:InteractiveBootstrap) {
    Finish-BootstrapProgress
    Clear-Host
    $env:BRAINFORGE_WELCOME = "1"
} else {
    Write-Host ""
    Write-Info "Executando: brainforge $($initArgs -join ' ')"
    Write-Host ""
}

& $exePath @initArgs
$code = $LASTEXITCODE
Remove-Item Env:BRAINFORGE_WELCOME -ErrorAction SilentlyContinue

Remove-LegacyHostKitFolder -ProjectRoot $ProjectRoot

foreach ($dir in $script:CleanupDirs) {
    if (Test-Path -LiteralPath $dir) {
        Remove-Item -LiteralPath $dir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

exit $code
