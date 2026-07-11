use std::{fs, process::Command};

#[test]
fn prints_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_codexa"))
        .arg("--version")
        .output()
        .expect("codexa binary should run");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("codexa {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn validates_and_builds_repository_bundle() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path().join("repo");
    let docs = repo.join("docs");
    fs::create_dir_all(&docs).unwrap();
    fs::write(
        docs.join("index.md"),
        r#"---
schema: codexa.document@2
id: sample.index
title: Sample
description: Sample document.
kind: guide
status: active
visibility: public
tags: [sample]
navigation:
  root: docs
  product: sample
  section: Guides
  order: 10
distribution:
  notion: true
  web: public
  obsidian: true
notion:
  workspace: codexa
web:
  slug: /docs/sample
---
# Sample
"#,
    )
    .unwrap();
    let output_dir = temp.path().join("dist");

    let output = Command::new(env!("CARGO_BIN_EXE_codexa"))
        .args([
            "build",
            "--source-root",
            repo.to_str().unwrap(),
            "example/sample",
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_dir.join("bundle.json").is_file());
    assert!(output_dir.join("notion/manifest.json").is_file());
    assert!(output_dir.join("web/manifest.json").is_file());
}
