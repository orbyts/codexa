use serde::{Deserialize, Serialize};

use super::Block;

/// Target-neutral representation of a parsed Codexa document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    /// Ordered document blocks.
    pub blocks: Vec<Block>,
}
