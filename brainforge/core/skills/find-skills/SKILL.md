---
name: find-skills
description: "Use when: list or verify workspace skills and commands."
metadata:
  brainforge:
    tags: [skills, audit, discovery]
    related_skills: [skill-authoring, curator]
---

# Find Skills

## Goal

Quickly audit which skills are available and whether they are operational.

## Checks

1. List workspace skills under `.cursor/skills/*/SKILL.md`.
2. Verify memory files: `.cursor/project/.context.md`, `.cursor/project/.user.md`.
3. Optional: `.cursor/learnings.md`; `.cursor/skills/.usage.json`; `.cursor/installed-skills.json`.
4. List commands: `/brainforge`, `/compress-context`, `/brainforge-doctor`, `/install-skill`.
5. List optional catalog: `.cursor/skills-catalog.json`; folders under `.cursor/skills-optional/`.
6. Report missing expected core skills.
7. Mark optional skills: in catalog vs installed vs source-only.
8. Suggest reload window if skills not visible in UI.

## Expected Core Skills

caveman, cavemem, cavecrew, compress-context, find-skills, learning-loop, skill-authoring, curator, install-skill, brainforge-doctor, refactor-master, clean-coder, bug-hunter, systematic-debugging, architect, task-master.

**Note:** `bug-hunter` is alias → follow `systematic-debugging`.

## Output Contract

- Concise list in **pt-BR**.
- Mark each: present / missing.
- Flag memory path migration if root `.context.md` is only a stub.

## Verification

- [ ] All core skill folders scanned
- [ ] Memory paths reported
- [ ] Commands listed
