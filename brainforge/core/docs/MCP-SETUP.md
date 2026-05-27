# BrainForge MCP — setup

Servidor **stdio** via `brainforge mcp`. Mesmas tools em Cursor, VS Code (Copilot) e Antigravity.

## Build

```powershell
cargo build --release -p brainforge-cli
```

Binário: `target/release/brainforge.exe` (ou `brainforge` no PATH após install).

## Tools

| Tool | Uso |
|------|-----|
| `brainforge_routine` | Rotina + caveman + cavemem (`level`: lite \| full \| ultra) |
| `memory_read` | `which`: context \| user |
| `memory_write` | append/replace § (validação de âncoras) |
| `memory_compress` | heurística local; `allow_merge` opcional |
| `doctor` | relatório JSON |
| `session_recall` | `mode`: list \| last \| search (+ `query`, `limit`) |

Memória canônica: `brainforge/memory/`. Após write/compress o servidor espelha em `.cursor/project/` quando aplicável.

`memory_write` roda **secret scan** (bloqueia API keys, tokens, private keys em §).

## Cursor

`.cursor/mcp.json` no projeto host:

```json
{
  "mcpServers": {
    "brainforge": {
      "command": "D:\\Projetos\\SeuProjeto\\target\\release\\brainforge.exe",
      "args": ["mcp"],
      "cwd": "D:\\Projetos\\SeuProjeto"
    }
  }
}
```

Ajuste `command` e `cwd` para o projeto. Se o kit não está na raiz, use `--kit`:

```json
"args": ["--kit", "D:\\caminho\\brainforge", "mcp"]
```

Reinicie o agente após salvar. No início da tarefa: `brainforge_routine` + `memory_read` (context).

## VS Code / GitHub Copilot

`.vscode/mcp.json` (ou User settings → MCP):

```json
{
  "servers": {
    "brainforge": {
      "type": "stdio",
      "command": "${workspaceFolder}/target/release/brainforge.exe",
      "args": ["mcp"],
      "cwd": "${workspaceFolder}"
    }
  }
}
```

Copilot: combine com `copilot-instructions.md` (rotina via MCP ou `brainforge doctor` no terminal).

## Antigravity

Config MCP do IDE (stdio), mesmo padrão:

- **command:** caminho absoluto para `brainforge.exe`
- **args:** `["mcp"]`
- **cwd:** raiz do workspace

Consulte a doc do Antigravity para o arquivo de config (equivalente a `mcp.json`).

## Variáveis de ambiente

| Var | Efeito |
|-----|--------|
| `BRAINFORGE_KIT` | Pasta `brainforge/` se não estiver na raiz do projeto |

## Troubleshooting

1. `brainforge doctor` no terminal — kit e espelhos OK?
2. MCP só fala JSON no **stdout**; logs vão em **stderr** (mensagem de startup).
3. Teste manual: `echo` não funciona bem; use o painel MCP do IDE.
4. Falha de path: confira `cwd` e `--kit`.

Ver também: `RUST-PHASES.md` (Fase 3), `MEMORY-FORMAT.md`.
