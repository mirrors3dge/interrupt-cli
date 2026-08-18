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

/// A prompt displaying multiple choices and returning the **index** (as [`usize`]) of the selected
/// one, with the following format:
///
/// ```ignore
/// "{msg}\n\
/// 1) {choice_0}\n\
/// 2) {choice_1}\n\
/// 3) {choice_2}\n\
/// \n\
/// enter a choice number: "
/// ```
///
/// Like all prompts, it implements the [`Prompt`] trait to configure and display prompts.
#[derive(Clone)]
pub struct SelectPrompt<'f, C: Command> {
    params: PromptParams<'f, usize>,
    choices: Vec<String>,
    /// Just for convenience for building the prompt with simple generics.
    cmd_type: PhantomData<C>,
}

impl<'f, C: Command> SelectPrompt<'f, C> {
    /// Creates a new [`SelectPrompt`] with the given message and choices.
    ///
    /// # Panics
    ///
    /// If `msg` or `choices` is empty.
    pub fn new(msg: impl AsRef<str>, choices: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let choices: Vec<String> = choices.into_iter().map(|s| s.as_ref().to_owned()).collect();
        assert!(!choices.is_empty(), "no choices given in selector prompt");

        Self {
            params: PromptParams::new(msg),
            choices,
            cmd_type: PhantomData,
        }
    }

    fn format_choices(&self) -> String {
        let mut formatted: String = self.params.msg.clone() + "\n";

        for (idx, choice) in self.choices.iter().enumerate() {
            formatted += format!("{}) {}\n", idx + 1, choice).as_str();
        }

        formatted += "\nenter a choice number";
        formatted
    }
}

impl<'f, C: Command> PromptBuilder<'f, C> for SelectPrompt<'f, C> {
    fn get_params_mut(&mut self) -> &mut PromptParams<'f, Self::T> {
        &mut self.params
    }
}

impl<'f, C: Command> Prompt<C> for SelectPrompt<'f, C> {
    type T = usize;

    fn prompt(self) -> Result<Self::T, Interrupt<C>> {
        let msg: String = self.format_choices();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line
            let input: String = read_input(&msg, help)?;

            // parse choice number
            let idx: usize = match input.parse() {
                Ok(parsed) => {
                    if !(1..=self.choices.len()).contains(&parsed) {
                        if let Some(fallback) = self.params.fallback {
                            println_red!("invalid choice number: '{parsed}' (using fallback)");
                            return Ok(fallback);
                        } else {
                            println_red!("invalid choice number: '{parsed}'");
                            continue 'attempt;
                        }
                    }
                    parsed - 1 // index
                }
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
                if let Err(msg) = (filter)(&idx) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(idx);
        }
    }

    fn prompt_optional(self) -> Result<Option<Self::T>, Interrupt<C>> {
        let msg: String = self.format_choices();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line
            let input: String = read_input(&msg, help)?;

            // optional input
            if input.is_empty() {
                if let Some(fallback) = self.params.fallback {
                    return Ok(Some(fallback));
                } else {
                    return Ok(None);
                }
            }

            // parse choice number
            let idx: usize = match input.parse() {
                Ok(parsed) => {
                    if !(1..=self.choices.len()).contains(&parsed) {
                        if let Some(fallback) = self.params.fallback {
                            println_red!("invalid choice number: '{parsed}' (using fallback)");
                            return Ok(Some(fallback));
                        } else {
                            println_red!("invalid choice number: '{parsed}'");
                            continue 'attempt;
                        }
                    }
                    parsed - 1 // index
                }
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
                if let Err(msg) = (filter)(&idx) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(Some(idx));
        }
    }

    fn prompt_timeout(self, timeout: Duration) -> Result<InputTimeout<Self::T>, Interrupt<C>> {
        let msg: String = self.format_choices();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line with timeout
            let input: String = match read_input_timeout(&msg, help, timeout)? {
                InputTimeout::Input(input) => input,
                InputTimeout::Timeout => {
                    if let Some(fallback) = self.params.fallback {
                        return Ok(InputTimeout::Input(fallback));
                    }
                    return Ok(InputTimeout::Timeout);
                }
            };

            // parse choice number
            let idx: usize = match input.parse() {
                Ok(parsed) => {
                    if !(1..=self.choices.len()).contains(&parsed) {
                        if let Some(fallback) = self.params.fallback {
                            println_red!("invalid choice number: '{parsed}' (using fallback)");
                            return Ok(InputTimeout::Input(fallback));
                        } else {
                            println_red!("invalid choice number: '{parsed}'");
                            continue 'attempt;
                        }
                    }
                    parsed - 1 // index
                }
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
                if let Err(msg) = (filter)(&idx) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(InputTimeout::Input(idx));
        }
    }

    fn prompt_timeout_optional(
        self,
        timeout: Duration,
    ) -> Result<OptionalInputTimeout<Self::T>, Interrupt<C>> {
        let msg: String = self.format_choices();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line with timeout
            let input: String = match read_input_timeout(&msg, help, timeout)? {
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

            // parse choice number
            let idx: usize = match input.parse() {
                Ok(parsed) => {
                    if !(1..=self.choices.len()).contains(&parsed) {
                        if let Some(fallback) = self.params.fallback {
                            println_red!("invalid choice number: '{parsed}' (using fallback)");
                            return Ok(OptionalInputTimeout::Input(fallback));
                        } else {
                            println_red!("invalid choice number: '{parsed}'");
                            continue 'attempt;
                        }
                    }
                    parsed - 1 // index
                }
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
                if let Err(msg) = (filter)(&idx) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(OptionalInputTimeout::Input(idx));
        }
    }
}
