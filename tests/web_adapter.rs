use std::fs;

use codexa::{adapter::web::write_artifact, parser::parse_markdown};

#[test]
fn writes_deterministic_web_json() {
    let temp = tempfile::tempdir().expect("temporary directory should be created");
    let document = parse_markdown("Hello from the web adapter.").expect("paragraph should parse");

    write_artifact(document, "sample.md", temp.path()).expect("artifact should be written");

    let manifest =
        fs::read_to_string(temp.path().join("manifest.json")).expect("manifest should exist");
    let document = fs::read_to_string(temp.path().join("documents/sample.json"))
        .expect("document should exist");

    assert!(manifest.contains("codexa.web/manifest@1"));
    assert!(manifest.contains("documents/sample.json"));
    assert!(document.contains("codexa.web/document@1"));
    assert!(document.contains("Hello from the web adapter."));
    assert!(manifest.ends_with('\n'));
    assert!(document.ends_with('\n'));
}
