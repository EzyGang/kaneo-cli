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
