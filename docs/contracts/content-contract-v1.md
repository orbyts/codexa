# Codexa Content Contract v1

Codexa is a Git-native content compiler. Git repositories are the source of truth. Codexa reads agreed-upon Markdown documents, validates their metadata, and emits stable artifacts for downstream endpoints such as Notion, websites, search indexes, feeds, or future apps.

Notion is only one endpoint. It must not define the canonical document model.

## Goals

1. Keep GitHub repositories as the durable source of truth.
2. Let different repositories provide different kinds of content.
3. Keep Markdown frontmatter small, stable, and versioned.
4. Allow downstream adapters to map fields differently per target.
5. Keep endpoint-specific bookkeeping out of user-facing surfaces.
6. Support future website, search, vector database, and publishing adapters.

## Repository classes

Codexa should support many repository classes:

```text
Tool/project docs
Knowledge repositories
Personal notes and thoughts
Book and movie reviews
Recipes
Inventory and hardware records
Network structures
Photo/share catalogs with external asset URLs
```

Each repository may define its own config, but every document should still conform to the shared Codexa document contract.

## Markdown frontmatter schema

The first stable Markdown schema is:

```text
codexa.document@1
```

A Markdown document using this contract should start with YAML frontmatter:

```yaml
---
schema: codexa.document@1
id: workflows.playbooks.lureva-lightroom-handoff
title: Lureva Lightroom Handoff Manual
description: Daily Lightroom Classic handoff workflow for switching between quasar and eclipse during the Lureva 960 review.
kind: playbook
status: active
visibility: private

tags:
  - lureva
  - lightroom
  - nazariya
  - runbook

distribution:
  notion: true
  web: private
  search: private

routes:
  canonical: /workflows/playbooks/lureva-lightroom-handoff

targets:
  notion:
    workspace: codexa
    collection: documents
---
```

## Required fields

Every `codexa.document@1` document must include:

```yaml
schema: codexa.document@1
id: stable.document.id
title: Human Title
description: Short one-line description.
kind: reference
status: active
visibility: private
tags: []
distribution: {}
```

### `schema`

The schema version of the Markdown document metadata.

```yaml
schema: codexa.document@1
```

### `id`

A stable document identity. It should not change when the file moves.

Recommended style:

```text
workflows.playbooks.lureva-lightroom-handoff
recipes.hyderabadi-keema
reviews.movies.heat-1995
inventory.hardware.ms01-zion
photos.series.sf-fog-study
```

### `title`

The display title. This maps naturally to GitHub repository naming conventions and to Notion page names.

### `description`

A short one-line description, similar to a GitHub repository description. Use `description`, not `summary`, because it is simpler and maps better across GitHub, websites, Notion, and search indexes.

### `kind`

A flexible string that describes what kind of document this is.

Examples:

```text
reference
playbook
flow
runbook
recipe
movie_review
book_review
inventory_item
network_map
photo_story
tool_doc
project_note
personal_note
```

Codexa should not hard-code all possible `kind` values globally. Repository config may restrict or describe allowed kinds.

### `status`

The lifecycle state of the document.

Recommended baseline values:

```text
draft
active
archived
private_hold
```

Repositories may define their own status values in config, but downstream adapters can rely on this being a string.

### `visibility`

The broad default visibility of the document.

Recommended baseline values:

```text
private
unlisted
public
```

Target-specific visibility can override this in `distribution`.

### `tags`

Search/display tags. These are user-facing and portable.

Tags may be used by:

```text
Notion multi-select
Website filters
Search index facets
Vector index metadata
RSS/feed categories
```

### `distribution`

Target-level routing and visibility.

Example:

```yaml
distribution:
  notion: true
  web: private
  search: private
```

Suggested semantics:

```text
true       include using default visibility
false      exclude from that target
private    include only in private/internal target
unlisted   include but do not list broadly
public     include publicly
```

### `targets`

Endpoint placement hints. These are not the source of truth for endpoint IDs. They name logical destinations.

Example:

```yaml
targets:
  notion:
    workspace: codexa
    collection: documents
```

Orbexa maps `workspace: codexa` and `collection: documents` to real Notion IDs through its registry.

## Optional fields

Documents may include additional metadata depending on repository type.

Examples:

```yaml
aliases:
  - Lureva handoff
  - Lightroom transfer manual

created: 2026-07-09
updated: 2026-07-09

people:
  - Suhail

projects:
  - lureva
  - nazariya

assets:
  hero:
    url: https://res.cloudinary.com/example/image/upload/example.jpg
    alt: Foggy San Francisco street at night.
```

Repository-specific schemas may define additional fields for recipes, reviews, inventory, photos, or network structures.

## Repository config

Each Git repository should have a Codexa config file.

Suggested path:

```text
codexa.toml
```

Example:

```toml
schema = "codexa.repo@1"

[repo]
owner = "archivora"
name = "knowledge"
class = "knowledge"
default_visibility = "private"

[content]
root = "."
include = ["**/*.md"]
exclude = ["README.md", "node_modules/**", "dist/**"]

[kinds]
allowed = [
  "reference",
  "playbook",
  "flow",
  "runbook",
  "project_note",
  "personal_note",
]

[targets.notion]
enabled = true
workspace = "codexa"
collection = "documents"

[targets.web]
enabled = true
site = "suhail-ink"

[fields]
title = "title"
description = "description"
tags = "tags"
kind = "kind"
status = "status"
visibility = "visibility"
```

