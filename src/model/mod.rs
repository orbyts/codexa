mod block;
mod document;
mod link;
mod metadata;

pub use block::{Block, Paragraph};
pub use document::Document;
pub use link::DocumentLink;
pub use metadata::{
    Distribution, DocumentMetadata, Navigation, NotionTarget, SourceDocument, WebTarget,
};
