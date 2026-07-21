// --- BOLD --- //
/// Like [`print!`], but bold.
#[macro_export]
macro_rules! print_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.bold());
    }};
}
/// Like [`println!`], but bold.
#[macro_export]
macro_rules! println_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.bold());
    }};
}

// --- RED --- //
/// Like [`print!`], but red.
#[macro_export]
macro_rules! print_red {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.red());
    }};
}
/// Like [`println!`], but red.
#[macro_export]
macro_rules! println_red {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.red());
    }};
}
/// Like [`print!`], but red and bold.
#[macro_export]
macro_rules! print_red_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.red().bold());
    }};
}
/// Like [`println!`], but red and bold.
#[macro_export]
macro_rules! println_red_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.red().bold());
    }};
}

// --- YELLOW --- //
/// Like [`print!`], but yellow.
#[macro_export]
macro_rules! print_yellow {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.yellow());
    }};
}
/// Like [`println!`], but yellow.
#[macro_export]
macro_rules! println_yellow {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.yellow());
    }};
}
/// Like [`print!`], but yellow and bold.
#[macro_export]
macro_rules! print_yellow_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.yellow().bold());
    }};
}
/// Like [`println!`], but yellow and bold.
#[macro_export]
macro_rules! println_yellow_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.yellow().bold());
    }};
}

// --- GREEN --- //
/// Like [`print!`], but green.
#[macro_export]
macro_rules! print_green {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.green());
    }};
}
/// Like [`println!`], but green.
#[macro_export]
macro_rules! println_green {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.green());
    }};
}
/// Like [`print!`], but geen and bold.
#[macro_export]
macro_rules! print_green_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.green().bold());
    }};
}
/// Like [`println!`], but green and bold.
#[macro_export]
macro_rules! println_green_bold {
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.green().bold());
    }};
}
