# BrainForge

[English](README.md) · **Português (Brasil)**

Kit para o assistente de IA: memória do projeto, modo direto e skills. **Cursor**, **Copilot**, **Antigravity**.

---

## Instalar (uma linha)

Na **pasta do seu projeto**:

```powershell
iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)
```

Baixa a release **v1.0.4** e abre o menu de IDE (`brainforge init`).

**Todas as IDEs, sem menu:**

```powershell
$env:BRAINFORGE_NO_MENU = "1"; iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)
```

**URL alternativa:**

```powershell
iex (irm https://raw.githubusercontent.com/mglucas0123/BrainForge/v1.0.4/bf.ps1 -UseBasicParsing)
```

---

## Cursor

```text
/brainforge
```

Desligar: `modo normal` ou `stop brainforge`.

---

## Atualizar o kit

```powershell
.\brainforge.exe sync
```

Memória: só em `.brainforge/memory/` (`.cursor/` é espelho gerado).

---

## Docs

[HOST-SETUP.md](brainforge/HOST-SETUP.md) · [MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md) · [Releases](https://github.com/mglucas0123/BrainForge/releases)
