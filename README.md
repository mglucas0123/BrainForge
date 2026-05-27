# BrainForge

**English** · [Português (Brasil)](README.pt-BR.md)

BrainForge is a **portable kit** that teaches your AI coding assistant how you like to work: concise replies, project memory, useful skills, and less wasted context. It works with **Cursor**, **GitHub Copilot**, and **Antigravity** — you pick which ones you use.

Think of it as a small “brain” folder (`brainforge/`) plus a helper program (`brainforge.exe`) that copies the right files into your project.

---

## What does it do? (in plain terms)

| Piece | What it means for you |
|--------|------------------------|
| **Memory** | The assistant remembers facts about *this* project and *your* preferences (in `brainforge/memory/`). |
| **Caveman mode** | Shorter, direct answers in chat (enable with `/brainforge` in Cursor). |
| **Skills** | Ready-made instructions for debugging, refactoring, memory cleanup, etc. |
| **Sync** | One command updates `.cursor/`, `.github/`, or `.agents/` from the kit — you edit `brainforge/`, not those folders by hand. |
| **Doctor** | Checks that everything is installed correctly. |

You **do not** need to understand Rust or MCP to use the basics. You only need Rust if you want to **build** the CLI from source.

---

## What you need

- **Windows** (primary target; PowerShell examples below)
- An IDE you use: **Cursor** recommended for the full experience
- Optional: [Rust](https://www.rust-lang.org/tools/install) — to compile `brainforge.exe` yourself  
  (or copy a pre-built `.exe` from a [release](https://github.com/mglucas0123/BrainForge/releases))

---

## Quick start (about 5 minutes)

### 1. Get the program

**Option A — build from this repo**

```powershell
cargo build --release
```

The executable will be at `.\target\release\brainforge.exe`.

**Option B — download a release**

Download `brainforge.exe` from the [Releases](https://github.com/mglucas0123/BrainForge/releases) page (when available).

### 2. Connect the kit to your project

Open PowerShell **in your project folder** (where your code lives) and run:

```powershell
# If you built from the BrainForge repo, use the full path to brainforge.exe:
C:\path\to\BrainForge\target\release\brainforge.exe sync

# Or, if brainforge.exe is already in your project folder:
.\brainforge.exe sync
```

A menu asks which IDE you use (Cursor, Copilot, Antigravity).  
To skip the menu and enable everything:

```powershell
.\brainforge.exe sync --adapter all --no-menu
```

### 3. Turn it on in the chat

In **Cursor**, type:

```text
/brainforge
```

The assistant switches to BrainForge mode (memory + concise style).  
To go back to normal chat, say: `modo normal` or `stop brainforge`.

### 4. Check that it worked

```powershell
.\brainforge.exe doctor
```

If something fails, the command tells you what is missing.

---

## Day-to-day use

| Goal | What to do |
|------|------------|
| Update IDE files after editing the kit | `brainforge sync` |
| See project memory | `brainforge memory read` |
| Refresh memory size line only | `brainforge memory refresh` |
| Validate memory format (e.g. before commit) | `brainforge memory validate` |
| Search past agent chats | `brainforge recall search "keyword"` |

**Important:** edit memory only in `brainforge/memory/` (`.context.md` = project, `.user.md` = your preferences).  
Do not edit `.cursor/project/` by hand — `sync` copies from the canonical folder.

---

## Install BrainForge on another project

You do **not** need to clone this whole repo into every app.

From the BrainForge repo (after `cargo build --release`):

```powershell
.\target\release\brainforge.exe install C:\path\to\your-app --with-exe
```

This copies:

- the `brainforge/` kit folder  
- `brainforge.toml` (settings)  
- IDE adapter files (via sync)

Then open that project in Cursor and run `/brainforge`.

More detail: [brainforge/HOST-SETUP.md](brainforge/HOST-SETUP.md)

---

## Folder map (simple)

| Path | You should… |
|------|-------------|
| `brainforge/` | **Edit here** — rules, skills, memory, adapters |
| `brainforge/memory/` | Keep project + user notes for the AI |
| `.cursor/`, `.github/`, `.agents/` | **Generated** — run `brainforge sync`; don’t hand-edit |
| `crates/` | Rust source (only if you develop BrainForge itself) |
| `.github/workflows/` | CI for *this* repo on GitHub — not required on your apps |

---

## Do I need the `.github` folder?

- **In your app:** only if you use **GitHub Copilot** and enabled that adapter in sync. Otherwise you can use `sync --adapter cursor,antigravity` and skip Copilot output.
- **In this repo:** `.github/workflows/` runs automated tests on GitHub. Your projects can rely on `brainforge doctor` and `cargo test` locally instead.

---

## MCP (optional, for power users)

MCP lets Cursor call BrainForge tools (memory, doctor, routine) over a protocol.  
Setup: [brainforge/core/docs/MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md)

```powershell
.\brainforge.exe install . --print-mcp-config
```

Paste the snippet into `.cursor/mcp.json`.

---

## Copy kit manually (without `install`)

1. Copy the entire `brainforge/` folder into your project.  
2. Copy `brainforge.exe` to the project root (or install via `cargo install --path crates/brainforge-cli`).  
3. Delete old memory if this is a **new** project: `brainforge/memory/.context.md` (start fresh).  
4. Run `brainforge sync`.

---

## Legacy

`brainforge/sync.ps1` is **deprecated** — use `brainforge sync` (Rust CLI).

---

## More documentation

| Topic | File |
|--------|------|
| Host install & config | [brainforge/HOST-SETUP.md](brainforge/HOST-SETUP.md) |
| Kit layout (source) | [brainforge/README.md](brainforge/README.md) |
| MCP setup | [brainforge/core/docs/MCP-SETUP.md](brainforge/core/docs/MCP-SETUP.md) |
| Rust roadmap / phases | [brainforge/core/docs/RUST-PHASES.md](brainforge/core/docs/RUST-PHASES.md) |

---

## License

See repository license file. Contributions welcome — open an issue or PR if something is unclear in the docs.
