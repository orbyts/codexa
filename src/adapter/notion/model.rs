use serde::{Deserialize, Serialize};

use crate::{compiler::CompiledDocument, model::DocumentLink};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionPageArtifact {
    pub schema: String,
    pub producer: String,
    pub producer_version: String,
    pub document: NotionDocument,
    pub navigation: NotionNavigation,
    pub source: NotionSource,
    pub target: NotionTarget,
    pub web: Option<NotionWeb>,
    pub links: Vec<DocumentLink>,
    pub content: NotionContent,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionNavigation {
    pub root: String,
    pub product: String,
    pub section: String,
    pub order: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionSource {
    pub repository: String,
    pub path: String,
    pub commit: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionTarget {
    pub workspace: String,
    pub root: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionWeb {
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionContent {
    pub format: String,
    pub markdown: String,
}

impl NotionPageArtifact {
    #[must_use]
    pub fn new(compiled: &CompiledDocument) -> Self {
        let metadata = &compiled.source.metadata;
        let notion = metadata
            .notion
            .as_ref()
            .expect("notion artifact requires notion target");

        Self {
            schema: "codexa.notion.page@2".into(),
            producer: "codexa".into(),
            producer_version: crate::VERSION.into(),
            document: NotionDocument {
                schema: metadata.schema.clone(),
                id: metadata.id.clone(),
                title: metadata.title.clone(),
                description: metadata.description.clone(),
                kind: metadata.kind.clone(),
                status: metadata.status.clone(),
                visibility: metadata.visibility.clone(),
                tags: metadata.tags.clone(),
            },
            navigation: NotionNavigation {
                root: metadata.navigation.root.clone(),
                product: metadata.navigation.product.clone(),
                section: metadata.navigation.section.clone(),
                order: metadata.navigation.order,
            },
            source: NotionSource {
                repository: compiled.repository.clone(),
                path: compiled.source_path.clone(),
                commit: compiled.commit.clone(),
                content_hash: compiled.content_hash.clone(),
            },
            target: NotionTarget {
                workspace: notion.workspace.clone(),
                root: metadata.navigation.root.clone(),
            },
            web: metadata.web.as_ref().map(|web| NotionWeb {
                slug: web.slug.clone(),
            }),
            links: compiled.links.clone(),
            content: NotionContent {
                format: "markdown".into(),
                markdown: compiled.source.body.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionManifest {
    pub schema: String,
    pub producer: String,
    pub producer_version: String,
    pub pages: Vec<NotionManifestPage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionManifestPage {
    pub document_id: String,
    pub root: String,
    pub path: String,
}
