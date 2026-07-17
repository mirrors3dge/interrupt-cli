//! Contains various useful functions that you can import with `utils::*`.

#[cfg(feature = "colored_print")]
mod colored_print;

use std::io::{self, Write as _};

/// Clears ANSI terminal.
#[expect(clippy::missing_panics_doc)]
pub fn clear_terminal() {
    print!("\u{1b}[2J\u{1b}[H");
    io::stdout().flush().expect("failed to flush stdout");
}

/// Returns (`formatted_link`, `len`):
/// - `formatted_link`: formatted string ready to be printed to the terminal
/// - `len`: number of formatted characters actually shown in the terminal
pub fn fmt_link(url: impl AsRef<str>, text: impl AsRef<str>) -> (String, usize) {
    let text: &str = text.as_ref();
    let formatted: String = format!(
        "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
        url.as_ref(),
        text
    );
    (formatted, text.chars().count())
}
