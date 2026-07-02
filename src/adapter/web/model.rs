use serde::{Deserialize, Serialize};

use crate::model::Document;

/// Versioned JSON representation consumed by a web frontend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebDocumentArtifact {
    /// Serialized artifact schema.
    pub schema: String,
    /// Target-neutral Codexa document.
    pub document: Document,
}

impl WebDocumentArtifact {
    /// Creates a version 1 web document artifact.
    #[must_use]
    pub fn new(document: Document) -> Self {
        Self {
            schema: "codexa.web/document@1".into(),
            document,
        }
    }
}

/// Manifest describing the generated web artifact bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebManifest {
    /// Serialized manifest schema.
    pub schema: String,
    /// Generator name.
    pub generator: String,
    /// Generator package version.
    pub generator_version: String,
    /// Relative document entry point.
    pub entrypoint: String,
    /// Original source filename.
    pub source: String,
}

impl WebManifest {
    /// Creates a manifest for the initial single-document bundle.
    #[must_use]
    pub fn new(source: &str) -> Self {
        Self {
            schema: "codexa.web/manifest@1".into(),
            generator: "codexa".into(),
            generator_version: crate::VERSION.into(),
            entrypoint: "documents/sample.json".into(),
            source: source.into(),
        }
    }
}
