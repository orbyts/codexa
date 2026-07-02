# Codexa Roadmap

Codexa is a Git-native content compiler. It gathers structured content from one or more repositories, parses it into a typed internal model, resolves hierarchy and relationships, and renders target-specific artifacts such as public and private websites, Obsidian vaults, search indexes, and future publishing formats.

The version in `Cargo.toml` is the canonical project-phase marker. Given a repository snapshot, an LLM or contributor should be able to read this roadmap, compare it with the current package version, inspect the matching release section, and determine:

- what Codexa is expected to support now;
- what must already be tested and documented;
- what is intentionally deferred;
- what the next release should implement.

## Version policy

| Version range | Meaning |
|---|---|
| `0.0.1` through `0.0.9` | Internal pre-release development. Architecture and serialized formats may change without compatibility guarantees. |
| `0.1.0` | First complete working release. The initial end-to-end content pipeline is usable and documented. |
| `0.1.x` | Compatible improvements, fixes, additional renderers, and search enhancements within the first public architecture. |

Every release must satisfy its exit criteria before `Cargo.toml` advances to the next version.

## Architectural direction

Codexa follows a compiler-style pipeline:

```text
Git repositories
    ↓
source acquisition
    ↓
Markdown and metadata parsing
    ↓
typed documents and blocks
    ↓
resolved navigation tree and knowledge graph
    ↓
visibility and audience projections
    ↓
artifact plans
    ├── web
    ├── Obsidian
    ├── search
    ├── JSON
    └── future renderers
```

The canonical source remains human-readable files in Git. Databases, embeddings, indexes, generated navigation, and rendered pages are disposable artifacts that can be rebuilt from those files.

## Core invariants

These rules apply throughout the roadmap:

1. Git files are canonical.
2. Parser-specific types never escape the syntax layer.
3. The normalized model is target-neutral.
4. Navigation hierarchy is a tree.
5. Semantic relationships form a graph.
6. Public and private content are projected before rendering.
7. Search indexes and embeddings are derived artifacts.
8. Renderers consume resolved artifact plans, not raw Markdown.
9. Every serialized Codexa format is versioned.
10. Unsupported syntax must produce a diagnostic rather than being silently discarded.
11. Stable document and block identities must not depend exclusively on file paths.
12. Generated metadata must retain provenance and must not silently replace authored intent.

---

# Release plan

## `0.0.1` — Repository bootstrap

### Goal

Establish a publishable Rust crate and CLI with tests, CI, documentation, and release hygiene.

### Included

- Rust library and binary targets.
- `codexa` executable.
- `codexa --version`.
- Hello-world output.
- Unit and CLI integration tests.
- Formatting and Clippy configuration.
- GitHub Actions CI.
- README, license, changelog, contributing guide, security policy, and architecture notes.
- Publication to crates.io and GitHub.

### Exit criteria

- `cargo fmt --all --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo test --all-targets --all-features` passes.
- `cargo package` passes from a clean Git tree.
- Version `0.0.1` is published to crates.io.

### Explicitly deferred

- Markdown parsing.
- Typed content structures.
- Website generation.
- Repository aggregation.
- Search and embeddings.

---

## `0.0.2` — First vertical slice: paragraph to web artifact

### Goal

Prove the complete path from one Markdown source file to one rendered website paragraph.

### Included

- One sample Markdown fixture.
- Markdown parsing through an established parser library.
- Initial target-neutral model:

```rust
Document
Block::Paragraph
Paragraph
Inline::Text
```

- Stable block identity.
- Source span or source-location information.
- JSON serialization of the normalized document.
- Versioned web-model artifact.
- Minimal `codexa build` command.
- Reference website renderer capable of displaying paragraph blocks.
- Clear diagnostics for unsupported Markdown nodes.

### Initial artifact shape

```text
dist/web/
├── manifest.json
└── documents/
    └── sample.json
```

### Search foundation

No user-facing search is required yet, but every paragraph must be convertible into a derived `SearchChunk` later. The paragraph model must not contain an embedding vector directly.

