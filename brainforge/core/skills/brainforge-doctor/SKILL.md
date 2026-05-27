---
name: brainforge-doctor
description: "Use when: BrainForge health check or broken setup."
metadata:
  brainforge:
    tags: [doctor, diagnostics, setup]
    related_skills: [find-skills, install-skill]
---

# BrainForge Doctor

## Overview

Run a structured health check of the BrainForge kit. Report pass/warn/fail in **pt-BR**. Fix only when user asks.

## When to Use

- `/brainforge-doctor` or "doctor", "diagnóstico brainforge"
- After copying `.cursor` to another repo
- Agent behaves as if memory/skills missing

## Prefer CLI (Rust)

When `brainforge` / `brainforge.exe` is available, **run this first** (source of truth):

```powershell
brainforge doctor
```

Exit code `1` on FAIL. Report PASS/WARN/FAIL in pt-BR. Do not duplicate checks manually if CLI output is complete.

Fallback: manual checks below (e.g. Copilot without CLI, or agent without shell).

## Checks (fallback)

Run with `Shell` on Windows (PowerShell). Use RTK only if output is large.

### 1. RTK

- Path: `brainforge/tools/rtk/rtk.exe` — Pass if exists
- Warn if missing → `powershell -ExecutionPolicy Bypass -File .\brainforge\tools\rtk\install-rtk-local.ps1 -Force`
- Warn if `.cursor/tools/rtk/` exists (legacy duplicate — safe to delete)

### 2. Memory

| Path | Pass |
|------|------|
| `brainforge/memory/.context.md` | exists, non-empty, `**Capacity:**` + `§` entries |
| `brainforge/memory/.user.md` | exists, non-empty, `**Capacity:**` + `§` entries |
| `.cursor/project/*.md` | mirror of memory after sync (warn if drift) |
| `.context.md` (repo root) | stub → `brainforge/memory/` OR absent |

Warn if root `.context.md` is full duplicate of project memory (migration incomplete).
Warn if memory file missing `**Capacity:**` or `## Entries` with `§` (see `MEMORY-FORMAT.md`).
Warn if used chars > limit (1760 context / 1100 user) without consolidation.

### 3. Always-on rule

- `.cursor/rules/cavecrew-default.mdc` exists

### 4. Core skills

Each folder must contain `SKILL.md`:

caveman, cavemem, cavecrew, compress-context, find-skills, learning-loop, skill-authoring, curator, install-skill, brainforge-doctor, refactor-master, clean-coder, bug-hunter, systematic-debugging, architect, task-master

### 5. Commands

- `.cursor/commands/brainforge.md`
- `.cursor/commands/compress-context.md`
- `.cursor/commands/brainforge-doctor.md`
- `.cursor/commands/install-skill.md`

### 6. Learning & telemetry

- `.cursor/learnings.md` exists
- `.cursor/skills/.usage.json` exists and is valid JSON

### 7. Optional skills infra

- `.cursor/skills-catalog.json` exists and is valid JSON
- `.cursor/skills-optional/` exists
- `.cursor/installed-skills.json` exists and is valid JSON
- `.cursor/plans/` exists (README ok)

### 7b. Plans directory

- `.cursor/plans/README.md` exists
- Warn if many `*.md` plans (>20) — suggest archive old files

### 8. Optional installs

- For each `id` in `installed-skills.json`, verify `.cursor/skills/<id>/SKILL.md` exists
- Warn if catalog entry installed but folder missing

## Output Contract

```
BrainForge Doctor
================
[PASS] ...
[WARN] ... → ação sugerida
[FAIL] ... → ação obrigatória
Resumo: X pass, Y warn, Z fail
```

- **pt-BR**, concise
- Do not auto-fix unless user requests

## Verification

- [ ] All 8 sections evaluated
- [ ] Every FAIL has suggested fix command/path
- [ ] No false PASS on missing core skill
