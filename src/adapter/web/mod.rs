mod model;

use std::{fs, io, path::Path};

pub use model::{WebManifest, WebManifestPage, WebPageArtifact};

use crate::compiler::CompiledDocument;

pub fn write_bundle(
    documents: &[CompiledDocument],
    output_dir: &Path,
) -> Result<(), WebAdapterError> {
    let pages_dir = output_dir.join("pages");
    fs::create_dir_all(&pages_dir)?;
    let mut pages = Vec::new();

    for document in documents
        .iter()
        .filter(|document| document.source.metadata.distribution.web != "off")
    {
        let artifact = WebPageArtifact::new(document);
        let file_name = format!("{}.json", artifact.id);
        write_json(&pages_dir.join(&file_name), &artifact)?;
        pages.push(WebManifestPage {
            document_id: artifact.id.clone(),
            slug: artifact.slug.clone(),
            path: format!("pages/{file_name}"),
        });
    }

    let manifest = WebManifest {
        schema: "codexa.web.manifest@1".into(),
        producer: "codexa".into(),
        producer_version: crate::VERSION.into(),
        pages,
    };
    write_json(&output_dir.join("manifest.json"), &manifest)?;
    Ok(())
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), WebAdapterError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

#[derive(Debug)]
pub enum WebAdapterError {
    Io(io::Error),
    Json(serde_json::Error),
}
impl std::fmt::Display for WebAdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "web artifact I/O error: {e}"),
            Self::Json(e) => write!(f, "web artifact JSON error: {e}"),
        }
    }
}
impl std::error::Error for WebAdapterError {}
impl From<io::Error> for WebAdapterError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}
impl From<serde_json::Error> for WebAdapterError {
    fn from(v: serde_json::Error) -> Self {
        Self::Json(v)
    }
}
