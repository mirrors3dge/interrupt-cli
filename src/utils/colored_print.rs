// --- BOLD --- //
/// Like [`print!`], but bold.
#[macro_export]
macro_rules! print_bold {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.bold());
    }};
}
/// Like [`println!`], but bold.
#[macro_export]
macro_rules! println_bold {
    () => {
        println!();
    };
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
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.red());
    }};
}
/// Like [`println!`], but red.
#[macro_export]
macro_rules! println_red {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.red());
    }};
}
/// Like [`print!`], but red and bold.
#[macro_export]
macro_rules! print_red_bold {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.red().bold());
    }};
}
/// Like [`println!`], but red and bold.
#[macro_export]
macro_rules! println_red_bold {
    () => {
        println!();
    };
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
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.yellow());
    }};
}
/// Like [`println!`], but yellow.
#[macro_export]
macro_rules! println_yellow {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.yellow());
    }};
}
/// Like [`print!`], but yellow and bold.
#[macro_export]
macro_rules! print_yellow_bold {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.yellow().bold());
    }};
}
/// Like [`println!`], but yellow and bold.
#[macro_export]
macro_rules! println_yellow_bold {
    () => {
        println!();
    };
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
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.green());
    }};
}
/// Like [`println!`], but green.
#[macro_export]
macro_rules! println_green {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.green());
    }};
}
/// Like [`print!`], but geen and bold.
#[macro_export]
macro_rules! print_green_bold {
    () => {
        print!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        print!("{}", s.green().bold());
    }};
}
/// Like [`println!`], but green and bold.
#[macro_export]
macro_rules! println_green_bold {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
        use $crate::_private::colored::Colorize as _;
        let s: String = format!($($arg)*);
        println!("{}", s.green().bold());
    }};
}
