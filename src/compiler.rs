use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::{
    model::{DocumentLink, SourceDocument},
    parser::{parse_document_links, parse_source_document},
};

#[derive(Debug, Clone)]
pub struct SourceRoot {
    pub path: PathBuf,
    pub repository: String,
}

#[derive(Debug, Clone)]
pub struct CompiledDocument {
    pub source: SourceDocument,
    pub repository: String,
    pub source_root: PathBuf,
    pub source_path: String,
    pub commit: String,
    pub content_hash: String,
    pub links: Vec<DocumentLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub schema: String,
    pub producer: String,
    pub producer_version: String,
    pub documents: BTreeMap<String, BundleDocument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleDocument {
    pub title: String,
    pub source_repository: String,
    pub source_path: String,
    pub root: String,
    pub product: String,
    pub section: String,
    pub order: i64,
    pub web_slug: Option<String>,
    pub links: Vec<DocumentLink>,
}

pub fn compile_roots(roots: &[SourceRoot]) -> Result<Vec<CompiledDocument>, CompileError> {
    let mut documents = Vec::new();
    let mut ids = BTreeSet::new();

    for root in roots {
        let commit = git_commit(&root.path).unwrap_or_else(|| "unknown".into());
        for entry in WalkDir::new(&root.path)
            .into_iter()
            .filter_entry(|entry| !is_ignored(entry.path()))
        {
            let entry = entry.map_err(CompileError::Walk)?;
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|v| v.to_str()) != Some("md")
            {
                continue;
            }

            let markdown = fs::read_to_string(entry.path())?;
            if !markdown.starts_with("---\n") {
                continue;
            }

            let source = parse_source_document(&markdown)?;
            if !ids.insert(source.metadata.id.clone()) {
                return Err(CompileError::DuplicateId(source.metadata.id));
            }
            let links = parse_document_links(&source.body)?;
            let relative = entry
                .path()
                .strip_prefix(&root.path)
                .map_err(|_| CompileError::Path(entry.path().display().to_string()))?;

            documents.push(CompiledDocument {
                source,
                repository: root.repository.clone(),
                source_root: root.path.clone(),
                source_path: relative.to_string_lossy().replace('\\', "/"),
                commit: commit.clone(),
                content_hash: sha256(&markdown),
                links,
            });
        }
    }

    for document in &documents {
        for link in &document.links {
            if !ids.contains(&link.target_id) {
                return Err(CompileError::MissingLink {
                    source_id: document.source.metadata.id.clone(),
                    target_id: link.target_id.clone(),
                });
            }
        }
    }

    documents.sort_by(|a, b| a.source.metadata.id.cmp(&b.source.metadata.id));
    Ok(documents)
}

pub fn bundle(documents: &[CompiledDocument]) -> Bundle {
    let documents = documents
        .iter()
        .map(|compiled| {
            let metadata = &compiled.source.metadata;
            (
                metadata.id.clone(),
                BundleDocument {
                    title: metadata.title.clone(),
                    source_repository: compiled.repository.clone(),
                    source_path: compiled.source_path.clone(),
                    root: metadata.navigation.root.clone(),
                    product: metadata.navigation.product.clone(),
                    section: metadata.navigation.section.clone(),
                    order: metadata.navigation.order,
                    web_slug: metadata.web.as_ref().map(|web| web.slug.clone()),
                    links: compiled.links.clone(),
                },
            )
        })
        .collect();

    Bundle {
        schema: "codexa.bundle@1".into(),
        producer: "codexa".into(),
        producer_version: crate::VERSION.into(),
        documents,
    }
}

fn is_ignored(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some(".git" | "target" | "dist" | "node_modules")
        )
    })
}

fn git_commit(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn sha256(source: &str) -> String {
    let digest = Sha256::digest(source.as_bytes());
    format!("sha256:{digest:x}")
}

#[derive(Debug)]
pub enum CompileError {
    Io(io::Error),
    Walk(walkdir::Error),
    Frontmatter(crate::parser::FrontmatterError),
    Link(crate::parser::LinkError),
    DuplicateId(String),
    MissingLink {
        source_id: String,
        target_id: String,
    },
    Path(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Walk(error) => write!(f, "repository walk error: {error}"),
            Self::Frontmatter(error) => write!(f, "frontmatter error: {error}"),
            Self::Link(error) => write!(f, "link error: {error}"),
            Self::DuplicateId(id) => write!(f, "duplicate document id `{id}`"),
            Self::MissingLink {
                source_id,
                target_id,
            } => write!(
                f,
                "document `{source_id}` links to missing document `{target_id}`"
            ),
            Self::Path(path) => write!(f, "source path is outside source root: {path}"),
        }
    }
}

impl std::error::Error for CompileError {}
impl From<io::Error> for CompileError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}
impl From<walkdir::Error> for CompileError {
    fn from(v: walkdir::Error) -> Self {
        Self::Walk(v)
    }
}
impl From<crate::parser::FrontmatterError> for CompileError {
    fn from(v: crate::parser::FrontmatterError) -> Self {
        Self::Frontmatter(v)
    }
}
impl From<crate::parser::LinkError> for CompileError {
    fn from(v: crate::parser::LinkError) -> Self {
        Self::Link(v)
    }
}
