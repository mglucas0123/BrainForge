---
name: install-skill
description: "Use when: install optional skill from catalog."
metadata:
  brainforge:
    tags: [skills, optional, install]
    related_skills: [find-skills, brainforge-doctor, skill-authoring]
---

# Install Skill

## Overview

Copy an optional skill from `.cursor/skills-optional/` to `.cursor/skills/` using `.cursor/skills-catalog.json`. Keeps core kit small until needed.

## When to Use

- `/install-skill <id>` or "instalar skill X"
- User wants catalog entry activated

## Core Skills (never overwrite)

Block install if `id` matches: caveman, cavemem, cavecrew, compress-context, find-skills, learning-loop, skill-authoring, curator, install-skill, brainforge-doctor, refactor-master, clean-coder, bug-hunter, systematic-debugging, architect, task-master.

## Procedure

1. Read `.cursor/skills-catalog.json` and `.cursor/installed-skills.json`.
2. Resolve `id` from user input (list catalog if missing/invalid).
3. Find entry where `optional_skills[].id === id`.
4. Source: `.cursor/<source_path>/` (entire folder).
5. Target: `.cursor/skills/<id>/`
6. If target exists → **warn**, stop unless user said `--force`.
7. Copy folder: use `Shell` (PowerShell `Copy-Item -Recurse`) or read/write files with `Read`/`Write`.
8. Append `id` to `installed-skills.json` `installed` array (no duplicates).
9. Register in `.cursor/skills/.usage.json` if missing: `{ "created_by": "human", "use_count": 0, "state": "active", "pinned": false }`.
10. Tell user: reload Cursor window if skill not visible in Agent Customizations.

## Uninstall (optional request)

- Remove `.cursor/skills/<id>/` only if `id` is in `installed-skills.json` and not a core skill.
- Remove `id` from `installed` array.
- Do not delete from `skills-optional/`.

## Common Pitfalls

1. Installing core skill id → blocked.
2. Forgetting `installed-skills.json` update → doctor WARN.
3. Editing optional source expecting active skill → must reinstall or edit copy in `skills/`.

## Verification

- [ ] Catalog entry exists
- [ ] `.cursor/skills/<id>/SKILL.md` present after install
- [ ] `installed-skills.json` updated
- [ ] Core skill not overwritten

## User Communication

- **pt-BR**: id installed, path, reload hint; or error reason.
