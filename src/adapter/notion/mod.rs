mod model;

use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use sha2::{Digest, Sha256};

pub use model::{NotionManifest, NotionPageArtifact};

use crate::parser::parse_source_document;

/// Writes a Notion artifact bundle for one Markdown source document.
pub fn write_artifact(
    markdown: &str,
    repository: &str,
    source_path: &str,
    output_dir: &Path,
) -> Result<(), NotionAdapterError> {
    let source_document = parse_source_document(markdown)?;

    if !source_document.metadata.distribution.notion {
        return Err(NotionAdapterError::NotionDisabled(
            source_document.metadata.id,
        ));
    }

    let notion = source_document.metadata.notion.as_ref().ok_or_else(|| {
        NotionAdapterError::MissingNotionTarget(source_document.metadata.id.clone())
    })?;

    if notion.workspace.trim().is_empty() || notion.data_source.trim().is_empty() {
        return Err(NotionAdapterError::MissingNotionTarget(
            source_document.metadata.id.clone(),
        ));
    }

    let document_id = source_document.metadata.id.clone();
    let commit = git_commit_for_path(source_path).unwrap_or_else(|| "unknown".into());
    let content_hash = sha256(markdown);

    let artifact = NotionPageArtifact::new(
        source_document,
        repository,
        source_path,
        commit,
        content_hash,
    );

    let pages_dir = output_dir.join("pages");
    fs::create_dir_all(&pages_dir)?;

    let page_file_name = format!("{document_id}.json");
    let page_path = pages_dir.join(&page_file_name);

    write_json(&page_path, &artifact)?;

    let manifest = NotionManifest::single(document_id, format!("pages/{page_file_name}"));
    write_json(&output_dir.join("manifest.json"), &manifest)?;

    Ok(())
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), NotionAdapterError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

fn sha256(source: &str) -> String {
    let digest = Sha256::digest(source.as_bytes());
    format!("sha256:{digest:x}")
}

fn git_commit_for_path(source_path: &str) -> Option<String> {
    let path = PathBuf::from(source_path);
    let dir = path.parent().unwrap_or_else(|| Path::new("."));

    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Error emitted by the Notion artifact adapter.
#[derive(Debug)]
pub enum NotionAdapterError {
    Io(io::Error),
    Json(serde_json::Error),
    Frontmatter(crate::parser::FrontmatterError),
    NotionDisabled(String),
    MissingNotionTarget(String),
}

impl std::fmt::Display for NotionAdapterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "notion adapter I/O error: {error}"),
            Self::Json(error) => write!(formatter, "notion adapter JSON error: {error}"),
            Self::Frontmatter(error) => {
                write!(formatter, "notion adapter frontmatter error: {error}")
            }
            Self::NotionDisabled(document_id) => {
                write!(
                    formatter,
                    "document `{document_id}` is not enabled for Notion"
                )
            }
            Self::MissingNotionTarget(document_id) => {
                write!(
                    formatter,
                    "document `{document_id}` is missing notion target metadata"
                )
            }
        }
    }
}

impl std::error::Error for NotionAdapterError {}

impl From<io::Error> for NotionAdapterError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for NotionAdapterError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<crate::parser::FrontmatterError> for NotionAdapterError {
    fn from(error: crate::parser::FrontmatterError) -> Self {
        Self::Frontmatter(error)
    }
}
