# BrainForge

[English](README.md) · **Português (Brasil)**

Kit leve que ensina o assistente de IA a trabalhar do seu jeito: memória do projeto, respostas diretas e skills prontas. Funciona com **Cursor**, **GitHub Copilot** e **Antigravity**.

---

## Instalar no seu projeto

```powershell
# 1) Compilar uma vez (neste repositório)
cargo build --release

# 2) Na pasta DO SEU projeto
C:\caminho\para\BrainForge\target\release\brainforge.exe init
```

O `init` copia o kit, pergunta qual IDE você usa e configura tudo.

**Todas as IDEs, sem menu:**

```powershell
brainforge init --adapter all --no-menu
```

**Conferir instalação:**

```powershell
brainforge init --show
```

---

## Usar no Cursor

No chat, digite:

```text
/brainforge
```

Para desligar: `modo normal` ou `stop brainforge`.

---

## Depois de editar o kit

Edite `brainforge/` e rode:

```powershell
.\brainforge.exe sync
```

Memória fica em `brainforge/memory/` — não edite `.cursor/project/` na mão.

---

## Mais detalhes

| Assunto | Doc |
|---------|-----|
| Instalação e config | [brainforge/HOST-SETUP.md](brainforge/HOST-SETUP.md) |
| MCP (opcional) | [brainforge/core/docs/MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md) |

---

## Comandos

```powershell
brainforge init          # primeira instalação (recomendado)
brainforge sync          # atualizar arquivos da IDE
brainforge doctor        # verificar saúde
brainforge memory read   # ver memória
```
