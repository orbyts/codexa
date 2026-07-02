# Codexa

Codexa is a Git-native content compiler that assembles structured content from multiple repositories into one typed, searchable body of knowledge.

Each repository continues to own the documentation and content relevant to it. Codexa retrieves those sources, interprets their metadata and structure, mounts them into a larger workspace tree, and generates public, private, and platform-specific artifacts.

The website is not a second source of truth. It is a generated projection of content maintained where it naturally belongs.

## Why Codexa

Documentation is commonly maintained in Git and rendered into a dedicated website. This provides version history, reviewable changes, branches, tags, and reproducible builds.

Codexa extends that model across many repositories and content domains:

```text
Git repositories
    ├── project documentation
    ├── architecture notes
    ├── ideas and theories
    ├── essays and reviews
    ├── photography
    └── other structured content
            ↓
        Codexa
            ↓
    typed content graph
            ↓
    generated artifacts
        ├── public website
        ├── private website
        ├── Obsidian vault
        ├── search indexes
        ├── Substack exports
        ├── Notion exports
        └── future targets
```

A repository owns its local content tree. The Codexa workspace decides where that tree belongs within the larger publication.

This allows content to be written and updated once, while Codexa handles aggregation, navigation, visibility, indexing, search, and target-specific presentation.

## Core principles

- Git files are the canonical source of truth.
- Content remains close to the project or subject it documents.
- Markdown is parsed into a typed, target-neutral content model.
- The global workspace mounts repository-owned trees into a larger hierarchy.
- Navigation is represented as a tree.
- Cross-document meaning is represented as a graph.
- Public and private outputs are separate visibility projections.
- Search combines lexical, structured, graph-based, and semantic retrieval.
- Embeddings and inferred metadata are generated artifacts, not canonical source.
- Renderers adapt the same content model for different destinations.
- Static generation is the default, but dynamic components can be added where needed.
- Presentation, themes, JavaScript, commerce, and authentication remain separate from canonical content.

## Version policy

- `0.0.1` through `0.0.9`: internal pre-release development
- `0.1.0`: first working release

Codexa evolves through small, complete vertical slices.

The package version in `Cargo.toml` identifies the active architectural milestone and the capabilities that should already be implemented. An inspection of the repository, its tests, and [the roadmap](docs/roadmap.md) should be enough to determine the current project state and remaining work.

## Current milestone: 0.0.2

Version `0.0.2` introduces the first complete content pipeline:

```text
Markdown paragraph
    ↓
Markdown parser
    ↓
typed Paragraph block
    ↓
Document model
    ↓
JSON web artifact
    ↓
website renderer
```

This milestone intentionally supports only paragraphs.

The goal is to prove the boundary between Codexa and its first web consumer before expanding the content model.

Codexa owns:

- parsing
- typed content
- stable serialization
- validation
- artifact generation

The `0.0.2` web adapter emits a versioned JSON bundle. A separate website repository will consume that bundle.

The website owns:

- HTML components
- typography
- layout
- design tokens
- light and dark themes
- domain accent colors
- browser interaction

## Planned architecture

```text
source repositories
        ↓
source acquisition
        ↓
document discovery
        ↓
frontmatter and Markdown parsing
        ↓
typed source documents
        ↓
normalized content model
        ↓
resolved workspace
    ├── navigation tree
    ├── knowledge graph
    ├── asset graph
    └── visibility graph
        ↓
artifact projections
    ├── public
    ├── private
    └── target-specific
        ↓
renderers
    ├── web
    ├── Obsidian
    ├── search
    ├── Substack
    ├── Notion
    └── additional targets
```

## Search direction

Codexa will support several complementary forms of retrieval:

```text
lexical search
    exact words, phrases, symbols, and names

structured search
    metadata, topics, document kinds, repositories, and branches

graph search
    links, backlinks, relations, and shared concepts

semantic search
    embedding-based conceptual similarity
```

Search will operate on derived contextual chunks rather than attaching a permanent embedding directly to every Markdown node.

Each search chunk may include:

- its source document
- source repository
- canonical tree location
- section hierarchy
- block identities
- authored topics and keywords
- inferred metadata
- visibility
- lexical text
- semantic text
- embedding provenance

The initial website can use a static lexical search tool such as Pagefind. Later Codexa-native indexes may use SQLite FTS5 and generated embedding indexes for hybrid retrieval.

## Static-first, not static-only

Most Codexa content can be statically generated.

Dynamic functionality may later be layered onto the website for:

- interactive diagrams
- knowledge graph exploration
- authenticated private content
- live project information
- browser editing
- customer accounts
- software licensing
- commerce and paid downloads

Codexa will provide the structured content and product references. Website runtimes and external services will handle transactions, credentials, customer records, and authorization.

## Build the 0.0.2 web artifact

```bash
cargo run -- build examples/paragraph/document.md --adapter web --output dist/web
```

The command writes:

```text
dist/web/
├── manifest.json
└── documents/
    └── sample.json
```

## Development

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo run
```

See [docs/roadmap.md](docs/roadmap.md) for the complete `0.0.1` through `0.1.0` development plan.

## License

MIT
