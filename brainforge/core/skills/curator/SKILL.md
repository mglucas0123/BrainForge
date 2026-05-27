---
name: curator
description: "Use when: archive stale agent skills or review usage."
metadata:
  brainforge:
    tags: [skills, lifecycle, archive]
    related_skills: [skill-authoring, find-skills, learning-loop]
---

# Curator (lite)

## Overview

Track and lifecycle-manage **custom** skills under `.cursor/skills/`. Never delete — archive only. Core BrainForge skills are protected.

## When to Use

- User asks to clean up unused skills
- After `skill-authoring` creates agent skill — register usage
- Periodic review of `.cursor/skills/.usage.json`

**Don't use on:** caveman, cavemem, cavecrew, compress-context, find-skills, learning-loop, skill-authoring, curator, install-skill, brainforge-doctor, refactor-master, clean-coder, bug-hunter, systematic-debugging, architect, task-master.

## Telemetry File

`.cursor/skills/.usage.json`:

```json
{
  "skills": {
    "my-custom-skill": {
      "created_by": "agent",
      "use_count": 0,
      "last_used_at": null,
      "state": "active",
      "pinned": false
    }
  }
}
```

## Register (on create or use)

1. Read `.usage.json`; add or update skill entry.
2. Increment `use_count`; set `last_used_at` to ISO date.
3. `created_by`: `"agent"` or `"human"`.

## Archive Rules

- Only `created_by: "agent"` and `state: "active"`.
- `stale_after_days`: **30** without `last_used_at` update.
- `pinned: true` → never archive.
- Move folder: `.cursor/skills/<name>/` → `.cursor/skills/.archive/<name>/`
- Set `state: "archived"` in JSON (keep entry for history).

## Restore

- Move from `.cursor/skills/.archive/<name>/` back to `.cursor/skills/<name>/`
- Set `state: "active"` in JSON.

## Common Pitfalls

1. Archiving core skill — check protected list first.
2. Deleting folder — forbidden; archive only.
3. Forgetting JSON update after move — breaks tracking.

## Verification

- [ ] Core skills untouched
- [ ] Pinned skills not archived
- [ ] Archive path used, not delete
- [ ] `.usage.json` consistent with filesystem

## User Communication

- Report in **pt-BR**: archived/restored skills, stale candidates.
