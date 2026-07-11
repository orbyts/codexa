use std::path::PathBuf;

use codexa::compiler::{SourceRoot, compile_roots};

#[test]
fn writes_notion_bundle_for_runbook() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let root = PathBuf::from("tests/fixtures");
    let documents = compile_roots(&[SourceRoot {
        path: root,
        repository: "archivora/knowledge".into(),
    }])
    .expect("fixture repository should compile");

    codexa::adapter::notion::write_bundle(&documents, temp.path())
        .expect("notion bundle should write");

    let manifest = std::fs::read_to_string(temp.path().join("manifest.json")).unwrap();
    assert!(manifest.contains("codexa.notion.manifest@2"));
    assert!(manifest.contains("lureva.playbooks.lightroom-handoff"));

    let page = std::fs::read_to_string(
        temp.path()
            .join("pages/lureva.playbooks.lightroom-handoff.json"),
    )
    .unwrap();
    assert!(page.contains("codexa.notion.page@2"));
    assert!(page.contains("\"root\": \"knowledge\""));
    assert!(page.contains("\"workspace\": \"codexa\""));
    assert!(!page.contains("data_source"));
}
