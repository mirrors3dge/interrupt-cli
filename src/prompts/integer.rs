use crate::prompts::ParsablePrompt;

/// A prompt returning a [`i8`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I8Prompt<C> = ParsablePrompt<i8, C>;
/// A prompt returning a [`i16`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I16Prompt<C> = ParsablePrompt<i16, C>;
/// A prompt returning a [`i32`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I32Prompt<C> = ParsablePrompt<i32, C>;
/// A prompt returning a [`i64`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I64Prompt<C> = ParsablePrompt<i64, C>;
/// A prompt returning a [`i128`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type I128Prompt<C> = ParsablePrompt<i128, C>;
/// A prompt returning a [`isize`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type ISizePrompt<C> = ParsablePrompt<isize, C>;

/// A prompt returning a [`u8`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U8Prompt<C> = ParsablePrompt<u8, C>;
/// A prompt returning a [`u16`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U16Prompt<C> = ParsablePrompt<u16, C>;
/// A prompt returning a [`u32`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U32Prompt<C> = ParsablePrompt<u32, C>;
/// A prompt returning a [`u64`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U64Prompt<C> = ParsablePrompt<u64, C>;
/// A prompt returning a [`u128`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type U128Prompt<C> = ParsablePrompt<u128, C>;
/// A prompt returning a [`usize`].
///
/// Like all prompts, it implements the [`Prompt`](crate::Prompt) trait to configure and display
/// prompts.
pub type USizePrompt<C> = ParsablePrompt<usize, C>;
