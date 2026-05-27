# BrainForge Rust — fases de execução

Created: 2026-05-27  
Updated: 2026-05-27

**Este arquivo é o plano operacional** (o que rodar, o que falta).  
Arquitetura / slash / decisões longas: [`.cursor/plans/2026-05-27-brainforge-rust-cli-mcp.md`](../../../.cursor/plans/2026-05-27-brainforge-rust-cli-mcp.md).

---

## Estado atual

| Item | Status |
|------|--------|
| Hub `brainforge/` (core, memory, adapters, RTK) | ✅ |
| Workspace `brainforge-core` + `brainforge-cli` + `brainforge-mcp` | ✅ |
| Fases 0–5 (CLI + MCP + install + polish) | ✅ |
| Fase 6 (recall, security, AGENTS.md, hooks, embed) | ✅ |
| Skills Hub remoto | ❌ backlog |

**Build:**

```powershell
cargo build --release
.\target\release\brainforge.exe --help
```

---

## Notas (changelog)

| Data | Fase | Resumo |
|------|------|--------|
| 2026-05-27 | 0–1 | Workspace, sync, doctor, paths |
| 2026-05-27 | 2 + 2b | memory.rs, P0/P1, validate |
| 2026-05-27 | 3 | brainforge-mcp, MCP-SETUP, rule MCP |
| 2026-05-27 | 4 | install, toml, CI, release workflow, HOST-SETUP |
| 2026-05-27 | 5 | skill install, prompt --copy, doctor→CLI, version+git |
| 2026-05-27 | 6 | recall, secret scan, AGENTS.md, hooks.example, embed commands |

---

## Fase 0 — Fundação ✅

Ver histórico acima. Workspace + `brainforge` binário.

---

## Fase 1 — CLI core ✅

`sync`, `doctor`, `paths` — ver critérios em commits anteriores.

---

## Fase 2 — Memória ✅ (+ 2b P1)

`memory read|compress|refresh|validate` — `MEMORY-FORMAT.md`.

---

## Fase 3 — MCP ✅

`brainforge mcp` + tools v1 + `MCP-SETUP.md`.

---

## Fase 4 — Distribuição ✅

`install`, `brainforge.toml`, CI, release.

---

## Fase 5 — Polish ✅

`skill install|list`, `prompt --copy`, `version`, doctor skill → CLI.

---

## Fase 6 — Extensões ✅

| Entrega | Critério |
|---------|----------|
| `brainforge recall` | ✅ list \| last \| search em agent-transcripts |
| MCP `session_recall` | ✅ mesmo comportamento |
| Secret scan | ✅ `memory_write` (MCP) bloqueia padrões sensíveis |
| `AGENTS.md` fino | ✅ criado no host se ausente (`brainforge sync`) |
| Cursor hooks exemplo | ✅ `.cursor/hooks.example/` + `CURSOR-HOOKS.md` |
| Embed commands | ✅ `sync --embed-commands` (`include_dir`) |
| Skills Hub remoto | ⏭ não implementado |
| Optionals spike / code-review | ⏭ adicionar ao catálogo quando existirem no kit |

**Critérios de aceite:**

- [x] `BRAINFORGE_TRANSCRIPTS` override
- [x] Secret scan testes unitários
- [x] Docs `CURSOR-HOOKS.md`

**Arquivos:** `recall.rs`, `security.rs`, `embedded.rs`, adapters `AGENTS.md`, `hooks.example/`.

---

## Backlog pós–Fase 6

- Skills Hub remoto (download catalog)
- Portar skills optional `spike`, `requesting-code-review`
- Hooks ativos por padrão (hoje só example)
- `memory_write` via CLI (hoje MCP-only)

---

## Referência rápida — comandos

```powershell
brainforge sync [-a all] [--no-menu] [--embed-commands]
brainforge doctor
brainforge paths
brainforge memory read|compress|refresh|validate [--sync] [--allow-merge]
brainforge mcp
brainforge install <path> [--with-exe] [--print-mcp-config]
brainforge skill list|install <id> [--force]
brainforge prompt --copy
brainforge version
brainforge recall list|last|search <query>
```
