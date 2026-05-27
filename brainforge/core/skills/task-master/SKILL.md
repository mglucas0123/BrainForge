---
name: task-master
description: "Use when: break work into ordered task slices."
metadata:
  brainforge:
    tags: [planning, tasks, workflow]
    related_skills: [architect, learning-loop]
---

# Task Master

## Goal
Turn a broad plan into small, independent, verifiable tasks.

## Workflow
1. Consolidate context and final objective. If plan exists at `.cursor/plans/*.md`, **Read** it — do not duplicate long plan in chat.
2. Break into vertical slices (avoid horizontal layer-only slices).
3. Mark task type: AFK or HITL.
4. Define real dependencies and maximize parallelism.
5. Write objective acceptance criteria per task.

## Rules
- Do not auto-activate on `/brainforge`; activate only when user requests task breakdown/planning.
- Each task must be demoable/verifiable alone.
- Prefer many small tasks over few large ones.
- Avoid fragile references (line/file) when describing objective.
- Revalidate granularity with user before batch execution.

## Output Contract
- Numbered list to user in **pt-BR**: title, type, blockers, acceptance.
- Recommended execution order.

## Verification

- [ ] Each task has acceptance criteria and type (AFK/HITL)
- [ ] Dependencies explicit; parallel work identified
- [ ] Slices are vertical, not layer-only
