---
name: writing-plans
description: "Use when: plan before coding; full plan saved to file."
metadata:
  brainforge:
    tags: [planning, tasks]
    related_skills: [task-master, architect]
---

# Writing Plans

## Overview

Write the **full** plan to `.cursor/plans/<slug>.md`. In chat: **short summary + path only** — never paste the full plan in the conversation (saves output + future context tokens).

## When to Use

- User asks for plan, roadmap, or "how to implement X"
- Multi-file or unclear order before coding
- Before large refactor or feature

**Don't use when:** trivial one-file change; user said "just do it".

## Procedure

1. Read `.cursor/project/.context.md` for constraints.
2. Choose **slug**: `YYYY-MM-DD-kebab-topic.md` (ASCII, lowercase, hyphens).
3. **Write** full plan to `.cursor/plans/<slug>.md` using file template below (`Write` tool).
4. **Chat reply** (max ~12 lines, **pt-BR**):
   - Goal (1 sentence)
   - Step count + 1-line highlight per step (optional, max 5)
   - Path: `.cursor/plans/<slug>.md`
   - Ask confirm to execute **only if** ambiguity blocks step 1
5. If user approves → hand off to implementation or `task-master` for slice breakdown.

## File Template (`.cursor/plans/<slug>.md`)

```markdown
# Plan: <title>

Created: YYYY-MM-DD

## Goal

...

## Steps

1. **<slice>** — files: ... — acceptance: ... — AFK|HITL
2. ...

## Risks

- ...

## Open questions

- ...

## Out of scope

- ...
```

File may exceed 40 lines; chat must stay short.

## Slug Rules

- Unique; if file exists, append `-2` or refine topic in slug.
- No spaces; no path separators inside slug.

## Common Pitfalls

1. **Full plan in chat** — forbidden; defeats token savings.
2. Horizontal layers only → use vertical slices.
3. Skipping acceptance per step.
4. Plan file in repo root or `docs/` — use `.cursor/plans/` only.

## Verification

- [ ] `.cursor/plans/<slug>.md` exists and is complete
- [ ] Chat has no full Steps/Risks dump (only summary + path)
- [ ] Steps ordered and demoable
- [ ] Aligned with `.context.md`

## User Communication

- **pt-BR** in chat; plan file may use EN for code-centric terms.
