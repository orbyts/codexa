use std::{fs, path::PathBuf};

use codexa::compiler::{SourceRoot, compile_roots};

#[test]
fn writes_notion_bundle_for_runbook() {
    let temp = tempfile::tempdir().expect("tempdir should be created");

    let repository = temp.path().join("repository");
    let source_dir = repository.join("workflows/playbooks");
    let output = temp.path().join("notion");

    fs::create_dir_all(&source_dir).expect("fixture source directory should be created");

    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/runbooks/lureva-lightroom-handoff.fixture");

    fs::copy(fixture, source_dir.join("lureva-lightroom-handoff.md"))
        .expect("fixture should be copied as Markdown");

    let documents = compile_roots(&[SourceRoot {
        path: repository,
        repository: "archivora/knowledge".into(),
    }])
    .expect("fixture repository should compile");

    codexa::adapter::notion::write_bundle(&documents, &output).expect("notion bundle should write");

    let manifest =
        fs::read_to_string(output.join("manifest.json")).expect("manifest should be readable");

    assert!(manifest.contains("codexa.notion.manifest@2"));
    assert!(manifest.contains("lureva.playbooks.lightroom-handoff"));

    let page = fs::read_to_string(
        output
            .join("pages")
            .join("lureva.playbooks.lightroom-handoff.json"),
    )
    .expect("page artifact should be readable");

    assert!(page.contains("codexa.notion.page@2"));
    assert!(page.contains("\"root\": \"knowledge\""));
    assert!(page.contains("\"workspace\": \"codexa\""));
    assert!(!page.contains("data_source"));
}
