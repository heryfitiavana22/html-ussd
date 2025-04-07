use crate::html::{Tag, TagElement};

pub trait TagAdapter {
    fn transform(&self, content: &str) -> Result<Vec<TagElement>, AdapterError>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdapterError {
    TagNotFound(Tag),
    UnexcepetedTag(String),
    ParsingError(String),
}
