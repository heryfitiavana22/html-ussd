use std::fmt;

use crate::html::{Tag, TagElement};

pub trait TagAdapter {
    fn transform(&self, content: &str) -> Result<Vec<TagElement>, AdapterError>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdapterError {
    TagNotFound(Tag),
    UnexpectedTag(String),
    ParsingError(String),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdapterError::TagNotFound(tag) => write!(f, "Tag not found: {:?}", tag),
            AdapterError::UnexpectedTag(tag) => write!(f, "Unexpected tag encountered: '{}'", tag),
            AdapterError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}
