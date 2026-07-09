use serde::{Deserialize, Serialize};

use crate::model::SourceDocument;

/// Versioned JSON artifact consumed by Orbexa.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionPageArtifact {
    /// Artifact schema version.
    pub schema: String,
    /// Artifact producer.
    pub producer: String,
    /// Producer package version.
    pub producer_version: String,
    /// Document metadata and source content.
    pub document: NotionDocument,
    /// Source provenance.
    pub source: NotionSource,
    /// Target placement metadata.
    pub target: NotionTarget,
    /// Page content payload.
    pub content: NotionContent,
}

/// User-facing document metadata for downstream targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionDocument {
    pub schema: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub kind: String,
    pub status: String,
    pub visibility: String,
    pub tags: Vec<String>,
}

/// Source provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionSource {
    pub repository: String,
    pub path: String,
    pub commit: String,
    pub content_hash: String,
}

/// Notion placement target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionTarget {
    pub workspace: String,
    pub data_source: String,
}

/// Page content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionContent {
    pub format: String,
    pub markdown: String,
}

impl NotionPageArtifact {
    /// Builds a page artifact from a parsed source document.
    #[must_use]
    pub fn new(
        source_document: SourceDocument,
        repository: impl Into<String>,
        source_path: impl Into<String>,
        commit: impl Into<String>,
        content_hash: impl Into<String>,
    ) -> Self {
        let metadata = source_document.metadata;
        let notion = metadata
            .notion
            .clone()
            .expect("notion artifact requires notion target metadata");

        Self {
            schema: "codexa.notion.page@1".into(),
            producer: "codexa".into(),
            producer_version: crate::VERSION.into(),
            document: NotionDocument {
                schema: metadata.schema,
                id: metadata.id,
                title: metadata.title,
                description: metadata.description,
                kind: metadata.kind,
                status: metadata.status,
                visibility: metadata.visibility,
                tags: metadata.tags,
            },
            source: NotionSource {
                repository: repository.into(),
                path: source_path.into(),
                commit: commit.into(),
                content_hash: content_hash.into(),
            },
            target: NotionTarget {
                workspace: notion.workspace,
                data_source: notion.data_source,
            },
            content: NotionContent {
                format: "markdown".into(),
                markdown: source_document.body,
            },
        }
    }
}

/// Manifest describing a Codexa Notion artifact bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionManifest {
    pub schema: String,
    pub producer: String,
    pub producer_version: String,
    pub pages: Vec<NotionManifestPage>,
}

/// One page entry in a Notion artifact manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionManifestPage {
    pub document_id: String,
    pub path: String,
}

impl NotionManifest {
    /// Creates a manifest for a single page artifact.
    #[must_use]
    pub fn single(document_id: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            schema: "codexa.notion.manifest@1".into(),
            producer: "codexa".into(),
            producer_version: crate::VERSION.into(),
            pages: vec![NotionManifestPage {
                document_id: document_id.into(),
                path: path.into(),
            }],
        }
    }
}
