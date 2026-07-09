use serde::{Deserialize, Serialize};

/// Versioned Codexa document metadata parsed from Markdown frontmatter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Frontmatter schema version.
    pub schema: String,
    /// Stable document identifier.
    pub id: String,
    /// Human-facing title.
    pub title: String,
    /// Short one-line description.
    pub description: String,
    /// Flexible document kind, such as playbook, recipe, review, inventory, or note.
    pub kind: String,
    /// Flexible lifecycle status, such as draft, active, archived, or private.
    pub status: String,
    /// Visibility policy, such as private, unlisted, or public.
    pub visibility: String,
    /// Search/display tags.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Distribution targets.
    pub distribution: Distribution,
    /// Optional Notion placement metadata.
    pub notion: Option<NotionTarget>,
}

/// Distribution policy for downstream targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Distribution {
    /// Whether this document should be emitted for Notion.
    pub notion: bool,
    /// Website distribution mode, such as private, public, hidden, or false-like values later.
    pub web: String,
}

/// Notion placement metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionTarget {
    /// Portable Orbexa workspace key.
    pub workspace: String,
    /// Portable data source key.
    pub data_source: String,
}

/// A Markdown source document split into metadata and body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDocument {
    /// Parsed frontmatter metadata.
    pub metadata: DocumentMetadata,
    /// Markdown body after the frontmatter.
    pub body: String,
}

impl DocumentMetadata {
    /// Validates the current metadata schema.
    pub fn validate_schema(&self) -> Result<(), String> {
        if self.schema != "codexa.document@1" {
            return Err(format!("unsupported document schema `{}`", self.schema));
        }

        Ok(())
    }
}
