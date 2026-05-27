use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use brainforge_core::{
    Adapter, CompressResult, DoctorStatus, InstallOptions, KitPaths, MemoryAuditReport,
    MemoryStats, MemoryTarget, WriteSkipReason, audit_memory, compress_memory, format_mcp_config_json,
    discover_source_kit, discover_transcripts_dir, extract_text_lines, install_optional_skill,
    list_sessions, load_catalog, read_memory, refresh_memory, run_doctor, run_init, run_install,
    run_sync, run_uninstall, search_transcripts, sync_memory_to_cursor, InitOptions,
};
use clap::{Parser, Subcommand, ValueEnum};
use console::style;
use dialoguer::{MultiSelect, theme::ColorfulTheme};

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("BRAINFORGE_GIT_DESC"),
    ")"
);

#[derive(Parser)]
#[command(
    name = "brainforge",
    version,
    long_version = LONG_VERSION,
    about = "BrainForge — sync agent kit to Cursor, Copilot, Antigravity"
)]
struct Cli {
    /// Project directory (default: current)
    #[arg(short, long, global = true)]
    target: Option<PathBuf>,

    /// Path to brainforge/ kit (default: discover or BRAINFORGE_KIT)
    #[arg(short, long, global = true)]
    kit: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync kit adapters into the project
    Sync {
        /// Adapters to sync (comma-separated: cursor,copilot,antigravity). Omit for interactive menu.
        #[arg(short, long, value_delimiter = ',')]
        adapter: Vec<AdapterArg>,

        /// Skip interactive menu when no --adapter given (sync all)
        #[arg(long)]
        no_menu: bool,

        /// Write slash commands from compile-time embed if kit commands/ missing
        #[arg(long)]
        embed_commands: bool,
    },
    /// Health check for kit and project outputs
    Doctor,
    /// Print resolved kit and project paths
    Paths,
    /// Read or compress memory files
    Memory {
        #[command(subcommand)]
        action: MemoryAction,
    },
    /// MCP stdio server (Cursor, VS Code, Antigravity)
    Mcp,
    /// Bootstrap host project: kit + IDE menu + sync + doctor (recommended)
    Init {
        /// Adapters (cursor,copilot,antigravity). Omit for interactive menu.
        #[arg(short, long, value_delimiter = ',')]
        adapter: Vec<AdapterArg>,

        /// Skip IDE menu when no --adapter (use all adapters)
        #[arg(long)]
        no_menu: bool,

        /// Verify install (doctor only)
        #[arg(long)]
        show: bool,

        /// Remove adapter outputs created by sync
        #[arg(long)]
        uninstall: bool,

        /// Replace existing brainforge/ kit
        #[arg(long)]
        force: bool,

        /// Do not copy this CLI as brainforge.exe in the project
        #[arg(long)]
        no_exe: bool,

        /// Embed slash commands if kit commands/ is missing
        #[arg(long)]
        embed_commands: bool,
    },
    /// Copy kit + optional exe into a host project; writes brainforge.toml; runs sync
    Install {
        /// Target project directory
        path: PathBuf,

        /// Replace existing brainforge/ kit
        #[arg(long)]
        force: bool,

        /// Skip adapter sync after copy
        #[arg(long)]
        no_sync: bool,

        /// Copy this CLI binary as brainforge.exe in the target project
        #[arg(long)]
        with_exe: bool,

        /// Print .cursor/mcp.json snippet and exit (no copy)
        #[arg(long)]
        print_mcp_config: bool,
    },
    /// Optional skills from catalog
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
    /// Canonical routine helpers
    Prompt {
        /// Copy BRAINFORGE.md to system clipboard
        #[arg(long)]
        copy: bool,
    },
    /// Print version and optional git describe
    Version,
    /// Search Cursor agent transcripts for this project
    Recall {
        #[command(subcommand)]
        action: RecallAction,
    },
}

