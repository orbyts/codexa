use serde::{Deserialize, Serialize};

/// A typed block in a Codexa document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    /// A paragraph containing plain text.
    Paragraph(Paragraph),
}

/// A plain-text paragraph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paragraph {
    /// Stable block identifier within the source document.
    pub id: String,
    /// Plain paragraph text.
    pub text: String,
}
