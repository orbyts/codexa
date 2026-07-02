//! Core library for Codexa.
//!
//! Codexa aggregates structured content from Git repositories into a typed,
//! target-neutral model and renders that model through output adapters.

pub mod adapter;
pub mod model;
pub mod parser;

/// Current Codexa package version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the initial greeting retained from the `0.0.1` scaffold.
#[must_use]
pub const fn greeting() -> &'static str {
    "Hello from Codexa!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_stable() {
        assert_eq!(greeting(), "Hello from Codexa!");
    }

    #[test]
    fn version_comes_from_cargo_manifest() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
