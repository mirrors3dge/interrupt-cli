# Interrupt-cli

A CLI prompt library built around the idea of **interruptible input**:
every prompt can be interrupted at any time by user-defined commands to affect the control flow of your program.

To get started, implement the `Command` trait for your type to define which keywords interrupt all prompts globally.

Then, you can use the different prompts types (`prompts::TextPrompt`, `prompts::SelectPrompt`, `prompts::U64Prompt`, etc. that all implement the `Prompt` trait) to get the user input.

## Crate features

- `colored_print` (default): enables `println_red!`, `println_bold!`, etc. macros. When enabled, some errors printed by this crate's prompts at runtime will be colored (only works in ANSI terminals).

## Comprehensive exemple

```Rust
use interrupt_cli::prompts::TextPrompt;
use interrupt_cli::{Command, Interrupt, Prompt as _};
use std::str::FromStr;

// Define your interrupt commands
enum MyCmd { Restart, Back } // say, restart the session and go back to the previous screen

// How to parse them
impl FromStr for MyCmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // these commands are returned as an `Err(Interrupt<MyCmd>)` from a prompt
        // each time the user types them
        match s.to_lowercase().as_str() {
            "restart" => Ok(MyCmd::Restart),
            "back" => Ok(MyCmd::Back),
            _ => Err(()),
        }
    }
}

// Define your internal commands (optional), see the `Command` trait for more details
enum MyInternalCmd {
    Status,
}

// How to parse them
impl FromStr for MyInternalCmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // typing these commands will trigger a call to `Self::internal()`
        match s.to_lowercase().as_str() {
            "status" => Ok(MyInternalCmd::Status),
            _ => Err(()),
        }
    }
}

impl Command for MyCmd {
    type Internal = MyInternalCmd;

    // execute an internal command
    // (unlike `MyCmd`, it won't be returned from a prompt via `Interrupt`)
    fn internal(cmd: Self::Internal) {
        match cmd {
            // this is printed each time the user types "status" in any prompt
            // then, control resumes to that last prompt
            MyInternalCmd::Status => println!("current status: ..."),
        }
    }

    // define help for each command
    fn help() -> &'static [(&'static str, &'static str)] {
        &[
            ("restart", "restart the session"),
            ("back", "go to the previous menu"),
            ("status", "show the current status"),
        ]
    }
}

fn main() {
    // Now, you can create a prompt
    let input: Result<String, Interrupt<MyCmd>> = TextPrompt::new("enter your name")
        .with_help("some help message") // shown when the user types "help"
        .with_filter(|input| {
            if input.contains(char::is_numeric) {
                Err("numbers are not allowed")
            } else {
                Ok(())
            }
        }).prompt(); // displays the prompt, blocks the current thread until the user types an input

    match input {
        Ok(input) => println!("you entered: {input}"),
        Err(Interrupt::Exit) => std::process::exit(0),
        Err(Interrupt::Cmd(cmd)) => {
           // handle your commands here
        }
    }
}
```
