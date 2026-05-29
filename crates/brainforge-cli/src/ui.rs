//! Terminal UI — Claude Code–inspired welcome (BrainForge: cyan + amber).

use std::io::{self, Write};
use console::{Style, Term, style};
use dialoguer::theme::ColorfulTheme;

/// CaveCrew pet — 5-line sprite; each row is exactly `PET_W` columns (monospace).
const PET_W: usize = 13;

/// CaveCrew pet (fixed-width rows for alignment in the left column).
pub const PET_PIXEL: &[&str] = &[
    "      ▲      ",
    "     ███     ",
    "    ▐███▌    ",
    "   ▄█████▄   ",
    "    ▀ ▀ ▀    ",
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

fn center_to(s: &str, width: usize) -> String {
    let vis = console::measure_text_width(s);
    if vis >= width {
        s.to_string()
    } else {
        let pad = width - vis;
        let left = pad / 2;
        format!("{}{}{}", " ".repeat(left), s, " ".repeat(pad - left))
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

fn pad_cols_center_both(left: &str, right: &str) -> String {
    debug_assert_eq!(LEFT_W + COL_SEP.len() + RIGHT_W, INNER_W);
    format!(
        "{}{}{}{}{}",
        edge(),
        center_to(left, LEFT_W),
        style(COL_SEP).cyan().bold(),
        center_to(right, RIGHT_W),
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

/// One pet row: pad to column width once, then color (avoids double-center drift).
fn pet_row(line: &str) -> String {
    debug_assert!(
        line.len() >= PET_W,
        "pet line must be at least {PET_W} chars: {line:?}"
    );
    style(center_to(line, LEFT_W)).cyan().bold().to_string()
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
pub fn print_welcome() {
    if !is_tty() {
        return;
    }
    clear_screen();

    let ver = env!("CARGO_PKG_VERSION");
    let welcome = style("Bem-vindo").cyan().bold().to_string();
    let tips_title = h("Dicas para começar");
    let tips_body = "Digite /brainforge no chat.";

    println!();
    println!("{}", border_top(&format!("BrainForge v{ver}")));
    println!("{}", pad_full(""));
    println!("{}", pad_cols_center_both(&welcome, &tips_title));
    println!("{}", pad_cols_center_both("", tips_body));
    println!("{}", pad_full(""));
    for pet_line in PET_PIXEL {
        println!("{}", pad_cols(&pet_row(pet_line), ""));
    }
    println!("{}", pad_full(""));
    println!("{}", border_bottom());
    println!();
    println!(
        "  {} {}",
        style(PROMPT_GLYPH).cyan().bold(),
        style("Escolha onde instalar").dim()
    );
    println!("     {}", style("Espaço marca · Enter OK").dim());
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
    for line in PET_PIXEL {
        println!("{}", pad_cols(&pet_row(line), ""));
    }
    println!("{}", pad_full(""));
    println!("{}", border_bottom());
    println!();
    let _ = io::stdout().flush();
}
