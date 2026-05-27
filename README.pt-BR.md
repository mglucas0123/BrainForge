# BrainForge

[English](README.md) · **Português (Brasil)**

O BrainForge é um **kit portátil** que ensina o seu assistente de IA a trabalhar do seu jeito: respostas mais diretas, memória do projeto, habilidades úteis e menos desperdício de contexto. Funciona com **Cursor**, **GitHub Copilot** e **Antigravity** — você escolhe o que usa.

Pense numa pasta “cérebro” (`brainforge/`) mais um programinha (`brainforge.exe`) que copia os arquivos certos para o seu projeto.

---

## O que isso faz? (sem jargão)

| Parte | O que muda para você |
|--------|----------------------|
| **Memória** | O assistente lembra fatos *deste* projeto e *suas* preferências (`brainforge/memory/`). |
| **Modo caveman** | Respostas mais curtas e diretas no chat (ative com `/brainforge` no Cursor). |
| **Skills** | Instruções prontas para debug, refatoração, limpar memória, etc. |
| **Sync** | Um comando atualiza `.cursor/`, `.github/` ou `.agents/` a partir do kit — você edita `brainforge/`, não essas pastas na mão. |
| **Doctor** | Verifica se está tudo instalado certo. |

Você **não** precisa entender Rust nem MCP para o básico. Rust só entra se quiser **compilar** o CLI a partir do código.

---

## O que você precisa

- **Windows** (foco principal; exemplos em PowerShell)
- Uma IDE: **Cursor** recomendado para a experiência completa
- Opcional: [Rust](https://www.rust-lang.org/tools/install) — para gerar o `brainforge.exe`  
  (ou copie um `.exe` pronto de uma [release](https://github.com/mglucas0123/BrainForge/releases))

---

## Começo rápido (uns 5 minutos)

### 1. Obter o programa

**Opção A — compilar neste repositório**

```powershell
cargo build --release
```

O executável fica em `.\target\release\brainforge.exe`.

**Opção B — baixar release**

Baixe `brainforge.exe` na página [Releases](https://github.com/mglucas0123/BrainForge/releases) (quando houver).

### 2. Ligar o kit ao seu projeto

Abra o PowerShell **na pasta do seu projeto** (onde está o seu código) e rode:

```powershell
# Se você compilou no repo BrainForge, use o caminho completo:
C:\caminho\para\BrainForge\target\release\brainforge.exe sync

# Ou, se o brainforge.exe já está na raiz do projeto:
.\brainforge.exe sync
```

Um menu pergunta qual IDE você usa.  
Para pular o menu e ativar tudo:

```powershell
.\brainforge.exe sync --adapter all --no-menu
```

### 3. Ligar no chat

No **Cursor**, digite:

```text
/brainforge
```

O assistente entra no modo BrainForge (memória + estilo conciso).  
Para voltar ao normal: `modo normal` ou `stop brainforge`.

### 4. Conferir se funcionou

```powershell
.\brainforge.exe doctor
```

Se algo falhar, o comando indica o que falta.

---

## Uso do dia a dia

| Objetivo | Comando |
|----------|---------|
| Atualizar arquivos da IDE depois de editar o kit | `brainforge sync` |
| Ver memória do projeto | `brainforge memory read` |
| Só recalcular a linha de capacidade | `brainforge memory refresh` |
| Validar formato da memória (ex.: antes de commit) | `brainforge memory validate` |
| Buscar em chats antigos do agente | `brainforge recall search "palavra"` |

**Importante:** edite memória só em `brainforge/memory/` (`.context.md` = projeto, `.user.md` = suas preferências).  
Não edite `.cursor/project/` na mão — o `sync` copia da pasta canônica.

---

## Instalar em outro projeto

Você **não** precisa clonar este repositório inteiro em cada app.

No repo BrainForge (depois de `cargo build --release`):

```powershell
.\target\release\brainforge.exe install C:\caminho\do\seu-app --with-exe
```

Isso copia:

- a pasta do kit `brainforge/`  
- `brainforge.toml` (configuração)  
- arquivos dos adaptadores da IDE (via sync)

Depois abra esse projeto no Cursor e use `/brainforge`.

Mais detalhes: [brainforge/HOST-SETUP.md](brainforge/HOST-SETUP.md)

---

## Mapa de pastas (simples)

| Caminho | Você deve… |
|---------|------------|
| `brainforge/` | **Editar aqui** — regras, skills, memória, adaptadores |
| `brainforge/memory/` | Guardar notas do projeto e preferências para a IA |
| `.cursor/`, `.github/`, `.agents/` | **Geradas** — rode `brainforge sync`; não edite na mão |
| `crates/` | Código Rust (só se for desenvolver o BrainForge) |
| `.github/workflows/` | CI *deste* repo no GitHub — não é obrigatório nos seus apps |

---

## Preciso da pasta `.github`?

- **No seu app:** só se usar **GitHub Copilot** e tiver ativado esse adaptador no sync. Senão: `sync --adapter cursor,antigravity` e pronto.
- **Neste repositório:** `.github/workflows/` roda testes automáticos no GitHub. Nos seus projetos dá para usar `brainforge doctor` e `cargo test` localmente.

---

## MCP (opcional, usuário avançado)

MCP permite o Cursor chamar ferramentas do BrainForge (memória, doctor, rotina).  
Configuração: [brainforge/core/docs/MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md)

```powershell
.\brainforge.exe install . --print-mcp-config
```

Cole o trecho em `.cursor/mcp.json`.

---

## Copiar o kit na mão (sem `install`)

1. Copie a pasta `brainforge/` inteira para o projeto.  
2. Copie `brainforge.exe` para a raiz (ou `cargo install --path crates/brainforge-cli`).  
3. Em projeto **novo**, apague memória antiga se existir: `brainforge/memory/.context.md` (começar limpo).  
4. Rode `brainforge sync`.

---

## Legado

`brainforge/sync.ps1` está **obsoleto** — use `brainforge sync` (CLI Rust).

---

## Mais documentação

| Assunto | Arquivo |
|---------|---------|
| Instalação no host e config | [brainforge/HOST-SETUP.md](brainforge/HOST-SETUP.md) |
| Layout do kit (fonte) | [brainforge/README.md](brainforge/README.md) |
| Configuração MCP | [brainforge/core/docs/MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md) |
| Fases / roadmap Rust | [brainforge/core/docs/RUST-PHASES.md](brainforge/core/docs/RUST-PHASES.md) |

---

## Licença

Veja o arquivo de licença do repositório. Sugestões e PRs são bem-vindos — se algo na documentação não ficou claro, abra uma issue.
