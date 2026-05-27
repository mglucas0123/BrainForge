---
description: Install optional BrainForge skill from catalog into .cursor/skills/.
---

# Install Skill

**Prefer CLI** (if `brainforge` available):

```powershell
brainforge skill list
brainforge skill install <id>
brainforge skill install <id> --force
```

Otherwise run skill `install-skill`:

1. Parse skill `id` from user message (e.g. `/install-skill writing-plans` → `writing-plans`).
2. If no id: print catalog from `brainforge/core/skills-catalog.json`.
3. Follow `brainforge/core/skills/install-skill/SKILL.md` (writes to `.cursor/skills/`).
4. Report success or failure in **pt-BR**.
