//! Contains all the different prompt types (text, number, select, ...).

mod confirm;
mod float;
mod integer;
mod parsable;
mod select;
mod text;

use crate::command::{Command, Interrupt};
use crate::input::{InputTimeout, OptionalInputTimeout};

use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

// re-export prompt types
pub use confirm::ConfirmPrompt;
pub use float::{F32Prompt, F64Prompt};
pub use integer::{
    I8Prompt, I16Prompt, I32Prompt, I64Prompt, I128Prompt, ISizePrompt, U8Prompt, U16Prompt,
    U32Prompt, U64Prompt, U128Prompt, USizePrompt,
};
pub use parsable::ParsablePrompt;
pub use select::SelectPrompt;
pub use text::TextPrompt;

// use `Arc` to allow cloning prompts.
type Filter<'f, T> = Arc<dyn Fn(&T) -> Result<(), String> + 'f>;

#[derive(Clone)]
struct PromptParams<'f, T> {
    msg: String,
    /// If any, it's always valid in regards to current filters.
    fallback: Option<T>,
    filters: Vec<Filter<'f, T>>,
    help: Option<String>,
}

impl<'f, T> PromptParams<'f, T> {
    /// Creates new `PromptParams` for a prompt returning type `T`.
    ///
    /// Panics if `msg` is empty.
    fn new(msg: impl AsRef<str>) -> Self {
        let msg: String = msg.as_ref().to_owned();
        assert!(!msg.is_empty(), "empty message prompt");

        Self {
            msg,
            fallback: None,
            filters: Vec::new(),
            help: None,
        }
    }
}

// Use a sealed trait to hide private functions.
mod private {
    use super::{Prompt, PromptParams};
    use crate::Command;

    pub(super) trait PromptBuilder<'f, C: Command>: Prompt<C> {
        fn get_params_mut(&mut self) -> &mut PromptParams<'f, Self::T>;
    }
}
use private::PromptBuilder;

/// Provides functions to configure a prompt that returns type `T`, and functions that displays the
/// prompt to get user input.
///
/// All prompt type implemenet this trait, you don't have to implement it yourself.
///
/// When displayed, prompts block the calling thread and loop internally until they receive a valid
/// answer, an interrupt command or a preconfigured timeout is fired.
///
/// All prompts implement [`Clone`] if the interrupt [`Command`] type implements [`Clone`], this is useful
/// when defining a template prompt and cloning it for each use.
///
/// Timeout caveat: after a timeout, stdin remains locked (on a background thread) until the user
/// presses Enter. This is a known limitation arising from the inability to interrupt a blocking
/// [`read_line`](`std::io::Stdin::read_line`) call.
///
/// ## Prompt example
///
/// ```ignore
/// use interrupt_cli::prompts::TextPrompt;
/// use interrupt_cli::{Interrupt, Prompt as _};
///
/// enum MyCmd {
///     // your interrupt commands...
/// }
///
/// impl Command for MyCmd {
///     // your implementation...
/// }
///
/// fn main() {
///     let input: Result<String, Interrupt<MyCmd>> = TextPrompt::new("enter your name")
///         .with_help("some help message") // shown when the user types "help"
///         .with_filter(|input| {
///             if input.contains(char::is_numeric) {
///                 Err("numbers are not allowed".to_string())
///             } else {
///                 Ok(())
///             }
///         }).prompt(); // displays the prompt, blocks current thread until the user types an input
///
///     match input {
///         Ok(input) => println!("you entered: {input}"),
///         Err(Interrupt::Exit) => std::process::exit(0),
///         Err(Interrupt::Cmd(cmd)) => {
///            // handle your commands...
///         }
///     }
/// }
/// ```
#[expect(private_bounds)]
pub trait Prompt<C: Command>: Sized {
    /// Type returned by the prompt.
    type T;

    /// Show the prompt and block the current thread until receiving user input.
    ///
    /// # Errors
    ///
    /// When an interrupt command is triggered.
    fn prompt(self) -> Result<Self::T, Interrupt<C>>;

    /// Show the prompt and block the current thread until receiving `Some(user_input)`, or `None`
    /// if the user explicitly entered nothing.
    ///
    /// This will never return `Ok(None)` when a fallback is set.
    ///
    /// # Errors
    ///
    /// When an interrupt command is triggered.
    fn prompt_optional(self) -> Result<Option<Self::T>, Interrupt<C>>;

