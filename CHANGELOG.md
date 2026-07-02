# Changelog

## [0.0.2] - Unreleased

### Goal

Prove the first complete Codexa vertical slice:

```text
Markdown paragraph
    ↓
typed Rust content model
    ↓
JSON web artifact
    ↓
external website renderer
```

### Added

- Typed `Document`, `Block`, and `Paragraph` models.
- Paragraph-only Markdown parsing through `pulldown-cmark`.
- Stable diagnostics for unsupported Markdown constructs.
- Versioned JSON web document and manifest artifacts.
- `codexa build <INPUT> --adapter web --output <DIR>`.
- Parser, adapter, fixture, and CLI integration tests.
- CI packaging from a clean checkout.
- Documentation for future Notion and platform-specific adapters.


All notable changes to Codexa will be documented in this file.

## [0.0.1] - 2026-07-01

### Added

- Initial Rust library and CLI scaffold.
- `codexa`, `codexa --help`, and `codexa --version` behavior.
- Unit and integration tests.
- GitHub Actions checks for formatting, linting, tests, and packaging.
- Initial project documentation and release policy.
