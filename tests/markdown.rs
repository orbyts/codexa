use codexa::{
    model::{Block, Paragraph},
    parser::{ParseErrorCode, parse_markdown},
};

#[test]
fn parses_the_paragraph_fixture() {
    let markdown = include_str!("fixtures/paragraph.md");
    let document = parse_markdown(markdown).expect("fixture should parse");

    assert_eq!(
        document.blocks,
        vec![
            Block::Paragraph(Paragraph {
                id: "paragraph-1".into(),
                text: "This is the first paragraph rendered by Codexa.".into(),
            }),
            Block::Paragraph(Paragraph {
                id: "paragraph-2".into(),
                text: "This is another paragraph so that Codexa can verify source ordering.".into(),
            }),
        ]
    );
}

#[test]
fn rejects_an_unsupported_heading() {
    let error = parse_markdown("# Heading").expect_err("heading should fail");

    assert_eq!(error.code(), ParseErrorCode::UnsupportedBlock);
    assert_eq!(error.code().as_str(), "codexa::unsupported-block");
}
