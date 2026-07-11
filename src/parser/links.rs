use crate::model::DocumentLink;

pub fn parse_document_links(markdown: &str) -> Result<Vec<DocumentLink>, LinkError> {
    let mut links = Vec::new();
    let bytes = markdown.as_bytes();
    let mut cursor = 0;

    while cursor + 1 < bytes.len() {
        if bytes[cursor] == b'[' && bytes[cursor + 1] == b'[' {
            let start = cursor;
            cursor += 2;
            let rest = &markdown[cursor..];
            let Some(relative_end) = rest.find("]]") else {
                return Err(LinkError::Unclosed(start));
            };
            let end = cursor + relative_end;
            let inner = &markdown[cursor..end];
            let raw = markdown[start..end + 2].to_string();
            links.push(parse_inner(inner, raw)?);
            cursor = end + 2;
        } else {
            cursor += 1;
        }
    }

    Ok(links)
}

fn parse_inner(inner: &str, raw: String) -> Result<DocumentLink, LinkError> {
    let (target_and_heading, label) = match inner.split_once('|') {
        Some((left, right)) => (left.trim(), Some(right.trim().to_string())),
        None => (inner.trim(), None),
    };

    let (target_id, heading) = match target_and_heading.split_once('#') {
        Some((target, heading)) => (target.trim(), Some(heading.trim().to_string())),
        None => (target_and_heading, None),
    };

    if target_id.is_empty() {
        return Err(LinkError::EmptyTarget(raw));
    }

    Ok(DocumentLink {
        raw,
        target_id: target_id.to_string(),
        heading: heading.filter(|value| !value.is_empty()),
        label: label.filter(|value| !value.is_empty()),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkError {
    Unclosed(usize),
    EmptyTarget(String),
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unclosed(offset) => write!(f, "unclosed logical link at byte {offset}"),
            Self::EmptyTarget(raw) => write!(f, "logical link has an empty target: {raw}"),
        }
    }
}

impl std::error::Error for LinkError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_link_shapes() {
        let links =
            parse_document_links("[[a.b]] [[a.b|Label]] [[a.b#Heading]] [[a.b#Heading|Label]]")
                .unwrap();
        assert_eq!(links.len(), 4);
        assert_eq!(links[3].target_id, "a.b");
        assert_eq!(links[3].heading.as_deref(), Some("Heading"));
        assert_eq!(links[3].label.as_deref(), Some("Label"));
    }
}
