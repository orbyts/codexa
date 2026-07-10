use std::{error::Error, fmt};

use crate::model::{DocumentMetadata, SourceDocument};

/// Parses a Markdown source document with required YAML frontmatter.
pub fn parse_source_document(source: &str) -> Result<SourceDocument, FrontmatterError> {
    let (frontmatter, body) = split_frontmatter(source)?;
    let metadata: DocumentMetadata = serde_yaml::from_str(frontmatter)?;

    metadata
        .validate_schema()
        .map_err(FrontmatterError::InvalidSchema)?;

    Ok(SourceDocument {
        metadata,
        body: body.trim_start_matches('\n').to_string(),
    })
}

fn split_frontmatter(source: &str) -> Result<(&str, &str), FrontmatterError> {
    let source = source
        .strip_prefix("---\n")
        .ok_or(FrontmatterError::Missing)?;

    let end = source.find("\n---").ok_or(FrontmatterError::Unclosed)?;
    let frontmatter = &source[..end];

    let after_marker = &source[end + "\n---".len()..];
    let body = after_marker
        .strip_prefix("\r\n")
        .or_else(|| after_marker.strip_prefix('\n'))
        .unwrap_or(after_marker);

    Ok((frontmatter, body))
}

/// Frontmatter parsing errors.
#[derive(Debug)]
pub enum FrontmatterError {
    Missing,
    Unclosed,
    Yaml(serde_yaml::Error),
    InvalidSchema(String),
}

impl fmt::Display for FrontmatterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => write!(formatter, "missing YAML frontmatter"),
            Self::Unclosed => write!(formatter, "unclosed YAML frontmatter"),
            Self::Yaml(error) => write!(formatter, "frontmatter YAML error: {error}"),
            Self::InvalidSchema(message) => write!(formatter, "{message}"),
        }
    }
}

impl Error for FrontmatterError {}

impl From<serde_yaml::Error> for FrontmatterError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::Yaml(error)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_source_document;

    #[test]
    fn parses_frontmatter_and_body() {
        let source = r#"---
schema: codexa.document@1
id: notes.example
title: Example
description: One line.
kind: note
status: active
visibility: private
tags:
  - test
navigation:
  root: docs
  product: codexa
  section: Guides
  order: 10
distribution:
  notion: true
  web: private
notion:
  workspace: codexa
  data_source: documents
web:
  collection: docs
  slug: /docs/codexa/guides/example
---

# Example

Body text.
"#;

        let document = parse_source_document(source).expect("source document should parse");

        assert_eq!(document.metadata.schema, "codexa.document@1");
        assert_eq!(document.metadata.id, "notes.example");
        assert_eq!(document.metadata.title, "Example");
        assert_eq!(document.metadata.description, "One line.");
        assert_eq!(document.metadata.tags, vec!["test"]);

        let navigation = document.metadata.navigation.as_ref().unwrap();
        assert_eq!(navigation.root, "docs");
        assert_eq!(navigation.product, "codexa");

        let web = document.metadata.web.as_ref().unwrap();
        assert_eq!(web.collection, "docs");
        assert_eq!(web.slug, "/docs/codexa/guides/example");

        assert!(document.body.starts_with("# Example"));
    }
}
