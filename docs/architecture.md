# Architecture

Codexa will follow a compiler-style pipeline:

1. Resolve configured Git sources.
2. Parse repository manifests and content documents.
3. Normalize content into target-neutral typed structures.
4. Resolve the global navigation tree and cross-content graph.
5. Project content by visibility and audience.
6. Render target-specific artifacts.

The precise crate and module structure will be decided iteratively before the first working `0.1.0` release.
