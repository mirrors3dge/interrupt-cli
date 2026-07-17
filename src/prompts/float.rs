use crate::prompts::ParsablePrompt;

/// A prompt returning a [`f32`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display prompts.
pub type F32Prompt<'f, C> = ParsablePrompt<'f, f32, C>;
/// A prompt returning a [`f64`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display prompts.
pub type F64Prompt<'f, C> = ParsablePrompt<'f, f64, C>;