    /// Show the prompt and block the current thread until receiving user input.
    ///
    /// `Ok(InputTimeout::Timeout)` is returned if the timeout is reached before any input
    /// was entered.
    ///
    /// # Errors
    ///
    /// When an interrupt command is triggered.
    fn prompt_timeout(self, timeout: Duration) -> Result<InputTimeout<Self::T>, Interrupt<C>>;

    /// Show the prompt and block the current thread until receiving
    /// `OptionalInputTimeout::Input(user_input)`, or `OptionalInputTimeout::None` if
    /// the user explicitly entered nothing.
    ///
    /// `Ok(OptionalInputTimeout::Timeout)` is returned if the timeout is reached before any
    /// input was entered.
    ///
    /// This will never return `Ok(OptionalInputTimeout::None)` when a fallback is set.
    ///
    /// # Errors
    ///
    /// When an interrupt command is triggered.
    fn prompt_timeout_optional(
        self,
        timeout: Duration,
    ) -> Result<OptionalInputTimeout<Self::T>, Interrupt<C>>;

    // --- Provided functions --- //

    /// Replace the prompt message on an already-configured prompt.
    ///
    /// Mostly useful when cloning a template prompt and only changing the message.
    #[must_use]
    fn with_message<'f>(mut self, msg: impl AsRef<str>) -> Self
    where
        Self: PromptBuilder<'f, C>,
    {
        let params = self.get_params_mut();
        params.msg = msg.as_ref().to_owned();
        self
    }

    /// Set a fallback value returned when failing to get an input, this can occur when:
    ///
    /// - the prompt return type `T` require parsing and parsing fails.
    /// - a timeout is fired in a prompt with timeout.
    /// - the user submits an empty line in an optional prompt.
    ///
    /// Optional prompts will never return `None` (on an empty line) when a fallback is set.
    ///
    /// The fallback is silently dropped if it fails any of the filters already attached to the
    /// prompt (see [`with_filter()`](Self::with_filter). The fallback is also silently dropped if
    /// any new filter added by [`with_filter()`](Self::with_filter) rejects it at any time.
    /// This behavior can be changed to a `panic!` instead of dropping the value by enabling the
    /// "no-fallback-drop" feature.
    #[must_use]
    fn with_fallback<'f>(mut self, fallback: Self::T) -> Self
    where
        Self: PromptBuilder<'f, C>,
    {
        let params = self.get_params_mut();

        if params
            .filters
            .iter()
            .any(|filter| (filter)(&fallback).is_err())
        {
            if cfg!(feature = "no-fallback-drop") {
                panic!("the prompt fallback value got rejected by an existing filter")
            } else {
                return self;
            }
        }

        params.fallback = Some(fallback);
        self
    }

    /// Add a validation step. Filters are evaluated in insertion order. On the first failure the
    /// error message is printed 'as is' and the prompt re-prompts; later filters are not evaluated.
    ///
    /// It is preferable for the error message to be a one liner for consistent formatting.
    ///
    /// If the fallback value already set by [`with_fallback()`](`Self::with_fallback`) is
    /// invalidated by a new filter, the fallback is silently dropped. This behavior can be changed
    /// to a `panic!` instead of dropping the value by enabling the "no-fallback-drop" feature.
    #[must_use]
    fn with_filter<'f, F, E>(mut self, filter: F) -> Self
    where
        F: Fn(&Self::T) -> Result<(), E> + 'f,
        E: Display,
        Self: PromptBuilder<'f, C>,
    {
        let params = self.get_params_mut();

        // remove fallback if invalidated by the new filter
        if let Some(fallback) = &params.fallback
            && (filter)(fallback).is_err()
        {
            if cfg!(feature = "no-fallback-drop") {
                panic!("the new prompt filter rejected the current fallback value")
            } else {
                params.fallback = None;
            }
        }

        params.filters.push(Arc::new(move |input| {
            filter(input).map_err(|err| err.to_string())
        }));
        self
    }

    /// Add a help message to the prompt.
    ///
    /// Help is shown when the user types 'help'.
    #[must_use]
    fn with_help<'f>(mut self, help: impl AsRef<str>) -> Self
    where
        Self: PromptBuilder<'f, C>,
    {
        let params = self.get_params_mut();
        params.help = Some(help.as_ref().to_owned());
        self
    }
}
