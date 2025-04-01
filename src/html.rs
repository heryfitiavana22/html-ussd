use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Html,
    Head,
    Title,
    Body,
    P,
    Text(String),
    Link,
    Form,
    Input,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TagElement {
    pub tag_name: Tag,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<TagElement>,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Html => write!(f, "html"),
            Tag::Head => write!(f, "head"),
            Tag::Title => write!(f, "title"),
            Tag::Body => write!(f, "body"),
            Tag::P => write!(f, "p"),
            Tag::Text(content) => write!(f, "{}", content),
            Tag::Link => write!(f, "link"),
            Tag::Form => write!(f, "form"),
            Tag::Input => write!(f, "input"),
        }
    }
}
