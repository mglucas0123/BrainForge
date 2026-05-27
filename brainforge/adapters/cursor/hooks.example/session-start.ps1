# BrainForge sessionStart hook (example)
# Copy hooks.example → .cursor/hooks and edit paths before enabling.
# See brainforge/core/docs/CURSOR-HOOKS.md

$ErrorActionPreference = "SilentlyContinue"
$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$bf = Join-Path $root "brainforge.exe"
if (-not (Test-Path $bf)) { $bf = "brainforge" }

& $bf memory refresh --file context 2>$null | Out-Null
