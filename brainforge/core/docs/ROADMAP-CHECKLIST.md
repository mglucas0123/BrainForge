# BrainForge Roadmap Checklist

Foco do projeto: **otimização de tokens** + **agente refinado e eficiente**.

Referência Hermes: [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent)

---

## Já entregue (baseline)

- Memória dual: `.cursor/project/.context.md` + `.user.md` (`§` + capacity header)
- Compress + caveman + RTK + router opt-in
- Skills: standards, optional catalog, `/install-skill`, doctor
- Learning: `learnings.md`, `learning-loop`, `curator`
- Debug: `systematic-debugging` (+ `bug-hunter` alias)
- Tier A: descriptions ≤60, save/skip, structured memory
- Tier B: plans em arquivo (`writing-plans` → `.cursor/plans/`)

---

## Tier A — concluído / descartado

- [x] **1 — Descriptions ≤60 chars**
- [x] **2 — Memória estruturada (`§` + header de capacidade)**
- [x] **3 — Regras o que gravar / o que não gravar (cavemem)**
- [~] **4 — AGENTS.md + contexto progressivo** — *descartado* (risco duplicação / pouco ganho no kit; usar `.context.md` § + `AGENTS.md` manual no host se precisar)

---

## Tier B — concluído

- [x] **8 — Planos em arquivo (não no chat)**
  - Plano completo: `.cursor/plans/<slug>.md`
  - Chat: resumo curto + path
  - Skill: `skills-optional/writing-plans` — `/install-skill writing-plans`

---

## Tier C — hub portátil

- [x] **`brainforge/`** — core, memory, adapters
- [x] **`brainforge/HOST-SETUP.md`**
- [x] README raiz + `.gitignore`

## Tier D — Rust CLI ✅ (v1)

- [x] **Fase 0–1** — workspace, `sync`, `doctor`, `paths`, menu `dialoguer`
- [x] **Fase 2** — `memory read|compress|refresh|validate` + P0/P1
- [x] **Fase 3** — `brainforge mcp` (rmcp 1.7)
- [x] **Fase 4** — `install`, `brainforge.toml`, CI + release
- [x] **Fase 5** — `skill install|list`, `prompt --copy`, `version`, doctor → CLI

`sync.ps1` obsoleto — `brainforge sync` / `brainforge install`. Plano: `brainforge/core/docs/RUST-PHASES.md`.

---

## Fase 6 — extensões ✅

- [x] `brainforge recall` (agent-transcripts)
- [x] Secret scan em `memory_write` (MCP)
- [x] `AGENTS.md` + hooks.example + `CURSOR-HOOKS.md`
- [x] `sync --embed-commands`

## Backlog pós–v1

- Skills Hub remoto
- Optionals spike / requesting-code-review no catálogo

---

## Notas de revisão

| Data | Nota |
|------|------|
| 2026-05-27 | Roadmap recriado — Tier A (1–4) + Tier B (8) |
| 2026-05-27 | Tier A 1–3 + Tier B 8 implementados; item 4 descartado |

Ao fechar item: marcar `[x]`, data na tabela, opcional commit/PR.
