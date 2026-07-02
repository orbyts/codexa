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

### Planned

- Add a typed `Document` model.
- Add a `Block` enum.
- Add the first `Paragraph` block.
- Parse paragraph-only Markdown.
- Reject unsupported Markdown constructs with clear diagnostics.
- Serialize a stable JSON web artifact.
- Add a `codexa build` command.
- Add parser, serialization, fixture, and CLI integration tests.
- Keep website presentation outside the Codexa core.


All notable changes to Codexa will be documented in this file.

## [0.0.1] - 2026-07-01

### Added

- Initial Rust library and CLI scaffold.
- `codexa`, `codexa --help`, and `codexa --version` behavior.
- Unit and integration tests.
- GitHub Actions checks for formatting, linting, tests, and packaging.
- Initial project documentation and release policy.
