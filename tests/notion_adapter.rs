use std::{fs, path::Path};

#[test]
fn writes_notion_artifact_for_runbook() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let input = Path::new("tests/fixtures/runbooks/lureva-lightroom-handoff.md");

    let markdown = fs::read_to_string(input).expect("fixture should read");

    codexa::adapter::notion::write_artifact(
        &markdown,
        "archivora/knowledge",
        "workflows/playbooks/lureva-lightroom-handoff.md",
        temp.path(),
    )
    .expect("notion artifact should write");

    let manifest =
        fs::read_to_string(temp.path().join("manifest.json")).expect("manifest should exist");

    assert!(manifest.contains("codexa.notion.manifest@1"));
    assert!(manifest.contains("workflows.playbooks.lureva-lightroom-handoff"));

    let page = fs::read_to_string(
        temp.path()
            .join("pages/workflows.playbooks.lureva-lightroom-handoff.json"),
    )
    .expect("page artifact should exist");

    assert!(page.contains("codexa.notion.page@1"));
    assert!(page.contains("Lureva Lightroom Handoff Manual"));
    assert!(page.contains("\"description\""));
    assert!(page.contains("\"data_source\": \"documents\""));
    assert!(page.contains("# Lureva Lightroom Workflow Manual"));
}
