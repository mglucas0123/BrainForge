---
name: cavemem
description: "Use when: read or update project memory files."
metadata:
  brainforge:
    tags: [memory, context, cavemem]
    related_skills: [compress-context, learning-loop]
---

# Cavemem

## Goal

Preserve and reuse project + user knowledge with minimal overhead. Structured `§` entries + capacity header.

## Memory Files

| File | Limit (chars) | Content |
|------|---------------|---------|
| `brainforge/memory/.context.md` | 2200 | Stack, architecture, decisions, paths, RTK |
| `brainforge/memory/.user.md` | 1375 | pt-BR, caveman, workflow, peevs |

Mirrored to `.cursor/project/` by `brainforge/sync.ps1`. Full format: `brainforge/core/docs/MEMORY-FORMAT.md`.

## Capacity Header

Line after `# Title`:

```markdown
**Capacity:** 41% · 896/2200 chars · ≥80% → consolidate before add
```

After any edit to `## Entries`:

1. Sum chars inside all `§...§` (include `§` markers).
2. `pct = round(used/limit*100)`.
3. Rewrite `**Capacity:**` line.

## At ≥80% Capacity

Before **add**: merge related `§` entries (combine duplicates, drop stale facts). Then add one new `§...§`.

If still over limit after consolidate → `replace` or `remove` until under limit.

## Entry Operations

- **Add:** `§compact fact§` at end of `## Entries`.
- **Replace:** unique substring inside one `§` block only.
- **Remove:** delete entire `§...§` block.
- **No** bullet lists in entries; **no** duplicate rules/skills text.

## What to Save (proactive)

Save without being asked when the fact will matter in **future sessions**. One `§` per consolidated fact.

### → `.cursor/project/.context.md`

- Stack, paths, ports, env tools (RTK, shell)
- Architecture and layer decisions (rules, commands, skills layout)
- Repo conventions (naming, test command, package manager)
- Corrections the agent must not repeat ("never edit X", "use Y not Z")
- Completed milestone worth remembering (migration date, major refactor done)
- Explicit user request: "remember that …" for **project** facts

### → `.cursor/project/.user.md`

- Language, tone, caveman level / disable phrases
- Workflow (commits, scope, PR style, questions policy)
- Pet peeves and "do not" preferences
- Tooling habits (RTK, opt-out compress)
- Explicit user request: "remember that …" for **personal** prefs

## What to Skip (never write to memory)

- Trivial chat: "user asked about Python" with no durable rule
- Facts easy to re-read from repo (file exists, obvious from `package.json`)
- Raw dumps: logs, stack traces, long code, tables, paste of conversation
- Session ephemera: temp paths, branch name today, "currently debugging line 42"
- Duplicate of **rules** (`.cursor/rules/*.mdc`), **skills** (`SKILL.md`), or **commands** — point there, don't copy
- Duplicate of `learnings.md` — use learning-loop for discoveries; only promote to memory if always-on critical
- Vague filler: "user has a project", "we discussed bugs"
- Marketing or process essay inside `§` — one actionable line only

**When unsure:** prefer `learnings.md` (discovery log) or skip; ask user only if blocking.

## Workflow

1. Task start: if over compress threshold (lines/bytes) → `compress-context` before read.
2. Read both memory files; note `**Capacity:**` on each.
3. Missing file → create per `MEMORY-FORMAT.md` baseline.
4. Reuse entries before proposing changes.
5. Structural project change → offer `.context.md` update; user prefs → `.user.md`.
6. After write → update capacity header → compress if threshold exceeded (final step).

## Compression

- Skill: `compress-context`.
- Opt-out session: `"sem auto-compress"` / `"skip compress"`.

## Verification

- [ ] Both files have valid `**Capacity:**` line
- [ ] Entries use `§` only under `## Entries`
- [ ] Used chars ≤ limit (or consolidated at ≥80%)
- [ ] Project vs user split respected
- [ ] New entry passes save/skip rules (not duplicate, not ephemeral)

## User Communication

- **pt-BR**; mention capacity % if near limit.
