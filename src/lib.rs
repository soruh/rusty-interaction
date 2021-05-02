#![warn(missing_docs)]
//! Rusty-interaction is a library that allows you to work with Discord's new [Interactions](https://blog.discord.com/slash-commands-are-here-8db0a385d9e6).
//! It can expose types and provides helper functions to validate your Interactions.
//! It can optionally provide a handler that allows you to receive interactions via outgoing webhook.

#[cfg(feature = "types")]
/// Exposes useful data models
pub mod types;

/// Provides a helper function to validate Discord interactions.
#[cfg(feature = "security")]
pub mod security;

/// Provides an entire handler to handle Discord interactions.
#[cfg(feature = "handler")]
pub mod handler;

pub use attributes::*;

#[cfg(test)]
mod tests;

const BASE_URL: &str = "https://discord.com/api/v9";
