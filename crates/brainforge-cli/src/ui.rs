//! Terminal UI — welcome screen, pet mascot, Claude Code–style menus.

use std::io::{self, Write};
use std::path::Path;

use console::{Style, Term, style};
use dialoguer::theme::ColorfulTheme;

/// CaveCrew pet (terminal mascot).
pub const PET: &str = r"
      ╭──────────╮
      │  ◕    ◕  │
      │    ▽     │   CaveCrew
      ╰────┬─────╯
       ┌───┴───┐
       │ Brain │
       │ Forge │
       └───┬───┘
          ║║
";

pub fn is_tty() -> bool {
    Term::stdout().is_term()
}

pub fn clear_screen() {
    if !is_tty() {
        return;
    }
    let _ = Term::stdout().clear_screen();
}

pub fn welcome_enabled() -> bool {
    std::env::var("BRAINFORGE_WELCOME").as_deref() == Ok("1")
}

/// Full-screen welcome before the IDE picker (after bootstrap download).
pub fn print_welcome(project: &Path) {
    if !is_tty() {
        return;
    }
    clear_screen();
    let ver = env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        style(PET).cyan().bold()
    );
    println!(
        "  {}  {}",
        style("BrainForge").bold().underlined(),
        style(format!("v{ver}")).dim()
    );
    println!(
        "  {}",
        style("Kit de IA para Cursor · Copilot · Antigravity").dim()
    );
    println!();
    println!(
        "  {} {}",
        style("Projeto").dim(),
        style(project.display()).cyan()
    );
    println!();
    println!(
        "  {}",
        style("❯ Espaço alterna · Enter confirma · Esc cancela").dim()
    );
    println!();
    let _ = io::stdout().flush();
}

pub fn brainforge_theme() -> ColorfulTheme {
    ColorfulTheme {
        active_item_style: Style::new().cyan().bold(),
        inactive_item_style: Style::new().dim(),
        prompt_style: Style::new().cyan().bold(),
        ..ColorfulTheme::default()
    }
}

/// Header after IDE selection, before sync output.
pub fn print_setup_header(adapters: &[brainforge_core::Adapter]) {
    if !is_tty() {
        return;
    }
    clear_screen();
    println!("{}", style(PET).cyan());
    println!(
        "  {} {}",
        style("Configurando").bold(),
        adapters
            .iter()
            .map(|a| a.label())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
    let _ = io::stdout().flush();
}
