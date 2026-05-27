# BrainForge Memory Format

Hermes-inspired structured memory for token economy.

## Files

| File (canonical) | Mirror | Char limit | Line/byte threshold (compress) |
|------|--------|------------|--------------------------------|
| `brainforge/memory/.context.md` | `.cursor/project/.context.md` | **2200** | >80 lines or >4096 bytes |
| `brainforge/memory/.user.md` | `.cursor/project/.user.md` | **1375** | >50 lines or >2048 bytes |

## Header (required, line 3)

```markdown
**Capacity:** 41% · 896/2200 chars · ≥80% → consolidate before add
```

- **%** = `used/limit` rounded (count chars from `## Entries` body: all `§...§` text including `§`).
- Recompute after every add/replace/remove/consolidate/compress.
- At **≥80%**: merge related `§` entries before adding new ones.

## Entries

- One fact per `§...§` block; dense prose; no bullet lists inside entries.
- Multiline allowed inside one `§` pair only when necessary.
- Separate entries with `§` only — no blank lines between entries required.

## What to save / skip

Authoritative lists in `.cursor/skills/cavemem/SKILL.md` (sections **What to Save** / **What to Skip**).

Quick rule: **project facts → `.context.md`** · **user prefs → `.user.md`** · **discoveries → `learnings.md`** · never duplicate rules/skills.

## Add / replace / remove

- **Add:** append `§new fact§` if under limit after header update.
- **Replace:** match unique substring inside target `§` entry; rewrite that entry only.
- **Remove:** delete whole `§...§` block.
- If add would exceed limit → consolidate first, then add.

## Sections outside entries

- `# Title` and `**Capacity:**` — always keep.
- `## Reference` — fenced code blocks OK; not counted in § char budget but count toward file byte threshold.
- Do not duplicate content from `.cursor/rules/` or skills into § entries.

## Compress

| Mode | When |
|------|------|
| **Agent** — skill `compress-context` / `/compress-context` | Semantic caveman compression in chat |
| **CLI** — `brainforge memory compress` | Local heuristic (filler + merge); skips write if unchanged |
| **CLI** — `brainforge memory refresh` | Recompute `**Capacity:**` only; safe after manual § edits |

Preserve: header pattern, every `§`, code fences, URLs/paths in entries **and** `## Reference`.
