---
name: skill-authoring
description: "Use when: create or edit a skill in .cursor/skills/."
metadata:
  brainforge:
    tags: [skills, authoring, standards]
    related_skills: [find-skills, curator, learning-loop]
---

# Skill Authoring

## Overview

Create or update workspace skills in `.cursor/skills/<name>/SKILL.md`. Follow `.cursor/docs/SKILL-STANDARDS.md`.

For **optional** skills (heavy/niche): author under `.cursor/skills-optional/<name>/`, add entry to `.cursor/skills-catalog.json`, install via `/install-skill` — do not drop into core `skills/` unless user asks.

## When to Use

- User asks to create, edit, standardize, or review a skill
- After porting a workflow from Hermes or another repo
- When a skill description is too vague for the router

**Don't use when:** only listing skills → `find-skills`.

## Procedure

1. **Survey peers** — `Glob` `.cursor/skills/*/SKILL.md`; read 2 skills in same domain.
2. **Check duplicate** — `Grep` for similar `name` or purpose; extend existing skill if overlap.
3. **Draft** — folder `kebab-case`, file `SKILL.md`, frontmatter per SKILL-STANDARDS.
4. **Validate**
   - `description` starts with `Use when:`; **≤60 chars**; one sentence; ends with `.`
   - Body has Overview, When to Use, Procedure, Pitfalls, Verification
   - Tools named: Read, Grep, Shell, StrReplace, Write, Task
5. **Register** if agent-created custom skill:
   - Append entry in `.cursor/skills/.usage.json` (`created_by: "agent"`, `state: "active"`)
6. **Wire router** — if opt-in skill, add trigger line in `.cursor/commands/brainforge.md`
7. **Update find-skills** expected list if core skill

## Common Pitfalls

1. **Generic description** — "Helps with code" fails router; use trigger class: "Use when: user reports flaky tests in CI."
2. **Hermes tool names** — map to Cursor tools in prose.
3. **Skill too long** — move reference material to `references/` subfolder.
4. **Forgot .usage.json** — agent-created skills won't be tracked by curator.

## Verification

- [ ] Path is `.cursor/skills/<name>/SKILL.md`
- [ ] Frontmatter valid; description ≤60 chars with `Use when:`
- [ ] SKILL-STANDARDS sections present
- [ ] No duplicate purpose with existing skill
- [ ] Router/find-skills updated if needed
- [ ] `.usage.json` entry if `created_by: agent`

## User Communication

- Summarize in **pt-BR**: skill name, path, trigger, related skills.
