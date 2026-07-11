use std::{fs, path::PathBuf};

use codexa::compiler::{CompileError, SourceRoot, compile_roots};

fn document(id: &str, body: &str) -> String {
    format!(
        r#"---
schema: codexa.document@2
id: {id}
title: {id}
description: Test document.
kind: guide
status: active
visibility: public
tags: [test]
navigation:
  root: docs
  product: test
  section: Guides
  order: 10
distribution:
  notion: true
  web: public
  obsidian: true
notion:
  workspace: codexa
web:
  slug: /docs/{id}
---
{body}
"#
    )
}

#[test]
fn validates_cross_document_links() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(
        temp.path().join("a.md"),
        document("a", "See [[b|Document B]]."),
    )
    .unwrap();
    fs::write(temp.path().join("b.md"), document("b", "# B")).unwrap();
    let docs = compile_roots(&[SourceRoot {
        path: PathBuf::from(temp.path()),
        repository: "test/repo".into(),
    }])
    .unwrap();
    assert_eq!(docs[0].links[0].target_id, "b");
}

#[test]
fn rejects_missing_link_targets() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("a.md"), document("a", "See [[missing]].")).unwrap();
    let error = compile_roots(&[SourceRoot {
        path: PathBuf::from(temp.path()),
        repository: "test/repo".into(),
    }])
    .unwrap_err();
    assert!(matches!(error, CompileError::MissingLink { .. }));
}
