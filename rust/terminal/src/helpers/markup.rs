use std::sync::LazyLock;

use regex::Regex;

static TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(/?)(bold|dim|red|green|yellow)\]").unwrap());

/// rich's console markup (`[bold]…[/bold]`, `[red]…[/red]`) as ANSI codes, or stripped
/// when the output is not a terminal, as rich does.
pub fn render_markup(text: &str, is_terminal: bool) -> String {
    TAG.replace_all(text, |captures: &regex::Captures| {
        if !is_terminal {
            return String::new();
        }
        if &captures[1] == "/" {
            return "\u{1b}[0m".to_string();
        }
        match &captures[2] {
            "bold" => "\u{1b}[1m",
            "dim" => "\u{1b}[2m",
            "red" => "\u{1b}[31m",
            "green" => "\u{1b}[32m",
            _ => "\u{1b}[33m",
        }
        .to_string()
    })
    .into_owned()
}

/// rich's `Text.from_markup(text).plain`.
pub fn strip_markup(text: &str) -> String {
    render_markup(text, false)
}
