# Contributing

Codexa is in internal pre-release development.

Before committing changes, run:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo package --allow-dirty
```
