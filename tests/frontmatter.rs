use codexa::parser::parse_source_document;

#[test]
fn parses_lureva_runbook_contract() {
    let source = include_str!("fixtures/runbooks/lureva-lightroom-handoff.md");
    let document = parse_source_document(source).expect("runbook fixture should parse");

    assert_eq!(document.metadata.schema, "codexa.document@1");
    assert_eq!(
        document.metadata.id,
        "workflows.playbooks.lureva-lightroom-handoff"
    );
    assert_eq!(document.metadata.title, "Lureva Lightroom Handoff Manual");
    assert_eq!(document.metadata.kind, "playbook");
    assert_eq!(document.metadata.status, "active");
    assert_eq!(document.metadata.visibility, "private");
    assert_eq!(
        document.metadata.description,
        "Daily Lightroom Classic handoff workflow for switching between quasar and eclipse during the Lureva 960 review."
    );
    assert_eq!(
        document.metadata.tags,
        vec!["lureva", "lightroom", "nazariya", "runbook"]
    );
    assert!(document.metadata.distribution.notion);
    assert_eq!(document.metadata.distribution.web, "private");

    let notion = document
        .metadata
        .notion
        .as_ref()
        .expect("notion target should exist");

    assert_eq!(notion.workspace, "codexa");
    assert_eq!(notion.data_source, "documents");

    assert!(document.body.contains("# Lureva Lightroom Workflow Manual"));
    assert!(document.body.contains("```bash"));
}
