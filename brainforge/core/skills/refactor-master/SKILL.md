---
name: refactor-master
description: "Use when: refactor without changing behavior."
metadata:
  brainforge:
    tags: [refactor, safety, structure]
    related_skills: [clean-coder, architect]
---

# Refactor Master

## Goal
Refactor safely in small steps while keeping behavior stable.

## Workflow
1. Define boundary: what changes and what must not change.
2. Ensure feedback loop (build/test/validation command).
3. Split into reversible micro-steps (vertical slice, no big-bang).
4. Apply one step at a time and validate always.
5. Finish with dead code cleanup and decision summary.

## Rules
- Do not auto-activate on `/brainforge`; activate only on explicit refactor request.
- Do not change external behavior without explicit request.
- Prefer extract/modularize before rewrite.
- If missing test at critical point, create minimal protection test.
- Avoid giant diff files; prioritize readability.

## Output Contract
- Show short step plan to user in **pt-BR**.
- Execute and validate step by step.
- Report residual risk objectively.

## Verification

- [ ] Each micro-step validated before next
- [ ] External behavior unchanged unless requested
- [ ] Feedback loop (test/build) used between steps
