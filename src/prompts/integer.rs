use crate::prompts::ParsablePrompt;

/// A prompt returning a [`i8`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I8Prompt<'f, C> = ParsablePrompt<'f, i8, C>;
/// A prompt returning a [`i16`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I16Prompt<'f, C> = ParsablePrompt<'f, i16, C>;
/// A prompt returning a [`i32`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I32Prompt<'f, C> = ParsablePrompt<'f, i32, C>;
/// A prompt returning a [`i64`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I64Prompt<'f, C> = ParsablePrompt<'f, i64, C>;
/// A prompt returning a [`i128`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I128Prompt<'f, C> = ParsablePrompt<'f, i128, C>;
/// A prompt returning a [`isize`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type ISizePrompt<'f, C> = ParsablePrompt<'f, isize, C>;

/// A prompt returning a [`u8`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U8Prompt<'f, C> = ParsablePrompt<'f, u8, C>;
/// A prompt returning a [`u16`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U16Prompt<'f, C> = ParsablePrompt<'f, u16, C>;
/// A prompt returning a [`u32`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U32Prompt<'f, C> = ParsablePrompt<'f, u32, C>;
/// A prompt returning a [`u64`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U64Prompt<'f, C> = ParsablePrompt<'f, u64, C>;
/// A prompt returning a [`u128`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U128Prompt<'f, C> = ParsablePrompt<'f, u128, C>;
/// A prompt returning a [`usize`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type USizePrompt<'f, C> = ParsablePrompt<'f, usize, C>;
