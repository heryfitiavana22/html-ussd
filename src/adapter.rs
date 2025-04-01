use crate::html::{Tag, TagElement};

pub trait TagAdapter {
    fn transform(&self) -> Result<Vec<TagElement>, AdapterError>;
}

pub enum AdapterError {
    TagNotFound(Tag),
    UnexcepetedTag(String),
}
