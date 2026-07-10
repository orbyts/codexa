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
    /// Flexible document kind, such as guide, concept, playbook, recipe, review, inventory, or note.
    pub kind: String,
    /// Flexible lifecycle status, such as draft, active, archived, or private.
    pub status: String,
    /// Visibility policy, such as private, unlisted, or public.
    pub visibility: String,
    /// Search/display tags.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Optional navigation/tree placement.
    ///
    /// New documents should provide this. Older documents are still accepted
    /// and downstream artifacts will derive compatibility defaults.
    pub navigation: Option<Navigation>,
    /// Distribution targets.
    pub distribution: Distribution,
    /// Optional Notion placement metadata.
    pub notion: Option<NotionTarget>,
    /// Optional website placement metadata.
    pub web: Option<WebTarget>,
}

/// Tree/navigation placement shared by all downstream adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Navigation {
    /// Root tree, such as docs, knowledge, code-reference, or media.
    pub root: String,
    /// Product/project/tool subtree, such as codexa, orbexa, loomara, or lureva.
    pub product: String,
    /// Human-facing section, such as Guides, Concepts, Reference, or Playbooks.
    pub section: Option<String>,
    /// Sort order within the section.
    pub order: Option<i64>,
}

/// Distribution policy for downstream targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Distribution {
    /// Whether this document should be emitted for Notion.
    pub notion: bool,
    /// Website distribution mode, such as private, public, hidden, or unlisted.
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

/// Website placement metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebTarget {
    /// Website collection, such as docs or knowledge.
    pub collection: String,
    /// Canonical website slug.
    pub slug: String,
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

        if self.id.trim().is_empty() {
            return Err("document id must not be empty".into());
        }

        if self.title.trim().is_empty() {
            return Err("document title must not be empty".into());
        }

        if self.description.trim().is_empty() {
            return Err("document description must not be empty".into());
        }

        if self.kind.trim().is_empty() {
            return Err("document kind must not be empty".into());
        }

        if self.status.trim().is_empty() {
            return Err("document status must not be empty".into());
        }

        if self.visibility.trim().is_empty() {
            return Err("document visibility must not be empty".into());
        }

        if self.distribution.notion {
            let notion = self.notion.as_ref().ok_or_else(|| {
                "notion target is required when distribution.notion is true".to_string()
            })?;

            if notion.workspace.trim().is_empty() {
                return Err("notion.workspace must not be empty".into());
            }

            if notion.data_source.trim().is_empty() {
                return Err("notion.data_source must not be empty".into());
            }
        }

        if let Some(navigation) = &self.navigation {
            if navigation.root.trim().is_empty() {
                return Err("navigation.root must not be empty".into());
            }

            if navigation.product.trim().is_empty() {
                return Err("navigation.product must not be empty".into());
            }
        }

        if let Some(web) = &self.web {
            if web.collection.trim().is_empty() {
                return Err("web.collection must not be empty".into());
            }

            if web.slug.trim().is_empty() {
                return Err("web.slug must not be empty".into());
            }
        }

        Ok(())
    }

    /// Returns navigation, deriving compatibility defaults for older documents.
    #[must_use]
    pub fn resolved_navigation(&self) -> Navigation {
        if let Some(navigation) = &self.navigation {
            return navigation.clone();
        }

        let product = self
            .notion
            .as_ref()
            .map(|target| target.workspace.clone())
            .unwrap_or_else(|| "general".into());

        Navigation {
            root: "knowledge".into(),
            product,
            section: None,
            order: None,
        }
    }

    /// Returns website placement, deriving compatibility defaults for older documents.
    #[must_use]
    pub fn resolved_web(&self) -> WebTarget {
        if let Some(web) = &self.web {
            return web.clone();
        }

        let navigation = self.resolved_navigation();

        WebTarget {
            collection: navigation.root.clone(),
            slug: format!(
                "/{}/{}/{}",
                navigation.root,
                navigation.product,
                self.id.replace('.', "/")
            ),
        }
    }
}
