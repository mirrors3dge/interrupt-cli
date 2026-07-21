//! Contains various useful terminal related functions/macros.

#[cfg(feature = "styled-print")]
pub mod styled_print;

use std::io::{self, Write as _};

/// Clears ANSI terminal.
#[expect(clippy::missing_panics_doc)]
pub fn clear_terminal() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().expect("failed to flush stdout");
}

/// Returns a formatted link printable to the terminal (like markdown `[text](link)`).
pub fn fmt_link(text: impl AsRef<str>, url: impl AsRef<str>) -> String {
    format!(
        "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
        url.as_ref(),
        text.as_ref()
    )
}