Reserved concepts:

```rust
SearchChunkId
SearchChunk
SearchMetadata
```

### Exit criteria

- A Markdown paragraph is parsed into a typed `Paragraph`.
- Two paragraphs preserve source order.
- The normalized document serializes deterministically.
- The reference website reads the artifact and renders the paragraph.
- Unsupported headings, lists, and code blocks produce stable diagnostic codes.
- Snapshot tests cover the normalized JSON artifact.

### Explicitly deferred

- Inline formatting beyond plain text.
- Headings and navigation.
- Frontmatter.
- Multi-repository sources.
- Search indexes.
- Embeddings.

---

## `0.0.3` — Inline content and lexical search records

### Goal

Support useful paragraph content and establish the first derived search representation.

### Included

- Inline content types:

```rust
Inline::Text
Inline::Emphasis
Inline::Strong
Inline::Code
Inline::Link
Inline::SoftBreak
Inline::HardBreak
```

- Plain-text extraction from inline content.
- Link target preservation without global resolution.
- Derived search chunks for paragraphs.
- Search-record serialization independent of the web artifact.
- Authored search metadata fields:

```yaml
keywords:
aliases:
topics:
```

- Metadata provenance distinction:

```rust
Authored
Inferred
Generated
Repository
Git
```

### Search behavior

The initial search records must contain enough context for lexical indexing:

```rust
SearchChunk {
    id,
    document_id,
    block_ids,
    lexical_text,
    semantic_text,
    keywords,
    aliases,
    topics,
    visibility,
    canonical_route,
}
```

`semantic_text` may initially equal lexical text, but it is a separate field because later embedding input will include contextual headings and metadata.

### Exit criteria

- Inline semantics round-trip into the web model.
- Search chunks are deterministic.
- Authored keywords, aliases, and topics are preserved with provenance.
- Search-record snapshots are tested.
- Links are represented without being silently converted into plain text.

### Explicitly deferred

- Full-text search engine integration.
- Embedding generation.
- Resolved backlinks.
- Section-aware semantic chunking.

---

## `0.0.4` — Headings, sections, and dual navigation

### Goal

Support Starship-style navigation: a site/document tree on the left and an in-page outline on the right.

### Included

- Heading parsing.
- Typed section hierarchy.
- Stable heading and section IDs.
- Document outline generation.
- In-page table of contents.
- Heading-level metadata such as:

```text
include_in_toc
toc_title
explicit_id
```

- Section-aware search chunks.
- Section title and ancestor path included in semantic search text.

### Model additions

```rust
Block::Section
SectionBlock
DocumentOutline
OutlineNode
SectionPath
```

### Search behavior

Search results can target a section rather than only a whole document. Search chunks should include:

```text
document title
section path
block text
keywords
topics
```

### Exit criteria

- Nested headings produce a deterministic document tree.
- The reference website renders left-page content and right-side in-page navigation.
- Direct links to section IDs work.
- Search chunks contain heading context.
- Heading hierarchy errors produce diagnostics.

### Explicitly deferred

- Global multi-repository navigation.
- Frontmatter-driven document placement.
- Cross-document semantic relations.

---

## `0.0.5` — Frontmatter, document identity, and typed metadata

### Goal

Establish Codexa’s canonical authoring contract for individual documents.

### Included

- YAML frontmatter.
- Versioned document schema.
- Strongly typed identifiers.
- Required metadata:

```yaml
schema:
id:
title:
kind:
```

- Optional metadata:

```yaml
summary:
visibility:
status:
order:
slug:
topics:
keywords:
aliases:
relations:
distribution:
```

- Document kinds including documentation, guide, concept, architecture, reference, decision, idea, theory, essay, experiment, note, journal, review, gallery, and custom.
- Generated JSON Schema for frontmatter.
- `codexa check` validation.

### Search behavior

Metadata becomes searchable and filterable. Search ranking may boost title, aliases, explicit keywords, and stable/published content, but ranking weights remain generated configuration rather than Markdown fields.