#[derive(Subcommand)]
enum RecallAction {
    /// List transcript session ids (newest first)
    List,
    /// Show last N message excerpts from the newest session
    Last {
        #[arg(short, long, default_value = "15")]
        lines: usize,
    },
    /// Search all sessions for a keyword
    Search {
        query: String,
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

#[derive(Subcommand)]
enum SkillAction {
    /// Install optional skill into .cursor/skills/
    Install {
        /// Skill id from skills-catalog.json
        id: String,

        /// Replace existing .cursor/skills/<id>/
        #[arg(long)]
        force: bool,
    },
    /// List optional skill ids in catalog
    List,
}

#[derive(Subcommand)]
enum MemoryAction {
    /// Print memory entries and capacity stats
    Read {
        /// Which file to read (default: both)
        #[arg(short, long, default_value = "both")]
        file: MemoryFileArg,
    },
    /// Compress memory (local heuristic: filler removal + duplicate merge).
    ///
    /// Heuristic only — does not replace semantic /compress-context in chat.
    /// Skips disk write when header is accurate and § text is unchanged.
    Compress {
        /// Which file to compress (default: both)
        #[arg(short, long, default_value = "both")]
        file: MemoryFileArg,

        /// Preview changes without writing
        #[arg(long)]
        dry_run: bool,

        /// Sync to .cursor/project/ after compress
        #[arg(long)]
        sync: bool,

        /// Allow fewer § entries after duplicate merge (default: block + fail validation)
        #[arg(long)]
        allow_merge: bool,
    },
    /// Recompute **Capacity:** only (no filler removal or § merge).
    Refresh {
        /// Which file to refresh (default: both)
        #[arg(short, long, default_value = "both")]
        file: MemoryFileArg,

        /// Preview without writing
        #[arg(long)]
        dry_run: bool,

        /// Sync to .cursor/project/ after refresh
        #[arg(long)]
        sync: bool,
    },
    /// Validate memory files (format, header, mirror); exit 1 on errors
    Validate {
        /// Which file to validate (default: both)
        #[arg(short, long, default_value = "both")]
        file: MemoryFileArg,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum MemoryFileArg {
    Context,
    User,
    Both,
}

impl MemoryFileArg {
    fn targets(self) -> Vec<MemoryTarget> {
        match self {
            MemoryFileArg::Context => vec![MemoryTarget::ContextMd],
            MemoryFileArg::User => vec![MemoryTarget::UserMd],
            MemoryFileArg::Both => MemoryTarget::ALL.to_vec(),
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum AdapterArg {
    Cursor,
    Copilot,
    Antigravity,
    All,
}

impl From<AdapterArg> for Vec<Adapter> {
    fn from(v: AdapterArg) -> Self {
        match v {
            AdapterArg::All => Adapter::ALL.to_vec(),
            AdapterArg::Cursor => vec![Adapter::Cursor],
            AdapterArg::Copilot => vec![Adapter::Copilot],
            AdapterArg::Antigravity => vec![Adapter::Antigravity],
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Init {
        adapter,
        no_menu,
        show,
        uninstall,
        force,
        no_exe,
        embed_commands,
    } = &cli.command
    {
        return cmd_init(InitCmd {
            target: cli.target.clone(),
            kit: cli.kit.as_deref(),
            adapter_args: adapter,
            no_menu: *no_menu,
            show: *show,
            uninstall: *uninstall,
            force: *force,
            no_exe: *no_exe,
            embed_commands: *embed_commands,
        });
    }

    if let Commands::Install {
        path,
        force,
        no_sync,
        with_exe,
        print_mcp_config,
    } = &cli.command
    {
        return cmd_install(
            path.clone(),
            cli.kit.as_deref(),
            *force,
            *no_sync,
            *with_exe,
            *print_mcp_config,
        );
    }

    let project = cli
        .target
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("cwd"));
    let kit = cli.kit.as_deref();
    let paths = KitPaths::resolve(&project, kit)?;

    match cli.command {
        Commands::Sync {
            adapter,
            no_menu,
            embed_commands,
        } => {
            let adapters = resolve_adapters(&adapter, no_menu)?;
            println!(
                "{} {}",
                style("BrainForge sync →").cyan().bold(),
                paths.project_root.display()
            );
            println!(
                "{} {}",
                style("Kit:").dim(),
                paths.kit_root.display()
            );
            println!(
                "{} {}",
                style("Adapters:").dim(),
                adapters
                    .iter()
                    .map(|a| a.label())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            println!();
            run_sync(&paths, &adapters, embed_commands)?;
            println!();
            println!("{}", style("Concluído.").cyan());
        }
        Commands::Doctor => {
            let report = run_doctor(&paths)?;
            print_doctor(&report);
            if report.has_fail() {
                bail!("doctor: falhas encontradas");
            }
        }
        Commands::Paths => {
            println!("project: {}", paths.project_root.display());
            println!("kit:     {}", paths.kit_root.display());
            println!("rtk:     {}", paths.rtk_exe().display());
        }
        Commands::Memory { action } => match action {
            MemoryAction::Read { file } => {
                cmd_memory_read(&paths, file)?;
            }
            MemoryAction::Compress {
                file,
                dry_run,
                sync,
                allow_merge,
            } => {
                cmd_memory_compress(&paths, file, dry_run, sync, allow_merge)?;
            }
            MemoryAction::Refresh {
                file,
                dry_run,
                sync,
            } => {
                cmd_memory_write_op(&paths, file, dry_run, sync, MemoryWriteOp::Refresh)?;
            }
            MemoryAction::Validate { file } => {
                let failed = cmd_memory_validate(&paths, file)?;
                if failed {
                    std::process::exit(1);
                }
            }
        },
        Commands::Mcp => {
            eprintln!(
                "{} {}",
                style("BrainForge MCP stdio").cyan().bold(),
                paths.kit_root.display()
            );
            brainforge_mcp::run_mcp_server_blocking(paths)?;
        }
        Commands::Skill { action } => match action {
            SkillAction::Install { id, force } => cmd_skill_install(&paths, &id, force)?,
            SkillAction::List => cmd_skill_list(&paths)?,
        },
        Commands::Prompt { copy } => {
            if copy {
                cmd_prompt_copy(&paths)?;
            } else {
                bail!("use: brainforge prompt --copy");
            }
        }
        Commands::Version => {
            println!("brainforge {LONG_VERSION}");
        }
        Commands::Recall { action } => match action {
            RecallAction::List => cmd_recall_list(&paths)?,
            RecallAction::Last { lines } => cmd_recall_last(&paths, lines)?,
            RecallAction::Search { query, limit } => cmd_recall_search(&paths, &query, limit)?,
        },
        Commands::Init { .. } => unreachable!("handled above"),
        Commands::Install { .. } => unreachable!("handled above"),
    }

    Ok(())
}

fn resolve_source_kit(kit_override: Option<&std::path::Path>) -> Result<PathBuf> {
    discover_source_kit(kit_override)
}

struct InitCmd<'a> {
    target: Option<PathBuf>,
    kit: Option<&'a std::path::Path>,
    adapter_args: &'a [AdapterArg],
    no_menu: bool,
    show: bool,
    uninstall: bool,
    force: bool,
    no_exe: bool,
    embed_commands: bool,
}

fn cmd_init(cmd: InitCmd<'_>) -> Result<()> {
    let project = cmd
        .target
        .unwrap_or_else(|| std::env::current_dir().expect("cwd"))
        .canonicalize()
        .context("project directory")?;

    if cmd.show {
        let paths = KitPaths::resolve(&project, None)?;
        println!(
            "{} {}",
            style("BrainForge init --show").cyan().bold(),
            project.display()
        );
        let report = run_doctor(&paths)?;
        print_doctor(&report);
        if report.has_fail() {
            bail!("doctor: falhas encontradas");
        }
        return Ok(());
    }

    let adapters = resolve_adapters(cmd.adapter_args, cmd.no_menu)?;

    if cmd.uninstall {
        println!(
            "{} {}",
            style("BrainForge uninstall →").yellow().bold(),
            project.display()
        );
        run_uninstall(&project, &adapters)?;
        println!();
        println!("{}", style("Removido. O kit .brainforge/ não foi apagado.").dim());
        return Ok(());
    }

    let source_kit = discover_source_kit(cmd.kit)?;
    println!(
        "{} {}",
        style("BrainForge init →").cyan().bold(),
        project.display()
    );
    println!("{} {}", style("Kit source:").dim(), source_kit.display());
    println!(
        "{} {}",
        style("IDEs:").dim(),
        adapters
            .iter()
            .map(|a| a.label())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();

    let exe_source = if cmd.no_exe {
        None
    } else {
        Some(std::env::current_exe().context("current_exe")?)
    };

    let report = run_init(
        &source_kit,
        &project,
        exe_source.as_deref(),
        &adapters,
        InitOptions {
            force_kit: cmd.force,
            copy_exe: !cmd.no_exe,
            embed_commands: cmd.embed_commands,
        },
    )?;

    if report.kit_installed {
        println!("{} .brainforge/", style("OK").green());
    }
    if report.exe_copied {
        println!("{} brainforge.exe", style("OK").green());
    }
    if report.config_updated {
        println!("{} brainforge.toml", style("OK").green());
    }
    println!(
        "{} sync: {}",
        style("OK").green(),
        report
            .adapters_synced
            .iter()
            .map(|a| a.label())
            .collect::<Vec<_>>()
            .join(", ")
    );

    print_doctor(&report.doctor);

    println!();
    print_init_next_steps(&project, &adapters);

    if report.doctor.has_fail() {
        bail!("init concluído com falhas no doctor");
    }

    Ok(())
}

fn print_init_next_steps(project: &std::path::Path, adapters: &[Adapter]) {
    println!("{}", style("Próximos passos:").cyan().bold());
    println!(
        "  {}",
        style("Edite só .brainforge/ — .cursor/ e .agents/ são espelhos gerados pela IDE.").dim()
    );
    if adapters.contains(&Adapter::Cursor) || adapters.contains(&Adapter::Antigravity) {
        println!("  1. Abra o projeto no Cursor e digite: {}", style("/brainforge").green());
    }
    if adapters.contains(&Adapter::Copilot) {
        println!("  2. Copilot: use “BrainForge on” ou leia .brainforge/core/BRAINFORGE.md");
    }
    println!(
        "  · Verificar de novo: {}",
        style("brainforge init --show").dim()
    );
    println!(
        "  · MCP (opcional): {} --print-mcp-config",
        style(format!(
            "{} install .",
            project.join("brainforge.exe").display()
        ))
        .dim()
    );
    println!(
        "  · RTK (opcional): {}",
        style(r"brainforge\tools\rtk\install-rtk-local.ps1").dim()
    );
}

fn cmd_install(
    path: PathBuf,
    kit_override: Option<&std::path::Path>,
    force: bool,
    no_sync: bool,
    with_exe: bool,
    print_mcp_config: bool,
) -> Result<()> {
    let target = path
        .canonicalize()
        .with_context(|| format!("target project {}", path.display()))?;

    if print_mcp_config {
        let bundled = target.join("brainforge.exe");
        let exe = if bundled.is_file() {
            bundled
        } else {
            std::env::current_exe().context("current_exe")?
        };
        println!("{}", format_mcp_config_json(&target, &exe)?);
        return Ok(());
    }

    let source_kit = resolve_source_kit(kit_override)?;
    println!(
        "{} {}",
        style("BrainForge install →").cyan().bold(),
        target.display()
    );
    println!("{} {}", style("Kit source:").dim(), source_kit.display());

    let exe_source = if with_exe {
        Some(std::env::current_exe().context("current_exe")?)
    } else {
        None
    };

    let report = run_install(
        &source_kit,
        &target,
        exe_source.as_deref(),
        InstallOptions {
            force,
            copy_exe: with_exe,
            run_sync: !no_sync,
        },
    )?;

    if report.kit_copied {
        println!("{} .brainforge/", style("OK").green());
    }
    if report.exe_copied {
        println!("{} brainforge.exe", style("OK").green());
    }
    if report.config_created {
        println!("{} brainforge.toml (template)", style("OK").green());
    }
    if report.sync_ran {
        println!(
            "{} sync: {}",
            style("OK").green(),
            report
                .adapters_synced
                .iter()
                .map(|a| a.label())
                .collect::<Vec<_>>()
                .join(", ")
        );
    } else if !no_sync {
        println!("{}", style("sync skipped (no adapters enabled)").yellow());
    }

    println!();
    println!(
        "{}",
        style("Próximo: brainforge doctor · MCP: install --print-mcp-config")
            .dim()
    );
    Ok(())
}

fn cmd_skill_install(paths: &KitPaths, id: &str, force: bool) -> Result<()> {
    let report = install_optional_skill(paths, id, force)?;
    println!(
        "{} skill '{}' → {}",
        style("OK").green(),
        report.id,
        report.target.display()
    );
    println!(
        "{}",
        style("Recarregue a janela do Cursor se o skill não aparecer.").dim()
    );
    Ok(())
}

fn cmd_skill_list(paths: &KitPaths) -> Result<()> {
    let catalog = load_catalog(paths)?;
    if catalog.optional_skills.is_empty() {
        println!("Nenhum skill optional no catálogo.");
        return Ok(());
    }
    println!("{}", style("Optional skills (catalog):").cyan().bold());
    for entry in &catalog.optional_skills {
        println!(
            "  {} — {} [{}]",
            style(&entry.id).green(),
            entry.description,
            entry.source_path
        );
    }
    Ok(())
}

fn transcripts_dir(paths: &KitPaths) -> Result<std::path::PathBuf> {
    discover_transcripts_dir(&paths.project_root)?
        .with_context(|| {
            "agent-transcripts not found — set BRAINFORGE_TRANSCRIPTS or open project in Cursor first"
        })
}

fn cmd_recall_list(paths: &KitPaths) -> Result<()> {
    let dir = transcripts_dir(paths)?;
    println!(
        "{} {}",
        style("Transcripts:").cyan().bold(),
        dir.display()
    );
    for s in list_sessions(&dir)? {
        println!(
            "  {}  ({} lines)",
            style(&s.id).green(),
            s.line_count
        );
    }
    Ok(())
}

fn cmd_recall_last(paths: &KitPaths, lines: usize) -> Result<()> {
    let dir = transcripts_dir(paths)?;
    let sessions = list_sessions(&dir)?;
    let Some(session) = sessions.first() else {
        println!("Nenhuma sessão encontrada.");
        return Ok(());
    };
    println!("{} {}", style("Session:").dim(), session.id);
    for (role, text) in extract_text_lines(&session.path, lines)? {
        println!("\n{} {}", style(&role).cyan(), "─".repeat(40));
        println!("{text}");
    }
    Ok(())
}

fn cmd_recall_search(paths: &KitPaths, query: &str, limit: usize) -> Result<()> {
    let dir = transcripts_dir(paths)?;
    let hits = search_transcripts(&dir, query, limit)?;
    if hits.is_empty() {
        println!("Nenhum resultado para '{query}'.");
        return Ok(());
    }
    for (id, line, preview) in hits {
        println!("{}:{} {}", style(id).green(), line, preview);
    }
    Ok(())
}

fn cmd_prompt_copy(paths: &KitPaths) -> Result<()> {
    let routine = paths.core().join("BRAINFORGE.md");
    let text = std::fs::read_to_string(&routine)
        .with_context(|| format!("read {}", routine.display()))?;
    let mut clipboard = arboard::Clipboard::new().context("open system clipboard")?;
    clipboard
        .set_text(&text)
        .context("copy to clipboard (requires display session on Windows)")?;
    println!(
        "{} rotina copiada ({} chars) de {}",
        style("OK").green(),
        text.len(),
        routine.display()
    );
    Ok(())
}

// ── Memory commands ─────────────────────────────────────────────────

fn cmd_memory_read(paths: &KitPaths, file: MemoryFileArg) -> Result<()> {
    println!();
    println!("{}", style("BrainForge Memory").cyan().bold());
    println!("{}", "─".repeat(52));

    for target in file.targets() {
        let filepath = target.path(paths);
        if !filepath.is_file() {
            println!(
                "{} {} — {}",
                style(target.label()).bold(),
                style("MISSING").red(),
                filepath.display()
            );
            continue;
        }

        let (mem, stats) = read_memory(paths, target)?;
        print_memory_stats(&stats, &mem);
    }

    println!();
    Ok(())
}

enum MemoryWriteOp {
    Refresh,
}

fn cmd_memory_compress(
    paths: &KitPaths,
    file: MemoryFileArg,
    dry_run: bool,
    do_sync: bool,
    allow_merge: bool,
) -> Result<()> {
    println!();
    println!(
        "{} {}",
        style("BrainForge Memory Compress").cyan().bold(),
        if dry_run {
            style("(dry-run)").yellow().to_string()
        } else {
            String::new()
        }
    );
    if !allow_merge {
        println!(
            "  {}",
            style("merge guard on — use --allow-merge to accept fewer §").dim()
        );
    }
    println!("{}", "─".repeat(52));

    let mut any_written = false;
    for target in file.targets() {
        let filepath = target.path(paths);
        if !filepath.is_file() {
            println!(
                "{} {} — {}",
                style(target.label()).bold(),
                style("MISSING").red(),
                filepath.display()
            );
            continue;
        }
        let result = compress_memory(paths, target, dry_run, allow_merge)?;
        print_memory_write_result(&result);
        if result.written {
            any_written = true;
        }
    }
    if do_sync && any_written {
        sync_memory_to_cursor(paths)?;
        println!("\n{} .cursor/project/ synced", style("↳").cyan());
    }
    println!();
    Ok(())
}

fn cmd_memory_validate(paths: &KitPaths, file: MemoryFileArg) -> Result<bool> {
    println!();
    println!("{}", style("BrainForge Memory Validate").cyan().bold());
    println!("{}", "─".repeat(52));

    let mut any_error = false;

    for target in file.targets() {
        let report = audit_memory(paths, target)?;
        print_audit_report(&report);
        if !report.ok() {
            any_error = true;
        }
    }

    println!();
    if any_error {
        println!("{}", style("Result: FAIL").red().bold());
    } else {
        println!("{}", style("Result: PASS").green().bold());
    }
    println!();

    Ok(any_error)
}

fn print_audit_report(report: &MemoryAuditReport) {
    println!();
    println!(
        "  {} {}",
        style(report.target.label()).bold(),
        report.target.filename()
    );
    for err in &report.errors {
        println!("  {} {}", style("ERROR").red(), err);
    }
    for warn in &report.warnings {
        println!("  {} {}", style("WARN").yellow(), warn);
    }
    if report.ok() && report.warnings.is_empty() {
        println!("  {}", style("OK").green());
    }
}

fn cmd_memory_write_op(
    paths: &KitPaths,
    file: MemoryFileArg,
    dry_run: bool,
    do_sync: bool,
    op: MemoryWriteOp,
) -> Result<()> {
    let title = "BrainForge Memory Refresh";

    println!();
    println!(
        "{} {}",
        style(title).cyan().bold(),
        if dry_run {
            style("(dry-run)").yellow().to_string()
        } else {
            String::new()
        }
    );
    println!("{}", "─".repeat(52));

    let mut any_written = false;

    for target in file.targets() {
        let filepath = target.path(paths);
        if !filepath.is_file() {
            println!(
                "{} {} — {}",
                style(target.label()).bold(),
                style("MISSING").red(),
                filepath.display()
            );
            continue;
        }

        let result = match op {
            MemoryWriteOp::Refresh => refresh_memory(paths, target, dry_run)?,
        };
        print_memory_write_result(&result);

        if result.written {
            any_written = true;
        }
    }

    if do_sync && any_written {
        sync_memory_to_cursor(paths)?;
        println!(
            "\n{} .cursor/project/ synced",
            style("↳").cyan()
        );
    }

    println!();
    Ok(())
}

fn print_memory_stats(stats: &MemoryStats, mem: &brainforge_core::MemoryFile) {
    let pct_style = if stats.capacity_pct >= 80 {
        style(format!("{}%", stats.capacity_pct)).red().bold()
    } else if stats.capacity_pct >= 60 {
        style(format!("{}%", stats.capacity_pct)).yellow()
    } else {
        style(format!("{}%", stats.capacity_pct)).green()
    };

    println!();
    println!("  {} {}", style(stats.target.label()).bold(), stats.target.filename());
    println!(
        "  {} {}/{} chars  {} entries",
        pct_style,
        stats.char_used,
        stats.char_limit,
        stats.entry_count,
    );
    println!(
        "  {} lines  {} bytes{}",
        stats.line_count,
        stats.byte_count,
        if stats.has_reference {
            "  (has ## Reference)"
        } else {
            ""
        }
    );
    if stats.exceeds_threshold {
        println!(
            "  {}",
            style("⚠ exceeds auto-compress threshold").yellow()
        );
    }
    if mem.header_stale() {
        println!(
            "  {}",
            style("⚠ header stale — run `brainforge memory refresh`").yellow()
        );
    }
}

fn print_memory_write_result(result: &CompressResult) {
    println!();
    println!("  {} {}", style(result.target.label()).bold(), result.target.filename());

    let saved = result
        .chars_before
        .saturating_sub(result.chars_after);
    let arrow = format!(
        "{} → {} chars (−{})",
        result.chars_before, result.chars_after, saved
    );

    if saved > 0 {
        println!("  {} {}", style("↓").green(), arrow);
    } else {
        println!("  {} {}", style("=").dim(), arrow);
    }

    println!(
        "  entries: {} → {}",
        result.entries_before, result.entries_after
    );

    if !result.validation_issues.is_empty() {
        println!("  {}", style("validation issues:").red());
        for issue in &result.validation_issues {
            println!("    • {}", style(issue).red());
        }
    }

    match result.skip_reason {
        Some(WriteSkipReason::Unchanged) => {
            println!("  {}", style("○ unchanged — not written").dim());
        }
        Some(WriteSkipReason::DryRun) => {
            println!("  {}", style("(dry-run — not written)").dim());
        }
        Some(WriteSkipReason::ValidationFailed) => {
            println!(
                "  {}",
                style("✗ not written (validation failed)").red()
            );
        }
        None if result.written => {
            println!("  {}", style("✓ written").green());
        }
        None => {
            println!("  {}", style("— not written").dim());
        }
    }
}

// ── Adapter resolution ──────────────────────────────────────────────

fn resolve_adapters(args: &[AdapterArg], no_menu: bool) -> Result<Vec<Adapter>> {
    if !args.is_empty() {
        let mut out = Vec::new();
        for a in args {
            out.extend(Vec::<Adapter>::from(*a));
        }
        out.sort_by_key(|a| match a {
            Adapter::Cursor => 0,
            Adapter::Copilot => 1,
            Adapter::Antigravity => 2,
        });
        out.dedup();
        return Ok(out);
    }
    if no_menu {
        return Ok(Adapter::ALL.to_vec());
    }
    interactive_adapters()
}

fn interactive_adapters() -> Result<Vec<Adapter>> {
    let labels: Vec<String> = vec![
        format!("{}  {}", Adapter::Cursor.label(), Adapter::Cursor.detail()),
        format!("{}  {}", Adapter::Copilot.label(), Adapter::Copilot.detail()),
        format!(
            "{}  {}",
            Adapter::Antigravity.label(),
            Adapter::Antigravity.detail()
        ),
        "Todos — Cursor + Copilot + Antigravity".to_string(),
    ];

    let defaults = vec![true, true, true, false];
    let theme = ColorfulTheme::default();

    let picked = MultiSelect::with_theme(&theme)
        .with_prompt("BrainForge — escolha IDE(s) (espaço marca, enter confirma)")
        .items(&labels)
        .defaults(&defaults)
        .interact()
        .context("menu cancelado")?;

    if picked.is_empty() {
        bail!("nenhuma IDE selecionada");
    }

    if picked.contains(&3) {
        return Ok(Adapter::ALL.to_vec());
    }

    let mut out = Vec::new();
    if picked.contains(&0) {
        out.push(Adapter::Cursor);
    }
    if picked.contains(&1) {
        out.push(Adapter::Copilot);
    }
    if picked.contains(&2) {
        out.push(Adapter::Antigravity);
    }

    if out.is_empty() {
        bail!("nenhuma IDE selecionada");
    }
    Ok(out)
}

fn print_doctor(report: &brainforge_core::DoctorReport) {
    println!();
    println!("{}", style("BrainForge Doctor").cyan().bold());
    println!("{}", "─".repeat(52));
    for c in &report.checks {
        let icon = match c.status {
            DoctorStatus::Pass => style("PASS").green(),
            DoctorStatus::Warn => style("WARN").yellow(),
            DoctorStatus::Fail => style("FAIL").red(),
        };
        println!("{:<22} {}  {}", c.name, icon, c.detail);
    }
    println!();
}
