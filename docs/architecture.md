# Architecture

Codexa follows a compiler-style pipeline:

1. Acquire configured content sources.
2. Parse repository manifests and content documents.
3. Normalize source syntax into target-neutral typed structures.
4. Resolve the global navigation tree and cross-content graph.
5. Project content by visibility and audience.
6. Plan and render target-specific artifacts.

## Adapter boundary

Codexa owns content meaning and stable serialized contracts. Output adapters map the target-neutral model into artifacts for external consumers.

```text
Markdown
    ↓
parser
    ↓
target-neutral model
    ↓
output adapter
    ↓
target-specific artifact
```

The first adapter is `adapter::web`. In version `0.0.2`, it emits a versioned JSON bundle rather than final HTML:

```text
dist/web/
├── manifest.json
└── documents/
    └── sample.json
```

A separate website frontend owns HTML components, CSS, JavaScript, themes, dynamic content, authentication, and commerce. Future adapters may target Obsidian, Notion, Substack, feeds, APIs, or other platforms without changing the canonical Git-backed source.

## Current module structure

```text
src/
├── adapter/
│   └── web/
├── model/
├── parser/
├── lib.rs
└── main.rs
```

- `parser` converts Markdown syntax into Codexa types.
- `model` defines the target-neutral document and block model.
- `adapter` converts the neutral model into target-specific artifacts.
- `main.rs` exposes the CLI without owning parsing or rendering logic.

The precise crate boundaries will continue to evolve incrementally before the first working `0.1.0` release.