### Exit criteria

- Frontmatter deserializes into typed Rust structures.
- JSON Schema is generated from Rust types.
- Duplicate IDs and invalid enum values fail validation.
- Search records include document metadata and provenance.
- The web renderer can choose layouts from semantic `kind` without source-level CSS instructions.

### Explicitly deferred

- Repository acquisition.
- Global tree mounting.
- Public/private artifact separation.

---

## `0.0.6` — Assets and richer blocks

### Goal

Support the content needed for real documentation, essays, reviews, and photography-oriented pages.

### Included

- Images and assets.
- Figures and captions.
- Code blocks.
- Lists.
- Block quotes.
- Tables.
- Thematic breaks.
- Typed directives for semantic blocks:

```text
callout
concept
principle
figure
gallery
diagram
embed
```

- Block-level metadata.
- Asset identity, source path, media type, visibility, and content hash.
- Renderer capability and fallback declarations.

### Search behavior

- Figure captions and alt text are indexed.
- Code search remains lexical by default.
- Tables may generate row-level or table-level search chunks depending on renderer/index configuration.
- Very small or contextless blocks are grouped into section-aware chunks rather than embedded independently.

### Exit criteria

- Every supported block serializes through the target-neutral model.
- Assets are copied only when referenced by the artifact plan.
- Website fallbacks are tested.
- Search chunking rules are deterministic and documented.
- Unsupported renderer features generate warnings or explicit fallbacks.

### Explicitly deferred

- Remote Git sources.
- Public/private builds.
- Vector embeddings.

---

## `0.0.7` — Workspace, repositories, and tree grafting

### Goal

Aggregate content from multiple repositories and mount each repository-owned subtree into one global website tree.

### Included

- `codexa.toml` workspace configuration.
- `.codexa/source.toml` repository manifest.
- Local source adapter.
- Git CLI source adapter using existing SSH and credential configuration.
- Branch, tag, commit, and revision resolution.
- Reproducible `codexa.lock` with exact resolved commits.
- Content-root discovery.
- Global roots and mount points.
- Repository-owned local navigation tree.
- Tree grafting into the global site hierarchy.
- Route resolution and collision detection.
- Source provenance for every document and block.

### Search behavior

Search records inherit:

```text
workspace root
source repository
mount path
repository subtree
canonical route
Git commit
```

This allows search and generated views to filter by project, domain, repository, branch, or section without changing canonical document placement.

### Exit criteria

- At least two fixture repositories mount into one workspace.
- Exact Git revisions are recorded.
- Left navigation is generated from the global tree and source subtrees.
- Route and ID collisions produce diagnostics.
- Search records identify source and tree ancestry.

### Explicitly deferred

- Public/private projections.
- Full-text and vector index engines.
- Dynamic query-backed pages.

---

## `0.0.8` — Visibility projections and web search

### Goal

Produce separate public and private website artifacts with fast lexical and metadata-aware search.

### Included

- Visibility inheritance.
- Public, private, unlisted, and draft states.
- Public and private workspace projections.
- Independent navigation, backlinks, topics, routes, assets, search records, feeds, and sitemaps per projection.
- Validation preventing public artifacts from referencing private content or assets.
- Pagefind adapter for the static public website.
- SQLite FTS5 generated index for local and private use.
- Search filters for root, source, kind, status, visibility, topic, and date.
- Search-result explanations showing lexical and metadata matches where practical.

### Search architecture

```text
resolved workspace
    ↓
visibility projection
    ↓
search chunks
    ├── Pagefind web index
    └── SQLite FTS5 index
```

Search indexes are generated independently for public and private scopes. Private titles, keywords, assets, and embeddings must never enter public artifacts.

### Exit criteria

- Public output contains no private records or assets.
- Public and private navigation differ correctly.
- Pagefind returns fast static-site results.
- SQLite FTS5 supports CLI or local search.
- Metadata filters work across multiple repositories.
- Projection and search-leak tests pass.

