# Host setup (BrainForge Rust)

Distribuição via **`brainforge install`** — não precisa clonar o repo BrainForge no projeto host.

## 1. Instalar no projeto host

**Recomendado** — uma linha na pasta do projeto host:

```powershell
cd C:\meu-projeto
iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)
```

Ou, a partir deste repo já compilado:

```powershell
cargo build --release
cd C:\meu-projeto
C:\path\to\BrainForge\target\release\brainforge.exe init
```

Alternativa explícita (sem menu; usa `[install]` do `brainforge.toml`):

```powershell
.\target\release\brainforge.exe install C:\meu-projeto --with-exe
```

| Flag | Efeito |
|------|--------|
| `--with-exe` | Copia o CLI como `brainforge.exe` na raiz do host |
| `--force` | Substitui `brainforge/` existente |
| `--no-sync` | Só copia kit + `brainforge.toml` (sem `.cursor/`) |
| `--print-mcp-config` | Imprime snippet MCP (sem copiar) |

Cria:

- `brainforge/` — kit completo (core, memory, adapters, RTK)
- `brainforge.toml` — config do host (se não existir)
- `.cursor/`, `.github/`, `.agents/` — via sync (adapters em `[install]`)

RTK ausente após cópia:

```powershell
powershell -ExecutionPolicy Bypass -File .\brainforge\tools\rtk\install-rtk-local.ps1 -Force
```

## 2. Config (`brainforge.toml`)

Modelo: `brainforge.toml.example` na raiz do repo BrainForge.

```toml
[brainforge]
memory_dir = "brainforge/memory"
caveman_level = "full"   # lite | full | ultra

[install]
cursor = true
copilot = true
antigravity = true

[mcp]
enabled = true
```

## 3. Memória (novo projeto)

```powershell
Remove-Item brainforge\memory\.context.md -ErrorAction SilentlyContinue
Copy-Item brainforge\memory\.context.md.example brainforge\memory\.context.md
```

Edite só **`brainforge/memory/`** — sync espelha em `.cursor/project/`.

## 4. Comandos CLI (host)

```powershell
.\brainforge.exe sync -a all --no-menu
.\brainforge.exe doctor
.\brainforge.exe memory read
.\brainforge.exe mcp   # stdio — ver brainforge/core/docs/MCP-SETUP.md
```

MCP no Cursor:

```powershell
.\brainforge.exe install . --print-mcp-config
```

Cole em `.cursor/mcp.json`.

## 5. Slash por IDE

| IDE | Ativar |
|-----|--------|
| **Cursor** | `/brainforge` |
| **Antigravity** | `/brainforge` |
| **Copilot** | “BrainForge on” + MCP ou `brainforge/core/BRAINFORGE.md` |

## 6. Atualizar kit

Edite **`brainforge/core/`** ou **`brainforge/adapters/`** → `brainforge sync`.

## Legado

`brainforge/sync.ps1` — **obsoleto**; use `brainforge sync`.

## Release GitHub

Tags `v*` geram `brainforge.exe` + `brainforge.exe.sha256` (workflow `release.yml`).
