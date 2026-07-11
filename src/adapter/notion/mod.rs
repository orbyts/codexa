mod model;

use std::{fs, io, path::Path};

pub use model::{NotionManifest, NotionManifestPage, NotionPageArtifact};

use crate::compiler::CompiledDocument;

pub fn write_bundle(
    documents: &[CompiledDocument],
    output_dir: &Path,
) -> Result<(), NotionAdapterError> {
    let pages_dir = output_dir.join("pages");
    fs::create_dir_all(&pages_dir)?;
    let mut pages = Vec::new();

    for document in documents
        .iter()
        .filter(|document| document.source.metadata.distribution.notion)
    {
        let id = &document.source.metadata.id;
        let file_name = format!("{id}.json");
        write_json(
            &pages_dir.join(&file_name),
            &NotionPageArtifact::new(document),
        )?;
        pages.push(NotionManifestPage {
            document_id: id.clone(),
            root: document.source.metadata.navigation.root.clone(),
            path: format!("pages/{file_name}"),
        });
    }

    let manifest = NotionManifest {
        schema: "codexa.notion.manifest@2".into(),
        producer: "codexa".into(),
        producer_version: crate::VERSION.into(),
        pages,
    };
    write_json(&output_dir.join("manifest.json"), &manifest)?;
    Ok(())
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), NotionAdapterError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

#[derive(Debug)]
pub enum NotionAdapterError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for NotionAdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "notion artifact I/O error: {error}"),
            Self::Json(error) => write!(f, "notion artifact JSON error: {error}"),
        }
    }
}
impl std::error::Error for NotionAdapterError {}
impl From<io::Error> for NotionAdapterError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}
impl From<serde_json::Error> for NotionAdapterError {
    fn from(v: serde_json::Error) -> Self {
        Self::Json(v)
    }
}
