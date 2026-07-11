use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentLink {
    pub raw: String,
    pub target_id: String,
    pub heading: Option<String>,
    pub label: Option<String>,
}
