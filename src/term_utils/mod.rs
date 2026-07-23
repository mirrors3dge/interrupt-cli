//! Contains various functions and the [`ansi_codes`] module for working with the terminal.
//! Everything in this module only works with an ANSI terminal!
//!
//! Also exports [`println_red!`](`crate::println_red!`), [`println_bold!`](`crate::println_bold!`),
//! etc. macros (at the crate root) for convenience (although, not every style/color is covered).

pub mod ansi_codes;
use ansi_codes::*;

use std::io::{self, Write as _};

/// Clears the terminal screen.
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

/// Returns a String that will display in the given color when printed to the terminal.
pub fn fmt_color(rgb: (u8, u8, u8), text: impl AsRef<str>) -> String {
    let (r, g, b) = rgb;
    format!("\x1b[38;2;{r};{g};{b}m{}{RESET}", text.as_ref())
}

// --- BOLD --- //
/// Like [`print!`], but bold.
#[macro_export]
macro_rules! print_bold {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        print!("{BOLD}{}{NOT_BOLD}", format_args!($($arg)*))
    }};
}
/// Like [`println!`], but bold.
#[macro_export]
macro_rules! println_bold {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        println!("{BOLD}{}{NOT_BOLD}", format_args!($($arg)*))
    }};
}

// --- RED --- //
/// Like [`print!`], but in red.
#[macro_export]
macro_rules! print_red {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        print!("{RED}{}{RESET}", format_args!($($arg)*))
    }};
}
/// Like [`println!`], but in red.
#[macro_export]
macro_rules! println_red {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        println!("{RED}{}{RESET}", format_args!($($arg)*))
    }};
}

// --- YELLOW --- //
/// Like [`print!`], but in yellow.
#[macro_export]
macro_rules! print_yellow {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        print!("{YELLOW}{}{RESET}", format_args!($($arg)*))
    }};
}
/// Like [`println!`], but in yellow.
#[macro_export]
macro_rules! println_yellow {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        println!("{YELLOW}{}{RESET}", format_args!($($arg)*))
    }};
}

// --- GREEN --- //
/// Like [`print!`], but in green.
#[macro_export]
macro_rules! print_green {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        print!("{GREEN}{}{RESET}", format_args!($($arg)*))
    }};
}
/// Like [`println!`], but in green.
#[macro_export]
macro_rules! println_green {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        println!("{GREEN}{}{RESET}", format_args!($($arg)*))
    }};
}

// --- RGB --- //
/// Like [`print!`], but in the given rgb color passed first as a tuple:
///
/// `print_rgb!((255, 0, 0), "Hello, {}!", "world"); // prints in red`
#[macro_export]
macro_rules! print_rgb {
    ($rgb:expr) => {{
        use $crate::term_utils::ansi_codes::*;
        let (r, g, b): (u8, u8, u8) = $rgb;
        print!("\x1b[38;2;{r};{g};{b}m{RESET}")
    }};
    ($rgb:expr, $($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        let (r, g, b): (u8, u8, u8) = $rgb;
        print!("\x1b[38;2;{r};{g};{b}m{}{RESET}", format_args!($($arg)*))
    }};
}
/// Like [`println!`], but in the given rgb color passed first as a tuple:
///
/// `println_rgb!((255, 0, 0), "Hello, {}!", "world"); // prints in red`
#[macro_export]
macro_rules! println_rgb {
    ($rgb:expr) => {{
        use $crate::term_utils::ansi_codes::*;
        let (r, g, b): (u8, u8, u8) = $rgb;
        println!("\x1b[38;2;{r};{g};{b}m{RESET}")
    }};
    ($rgb:expr, $($arg:tt)*) => {{
        use $crate::term_utils::ansi_codes::*;
        let (r, g, b): (u8, u8, u8) = $rgb;
        println!("\x1b[38;2;{r};{g};{b}m{}{RESET}", format_args!($($arg)*))
    }};
}
