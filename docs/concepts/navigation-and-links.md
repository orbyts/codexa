---
schema: codexa.document@2
id: codexa.concepts.navigation-and-links
title: Navigation and Links
description: How Codexa models shared content trees and portable links across Notion, websites, and future endpoints.
kind: concept
status: active
visibility: public
tags:
  - codexa
  - navigation
  - links
  - endpoints
navigation:
  root: docs
  product: codexa
  section: Concepts
  order: 30
distribution:
  notion: true
  web: public
  obsidian: true
notion:
  workspace: codexa
web:
  slug: /docs/codexa/concepts/navigation-and-links
---

# Navigation and Links

Codexa separates source identity, navigation intent, and endpoint location.

## Navigation tree

A document declares its conceptual location:

```yaml
navigation:
  root: docs
  product: codexa
  section: Concepts
  order: 30
```

The same values can drive:

- the Docs database in Notion;
- `/docs/codexa/...` navigation on the primary website;
- a future Obsidian vault folder tree;
- future search and publishing indexes.

The source repository path remains provenance, but it does not define endpoint routing.

## Logical links

Internal links target stable document IDs:

```md
[[codexa.guides.quick-start]]
[[orbexa.guides.quick-start|Orbexa Quick Start]]
```

Codexa validates that every target exists in the full multi-repository build. It emits structured link metadata without assigning endpoint URLs.

## Endpoint resolution

Orbexa performs a two-pass apply. It first establishes every Notion page identity, then resolves logical links to Notion page URLs and updates final content. See [[orbexa.concepts.two-pass-sync|Orbexa Two-Pass Sync]].

Loomara will build a document-ID-to-web-slug index before rendering pages. Obsidian export can resolve the same IDs to vault-relative wiki links.

## External links

External destinations use normal Markdown:

```md
[GitHub](https://github.com/)
```

Codexa preserves external URLs because they are already endpoint-independent.
