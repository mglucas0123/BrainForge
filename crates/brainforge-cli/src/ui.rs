//! Terminal UI — Claude Code–inspired welcome (BrainForge: cyan + amber).

use std::io::{self, Write};
use std::path::Path;

use console::{Style, Term, style};
use dialoguer::theme::ColorfulTheme;

/// Pixel CaveCrew pet (compact mascot).
pub const PET_PIXEL: &str = "      ▄▀▀▀▄\n      █ ▄ █\n      ▀▄▄▄▀\n       ║║";

const BOX_W: usize = 64;
const LEFT_W: usize = 31;
const RIGHT_W: usize = 28;

fn edge() -> console::StyledObject<&'static str> {
    style("│").cyan().bold()
}

fn h(s: &str) -> String {
    style(s).yellow().bold().to_string()
}

fn pad_to(s: &str, width: usize) -> String {
    let vis = console::measure_text_width(s);
    if vis >= width {
        s.to_string()
    } else {
        format!("{s}{}", " ".repeat(width - vis))
    }
}

fn pad_cols(left: &str, right: &str) -> String {
    format!(
        "{} {} {} {} {}",
        edge(),
        pad_to(left, LEFT_W),
        edge(),
        pad_to(right, RIGHT_W),
        edge()
    )
}

fn pad_full(text: &str) -> String {
    format!("{} {} {}", edge(), pad_to(text, BOX_W - 4), edge())
}

fn border_top(title: &str) -> String {
    let label = format!("─ {title} ");
    let used = console::measure_text_width(&label) + 2;
    let pad = BOX_W.saturating_sub(used.max(2));
    let inner = format!("{label}{}", "─".repeat(pad));
    format!("╭{}╮", style(inner).cyan().bold())
}

fn border_bottom() -> String {
    format!("╰{}╯", style("─".repeat(BOX_W - 2)).cyan().bold())
}

fn local_user() -> String {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "dev".into())
}

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

/// Bordered welcome panel (Claude Code layout, BrainForge palette).
pub fn print_welcome(project: &Path) {
    if !is_tty() {
        return;
    }
    clear_screen();

    let ver = env!("CARGO_PKG_VERSION");
    let user = local_user();
    let path = project.display().to_string();

    let welcome = format!("Bem-vindo, {}!", style(user).cyan().bold());
    let pet_lines: Vec<&str> = PET_PIXEL.lines().collect();
    let tips = [
        "Dicas para começar",
        "/brainforge  no Cursor",
        "Edite só .brainforge/",
        "brainforge sync  atualiza",
        "Espaço marca · Enter OK",
    ];

    println!();
    println!("{}", border_top(&format!("BrainForge v{ver}")));
    println!("{}", pad_full(""));
    println!("{}", pad_cols(&welcome, &h(tips[0])));
    println!("{}", pad_cols("", tips[1]));
    for (i, line) in pet_lines.iter().enumerate() {
        let left = format!("  {}", style(line).cyan().bold());
        let right = tips.get(i + 2).unwrap_or(&"");
        println!("{}", pad_cols(&left, right));
    }
    println!(
        "{}",
        pad_cols(&style("  CaveCrew · kit portátil").dim().to_string(), "")
    );
    println!(
        "{}",
        pad_cols(&style(format!("  {path}")).dim().to_string(), "")
    );
    println!("{}", pad_full(""));
    println!("{}", border_bottom());
    println!();
    println!(
        "  {} {}",
        style("›").cyan().bold(),
        style("Escolha onde instalar (IDE)").dim()
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

/// Compact status panel after IDE selection.
pub fn print_setup_header(adapters: &[brainforge_core::Adapter]) {
    if !is_tty() {
        return;
    }
    clear_screen();

    let names = adapters
        .iter()
        .map(|a| a.label())
        .collect::<Vec<_>>()
        .join(" · ");

    println!();
    println!("{}", border_top("BrainForge"));
    println!("{}", pad_full(""));
    println!(
        "{}",
        pad_cols(
            &style("Configurando projeto…").cyan().bold().to_string(),
            &h("IDEs selecionadas")
        )
    );
    println!("{}", pad_cols(&format!("  {names}"), "  sync · doctor · kit"));
    println!("{}", pad_full(""));
    println!("{}", border_bottom());
    println!();
    let _ = io::stdout().flush();
}