Different repositories can define different allowed kinds and target defaults.

## Codexa artifact schemas

Codexa should emit endpoint-neutral artifacts first, then endpoint-specific artifacts where useful.

### Canonical document artifact

Schema:

```text
codexa.artifact.document@1
```

Example:

```json
{
  "schema": "codexa.artifact.document@1",
  "producer": "codexa",
  "producer_version": "0.0.x",
  "document": {
    "schema": "codexa.document@1",
    "id": "workflows.playbooks.lureva-lightroom-handoff",
    "title": "Lureva Lightroom Handoff Manual",
    "description": "Daily Lightroom Classic handoff workflow for switching between quasar and eclipse during the Lureva 960 review.",
    "kind": "playbook",
    "status": "active",
    "visibility": "private",
    "tags": ["lureva", "lightroom", "nazariya", "runbook"]
  },
  "source": {
    "repository": "archivora/knowledge",
    "path": "workflows/playbooks/lureva-lightroom-handoff.md",
    "commit": "fcf10a0",
    "content_hash": "sha256:..."
  },
  "content": {
    "format": "markdown",
    "body": "..."
  }
}
```

### Notion page artifact

Schema:

```text
codexa.artifact.notion_page@1
```

Example:

```json
{
  "schema": "codexa.artifact.notion_page@1",
  "producer": "codexa",
  "producer_version": "0.0.x",
  "document_id": "workflows.playbooks.lureva-lightroom-handoff",
  "target": {
    "workspace": "codexa",
    "collection": "documents"
  },
  "properties": {
    "title": "Lureva Lightroom Handoff Manual",
    "description": "Daily Lightroom Classic handoff workflow for switching between quasar and eclipse during the Lureva 960 review.",
    "kind": "playbook",
    "status": "active",
    "tags": ["lureva", "lightroom", "nazariya", "runbook"]
  },
  "source": {
    "repository": "archivora/knowledge",
    "path": "workflows/playbooks/lureva-lightroom-handoff.md",
    "commit": "fcf10a0",
    "content_hash": "sha256:..."
  },
  "content": {
    "format": "markdown",
    "body": "..."
  }
}
```

Orbexa should consume this artifact and map it to Notion using Orbexa config.

## Orbexa config responsibilities

Orbexa config should decide:

1. Which Codexa artifact directories to read.
2. Which logical workspaces map to which Notion registries.
3. Which artifact fields map to visible Notion properties.
4. Which minimal schema to create or verify in Notion.

Example:

```toml
schema = "orbexa/config@1"

[notion]
api_version = "2026-03-11"
parent_page_id = "398a1865b187802aa885d97afc99896f"

[notion.bootstrap]
mode = "create"
root = "parent_page"

[workspace]
page_name = "Codexa"
database_name = "Knowledge"

[workspace.data_sources.documents]
name = "Documents"
kind = "documents"

[artifacts]
inputs = [
  { name = "knowledge", path = "/path/to/archivora/knowledge/dist/notion" },
]

[notion.properties]
title = "Name"
description = "Description"
kind = "Kind"
tags = "Tags"
status = "Status"
```

Orbexa should only create visible Notion properties that are mapped here.

## Minimal Notion schema

The default reader-facing Notion schema should be:

```text
Name          title
Description   rich_text
Kind          select
Tags          multi_select
Status        select
```

Do not add internal sync/bookkeeping fields to the visible Notion database by default.

Internal fields such as source path, source commit, content hash, and Notion page IDs belong in Orbexa lock/registry files.

## Orbexa lock file

Orbexa should track synced pages outside Notion's visible database columns.

Suggested path:

```text
$XDG_STATE_HOME/orbexa/notion.lock
```

Portable registry stays in:

```text
$XDG_CONFIG_HOME/orbexa/workspaces/*.toml
```

Example lock file:

```toml
schema = "orbexa/lock@1"

[[pages]]
document_id = "workflows.playbooks.lureva-lightroom-handoff"
notion_page_id = "..."
workspace = "codexa"
collection = "documents"
source_repository = "archivora/knowledge"
source_path = "workflows/playbooks/lureva-lightroom-handoff.md"
source_commit = "fcf10a0"
content_hash = "sha256:..."
last_synced_at = "2026-07-09T00:00:00Z"
```

## Content change detection

Codexa should calculate `content_hash` from the normalized document body plus relevant public metadata.

Orbexa should use:

```text
source_repository
source_path
source_commit
content_hash
```

to decide whether a Notion page is unchanged, metadata-only changed, or content changed.

Suggested behavior:

```text
same content_hash
  skip content update

different content_hash, same document_id
  update Notion page body and visible properties

document_id missing from artifact but present in lock
  mark stale or archive, never delete by default
```

## Design rule

Codexa answers:

```text
What structured content exists in Git, and what artifacts should downstream tools receive?
```

Orbexa answers:

```text
Given Codexa artifacts and an Orbexa mapping config, what exactly should change in Notion?
```

The website builder answers:

```text
Given Codexa web artifacts, how should the website look and behave?
```

Keep these boundaries separate.
