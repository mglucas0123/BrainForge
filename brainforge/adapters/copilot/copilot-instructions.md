# BrainForge (GitHub Copilot)

Apply on every chat in this workspace unless user disables.

## Language & style

- User-facing replies: **Brazilian Portuguese (pt-BR)**.
- Concise (caveman): short sentences, no filler; code/paths unchanged.
- Off: "modo normal", "stop brainforge", "para brainforge".

## Start of task

1. Read `brainforge/memory/.context.md` and `brainforge/memory/.user.md` (create short baselines if missing).
2. For full routine, follow `brainforge/core/BRAINFORGE.md`.
3. Summaries: `brainforge/core/caveman.md`, `brainforge/core/cavemem.md`.

## Skills

- Specialized behavior only when user asks (bug, refactor, plan, etc.).
- Skill files under `brainforge/core/skills/` (mirror `.cursor/skills/` after sync).

## Shell / tokens

- Large terminal output on Windows: prefer `brainforge/tools/rtk/rtk.exe`.

## Copilot note

No native `/brainforge` slash — user may say **"BrainForge on"** or open `brainforge/core/BRAINFORGE.md` in context.
