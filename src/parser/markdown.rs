use std::{error::Error, fmt};

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::model::{Block, Document, Paragraph};

/// Stable diagnostic code for Markdown parsing failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorCode {
    /// The Markdown contains a block Codexa does not support yet.
    UnsupportedBlock,
    /// The Markdown contains inline syntax Codexa does not support yet.
    UnsupportedInline,
    /// The Markdown event stream is structurally invalid.
    InvalidStructure,
}

impl ParseErrorCode {
    /// Returns the stable external diagnostic code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedBlock => "codexa::unsupported-block",
            Self::UnsupportedInline => "codexa::unsupported-inline",
            Self::InvalidStructure => "codexa::invalid-markdown-structure",
        }
    }
}

/// Error returned when Markdown cannot be represented by the current model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    code: ParseErrorCode,
    message: String,
}

impl ParseError {
    fn new(code: ParseErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> ParseErrorCode {
        self.code
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code.as_str(), self.message)
    }
}

impl Error for ParseError {}

/// Parses paragraph-only Markdown into Codexa's target-neutral model.
///
/// Version 0.0.2 intentionally accepts only plain-text paragraphs. Other
/// Markdown constructs return stable diagnostics rather than being flattened.
pub fn parse_markdown(source: &str) -> Result<Document, ParseError> {
    let mut blocks = Vec::new();
    let mut paragraph_text: Option<String> = None;

    for event in Parser::new(source) {
        match event {
            Event::Start(Tag::Paragraph) => {
                if paragraph_text.replace(String::new()).is_some() {
                    return Err(ParseError::new(
                        ParseErrorCode::InvalidStructure,
                        "encountered a nested paragraph",
                    ));
                }
            }
            Event::Text(text) => {
                let paragraph = paragraph_text.as_mut().ok_or_else(|| {
                    ParseError::new(
                        ParseErrorCode::InvalidStructure,
                        "encountered text outside a paragraph",
                    )
                })?;
                paragraph.push_str(&text);
            }
            Event::SoftBreak => {
                let paragraph = paragraph_text.as_mut().ok_or_else(|| {
                    ParseError::new(
                        ParseErrorCode::InvalidStructure,
                        "encountered a soft break outside a paragraph",
                    )
                })?;
                paragraph.push('\n');
            }
            Event::End(TagEnd::Paragraph) => {
                let text = paragraph_text.take().ok_or_else(|| {
                    ParseError::new(
                        ParseErrorCode::InvalidStructure,
                        "encountered a paragraph end without a start",
                    )
                })?;
                let id = format!("paragraph-{}", blocks.len() + 1);
                blocks.push(Block::Paragraph(Paragraph { id, text }));
            }
            Event::Start(tag) => {
                return Err(ParseError::new(
                    ParseErrorCode::UnsupportedBlock,
                    format!("unsupported Markdown block `{}`", tag_name(&tag)),
                ));
            }
            Event::End(tag) => {
                return Err(ParseError::new(
                    ParseErrorCode::InvalidStructure,
                    format!("unexpected Markdown block end `{tag:?}`"),
                ));
            }
            Event::Code(_)
            | Event::Html(_)
            | Event::InlineHtml(_)
            | Event::FootnoteReference(_)
            | Event::TaskListMarker(_)
            | Event::InlineMath(_)
            | Event::DisplayMath(_)
            | Event::HardBreak
            | Event::Rule => {
                return Err(ParseError::new(
                    ParseErrorCode::UnsupportedInline,
                    "unsupported inline Markdown syntax",
                ));
            }
        }
    }

    if paragraph_text.is_some() {
        return Err(ParseError::new(
            ParseErrorCode::InvalidStructure,
            "unterminated paragraph",
        ));
    }

    Ok(Document { blocks })
}

fn tag_name(tag: &Tag<'_>) -> &'static str {
    match tag {
        Tag::Paragraph => "paragraph",
        Tag::Heading { .. } => "heading",
        Tag::BlockQuote(_) => "block_quote",
        Tag::CodeBlock(_) => "code_block",
        Tag::HtmlBlock => "html_block",
        Tag::List(_) => "list",
        Tag::Item => "list_item",
        Tag::FootnoteDefinition(_) => "footnote_definition",
        Tag::DefinitionList => "definition_list",
        Tag::DefinitionListTitle => "definition_list_title",
        Tag::DefinitionListDefinition => "definition_list_definition",
        Tag::Table(_) => "table",
        Tag::TableHead => "table_head",
        Tag::TableRow => "table_row",
        Tag::TableCell => "table_cell",
        Tag::Emphasis => "emphasis",
        Tag::Strong => "strong",
        Tag::Strikethrough => "strikethrough",
        Tag::Superscript => "superscript",
        Tag::Subscript => "subscript",
        Tag::Link { .. } => "link",
        Tag::Image { .. } => "image",
        Tag::MetadataBlock(_) => "metadata_block",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_one_paragraph() {
        let document = parse_markdown("Hello from Codexa.").expect("paragraph should parse");

        assert_eq!(
            document,
            Document {
                blocks: vec![Block::Paragraph(Paragraph {
                    id: "paragraph-1".into(),
                    text: "Hello from Codexa.".into(),
                })],
            }
        );
    }

    #[test]
    fn preserves_paragraph_order() {
        let document = parse_markdown("First paragraph.\n\nSecond paragraph.")
            .expect("paragraphs should parse");

        assert_eq!(document.blocks.len(), 2);
        assert_eq!(
            document.blocks[0],
            Block::Paragraph(Paragraph {
                id: "paragraph-1".into(),
                text: "First paragraph.".into(),
            })
        );
        assert_eq!(
            document.blocks[1],
            Block::Paragraph(Paragraph {
                id: "paragraph-2".into(),
                text: "Second paragraph.".into(),
            })
        );
    }

    #[test]
    fn rejects_headings_with_stable_code() {
        let error = parse_markdown("# Unsupported").expect_err("heading should fail");

        assert_eq!(error.code(), ParseErrorCode::UnsupportedBlock);
        assert!(error.to_string().contains("heading"));
    }
}
