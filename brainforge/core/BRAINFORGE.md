# BrainForge Routine (canonical)

Source for `/brainforge` in Cursor and Antigravity workflows. Sync copies to IDE adapters.

Activate **immediately** and keep until the user asks for another mode.

## 1. Cavemem — memory

- Read `brainforge/memory/.context.md` and `brainforge/memory/.user.md` now (create short baselines if missing).
- Same files mirrored at `.cursor/project/` after `sync.ps1` — prefer **brainforge/memory/** as canonical.
- If either file exceeds compress threshold, **compress first** (skill `compress-context` or `/compress-context`) before use.
- Before proposing new changes, cross-check with both memory files.
- After structural changes → offer update to `.context.md`; user prefs → `.user.md`.
- After writing either memory file, if over threshold → compress in the same turn (final step).

Detail: `brainforge/core/cavemem.md` · format: `brainforge/core/docs/MEMORY-FORMAT.md`.

## 2. Caveman — output

- User-facing replies in **pt-BR** (see `brainforge/core/caveman.md` and Cursor rule `cavecrew-default.mdc`).
- Persistence: concise mode active in **every** response; no verbose drift.
- Levels: `/brainforge lite|full|ultra` (default: full).
- Disable: "modo normal", "stop caveman", "stop brainforge", "para brainforge".
- Full rules: `brainforge/core/skills/caveman/SKILL.md`.

## 3. RTK / token economy

- **Only path:** `brainforge/tools/rtk/rtk.exe` (sync does not copy RTK elsewhere).
- If `rtk` on PATH, use `rtk`; else `brainforge/tools/rtk/rtk.exe` for large shell output on Windows.
- Missing: `powershell -ExecutionPolicy Bypass -File .\brainforge\tools\rtk\install-rtk-local.ps1`.

## 4. Skill Router (on demand)

- Base for `/brainforge`: **only** Caveman + Cavemem + RTK.
- Specialized skills in `brainforge/core/skills/` (synced to `.cursor/skills/`) — opt-in by trigger; max 1 per request (2 if truly needed).

| Skill | Trigger |
|-------|---------|
| `refactor-master` | safe incremental refactor |
| `clean-coder` | readability / hygiene |
| `systematic-debugging` / `bug-hunter` | bug, regression, perf |
| `architect` | structure, coupling |
| `task-master` | task breakdown |
| `writing-plans` | plan before code (`/install-skill writing-plans`) |
| `skill-authoring` | create/edit skills |
| `learning-loop` | lessons after discovery |
| `curator` | stale skills / usage |
| `brainforge-doctor` | `/brainforge-doctor` |
| `install-skill` | `/install-skill` |

## 5. Execution

- Execute yourself: commands, reads, edits — not instructions-only.
- Confirm facts in code before asserting behavior.

**State:** BrainForge ON for the rest of this chat.
