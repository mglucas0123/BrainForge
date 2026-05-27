#Requires -Version 5.1
<#
.DEPRECATED
  Use Rust CLI: brainforge sync -a all --no-menu
  See brainforge/HOST-SETUP.md

.SYNOPSIS
  Sync BrainForge core → Cursor, Copilot, Antigravity adapters.

.EXAMPLE
  .\brainforge\sync.ps1                    # menu interativo
  .\brainforge\sync.ps1 -Adapter cursor    # sem menu
  .\brainforge\sync.ps1 -Adapter all -NoMenu
  .\brainforge\sync.ps1 -Target D:\meu-app
#>
[CmdletBinding()]
param(
    [string]$Target = (Get-Location).Path,
    [ValidateSet('cursor', 'copilot', 'antigravity', 'all')]
    [string[]]$Adapter,
    [switch]$NoMenu
)

$ErrorActionPreference = 'Stop'
$KitRoot = $PSScriptRoot
$Core = Join-Path $KitRoot 'core'
$Memory = Join-Path $KitRoot 'memory'

function Ensure-Dir([string]$Path) {
    if (-not (Test-Path $Path)) { New-Item -ItemType Directory -Path $Path -Force | Out-Null }
}

function Copy-Tree([string]$From, [string]$To) {
    if (-not (Test-Path $From)) { return }
    Ensure-Dir $To
    Copy-Item -Path (Join-Path $From '*') -Destination $To -Recurse -Force
}

function Show-AdapterMenu {
    $menuRows = @(
        [PSCustomObject]@{
            Adapter = 'cursor'
            IDE     = 'Cursor'
            Gera    = '.cursor/ — skills, /brainforge, rules, RTK'
        }
        [PSCustomObject]@{
            Adapter = 'copilot'
            IDE     = 'GitHub Copilot (VS Code)'
            Gera    = '.github/copilot-instructions.md'
        }
        [PSCustomObject]@{
            Adapter = 'antigravity'
            IDE     = 'Antigravity'
            Gera    = '.agents/ — rules + workflows (/brainforge)'
        }
        [PSCustomObject]@{
            Adapter = 'all'
            IDE     = 'Todos'
            Gera    = 'Cursor + Copilot + Antigravity'
        }
    )

    Write-Host ''
    Write-Host '  BrainForge Sync' -ForegroundColor Cyan
    Write-Host '  ---------------' -ForegroundColor DarkGray
    Write-Host '  Abrindo seletor... (Ctrl+clique = varios; Esc = cancelar)' -ForegroundColor DarkGray
    Write-Host ''

    $gridCmd = Get-Command Out-GridView -ErrorAction SilentlyContinue
    $gridMulti = $gridCmd -and $gridCmd.Parameters.ContainsKey('AllowMultiple')

    if ($gridCmd) {
        if ($gridMulti) {
            $picked = $menuRows | Out-GridView -Title 'BrainForge — escolha a(s) IDE(s)' -PassThru -AllowMultiple
        } else {
            Write-Host '  (PowerShell 5.1: menu no terminal; PS 7.4+ usa grade visual com multi-selecao)' -ForegroundColor DarkGray
            return Show-AdapterMenuConsole
        }
        if (-not $picked) {
            Write-Host '  Cancelado.' -ForegroundColor Yellow
            exit 0
        }
        if ($picked.Adapter -contains 'all') { return @('all') }
        return @($picked.Adapter | Select-Object -Unique)
    }

    return Show-AdapterMenuConsole
}

function Show-AdapterMenuConsole {
    $lines = @(
        @{ N = 1; Adapter = 'cursor';      Label = 'Cursor';      Gera = '.cursor/  (skills, /brainforge, RTK)' }
        @{ N = 2; Adapter = 'copilot';     Label = 'Copilot';     Gera = '.github/copilot-instructions.md' }
        @{ N = 3; Adapter = 'antigravity'; Label = 'Antigravity'; Gera = '.agents/  (/brainforge workflows)' }
        @{ N = 4; Adapter = 'all';         Label = 'Todos';       Gera = 'Cursor + Copilot + Antigravity' }
    )

    Write-Host '  +----------------------------------------------+' -ForegroundColor DarkCyan
    Write-Host '  |  BrainForge Sync - escolha IDE(s)             |' -ForegroundColor Cyan
    Write-Host '  +----------------------------------------------+' -ForegroundColor DarkCyan
    foreach ($row in $lines) {
        Write-Host ('  |  [{0}] {1,-12}  {2}' -f $row.N, $row.Label, $row.Gera) -ForegroundColor White
    }
    Write-Host '  +----------------------------------------------+' -ForegroundColor DarkCyan
    Write-Host '  |  Ex.: 1   ou   1,3   ou   4   |  Enter = Todos |' -ForegroundColor DarkGray
    Write-Host '  +----------------------------------------------+' -ForegroundColor DarkCyan
    Write-Host ''
    $raw = Read-Host '  Escolha'

    if ([string]::IsNullOrWhiteSpace($raw)) { return @('all') }

    $nums = $raw -split '[,\s;]+' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne '' }
    $adapters = [System.Collections.Generic.List[string]]::new()

    foreach ($n in $nums) {
        switch ($n) {
            '1' { [void]$adapters.Add('cursor') }
            '2' { [void]$adapters.Add('copilot') }
            '3' { [void]$adapters.Add('antigravity') }
            '4' { return @('all') }
            'all' { return @('all') }
            'cursor' { [void]$adapters.Add('cursor') }
            'copilot' { [void]$adapters.Add('copilot') }
            'antigravity' { [void]$adapters.Add('antigravity') }
            default {
                Write-Host "  Opcao invalida: $n" -ForegroundColor Red
                exit 1
            }
        }
    }

    if ($adapters.Count -eq 0) { return @('all') }
    return @($adapters | Select-Object -Unique)
}

