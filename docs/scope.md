# Codexa Scope and Architectural Boundary

## Purpose

Codexa is a Git-native structured-content compiler.

Its responsibility is to collect source content from one or more Git repositories, parse and validate that content, resolve it into typed models, and emit stable, versioned artifacts for downstream consumers.

Codexa is not the website, not the commerce system, not the image-processing pipeline, and not the universal build tool for every asset type.

The canonical source of truth remains the files and repositories that Codexa reads. Codexa produces projections of that source content for specific targets.

---

## Core responsibility

Codexa owns:

- discovering configured content repositories
- reading structured Markdown and related source files
- parsing source content into typed internal models
- validating syntax, structure, metadata, references, and visibility rules
- resolving repository trees, navigation, relationships, and graph data
- producing deterministic, versioned artifacts
- recording source provenance and exact source commits
- generating public, private, and platform-specific projections
- exposing diagnostics suitable for local development and CI
- determining which logical outputs are affected by source changes
- remaining independent of any particular website framework

Codexa may eventually support multiple artifact targets.

The first target is:

```text
web JSON artifacts
```

The website is one consumer of those artifacts.

---

## Current vertical slice

```text
Markdown source
    ↓
Codexa parser
    ↓
typed document model
    ↓
validated web JSON artifact
    ↓
separate website assembler and renderer
```

Codexa currently owns everything above the serialized artifact boundary.

The website owns everything below it.

---

## Explicit non-goals

Codexa should not become responsible for:

- rendering HTML
- choosing a frontend framework
- website layouts and typography
- CSS design tokens or themes
- browser-side interaction
- authentication and user sessions
- payments, orders, subscriptions, or refunds
- customer entitlements
- software-license issuance
- secure customer downloads
- user progress tracking
- application databases
- deployment-provider integration
- image resizing, transcoding, or CDN upload
- RAW photography processing
- video encoding or streaming
- building platform-specific software binaries
- signing release binaries
- acting as the umbrella CLI for every future tool

These responsibilities belong to downstream applications or specialized artifact producers.

---

## Relationship to the website assembler

The website assembler is a separate tool and repository.

It consumes Codexa's versioned web artifact contract and owns:

- loading and validating Codexa artifacts
- resolving artifacts into website routes
- rendering HTML
- layouts and components
- typography and design tokens
- light and dark themes
- responsive behavior
- JavaScript interactions
- static and dynamic web routes
- accounts, checkout, libraries, and protected content
- deployment of the resulting website

The website must depend only on Codexa's published artifact contract.

It must not import or depend on Codexa's internal Rust modules.

```text
Codexa internal models
        ↓ serialization
versioned artifact contract
        ↓ deserialization
website-owned client models
```

A change to Codexa's internal implementation should not require a website change unless the published artifact contract changes.

---

## Relationship to future specialized tools

Codexa is one producer in a larger publishing toolkit.

Future producers may include:

```text
Codexa
    structured writing, documentation, articles, metadata,
    navigation, taxonomy, references, knowledge-graph data

Photography pipeline
    image derivatives, color profiles, thumbnails, responsive
    variants, EXIF, gallery manifests, asset upload

Software release pipeline
    builds, checksums, signatures, release manifests,
    platform variants, downloadable binaries

Tutorial media pipeline
    video transcodes, lesson attachments, exercise assets,
    media manifests
```

These tools may all emit web-consumable artifacts, but they should remain specialized and independently versioned.

The website assembler can consume artifacts from all of them.

```text
specialized producers
        ↓
versioned artifacts
        ↓
website assembler
        ↓
complete website
```

Codexa may reference objects owned by another producer without taking ownership of that producer's processing pipeline.

For example, Codexa may emit:

```yaml
type: gallery
gallery_id: sf-night-2026
```

The photography pipeline would own the gallery asset manifest and image derivatives.

Similarly, Codexa may reference a software product or tutorial package while another system owns binaries, videos, purchases, and access control.

---

## Shared artifact principles

Every producer should eventually follow a compatible artifact envelope.

Example:

```json
{
  "producer": "codexa",
  "producer_version": "0.0.2",
  "schema": "example.web-document/1",
  "generated_at": "2026-07-02T17:00:00Z",
  "source": {
    "repository": "owner/repository",
    "commit": "abc123"
  },
  "payload": {}
}
```

The exact schema is not fixed by this document, but the following concepts should have explicit homes:

- producer name
- producer version
- artifact schema name
- artifact schema version
- generation timestamp
- source repository
- exact source commit
- visibility or projection
- deterministic artifact identity
- deterministic route or output identity where applicable
- content hashes or fingerprints
- diagnostics
- dependency information
- affected logical outputs
- public/private separation

Codexa should define these clearly for its own artifacts and align with any future shared artifact protocol without becoming responsible for every producer.

---

## Build and publishing model

The normal publishing flow should be:

```text
edit content in a source repository
    ↓
commit and push
    ↓
merge to the configured publishing branch
    ↓
notify the website build
    ↓
website CI obtains a pinned Codexa version
    ↓
Codexa fetches and validates configured sources
    ↓
Codexa generates current artifacts
    ↓
website consumes all producer artifacts
    ↓
website builds and deploys atomically
```

Source repositories should only notify the website that relevant content changed.

They should not duplicate Codexa's dependency logic in GitHub Actions.

Codexa determines content relationships and affected logical outputs.

The website assembler decides how those outputs map to rendered pages and deployment behavior.

Initially, rebuilding the entire website is acceptable and preferred for simplicity and correctness.

Incremental rebuilding should only be implemented when build time makes it necessary.

---

## Configuration boundary

Codexa's future workspace configuration may declare:

- source repositories
- publishing branches
- content roots
- visibility rules
- artifact targets
- artifact output locations
- validation policies

The website assembler's configuration may declare:

- artifact sources
- route mappings
- themes
- rendering adapters
- application services
- deployment provider
- commerce and authentication integrations

Neither tool should silently absorb the other's configuration responsibilities.

---

## CLI boundary

Codexa should expose commands appropriate to a content compiler, such as:

```text
codexa validate
codexa build
codexa inspect
codexa graph
```

A future umbrella toolkit may expose commands such as:

```text
<toolkit> dev
<toolkit> build
<toolkit> publish
```

That umbrella CLI may orchestrate Codexa, photography tools, release tools, the website assembler, and deployment adapters.

Codexa should remain independently usable and should not itself become the umbrella merely because it is the first producer.

One user-facing workflow does not require one monolithic implementation.

---

## Decision test for future Codexa features

Before adding a feature to Codexa, ask:

1. Is this feature about discovering, parsing, validating, resolving, or serializing structured source content?
2. Does it belong in the canonical content model rather than in a specific presentation?
3. Can the result be useful to more than one downstream consumer?
4. Can it be represented as a stable artifact contract?
5. Would implementing it in Codexa keep Codexa independent of Astro, Cloudflare, Stripe, image CDNs, or other application-specific systems?

If most answers are yes, it likely belongs in Codexa.

If the feature primarily concerns presentation, deployment, users, payments, media processing, or a specialized asset lifecycle, it likely belongs elsewhere.

---

## Practical rule

Codexa should be excellent at this:

```text
Git repositories
    ↓
typed, validated, deterministic knowledge artifacts
```

It should not attempt to own this entire chain:

```text
source files
    ↓
all media processing
    ↓
all application behavior
    ↓
all commerce
    ↓
all deployment
```

Codexa is a foundational producer in the toolkit, not the whole toolkit.
