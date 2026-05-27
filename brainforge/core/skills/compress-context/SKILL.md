---
name: compress-context
description: "Use when: compress memory files over size threshold."
metadata:
  brainforge:
    tags: [memory, compression, tokens]
    related_skills: [cavemem]
---

# Compress Context

## CLI vs agent (do not confuse)

| Tool | Role |
|------|------|
| **This skill** / `/compress-context` | Semantic compression in chat (caveman-speak, merge facts) |
| `brainforge memory compress` | Terminal heuristic (regex filler + Jaccard merge); not a substitute |
| `brainforge memory refresh` | Realign `**Capacity:**` only — no § edits |

Use the skill when the user wants quality compression. Use the CLI for quick local maintenance or CI.

## Targets

| File | Char limit | Auto threshold |
|------|------------|----------------|
| `brainforge/memory/.context.md` | 2200 | >80 lines or >4096 bytes |
| `brainforge/memory/.user.md` | 1375 | >50 lines or >2048 bytes |

Format: `.cursor/docs/MEMORY-FORMAT.md`. Process context first, then user.

Skip auto if `"sem auto-compress"` or `"skip compress"` this session.

## Auto Triggers

1. **Task start** — before memory used for decisions.
2. **After writing either memory file** — final step if threshold exceeded.

Manual `/compress-context` forces both (skip empty baselines).

## Process (per file)

1. Read `original`.
2. Missing `.context.md` → cavemem baseline, stop; missing `.user.md` → user baseline or skip.
3. Auto + below threshold → skip file.
4. Manual + tiny (<20 lines, <1 KB) → skip.
5. Compress **inside each `§...§`** and prose headers; merge redundant § entries when possible.
6. Recompute `**Capacity:**` line (used/limit/pct).
7. Validate vs `original`.
8. Write or abort.

## Remove (inside § entries)

Articles, filler, hedging, connective fluff — same as before.

## Preserve EXACTLY

- Count of `§` entry pairs (may merge content but do not drop facts without merging text)
- `**Capacity:**` line pattern (update numbers)
- Fenced code blocks in `## Reference`
- URLs, paths, inline code, env vars, version numbers

## Preserve Structure

- `# Title`, `## Entries`, `## Reference` (if present)
- Heading titles unchanged

## Validation

- Same fenced code block count
- All URLs/paths from original present
- Every factual § from original still represented (merged OK)
- `**Capacity:**` present and arithmetically consistent

## Output Contract

**pt-BR**: mode, per-file chars before→after, validation pass/fail.

## Verification

- [ ] § format intact
- [ ] Capacity line updated
- [ ] Char limits documented in report
