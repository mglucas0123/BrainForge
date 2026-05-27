# BrainForge Skill Standards

Reference checklist for skills under `.cursor/skills/<name>/SKILL.md`. Not loaded as a skill — use via `skill-authoring`.

## Frontmatter (required)

```yaml
---
name: my-skill-name          # lowercase, hyphens, matches folder
description: "Use when: <trigger>. <one-line behavior>."
metadata:
  brainforge:
    tags: [tag1, tag2]
    related_skills: [other-skill]
---
```

| Field | Rule |
|-------|------|
| `name` | Matches directory name; ≤64 chars |
| `description` | Starts with `Use when:`; **one sentence**; ends with `.`; **≤60 chars** (Hermes HARDLINE — Cursor injects every skill description each session) |
| `metadata.brainforge.tags` | 2–6 short tags |
| `metadata.brainforge.related_skills` | In-repo skills only |

Optional: `version`, `author`, `license` for contributed skills.

## Body structure

1. `# Title` — matches skill purpose
2. `## Overview` — 1–2 paragraphs: what and what not
3. `## When to Use` — triggers + "Don't use when"
4. `## Procedure` or phased workflow — actionable steps
5. `## Common Pitfalls` — numbered mistakes + fixes
6. `## Verification` — 3–5 checkboxes

Target size: **8–15 KB** for complex skills; **3–8 KB** for simple. If larger → `references/*.md`.

## Tool references in prose

Use Cursor-native tools by name:

| Use | Not |
|-----|-----|
| `Read` | read_file |
| `Grep` | search_files, grep CLI as primary |
| `Shell` | terminal (Hermes) |
| `StrReplace` / `Write` | patch (unless user project uses patch) |
| `Task` | delegate_task |

Large shell output on Windows: prefer RTK wrapper per cavecrew-default.

## Core vs custom skills

**Core** (do not archive via curator): caveman, cavemem, cavecrew, compress-context, find-skills, learning-loop, skill-authoring, curator, install-skill, brainforge-doctor, refactor-master, clean-coder, bug-hunter, systematic-debugging, architect, task-master.

**Optional** (install via `/install-skill`): live under `.cursor/skills-optional/`; catalog in `.cursor/skills-catalog.json`.

**Custom**: any skill added by user or agent outside this list — register in `.cursor/skills/.usage.json`.

## Quality gates before merge

- [ ] Description is trigger-based, not marketing
- [ ] No duplicate skill in `.cursor/skills/`
- [ ] `find-skills` expected list updated if core skill
- [ ] `brainforge.md` router updated if opt-in skill
- [ ] Verification section present
