use std::{collections::HashMap, fmt::Display};

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
    pub attributes: HashMap<String, String>,
    pub children: Vec<TagElement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HtmlUssdTree {
    pub source: Html,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Html {
    pub attributes: HashMap<String, String>,
    pub head: Head,
    pub body: Body,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Head {
    pub attributes: HashMap<String, String>,
    pub title: Title,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Title {
    pub attributes: HashMap<String, String>,
    pub text: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    pub attributes: HashMap<String, String>,
    pub paragraphs: Vec<Paragraph>,
    pub content: BodyContent,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BodyContent {
    Form(Form),
    Links(Vec<Link>),
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Form {
    pub attributes: HashMap<String, String>,
    pub input: Input,
    pub method: FormMethod,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FormMethod {
    Get,
    Post,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Input {
    pub attributes: HashMap<String, String>,
    pub input_type: InputType,
    pub name: String,
    pub placeholder: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputType {
    Text,
    Number,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Paragraph {
    pub attributes: HashMap<String, String>,
    pub text: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Link {
    pub attributes: HashMap<String, String>,
    pub text: String,
    pub href: Href,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Href {
    pub url: String,
    pub href_type: HrefType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HrefType {
    File,
    Server,
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
