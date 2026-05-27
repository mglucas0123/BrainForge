---
name: cavecrew
description: "Use when: caveman and cavemem together."
metadata:
  brainforge:
    tags: [cavecrew, caveman, cavemem]
    related_skills: [caveman, cavemem]
---

# Cavecrew

## Goal
Run Caveman communication with Cavemem memory discipline.

## Operating Mode
1. Load `.cursor/project/.context.md` and `.cursor/project/.user.md`.
2. Execute tasks with concise, direct technical output.
3. Keep changes aligned with project memory and user profile.
4. Offer memory updates after major structural changes.
5. If `rtk` is available, prefer explicit `rtk` commands for high-output shell tasks on Windows native.

## Response Contract
- Short output first, in **pt-BR** for the user.
- Action over discussion.
- No unnecessary verbosity.
- Follow full caveman contract (Persistence, Auto-Clarity, Boundaries) from `caveman/SKILL.md`.
- Code output stays normal even when prose is terse.

## Verification

- [ ] Both memory files loaded at task start
- [ ] RTK used for large shell output on Windows when available
