---
name: clean-coder
description: "Use when: clean or simplify code, same behavior."
metadata:
  brainforge:
    tags: [readability, hygiene, refactor]
    related_skills: [refactor-master]
---

# Clean Coder

## Goal
Improve code clarity and maintainability without inventing complexity.

## Checklist
1. Clear names (types, methods, variables).
2. Small functions with single responsibility.
3. Explicit flow (less accidental branching).
4. Errors handled with useful messages.
5. Remove duplication and dead code.

## Rules
- Do not auto-activate on `/brainforge`; activate only when focus is cleanup/readability.
- Preserve public API unless requested otherwise.
- Prefer small changes, no irrelevant reformatting.
- Comment only what is not obvious.
- Run validation after cleanup (build/test/lint when applicable).

## Output Contract
- List smells found (high priority first) to user in **pt-BR**.
- Apply objective fixes.
- State what stayed out of scope.

## Verification

- [ ] Public API unchanged unless requested
- [ ] Validation run when applicable (build/test/lint)
- [ ] Scope limited to requested cleanup
