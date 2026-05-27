---
name: systematic-debugging
description: "Use when: bug or regression needs root-cause first."
metadata:
  brainforge:
    tags: [debugging, root-cause, troubleshooting]
    related_skills: [bug-hunter, task-master]
---

# Systematic Debugging

## Overview

No fixes without root cause. Symptom patches waste time and create new bugs.

**Iron law:** `NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST`

## When to Use

- Test failures, production bugs, unexpected behavior, performance regressions, build/integration failures

**Especially when:** time pressure, "quick fix" temptation, multiple failed fixes already, or cause not fully understood.

## Cursor Tools

| Activity | Tool |
|----------|------|
| Read source | `Read` |
| Find references / errors | `Grep` |
| Run tests, git, reproduce | `Shell` (RTK if large output on Windows) |
| Complex multi-area investigation | `Task` subagent (optional) |

## Phase 1 — Root Cause Investigation

**Before ANY fix:**

1. **Read errors completely** — stack trace, line numbers, codes. Use `Read` on cited files.
2. **Reproduce consistently** — exact steps via `Shell`; if not reproducible, gather more data.
3. **Check recent changes** — `git log --oneline -10`, `git diff`, file history.
4. **Multi-component systems** — instrument boundaries (log in/out per layer); find *where* it breaks before fixing.
5. **Trace data flow** — bad value upstream to source; fix at source, not symptom. Use `Grep` for callers/assignments.

**Phase 1 done when:** you can state *why* it fails, not only *what* fails.

## Phase 2 — Pattern Analysis

1. Find similar **working** code in repo (`Grep`).
2. Read reference implementation fully if applying a pattern.
3. List differences working vs broken — no "that can't matter" assumptions.
4. Note dependencies: config, env, versions, assumptions.

## Phase 3 — Hypothesis and Testing

1. One hypothesis: "X is root cause because Y".
2. **Smallest** change to test — one variable at a time.
3. Worked → Phase 4. Failed → new hypothesis, don't stack fixes.
4. If unknown → say so; ask user; research more.

## Phase 4 — Implementation

1. **Failing test first** when possible (minimal repro).
2. **One fix** for root cause — no drive-by refactors.
3. Verify: targeted test + broader suite via `Shell`.
4. **Rule of three:** if ≥3 fixes failed → stop; discuss architecture with user before fix #4.
5. Architecture smell: each fix exposes new coupling elsewhere → wrong pattern, not another patch.

## Red Flags — STOP, Return to Phase 1

- "Quick fix for now, investigate later"
- "Just try X and see"
- Multiple changes before one test
- Proposing solutions before tracing data flow
- "One more fix" after 2+ failures

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| Too simple for process | Simple bugs have root causes too |
| No time | Systematic beats thrashing |
| I'll test after | Untested fixes don't stick |
| I see the problem | Symptoms ≠ root cause |

## Quick Reference

| Phase | Success criteria |
|-------|------------------|
| 1 Root cause | Know WHAT and WHY |
| 2 Pattern | Know what's different vs working |
| 3 Hypothesis | Confirmed or replaced |
| 4 Implementation | Fixed + verified, no new regressions |

## Rules

- Do not auto-activate on `/brainforge` alone — need bug/regression/performance trigger.
- User output in **pt-BR**: repro, root cause, fix, regression check, residual risk.

## Verification

- [ ] Phase 1 complete before any fix proposed
- [ ] Hypothesis tested with minimal change
- [ ] Regression verified via command or test
- [ ] Rule of three respected if prior fixes failed
