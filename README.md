# BrainForge

**English** · [Português (Brasil)](README.pt-BR.md)

A small kit that teaches your AI assistant how you work: project memory, concise replies, and ready-made skills. Works with **Cursor**, **GitHub Copilot**, and **Antigravity**.

---

## Install in your project

```powershell
# 1) Build once (in this repo)
cargo build --release

# 2) In YOUR project folder
C:\path\to\BrainForge\target\release\brainforge.exe init
```

`init` copies the kit, asks which IDE you use, and sets everything up.

**All IDEs, no menu:**

```powershell
brainforge init --adapter all --no-menu
```

**Check install:**

```powershell
brainforge init --show
```

---

## Use in Cursor

Type in chat:

```text
/brainforge
```

To turn off: `modo normal` or `stop brainforge`.

---

## After you change the kit

Edit files under `brainforge/`, then:

```powershell
.\brainforge.exe sync
```

Memory lives in `brainforge/memory/` — not in `.cursor/project/`.

---

## More help

| Topic | Doc |
|--------|-----|
| Host setup & config | [brainforge/HOST-SETUP.md](brainforge/HOST-SETUP.md) |
| MCP (optional) | [brainforge/core/docs/MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md) |

---

## Commands

```powershell
brainforge init          # first setup (recommended)
brainforge sync          # refresh IDE files
brainforge doctor        # health check
brainforge memory read   # view memory
```
