mod model;

use std::{fs, io, path::Path};

pub use model::{WebDocumentArtifact, WebManifest};

use crate::model::Document;

/// Writes the versioned JSON web artifact for one document.
pub fn write_artifact(
    document: Document,
    source_name: &str,
    output_dir: &Path,
) -> Result<(), WebAdapterError> {
    let documents_dir = output_dir.join("documents");
    fs::create_dir_all(&documents_dir)?;

    let artifact = WebDocumentArtifact::new(document);
    write_json(&documents_dir.join("sample.json"), &artifact)?;

    let manifest = WebManifest::new(source_name);
    write_json(&output_dir.join("manifest.json"), &manifest)?;

    Ok(())
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), WebAdapterError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

/// Error emitted by the JSON web adapter.
#[derive(Debug)]
pub enum WebAdapterError {
    /// Filesystem operation failed.
    Io(io::Error),
    /// JSON serialization failed.
    Json(serde_json::Error),
}

impl std::fmt::Display for WebAdapterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "web adapter I/O error: {error}"),
            Self::Json(error) => write!(formatter, "web adapter JSON error: {error}"),
        }
    }
}

impl std::error::Error for WebAdapterError {}

impl From<io::Error> for WebAdapterError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for WebAdapterError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
