use serde::{Deserialize, Serialize};

use crate::{compiler::CompiledDocument, model::DocumentLink};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebPageArtifact {
    pub schema: String,
    pub producer: String,
    pub producer_version: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub kind: String,
    pub status: String,
    pub visibility: String,
    pub tags: Vec<String>,
    pub root: String,
    pub product: String,
    pub section: String,
    pub order: i64,
    pub slug: String,
    pub links: Vec<DocumentLink>,
    pub markdown: String,
    pub source_repository: String,
    pub source_path: String,
    pub source_commit: String,
    pub content_hash: String,
}

impl WebPageArtifact {
    pub fn new(document: &CompiledDocument) -> Self {
        let metadata = &document.source.metadata;
        let web = metadata
            .web
            .as_ref()
            .expect("web artifact requires web target");
        Self {
            schema: "codexa.web.page@1".into(),
            producer: "codexa".into(),
            producer_version: crate::VERSION.into(),
            id: metadata.id.clone(),
            title: metadata.title.clone(),
            description: metadata.description.clone(),
            kind: metadata.kind.clone(),
            status: metadata.status.clone(),
            visibility: metadata.visibility.clone(),
            tags: metadata.tags.clone(),
            root: metadata.navigation.root.clone(),
            product: metadata.navigation.product.clone(),
            section: metadata.navigation.section.clone(),
            order: metadata.navigation.order,
            slug: web.slug.clone(),
            links: document.links.clone(),
            markdown: document.source.body.clone(),
            source_repository: document.repository.clone(),
            source_path: document.source_path.clone(),
            source_commit: document.commit.clone(),
            content_hash: document.content_hash.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebManifest {
    pub schema: String,
    pub producer: String,
    pub producer_version: String,
    pub pages: Vec<WebManifestPage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebManifestPage {
    pub document_id: String,
    pub slug: String,
    pub path: String,
}
