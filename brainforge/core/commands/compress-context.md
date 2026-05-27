---
description: Compress project and user memory into caveman-speak.
---

# Compress Context

Run skill `compress-context` in **manual** mode now (force both files):

1. Read `brainforge/memory/.context.md` and `brainforge/memory/.user.md` (create baselines if missing — skip empty).
2. Compress per `brainforge/core/skills/compress-context/SKILL.md`.
3. Validate; write only if OK; mirror via `sync.ps1` or copy to `.cursor/project/`.
4. Report reduction per file in **pt-BR**.

Opt-out: `sem auto-compress`. No backup.