### Explicitly deferred

- Embedding generation.
- Hybrid lexical/semantic ranking.
- LLM enrichment workflow.

---

## `0.0.9` — Semantic search, generated views, and release hardening

### Goal

Add modern hybrid retrieval and prove that one canonical tree can support many dynamically composed contextual views.

### Included

- Embedding-provider abstraction.
- Versioned embedding model identity.
- Section-aware semantic chunking.
- Incremental embedding generation based on content and preprocessing hashes.
- Vector-index abstraction.
- Initial local vector implementation appropriate for the expected corpus size.
- Optional future adapters reserved for HNSW or external vector stores.
- Hybrid ranking combining:

```text
lexical relevance
semantic similarity
metadata boosts
structural context
graph relationships
```

- Individual score components retained for diagnostics and tuning.
- Query-backed generated pages and collections.
- Structured filters such as topic, source, kind, status, root, relation, and visibility.
- LLM-assisted metadata proposal workflow.
- Clear separation between authored, inferred, and generated metadata.
- Incremental artifact fingerprints and output manifests.
- Complete migration and architecture documentation for `0.1.0`.

### Embedding rules

- Embeddings are never stored in Markdown.
- Embeddings belong to derived `SearchChunk` records, not source blocks.
- Every embedding records:

```text
chunk ID
model ID
dimensions
content hash
preprocessing version
generation timestamp
```

- Unchanged chunks reuse their embeddings.
- Changing the embedding model or preprocessing version invalidates only the affected vector index.
- Public and private embeddings are generated from their respective projections.

### Generated views

Codexa can generate pages that do not exist as canonical Markdown files, such as:

```text
/topics/hdr-imaging/
/technologies/rust/
/hardware/cameras/
/projects/using-computer-vision/
```

These views reference canonical documents and blocks without duplicating source content.

### Exit criteria

- Hybrid search returns lexical and semantic scores.
- Search results preserve canonical routes and section anchors.
- Embeddings regenerate incrementally.
- Generated contextual pages render from queries against the resolved graph and indexes.
- LLM enrichment produces reviewable diffs and does not silently mutate canonical metadata.
- Full end-to-end fixture workspace passes from Git acquisition through website and search artifacts.

---

## `0.1.0` — First complete working release

### Goal

Deliver a coherent, documented, reusable Git-native content compiler suitable for Codexa’s first real personal website and other small multi-repository workspaces.

### Required capabilities

#### Source and configuration

- Workspace configuration through `codexa.toml`.
- Repository-local manifests through `.codexa/source.toml`.
- Local and Git sources.
- Branch, tag, commit, and revision support.
- Reproducible lock file.

#### Content model

- Versioned frontmatter.
- Stable typed document and block IDs.
- Paragraphs, inline formatting, headings, sections, links, images, figures, code, lists, quotes, tables, and selected directives.
- Source spans and provenance.
- Target-neutral normalized model.

#### Structure

- Global navigation tree.
- Repository subtree ownership.
- Tree grafting at workspace-defined mount points.
- In-page document outline.
- Semantic topic and relation graph.
- Canonical routes and section anchors.

#### Visibility

- Public and private projections.
- Separate navigation, assets, indexes, and artifacts.
- Validation against public/private leakage.

#### Web artifact

- Reference static website renderer or stable web-model adapter.
- Light and dark theme support through semantic design tokens.
- Root-domain accent inheritance.
- Paragraph, section, navigation, and supported rich-block rendering.

#### Search

- Fast lexical website search.
- Local/private FTS index.
- Metadata and structural filters.
- Block- or section-level search results.
- Optional semantic embeddings through a stable provider interface.
- Hybrid retrieval where embeddings are configured.
- Incremental indexing and embedding reuse.

#### Generated views

- Query-backed topic, technology, project, or relation pages.
- Canonical source content remains singular even when surfaced in multiple contexts.

#### Quality

