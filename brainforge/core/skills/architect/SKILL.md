---
name: architect
description: "Use when: architecture, coupling, or structural plan."
metadata:
  brainforge:
    tags: [architecture, coupling, design]
    related_skills: [refactor-master, task-master]
---
# Architect

## Goal
Identify architectural friction and propose high-leverage evolution.

## Workflow
1. Read domain context (`.cursor/project/.context.md`) and existing constraints.
2. Map friction: coupling, low testability, low locality.
3. Propose improvement candidates (problem, solution, benefit, risk).
4. Pick one candidate and design target seam/interface.
5. Define incremental migration plan with validation checkpoints.

## Rules
- Do not auto-activate on `/brainforge`; activate only for architectural decisions.
- Prioritize changes that simplify the future, not just the present.
- Do not propose big-bang rewrite.
- Respect prior decisions; flag real conflicts.
- Use module/interface/seam/adapter when discussing architecture.

## Output Contract

- Prioritized candidate list to user in **pt-BR**.
- Clear next-step recommendation.
- Short executable incremental plan.

## Verification

- [ ] Read `.cursor/project/.context.md` before proposals
- [ ] No big-bang rewrite proposed
- [ ] Incremental plan has validation checkpoints