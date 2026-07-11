use codexa::parser::parse_source_document;

#[test]
fn parses_lureva_runbook_contract() {
    let source = include_str!("fixtures/runbooks/lureva-lightroom-handoff.fixture");
    let document = parse_source_document(source).expect("runbook fixture should parse");

    assert_eq!(document.metadata.schema, "codexa.document@2");
    assert_eq!(document.metadata.id, "lureva.playbooks.lightroom-handoff");
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

    let navigation = &document.metadata.navigation;
    assert_eq!(navigation.root, "knowledge");
    assert_eq!(navigation.product, "lureva");
    assert_eq!(navigation.section, "Playbooks");
    assert_eq!(navigation.order, 10);

    let web = document
        .metadata
        .web
        .as_ref()
        .expect("web target should exist");
    assert_eq!(web.slug, "/knowledge/lureva/playbooks/lightroom-handoff");

    let notion = document
        .metadata
        .notion
        .as_ref()
        .expect("notion target should exist");

    assert_eq!(notion.workspace, "codexa");

    assert!(document.body.contains("# Lureva Lightroom Workflow Manual"));
    assert!(document.body.contains("```bash"));
}
