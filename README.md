# BrainForge

**English** · [Português (Brasil)](README.pt-BR.md)

Kit for your AI assistant: project memory, concise mode, and skills. **Cursor**, **Copilot**, **Antigravity**.

---

## Install (one line)

Run in **your project folder**:

```powershell
iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)
```

Downloads from GitHub release **v1.0.0**, then opens the IDE menu (`brainforge init`).

**All IDEs, no menu:**

```powershell
$env:BRAINFORGE_NO_MENU = "1"; iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)
```

**Alternate URL** (if Pages is slow):

```powershell
iex (irm https://raw.githubusercontent.com/mglucas0123/BrainForge/v1.0.0/bf.ps1 -UseBasicParsing)
```

---

## Cursor

```text
/brainforge
```

Off: `modo normal` or `stop brainforge`.

---

## Update kit files

```powershell
.\brainforge.exe sync
```

Memory: `brainforge/memory/` only.

---

## Docs

[HOST-SETUP.md](brainforge/HOST-SETUP.md) · [MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md) · [Releases](https://github.com/mglucas0123/BrainForge/releases)
