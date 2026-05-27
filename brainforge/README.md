# BrainForge — fonte única

Kit portátil: edite aqui, rode `sync.ps1`, use `/brainforge` no Cursor (e adaptadores em outros IDEs).

## Layout

```
brainforge/
  core/           BRAINFORGE.md, caveman.md, cavemem.md, commands/, skills/, docs/
  memory/         .context.md + .user.md (por projeto — apague ao copiar kit)
  tools/rtk/      rtk.exe + install-rtk-local.ps1 (canônico)
  adapters/       templates Cursor, Copilot, Antigravity
  sync.ps1        gera .cursor/, .github/, .agents/ (RTK fica só em tools/rtk/)
```

## Uso rápido (Rust CLI — recomendado)

```powershell
cargo build --release
.\target\release\brainforge.exe sync          # menu interativo
.\target\release\brainforge.exe sync -a all --no-menu
.\target\release\brainforge.exe doctor
```

## Legado: sync.ps1

```powershell
.\brainforge\sync.ps1   # substituído pelo CLI Rust

# Outro diretório
.\brainforge\sync.ps1 -Target C:\meu-app
```

## Copiar para outro projeto

1. Copie a pasta **`brainforge/`** inteira (inclui `tools/rtk/rtk.exe` após install).
2. Apague **`brainforge/memory/.context.md`** (e `.user.md` se prefs forem outras).
3. Rode **`.\brainforge\sync.ps1 -Adapter all`**.
4. Cursor: `/brainforge` · Antigravity: `/brainforge` · Copilot: rule em `.github/` + chat “BrainForge on”.

## Memória

- **Canônico:** `brainforge/memory/`
- **Espelho Cursor:** `.cursor/project/` (gerado pelo sync; não editar à mão)

## Rust / MCP

Plano futuro: `.cursor/plans/2026-05-27-brainforge-rust-cli-mcp.md`. Este layout é a base.
