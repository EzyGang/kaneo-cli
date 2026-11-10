pub fn success(msg: &str) {
    let green = console::style("✓").green().bold();
    eprintln!("  {green} {msg}");
}

pub fn warn(msg: &str) {
    let yellow = console::style("⚠").yellow();
    eprintln!("  {yellow} {msg}");
}

pub fn header(title: &str) {
    let bold = console::style(title).bold();
    eprintln!("\n{bold}");
}

pub fn dim(text: &str) -> console::StyledObject<&str> {
    console::style(text).color256(245)
}

/// Prints the note shown when a destructive command runs without `--force`.
pub fn force_required() {
    eprintln!(
        "  {} Re-run with `--force` to confirm deletion",
        dim("note:")
    );
}

/// Shortens `text` to at most `width` characters, marking cut text with an ellipsis.
#[must_use]
pub fn truncate(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_owned();
    }
    let kept: String = text.chars().take(width.saturating_sub(1)).collect();
    format!("{kept}…")
}

/// Returns the calendar day of an ISO timestamp, or `-` when absent.
#[must_use]
pub fn day(timestamp: Option<&str>) -> &str {
    match timestamp {
        Some(value) => value.get(..10).unwrap_or(value),
        None => "-",
    }
}

/// Returns an ISO timestamp as `YYYY-MM-DD HH:MM`, keeping unparseable input as is.
#[must_use]
pub fn minute(timestamp: &str) -> String {
    timestamp
        .get(..16)
        .unwrap_or(timestamp)
        .replacen('T', " ", 1)
}
