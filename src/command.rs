use std::str::FromStr;

/// Holds a command possible to trigger anywhere in cli that interrupts the current prompt.
///
/// Returned as [`Err`] by every prompt method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Interrupt<C: Command> {
    /// Must exit the program.
    Exit,
    /// An interrupt command entered by the user.
    Cmd(C),
}

/// Implement this trait on an enum to define the set of interrupt commands available across all
/// prompts, returned as `Err(Interrupt::Cmd(cmd))` when succesfully parsed from a prompt input.
///
/// Its [`FromStr::Err`] associated type must be `()` as it doesn't need to hold any information.
///
/// These keywords are already parsed as an interrupt by default (case insensitive):
/// - "exit": for closing the program.
/// - "help": for showing help on the current prompt.
///
/// ## Internal commands
///
/// This category exists for commands that do not affect the control flow of your program but may
/// have global side effects. They aren't returned to the caller via [`Interrupt`], but passed to
/// [`Self::internal()`] instead to be executed, then, control resumes to the last prompt.
///
/// Example: the default 'help' command behaves like an internal command; it prints to the terminal
/// and doesn't affect the control flow (control resumes to the last prompt), so it isn't retuned
/// via [`Interrupt`].
///
/// The downside is that [`Self::internal()`] runs in its own scope without access to shared
/// variable outside of statics.
///
/// ## Parse order
///
/// 1) Default keywords ("exit", "help") with their built-in behavior.
/// 2) `Self::from_str`: normal commands, returned as `Err(Interrupt::Cmd(cmd))`.
/// 3) `Self::Internal::from_str`: internal commands with a global side effect, re-prompts.
///
/// ## Example implementation
///
/// ```
/// use interrupt_cli::Command;
/// use std::str::FromStr;
///
/// // Define your commands
/// enum MyCmd { Restart, Back } // say, restart the session and go back to the previous screen
///
/// // How to parse them
/// impl FromStr for MyCmd {
///     type Err = ();
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         // these commands are returned as an `Err(Interrupt<MyCmd>)` from a prompt
///         // each time the user types them
///         match s.to_lowercase().as_str() {
///             "restart" => Ok(MyCmd::Restart),
///             "back"    => Ok(MyCmd::Back),
///             _         => Err(()),
///         }
///     }
/// }
///
/// // Define your internal commands (optional)
/// enum MyInternalCmd { Status }
///
/// // How to parse them
/// impl FromStr for MyInternalCmd {
///     type Err = ();
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         // typing these commands will trigger a call to `Self::internal()`
///         match s.to_lowercase().as_str() {
///             "status" => Ok(MyInternalCmd::Status),
///             _        => Err(()),
///         }
///     }
/// }
///
/// impl Command for MyCmd {
///     type Internal = MyInternalCmd; // or use `Unparsable` for no interal commands
///
///     // execute an internal command
///     // (unlike `MyCmd`, it won't be returned from a prompt via `Interrupt`)
///     fn internal(cmd: Self::Internal) {
///         match cmd {
///             // this is printed each time the user types "status" in any prompt
///             // then, control resumes to that last prompt
///             MyInternalCmd::Status => println!("current status: ..."),
///         }
///     }
///
///     // define help for each command
///     fn help() -> &'static [(&'static str, &'static str)] {
///         &[
///             ("restart", "Restart the session."),
///             ("back", "Go to the previous menu."),
///             ("status", "Show the current status."),
///         ]
///     }
/// }
/// ```
pub trait Command: FromStr<Err = ()> {
    /// Enum defining internal commands, see the trait documentation for more details.
    ///
    /// Its [`FromStr::Err`] associated type must be `()` as it doesn't need to hold any information.
    ///
    /// If you don't need internal commands, use the provided [`Unparsable`] type (with a [`FromStr`]
    /// impl that always returns an [`Err`]) as a placeholder.
    type Internal: FromStr<Err = ()>;

    /// Execute an internal command with global side effects,
    /// see the trait documentation for more details.
    fn internal(cmd: Self::Internal);

    /// Return the list of commands of `Self` and `Self::Internal` with their associated description.
    /// This is used to generate help whenever it is displayed to the user.
    /// This has no impact on parsing or anything else.
    ///
    /// Each item is a tuple `(keyword, descr)`:
    ///
    /// - `keyword`: the keyword for this command (at most 26 characters for proper formatting)
    /// - `descr`: a short description of the command
    ///
    /// For example, the "exit" command would have the tuple `("exit", "exit the program")`.
    ///
    /// the "exit" and "help" built-in commands are handled for you and must not be part of
    /// this function.
    ///
    /// The order of the items determines the order in which commands are displayed in the help.
    fn help() -> &'static [(&'static str, &'static str)];
}

/// A type whose [`FromStr`] implementation always returns `Err(())`.
/// Use it as placeholder for the [`Command::Internal`] associated type if you don't want to use
/// internal commands. See the [`Command`] trait for more details.
pub struct Unparsable;
impl FromStr for Unparsable {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}
