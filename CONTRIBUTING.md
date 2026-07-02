# Contributing

Codexa is in internal pre-release development.

Before committing changes, run:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo package --allow-dirty
```

## Scope check

Before adding a feature, read [docs/scope.md](docs/scope.md).

A feature normally belongs in Codexa when it concerns discovering, parsing,
validating, resolving, indexing, relating, or serializing structured source
content and can produce a target-neutral, versioned artifact.

Presentation, deployment, users, payments, media processing, software release
production, and specialized asset lifecycles belong in downstream applications
or separate tools.
