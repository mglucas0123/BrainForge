# BrainForge (host bridge)

Thin pointer for tools that read **`AGENTS.md`** at the repo root (e.g. Copilot, some agents).

## Canonical sources

| What | Path |
|------|------|
| Project memory | `brainforge/memory/.context.md`, `brainforge/memory/.user.md` |
| Routine | `brainforge/core/BRAINFORGE.md` |
| CLI | `brainforge.exe` or `brainforge` on PATH |

## Quick start

```powershell
brainforge doctor
brainforge sync -a all --no-menu
```

MCP (optional): `brainforge/core/docs/MCP-SETUP.md` — tools `brainforge_routine`, `memory_read`.

Do not duplicate long rules here — keep § memory in `brainforge/memory/`.
