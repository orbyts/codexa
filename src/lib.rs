//! Core library for Codexa.
//!
//! Codexa will aggregate structured content from Git repositories into a
//! typed intermediate representation and render it into multiple artifacts.

/// Current Codexa package version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the initial greeting used by the `0.0.1` scaffold.
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
    fn version_matches_manifest() {
        assert_eq!(VERSION, "0.0.1");
    }
}
