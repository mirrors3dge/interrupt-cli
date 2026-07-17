use super::Prompt;
use crate::command::{Command, Interrupt};
use crate::input::{InputTimeout, OptionalInputTimeout, read_input, read_input_timeout};
use crate::prompts::{PromptParams, private::PromptBuilder};

use std::marker::PhantomData;
use std::time::Duration;

/// The basic text prompt returning a plain [`String`].
///
/// Like all prompts, it implements the [`Prompt`] trait to configure and display prompts.
#[derive(Clone)]
pub struct TextPrompt<'f, C: Command> {
    params: PromptParams<'f, String>,
    /// Just for convenience for building the prompt with simple generics.
    cmd_type: PhantomData<C>,
}

impl<'f, C: Command> TextPrompt<'f, C> {
    /// Creates a new [`TextPrompt`] with the given message.
    ///
    /// Panics if `msg` is empty.
    pub fn new(msg: impl AsRef<str>) -> Self {
        Self {
            params: PromptParams::new(msg),
            cmd_type: PhantomData,
        }
    }
}

impl<'f, C: Command> PromptBuilder<'f, C> for TextPrompt<'f, C> {
    fn get_params_mut(&mut self) -> &mut PromptParams<'f, Self::T> {
        &mut self.params
    }
}

impl<'f, C: Command> Prompt<C> for TextPrompt<'f, C> {
    type T = String;

    fn prompt(self) -> Result<Self::T, Interrupt<C>> {
        let msg: &str = self.params.msg.as_str();
        let help: Option<&String> = self.params.help.as_ref();

        'attempt: loop {
            // get line
            let input: String = read_input(msg, help)?;

            // no usage of fallback here

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&input) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(input);
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

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&input) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(Some(input));
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

            // no usage of fallback except timeout

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&input) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(InputTimeout::Input(input));
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

            // filter
            for filter in &self.params.filters {
                if let Err(msg) = (filter)(&input) {
                    println!("{msg}");
                    continue 'attempt;
                }
            }

            return Ok(OptionalInputTimeout::Input(input));
        }
    }
}
