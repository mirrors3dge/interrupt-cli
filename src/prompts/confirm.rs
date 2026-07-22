use super::Prompt;
use crate::command::{Command, Interrupt};
use crate::input::{InputTimeout, OptionalInputTimeout, read_input, read_input_timeout};
#[cfg(feature = "term-utils")]
use crate::println_red;
use crate::prompts::{PromptParams, private::PromptBuilder};
#[cfg(not(feature = "term-utils"))]
use println as println_red;

use std::marker::PhantomData;
use std::time::Duration;

/// A prompt returning a boolean 'yes' or 'no'.
///
/// - `true` if the user types 'y' or 'yes'
/// - `false` if the user types 'n' or 'no'
///
/// Case insensitive.
///
/// Like all prompts, it implements the [`Prompt`] trait to configure and display prompts.
#[derive(Clone)]
pub struct ConfirmPrompt<'f, C: Command> {
    params: PromptParams<'f, bool>,
    /// Just for convenience for building the prompt with simple generics.
    cmd_type: PhantomData<C>,
}

impl<'f, C: Command> ConfirmPrompt<'f, C> {
    /// Creates a new [`ConfirmPrompt`] with the given message.
    ///
    /// Panics if `msg` is empty.
    pub fn new(msg: impl AsRef<str>) -> Self {
        Self {
            params: PromptParams::new(msg),
            cmd_type: PhantomData,
        }
    }
}

impl<'f, C: Command> PromptBuilder<'f, C> for ConfirmPrompt<'f, C> {
    fn get_params_mut(&mut self) -> &mut PromptParams<'f, Self::T> {
        &mut self.params
    }
}

impl<'f, C: Command> Prompt<C> for ConfirmPrompt<'f, C> {
    type T = bool;

    fn prompt(self) -> Result<Self::T, Interrupt<C>> {
        let msg: String = self.params.msg + " (y/n)";
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line
            let mut input: String = read_input(&msg, help)?;

            // parse
            input.make_ascii_lowercase();
            let parsed: bool = match input.as_str() {
                "y" | "yes" => true,
                "n" | "no" => false,
                _ => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("'{input}' is not a recognized input (using fallback)");
                        return Ok(fallback);
                    } else {
                        println_red!(
                            "'{input}' is not a recognized input, type y/n to confirm/deny"
                        );
                        continue 'attempt;
                    }
                }
            };

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&parsed) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(parsed);
        }
    }

    fn prompt_optional(self) -> Result<Option<Self::T>, Interrupt<C>> {
        let msg: String = self.params.msg + " (y/n)";
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line
            let mut input: String = read_input(&msg, help)?;

            // optional input
            if input.is_empty() {
                if let Some(fallback) = self.params.fallback {
                    return Ok(Some(fallback));
                } else {
                    return Ok(None);
                }
            }

            // parse
            input.make_ascii_lowercase();
            let parsed: bool = match input.as_str() {
                "y" | "yes" => true,
                "n" | "no" => false,
                _ => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("'{input}' is not a recognized input (using fallback)");
                        return Ok(Some(fallback));
                    } else {
                        println_red!(
                            "'{input}' is not a recognized input, type y/n to confirm/deny"
                        );
                        continue 'attempt;
                    }
                }
            };

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&parsed) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(Some(parsed));
        }
    }

    fn prompt_timeout(self, timeout: Duration) -> Result<InputTimeout<Self::T>, Interrupt<C>> {
        let msg: String = self.params.msg + " (y/n)";
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line with timeout
            let mut input: String = match read_input_timeout(&msg, help, timeout)? {
                InputTimeout::Input(input) => input,
                InputTimeout::Timeout => {
                    if let Some(fallback) = self.params.fallback {
                        return Ok(InputTimeout::Input(fallback));
                    }
                    return Ok(InputTimeout::Timeout);
                }
            };

            // parse
            input.make_ascii_lowercase();
            let parsed: bool = match input.as_str() {
                "y" | "yes" => true,
                "n" | "no" => false,
                _ => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("'{input}' is not a recognized input (using fallback)");
                        return Ok(InputTimeout::Input(fallback));
                    } else {
                        println_red!(
                            "'{input}' is not a recognized input, type y/n to confirm/deny"
                        );
                        continue 'attempt;
                    }
                }
            };

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&parsed) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(InputTimeout::Input(parsed));
        }
    }

    fn prompt_timeout_optional(
        self,
        timeout: Duration,
    ) -> Result<OptionalInputTimeout<Self::T>, Interrupt<C>> {
        let msg: String = self.params.msg + " (y/n)";
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line with timeout
            let mut input: String = match read_input_timeout(&msg, help, timeout)? {
                InputTimeout::Input(input) => input,
                InputTimeout::Timeout => {
                    if let Some(fallback) = self.params.fallback {
                        return Ok(OptionalInputTimeout::Input(fallback));
                    }
                    return Ok(OptionalInputTimeout::Timeout);
                }
            };

            // optional input
            if input.is_empty() {
                if let Some(fallback) = self.params.fallback {
                    return Ok(OptionalInputTimeout::Input(fallback));
                } else {
                    return Ok(OptionalInputTimeout::None);
                }
            }

            // parse
            input.make_ascii_lowercase();
            let parsed: bool = match input.as_str() {
                "y" | "yes" => true,
                "n" | "no" => false,
                _ => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("'{input}' is not a recognized input (using fallback)");
                        return Ok(OptionalInputTimeout::Input(fallback));
                    } else {
                        println_red!(
                            "'{input}' is not a recognized input, type y/n to confirm/deny"
                        );
                        continue 'attempt;
                    }
                }
            };

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&parsed) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(OptionalInputTimeout::Input(parsed));
        }
    }
}