- Compiler-style diagnostics with stable codes.
- Unit, snapshot, fixture, and CLI integration tests.
- JSON Schema generation.
- Architecture, authoring-format, search, privacy, and renderer documentation.
- Example multi-repository workspace.
- Clean `cargo package` and `cargo publish --dry-run`.

### Compatibility promise

At `0.1.0`, Codexa begins treating these as intentional public contracts:

- workspace schema;
- source manifest schema;
- document frontmatter schema;
- normalized artifact schema;
- diagnostic codes;
- renderer capability model;
- search-record schema;
- lock-file format.

Breaking changes may still occur before `1.0.0`, but they require documented migrations rather than silent format replacement.

---

# Search design summary

Codexa search is built as a set of adapters over a common derived representation:

```text
typed documents and blocks
    ↓
search chunk planner
    ↓
SearchChunk records
    ├── lexical text
    ├── semantic text
    ├── metadata
    ├── tree ancestry
    ├── graph relations
    ├── visibility
    └── canonical route
        ↓
indexes
    ├── Pagefind
    ├── SQLite FTS5
    ├── local vector index
    └── future search adapters
```

## Canonical versus derived search data

### Canonical, authored in Markdown or frontmatter

- title;
- summary;
- document kind;
- visibility;
- status;
- topics;
- aliases;
- keywords;
- explicit relations;
- stable IDs;
- block-level semantic directives.

### Inferred or generated

- plain-text extraction;
- heading and section context;
- backlinks;
- word count;
- reading time;
- entity extraction;
- suggested topics or aliases;
- semantic chunk boundaries;
- embeddings;
- lexical indexes;
- vector indexes;
- relatedness scores;
- ranking weights;
- search-result excerpts.

Generated data remains rebuildable and records its provenance, model, version, and content hash.

## Search principles

1. Lexical search remains available even when semantic search is disabled.
2. Exact names, code symbols, IDs, and phrases should favor lexical retrieval.
3. Conceptual questions should benefit from semantic similarity.
4. Metadata and tree ancestry constrain and explain results.
5. Search operates on contextual chunks, not blindly on every AST node.
6. Search results always point back to canonical documents and blocks.
7. Public search indexes contain only public projection data.
8. Embeddings are optional generated artifacts, never canonical content.
9. LLM-generated metadata must be reviewable or clearly marked as inferred.
10. Search adapters must remain replaceable without changing the content model.

---

# How to determine the current project state

Given a Codexa repository snapshot:

1. Read the package version from `Cargo.toml`.
2. Find the matching version section in this roadmap.
3. Treat all earlier release exit criteria as required completed behavior.
4. Treat the current version’s included work as the active implementation boundary.
5. Treat later versions as pending unless the repository explicitly documents an early implementation.
6. Confirm implementation status through tests, fixtures, schemas, and CLI behavior rather than version number alone.
7. Do not infer that a future feature is complete merely because placeholder types or modules exist.

For example:

```text
Cargo.toml version = 0.0.4
```

Expected state:

```text
complete:
  repository bootstrap
  paragraph web artifact
  inline content
  lexical search records
  headings and dual navigation

pending:
  frontmatter
  rich blocks and assets
  repository aggregation
  public/private projections
  full-text indexes
  embeddings
  generated contextual views
```

---

# Release checklist

Before advancing `Cargo.toml` to the next version:

```text
[ ] Current release exit criteria are implemented.
[ ] Unit tests pass.
[ ] Snapshot tests are reviewed.
[ ] Fixture tests pass.
[ ] CLI integration tests pass.
[ ] Diagnostics use stable codes.
[ ] Serialized schemas are versioned.
[ ] Documentation matches implemented behavior.
[ ] CHANGELOG.md contains the release entry.
[ ] cargo fmt --all --check passes.
[ ] cargo clippy --all-targets --all-features -- -D warnings passes.
[ ] cargo test --all-targets --all-features passes.
[ ] cargo package passes from a clean tree.
[ ] cargo publish --dry-run passes when the release is intended for crates.io.
```

The version number should advance only after the current milestone is complete, not when work on that milestone begins.
