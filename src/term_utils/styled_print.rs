//! Provides constant corresponding to ANSI escape codes to print to the terminal with different
//! styles and colors. Import them all with `use interrupt_cli::term_utils::styled_print::*;`.
//!
//! Also exports [`println_red!`](`crate::println_red!`), [`println_bold!`](`crate::println_bold!`),
//! etc. macros (at the crate root) for convenience (although, not every style/color is covered).
//!
//! Note: bright color is the same as bold + color.
//!
//! ## Usage
//!
//! ```
//! use interrupt_cli::term_utils::styled_print::*;
//! use interrupt_cli::println_red;
//!
//! // these two lines are equivalent:
//! println!("{RED}This will be printed in red!{RESET}");
//! println_red!("This will be printed in red!");
//! ```

// --- STYLE --- //
/// Reset all styles and colors.
pub const RESET: &str = "\x1b[0m";

/// Set bold style.
pub const BOLD: &str = "\x1b[1m";
/// Unset bold style.
pub const NOT_BOLD: &str = "\x1b[22m";

/// Set dim style.
pub const DIM: &str = "\x1b[2m";
/// Unset dim style.
pub const NOT_DIM: &str = "\x1b[22m";

/// Set italic style.
pub const ITALIC: &str = "\x1b[3m";
/// Unset italic style.
pub const NOT_ITALIC: &str = "\x1b[23m";

/// Set underline style.
pub const UNDERLINE: &str = "\x1b[4m";
/// Unset underline style.
pub const NOT_UNDERLINE: &str = "\x1b[24m";

/// Set blinking style.
pub const BLINK: &str = "\x1b[5m";
/// Unset blinking style.
pub const NOT_BLINK: &str = "\x1b[25m";

/// Set reverse/inverse style.
pub const REV: &str = "\x1b[7m";
/// Unset reverse/inverse style.
pub const NOT_REV: &str = "\x1b[27m";

/// Set hidden style.
pub const HIDDEN: &str = "\x1b[8m";
/// Unset hidden style.
pub const NOT_HIDDEN: &str = "\x1b[28m";

/// Set strikethrough style.
pub const STRIKETHROUGH: &str = "\x1b[9m";
/// Unset strikethrough style.
pub const NOT_STRIKETHROUGH: &str = "\x1b[29m";

// --- COLORS --- //
/// Set font color to black.
pub const BLACK: &str = "\x1b[30m";
/// Set background color to black.
pub const BG_BLACK: &str = "\x1b[40m";
/// Set font color to bright black.
pub const BRIGHT_BLACK: &str = "\x1b[90m";
/// Set background color to bright black.
pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";

/// Set font color to red.
pub const RED: &str = "\x1b[31m";
/// Set background color to red.
pub const BG_RED: &str = "\x1b[41m";
/// Set font color to bright red.
pub const BRIGHT_RED: &str = "\x1b[91m";
/// Set background color to bright red.
pub const BG_BRIGHT_RED: &str = "\x1b[101m";

/// Set font color to green.
pub const GREEN: &str = "\x1b[32m";
/// Set background color to green.
pub const BG_GREEN: &str = "\x1b[42m";
/// Set font color to bright green.
pub const BRIGHT_GREEN: &str = "\x1b[92m";
/// Set background color to bright green.
pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";

/// Set font color to yellow.
pub const YELLOW: &str = "\x1b[33m";
/// Set background color to yellow.
pub const BG_YELLOW: &str = "\x1b[43m";
/// Set font color to bright yellow.
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
/// Set background color to bright yellow.
pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";

/// Set font color to blue.
pub const BLUE: &str = "\x1b[34m";
/// Set background color to blue.
pub const BG_BLUE: &str = "\x1b[44m";
/// Set font color to bright blue.
pub const BRIGHT_BLUE: &str = "\x1b[94m";
/// Set background color to bright blue.
pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";

/// Set font color to magenta.
pub const MAGENTA: &str = "\x1b[35m";
/// Set background color to magenta.
pub const BG_MAGENTA: &str = "\x1b[45m";
/// Set font color to bright magenta.
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
/// Set background color to bright magenta.
pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";

/// Set font color to cyan.
pub const CYAN: &str = "\x1b[36m";
/// Set background color to cyan.
pub const BG_CYAN: &str = "\x1b[46m";
/// Set font color to bright cyan.
pub const BRIGHT_CYAN: &str = "\x1b[96m";
/// Set background color to bright cyan.
pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";

/// Set font color to white.
pub const WHITE: &str = "\x1b[37m";
/// Set background color to white.
pub const BG_WHITE: &str = "\x1b[47m";
/// Set font color to bright white.
pub const BRIGHT_WHITE: &str = "\x1b[97m";
/// Set background color to bright white.
pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";

/// Set font color to default.
pub const DEFAULT: &str = "\x1b[39m";
/// Set background color to default.
pub const BG_DEFAULT: &str = "\x1b[49m";

// --- BOLD --- //
/// Like [`print!`], but bold.
#[macro_export]
macro_rules! print_bold {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::term_utils::styled_print::*;
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
        use $crate::term_utils::styled_print::*;
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
        use $crate::term_utils::styled_print::*;
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
        use $crate::term_utils::styled_print::*;
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
        use $crate::term_utils::styled_print::*;
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
        use $crate::term_utils::styled_print::*;
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
        use $crate::term_utils::styled_print::*;
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
        use $crate::term_utils::styled_print::*;
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
        let (r, g, b): (u8, u8, u8) = $rgb;
        print!("\x1b[38;2;{r};{g};{b}m{RESET}")
    }};
    ($rgb:expr, $($arg:tt)*) => {{
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
        let (r, g, b): (u8, u8, u8) = $rgb;
        println!("\x1b[38;2;{r};{g};{b}m{RESET}")
    }};
    ($rgb:expr, $($arg:tt)*) => {{
        let (r, g, b): (u8, u8, u8) = $rgb;
        println!("\x1b[38;2;{r};{g};{b}m{}{RESET}", format_args!($($arg)*))
    }};
}
