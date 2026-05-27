# Cursor Hooks — BrainForge

Opt-in automation via [Cursor Hooks](https://cursor.com/docs/agent/hooks).

## Install no projeto

Após `brainforge sync`, existe:

```
.cursor/hooks.example/
  hooks.json
  session-start.ps1
```

Para ativar:

1. Copie para `.cursor/hooks/` (ou mescle em `.cursor/hooks.json`).
2. Ajuste o caminho do `brainforge.exe` no script se necessário.
3. Recarregue a janela do Cursor.

## Exemplo: sessionStart

O script de exemplo roda:

```powershell
brainforge memory refresh --file context
```

Isso só realinha o header **Capacity:** — não comprime § (seguro).

Para compress no início da sessão (mais agressivo), troque por:

```powershell
brainforge memory compress --file context --sync
```

## Variáveis

| Var | Uso |
|-----|-----|
| `BRAINFORGE_TRANSCRIPTS` | Override do diretório agent-transcripts |
| `CURSOR_HOME` | Override de `~/.cursor` |

## Recall de sessões (CLI)

```powershell
brainforge recall list
brainforge recall last --lines 20
brainforge recall search "fase 6"
```

MCP: tool `session_recall` (`mode`: list \| last \| search).
