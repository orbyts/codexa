use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub schema: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub kind: String,
    pub status: String,
    pub visibility: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub navigation: Navigation,
    pub distribution: Distribution,
    pub notion: Option<NotionTarget>,
    pub web: Option<WebTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Navigation {
    pub root: String,
    pub product: String,
    pub section: String,
    pub order: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Distribution {
    pub notion: bool,
    pub web: String,
    pub obsidian: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionTarget {
    pub workspace: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebTarget {
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDocument {
    pub metadata: DocumentMetadata,
    pub body: String,
}

impl DocumentMetadata {
    pub fn validate_schema(&self) -> Result<(), String> {
        if self.schema != "codexa.document@2" {
            return Err(format!("unsupported document schema `{}`", self.schema));
        }

        for (name, value) in [
            ("id", self.id.as_str()),
            ("title", self.title.as_str()),
            ("description", self.description.as_str()),
            ("kind", self.kind.as_str()),
            ("status", self.status.as_str()),
            ("visibility", self.visibility.as_str()),
            ("navigation.root", self.navigation.root.as_str()),
            ("navigation.product", self.navigation.product.as_str()),
            ("navigation.section", self.navigation.section.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(format!("{name} must not be empty"));
            }
        }

        if self.distribution.notion {
            let notion = self.notion.as_ref().ok_or_else(|| {
                "notion target is required when distribution.notion is true".to_string()
            })?;
            if notion.workspace.trim().is_empty() {
                return Err("notion.workspace must not be empty".into());
            }
        }

        if self.distribution.web != "off" {
            let web = self.web.as_ref().ok_or_else(|| {
                "web target is required when distribution.web is not `off`".to_string()
            })?;
            if !web.slug.starts_with('/') {
                return Err("web.slug must start with `/`".into());
            }
        }

        Ok(())
    }
}
