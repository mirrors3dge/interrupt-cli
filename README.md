# Interrupt-cli

Facilitate the handling of user input in your CLI around the idea of "_interrupt commands_": every prompt can be interrupted at any time by user-defined commands to affect the control flow of your program.

To get started, implement the `Command` trait for your type to define the set of interrupt commands available across all prompts.

Then, you can use the different prompts types (`prompts::TextPrompt`, `prompts::SelectPrompt`, `prompts::U64Prompt`, etc. that all implement the `Prompt` trait) to get the user input.

## Crate features

On by default:

- `term-utils`: enables de `term-utils` module, which contains various functions/macros/constants useful for working with the terminal. Only works with an ANSI terminal!

- `clear-output`: when enabled, clears the terminal screen after each prompt, giving a more interactive feel. Requires the `term-utils` feature which only work with an ANSI terminal!

Off by default:

- `no-fallback-drop`: panics instead of silently dropping the prompt fallback value if it gets rejected by a filter (see `Prompt::with_fallback` and `Prompt::with_filter` for more details).

## Comprehensive example

```rust
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
            ("restart", "Restart the session."),
            ("back", "Go to the previous menu."),
            ("status", "Show the current status."),
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
        }).prompt(); // displays the prompt, blocks current thread until the user types an input

    match input {
        Ok(input) => println!("you entered: {input}"),
        Err(Interrupt::Exit) => std::process::exit(0),
        Err(Interrupt::Cmd(cmd)) => {
           // handle your commands here
        }
    }
}
```
