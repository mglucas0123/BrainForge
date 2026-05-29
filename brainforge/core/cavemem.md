# Cavemem (always-on summary)

Full skill: `brainforge/core/skills/cavemem/SKILL.md` · format: `brainforge/core/docs/MEMORY-FORMAT.md`.

| File | Limit | Content |
|------|-------|---------|
| `brainforge/memory/.context.md` | 2200 chars | stack, architecture, decisions |
| `brainforge/memory/.user.md` | 1375 chars | pt-BR, caveman, workflow |

- Header: `**Capacity:**` + `§` entries; consolidate at ≥80%.
- Auto-compress thresholds: context >80 lines or >4 KB; user >50 lines or >2 KB — skill `compress-context` or `/compress-context`.
- Opt-out: `sem auto-compress` / `skip compress`.
- After structural work → offer `.context.md` update; prefs → `.user.md`.
- If `.context.md` is missing: **CREATE IT immediately** — even for empty projects. Minimum: `§Stack: (unknown)§`.
