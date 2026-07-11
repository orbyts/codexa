---
schema: codexa.document@2
id: codexa.reference.frontmatter
title: Codexa Frontmatter Reference
description: Required and optional YAML frontmatter fields for Codexa source documents.
kind: reference
status: active
visibility: public
tags:
  - codexa
  - frontmatter
  - yaml
  - schema
navigation:
  root: docs
  product: codexa
  section: Reference
  order: 20
distribution:
  notion: true
  web: public
  obsidian: true
notion:
  workspace: codexa
web:
  slug: /docs/codexa/reference/frontmatter
---

# Codexa Frontmatter Reference

Codexa frontmatter is YAML embedded at the beginning of a Markdown document. It is a versioned authoring contract, not endpoint configuration copied from Notion or a website.

## Complete example

```yaml
---
schema: codexa.document@2
id: codexa.guides.quick-start
title: Codexa Quick Start
description: Organize a repository and build endpoint artifacts.
kind: guide
status: active
visibility: public
tags:
  - codexa
  - quick-start
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
```

## Required fields

`schema`
: Must be `codexa.document@2` during the current pre-`0.1.0` development contract.

`id`
: Stable, globally unique document identity. Links and endpoint registries use this value rather than the source filename.

`title`
: Human-facing page title.

`description`
: Short description used by endpoint indexes, search, previews, and metadata.

`kind`
: Content role such as `guide`, `concept`, `reference`, `operation`, `contract`, `playbook`, or `index`.

`status`
: Lifecycle state such as `draft`, `active`, or `archived`.

`visibility`
: Intended visibility such as `public`, `private`, or `unlisted`.

`navigation`
: Required tree placement. `root` identifies the major content tree, `product` identifies the repository or project subtree, `section` groups related pages, and `order` controls sorting.

`distribution`
: Enables or disables endpoint artifact generation.

## Conditional endpoint fields

When `distribution.notion` is `true`, provide:

```yaml
notion:
  workspace: codexa
```

When `distribution.web` is not `off`, provide an absolute site path:

```yaml
web:
  slug: /docs/codexa/reference/frontmatter
```

`distribution.obsidian` records whether the document is eligible for a future Obsidian export. Obsidian remains an endpoint; Git remains authoritative.

## Internal links

Use stable document IDs:

```md
[[codexa.guides.quick-start]]
[[orbexa.guides.quick-start|Orbexa Quick Start]]
```

See [[codexa.concepts.navigation-and-links|Navigation and Links]] for resolution behavior.

## Version policy

Before Codexa, Orbexa, and Loomara reach `0.1.0`, the three projects may move together through breaking contract changes. At `0.1.0`, the first public contract is frozen. Subsequent feature releases must account for compatibility and migration.

