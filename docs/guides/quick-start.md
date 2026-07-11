---
schema: codexa.document@2
id: codexa.guides.quick-start
title: Codexa Quick Start
description: Organize a Git repository, author compatible Markdown, validate links, and build endpoint artifacts.
kind: guide
status: active
visibility: public
tags:
  - codexa
  - quick-start
  - markdown
  - git
navigation:
  root: docs
  product: codexa
  section: Guides
  order: 10
distribution:
  notion: true
  web: public
  obsidian: true
notion:
  workspace: codexa
web:
  slug: /docs/codexa/guides/quick-start
---

# Codexa Quick Start

Codexa reads Markdown documents from one or more Git repositories and emits a shared bundle plus endpoint-specific artifacts. The repositories remain authoritative; generated output is disposable.

## 1. Organize the repository

A typical documentation subtree looks like this:

```text
docs/
├── index.md
├── guides/
│   └── quick-start.md
├── concepts/
└── reference/
```

The physical folder layout helps humans and GitHub users. The canonical navigation tree comes from frontmatter.

## 2. Add required frontmatter

Every Codexa source document begins with YAML frontmatter using `codexa.document@2`:

```yaml
---
schema: codexa.document@2
id: example.guides.quick-start
title: Example Quick Start
description: Configure and build the example repository.
kind: guide
status: active
visibility: public
tags:
  - example
navigation:
  root: docs
  product: example
  section: Guides
  order: 10
distribution:
  notion: true
  web: public
  obsidian: true
notion:
  workspace: codexa
web:
  slug: /docs/example/guides/quick-start
---
```

See [[codexa.reference.frontmatter|required and optional frontmatter fields]] for the full contract.

## 3. Link documents by stable ID

Use logical links instead of relative file paths:

```md
[[codexa.reference.frontmatter]]
[[orbexa.guides.quick-start|Orbexa Quick Start]]
```

Codexa validates the target IDs across every source repository supplied to the build. See [[codexa.concepts.navigation-and-links|Navigation and Links]].

## 4. Validate repositories

```bash
codexa validate \
  --source-root "$MATRIX/crates/codexa" orbyts/codexa \
  --source-root "$MATRIX/crates/orbexa" orbyts/orbexa
```

Validation rejects unsupported schemas, duplicate document IDs, malformed logical links, and links to missing documents.

## 5. Build artifacts

```bash
codexa build \
  --source-root "$MATRIX/crates/codexa" orbyts/codexa \
  --source-root "$MATRIX/crates/orbexa" orbyts/orbexa \
  --output dist
```

The build produces:

```text
dist/
├── bundle.json
├── notion/
│   ├── manifest.json
│   └── pages/
└── web/
    ├── manifest.json
    └── pages/
```

## 6. Publish to an endpoint

Codexa does not call Notion or deploy a website. Endpoint tools consume its artifacts.

For Notion:

```bash
orbexa init
orbexa apply dist/notion
```

Continue with [[orbexa.guides.quick-start|Orbexa Quick Start]]. Loomara will consume the web artifacts using the same document IDs, navigation tree, and links.