function Resolve-Adapters {
    if ($PSBoundParameters.ContainsKey('Adapter') -and $Adapter.Count -gt 0) {
        return @($Adapter)
    }
    if ($NoMenu) {
        return @('all')
    }
    return Show-AdapterMenu
}

function Invoke-AdapterSync([string]$Name) {
    switch ($Name) {
        'cursor' { Sync-CursorKit }
        'copilot' { Sync-CopilotKit }
        'antigravity' { Sync-AntigravityKit }
        'all' {
            Sync-CursorKit
            Sync-CopilotKit
            Sync-AntigravityKit
        }
        default { throw "Adapter desconhecido: $Name" }
    }
}

function Sync-CursorKit {
    $cursor = Join-Path $Target '.cursor'
    Ensure-Dir $cursor
    Ensure-Dir (Join-Path $cursor 'skills')
    Ensure-Dir (Join-Path $cursor 'commands')
    Ensure-Dir (Join-Path $cursor 'rules')
    Ensure-Dir (Join-Path $cursor 'project')
    Ensure-Dir (Join-Path $cursor 'docs')

    if (Test-Path (Join-Path $Core 'skills')) {
        Remove-Item (Join-Path $cursor 'skills\*') -Recurse -Force -ErrorAction SilentlyContinue
        Copy-Item (Join-Path $Core 'skills\*') (Join-Path $cursor 'skills') -Recurse -Force
    }
    if (Test-Path (Join-Path $Core 'skills-optional')) {
        Ensure-Dir (Join-Path $cursor 'skills-optional')
        Copy-Tree (Join-Path $Core 'skills-optional') (Join-Path $cursor 'skills-optional')
    }
    Copy-Tree (Join-Path $Core 'docs') (Join-Path $cursor 'docs')
    foreach ($f in @('skills-catalog.json', 'installed-skills.json')) {
        $src = Join-Path $Core $f
        if (Test-Path $src) { Copy-Item $src (Join-Path $cursor $f) -Force }
    }

    Copy-Tree (Join-Path $Core 'commands') (Join-Path $cursor 'commands')

    $ruleSrc = Join-Path $KitRoot 'adapters\cursor\rules'
    if (Test-Path $ruleSrc) {
        Copy-Item (Join-Path $ruleSrc '*') (Join-Path $cursor 'rules') -Force
    }

    foreach ($m in @('.context.md', '.user.md')) {
        $src = Join-Path $Memory $m
        if (Test-Path $src) {
            Copy-Item $src (Join-Path (Join-Path $cursor 'project') $m) -Force
        }
    }

    $rtkExe = Join-Path $KitRoot 'tools\rtk\rtk.exe'
    if (-not (Test-Path $rtkExe)) {
        Write-Warning '[cursor] RTK missing at brainforge/tools/rtk/rtk.exe - run: powershell -ExecutionPolicy Bypass -File .\brainforge\tools\rtk\install-rtk-local.ps1 -Force'
    }

    Write-Host '[cursor] .cursor/ synced' -ForegroundColor Green
}

function Sync-CopilotKit {
    $github = Join-Path $Target '.github'
    Ensure-Dir $github
    $src = Join-Path $KitRoot 'adapters\copilot\copilot-instructions.md'
    if (Test-Path $src) {
        Copy-Item $src (Join-Path $github 'copilot-instructions.md') -Force
        Write-Host '[copilot] .github/copilot-instructions.md' -ForegroundColor Green
    }
}

function Sync-AntigravityKit {
    $agents = Join-Path $Target '.agents'
    Ensure-Dir (Join-Path $agents 'rules')
    Ensure-Dir (Join-Path $agents 'workflows')
    $ruleSrc = Join-Path $KitRoot 'adapters\antigravity\rules'
    $wfSrc = Join-Path $KitRoot 'adapters\antigravity\workflows'
    if (Test-Path $ruleSrc) { Copy-Item (Join-Path $ruleSrc '*') (Join-Path $agents 'rules') -Force }
    if (Test-Path $wfSrc) { Copy-Item (Join-Path $wfSrc '*') (Join-Path $agents 'workflows') -Force }
    Write-Host '[antigravity] .agents/ synced' -ForegroundColor Green
}

if (-not (Test-Path $Core)) {
    throw "brainforge/core not found under $KitRoot"
}

$Target = (Resolve-Path $Target).Path
$selected = Resolve-Adapters

if ($selected -contains 'all') {
    $toRun = @('all')
} else {
    $toRun = $selected
}

Write-Host ''
Write-Host "BrainForge sync -> $Target" -ForegroundColor Cyan
Write-Host ("Adaptadores: {0}" -f ($toRun -join ', ')) -ForegroundColor DarkGray
Write-Host ''

foreach ($name in $toRun) {
    Invoke-AdapterSync -Name $name
}

Write-Host ''
Write-Host 'Concluido.' -ForegroundColor Cyan
