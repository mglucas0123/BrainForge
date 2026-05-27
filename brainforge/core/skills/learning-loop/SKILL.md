---
name: learning-loop
description: "Use when: record actionable lessons after a task."
metadata:
  brainforge:
    tags: [learning, memory, improvement]
    related_skills: [curator, skill-authoring, cavemem]
---

# Learning Loop

## Goal

Turn recurring learnings into reusable project patterns without bloating memory files.

## When to Use

- After task with technical discovery worth repeating
- After fix for error that may recur
- When process improvement belongs in rules, skills, or project memory

**Don't use for:** trivial one-off notes, info already in `.cursor/project/.context.md`, or vague summaries.

## Procedure

1. Confirm learning is **actionable** (one clear rule or change).
2. Append entry to `.cursor/learnings.md` using the format in that file header.
3. If structural:
   - Rules → offer patch to `.cursor/rules/*.mdc`
   - Project facts → offer patch to `.cursor/project/.context.md` (**cavemem** save/skip + `§` format)
   - User prefs → offer patch to `.cursor/project/.user.md` (**cavemem** save/skip)
   - New skill → use `skill-authoring`; register in `.cursor/skills/.usage.json`
4. Promote to memory only if always-on critical; else keep in `learnings.md` only.
5. If agent created a new skill → `curator` pattern (register usage; do not archive core skills).

## Record Format

```markdown
## YYYY-MM-DD — title
- **Contexto:**
- **Aprendizado:**
- **Ação:**
- **Impacto:**
- **Arquivos:**
```

## Quality Rules

- One clear rule per entry.
- No long abstract text.
- Skip if duplicate of recent entry in `.cursor/learnings.md`.

## Verification

- [ ] Entry appended to `.cursor/learnings.md`
- [ ] Structural changes offered, not silently applied to rules/memory
- [ ] No duplicate or trivial entries

## User Communication

- Summarize in **pt-BR**: what was recorded and where.
