use super::Prompt;
use crate::command::{Command, Interrupt};
use crate::input::{InputTimeout, OptionalInputTimeout, read_input, read_input_timeout};
#[cfg(feature = "term-utils")]
use crate::println_red;
use crate::prompts::{PromptParams, private::PromptBuilder};
#[cfg(not(feature = "term-utils"))]
use println as println_red;

use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;
use std::time::Duration;

/// A prompt returning any predetermined type `P` implementing [`FromStr`], parsed from the user input.
///
/// Like all prompts, it implements the [`Prompt`] trait to configure and display prompts.
#[derive(Clone)]
pub struct ParsablePrompt<'f, P, C>
where
    P: FromStr,
    C: Command,
{
    params: PromptParams<'f, P>,
    /// Just for convenience for building the prompt with simple generics.
    cmd_type: PhantomData<C>,
}

impl<'f, P, C> ParsablePrompt<'f, P, C>
where
    P: FromStr,
    P::Err: Display,
    C: Command,
{
    /// Creates a new [`ParsablePrompt`] with the given message.
    ///
    /// Panics if `msg` is empty.
    pub fn new(msg: impl AsRef<str>) -> Self {
        Self {
            params: PromptParams::new(msg),
            cmd_type: PhantomData,
        }
    }
}

impl<'f, P, C> PromptBuilder<'f, C> for ParsablePrompt<'f, P, C>
where
    P: FromStr,
    P::Err: Display,
    C: Command,
{
    fn get_params_mut(&mut self) -> &mut PromptParams<'f, Self::T> {
        &mut self.params
    }
}

impl<'f, P, C> Prompt<C> for ParsablePrompt<'f, P, C>
where
    P: FromStr,
    P::Err: Display,
    C: Command,
{
    type T = P;

    fn prompt(self) -> Result<Self::T, Interrupt<C>> {
        let msg: &str = self.params.msg.as_str();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line
            let input: String = read_input(msg, help)?;

            // parse
            let parsed: P = match input.parse() {
                Ok(parsed) => parsed,
                Err(err) => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("failed to parse '{input}': {err} (using fallback)");
                        return Ok(fallback);
                    } else {
                        println_red!("failed to parse '{input}': {err}");
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
        let msg: &str = self.params.msg.as_str();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line
            let input: String = read_input(msg, help)?;

            // optional input
            if input.is_empty() {
                if let Some(fallback) = self.params.fallback {
                    return Ok(Some(fallback));
                } else {
                    return Ok(None);
                }
            }

            // parse
            let parsed: P = match input.parse() {
                Ok(parsed) => parsed,
                Err(err) => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("failed to parse '{input}': {err} (using fallback)");
                        return Ok(Some(fallback));
                    } else {
                        println_red!("failed to parse '{input}': {err}");
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
        let msg: &str = self.params.msg.as_str();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line with timeout
            let input: String = match read_input_timeout(msg, help, timeout)? {
                InputTimeout::Input(input) => input,
                InputTimeout::Timeout => {
                    if let Some(fallback) = self.params.fallback {
                        return Ok(InputTimeout::Input(fallback));
                    }
                    return Ok(InputTimeout::Timeout);
                }
            };

            // parse
            let parsed: P = match input.parse() {
                Ok(parsed) => parsed,
                Err(err) => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("failed to parse '{input}': {err} (using fallback)");
                        return Ok(InputTimeout::Input(fallback));
                    } else {
                        println_red!("failed to parse '{input}': {err}");
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
        let msg: &str = self.params.msg.as_str();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line with timeout
            let input: String = match read_input_timeout(msg, help, timeout)? {
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
            let parsed: P = match input.parse() {
                Ok(parsed) => parsed,
                Err(err) => {
                    if let Some(fallback) = self.params.fallback {
                        println_red!("failed to parse '{input}': {err} (using fallback)");
                        return Ok(OptionalInputTimeout::Input(fallback));
                    } else {
                        println_red!("failed to parse '{input}': {err}");
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
