# Codexa

Codexa is a Git-native content compiler for aggregating structured content from multiple repositories into typed public and private knowledge artifacts.

Version `0.0.1` is the initial internal pre-release scaffold.

## Version policy

- `0.0.1` through `0.0.9`: internal pre-release development
- `0.1.0`: first working release

## Current behavior

```console
$ codexa
Hello from Codexa!

$ codexa --version
codexa 0.0.1
```

## Planned architecture

```text
Git repositories
    ↓
source adapters
    ↓
typed content model
    ↓
resolved content graph
    ↓
renderers
    ├── public website
    ├── private website
    ├── Obsidian vault
    ├── Substack export
    └── additional artifacts
```

## Development

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo run
```

## License

MIT
