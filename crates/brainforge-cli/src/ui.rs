//! Terminal UI — Claude Code–inspired welcome (BrainForge: cyan + amber).

use std::io::{self, Write};
use std::path::Path;

use console::{Style, Term, style};
use dialoguer::theme::ColorfulTheme;

/// CaveCrew pet — fine pixel sprite (Claude-style: dome head, square eyes, 4 legs).
pub const PET_PIXEL: &[&str] = &[
    "     ▄▀▀▀▄",
    "    ▐█████▌",
    "    █ █▄█ █",
    "    ▐█████▌",
    "    ▄▀   ▀▄",
    "     █ █ █",
];

const BOX_W: usize = 64;
/// Inner width between side `│` (must equal LEFT_W + 3 + RIGHT_W).
const INNER_W: usize = BOX_W - 2;
const LEFT_W: usize = 30;
const RIGHT_W: usize = 29;
const COL_SEP: &str = " │ ";

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
    debug_assert_eq!(LEFT_W + COL_SEP.len() + RIGHT_W, INNER_W);
    format!(
        "{}{}{}{}{}",
        edge(),
        pad_to(left, LEFT_W),
        style(COL_SEP).cyan().bold(),
        pad_to(right, RIGHT_W),
        edge()
    )
}

fn pad_full(text: &str) -> String {
    format!("{}{}{}", edge(), pad_to(text, INNER_W), edge())
}

fn border_top(title: &str) -> String {
    let label = format!("─ {title} ");
    let label_w = console::measure_text_width(&label);
    let pad = INNER_W.saturating_sub(label_w);
    let inner = format!("{label}{}", "─".repeat(pad));
    format!("╭{}╮", style(inner).cyan().bold())
}

fn border_bottom() -> String {
    format!("╰{}╯", style("─".repeat(INNER_W)).cyan().bold())
}

/// Windows extended path (`\\?\`) stripped for display.
fn friendly_path(path: &Path) -> String {
    let s = path.display().to_string();
    s.strip_prefix(r"\\?\")
        .map(str::to_string)
        .unwrap_or(s)
}

fn truncate_vis(s: &str, max: usize) -> String {
    if console::measure_text_width(s) <= max {
        return s.to_string();
    }
    let mut out = String::from("…");
    for ch in s.chars().rev() {
        let candidate = format!("{ch}{out}");
        if console::measure_text_width(&candidate) > max {
            break;
        }
        out = candidate;
    }
    out
}

fn style_pet_line(line: &str) -> String {
    format!("  {}", style(line).cyan().bold())
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
    let path = truncate_vis(&friendly_path(project), LEFT_W.saturating_sub(2));

    let welcome = format!("Bem-vindo, {}!", style(user).cyan().bold());
    let tips = [
        "Dicas para começar",
        "/brainforge  no Cursor",
        "Edite só .brainforge/",
        "brainforge sync  atualiza",
        "Espaço marca · Enter OK",
        "",
    ];

    println!();
    println!("{}", border_top(&format!("BrainForge v{ver}")));
    println!("{}", pad_full(""));
    println!("{}", pad_cols(&welcome, &h(tips[0])));

    for (i, pet_line) in PET_PIXEL.iter().enumerate() {
        let left = style_pet_line(pet_line);
        let right = tips.get(i + 1).unwrap_or(&"");
        println!("{}", pad_cols(&left, right));
    }

    println!(
        "{}",
        pad_cols(
            &style("  CaveCrew · kit portátil").dim().to_string(),
            tips.get(6).unwrap_or(&"")
        )
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
        style(PROMPT_GLYPH).cyan().bold(),
        style("Escolha onde instalar (IDE)").dim()
    );
    println!();
    let _ = io::stdout().flush();
}

/// Menu prompt marker (Unicode — works in Windows Terminal / PowerShell 7).
pub const PROMPT_GLYPH: &str = "❯ ";

pub fn brainforge_theme() -> ColorfulTheme {
    ColorfulTheme {
        active_item_style: Style::new().cyan().bold(),
        inactive_item_style: Style::new().dim(),
        prompt_style: Style::new().cyan().bold(),
        prompt_prefix: Style::new().cyan().bold().apply_to(PROMPT_GLYPH.to_string()),
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
    // Mini pet (top 4 lines of full sprite)
    for line in PET_PIXEL.iter().take(4) {
        println!("{}", pad_cols(&style_pet_line(line), ""));
    }
    println!("{}", pad_full(""));
    println!("{}", border_bottom());
    println!();
    let _ = io::stdout().flush();
}
