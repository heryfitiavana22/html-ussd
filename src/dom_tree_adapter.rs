use std::collections::HashMap;

use parse_html::{
    dom::dom_tree::DomTree, lexer::tokenizer::Lexer, node::Node, parser::ast::Parser,
};

use crate::{
    adapter::{AdapterError, TagAdapter},
    html::{Tag, TagElement},
};

pub struct DomTreeAdapter;

impl DomTreeAdapter {
    fn transform_nodes(&self, nodes: Vec<Node>) -> Result<Vec<TagElement>, AdapterError> {
        let mut tags: Vec<TagElement> = vec![];
        for node in nodes {
            match node {
                Node::Element(element) => {
                    let tag_name = match element.tag_name.as_str() {
                        "html" => Tag::Html,
                        "head" => Tag::Head,
                        "title" => Tag::Title,
                        "body" => Tag::Body,
                        "a" => Tag::Link,
                        "form" => Tag::Form,
                        "input" => Tag::Input,
                        "p" => Tag::P,
                        _ => return Err(AdapterError::UnexcepetedTag(element.tag_name.clone())),
                    };

                    tags.push(TagElement {
                        tag_name,
                        attributes: element.attributes.clone().into_iter().collect(),
                        children: self.transform_nodes(element.children)?,
                    });
                }
                Node::Text(text) => {
                    tags.push(TagElement {
                        tag_name: Tag::Text(text),
                        attributes: HashMap::new(),
                        children: vec![],
                    });
                }
            }
        }

        Ok(tags)
    }
}

impl TagAdapter for DomTreeAdapter {
    fn transform(&self, html: &str) -> Result<Vec<TagElement>, AdapterError> {
        match DomTree::new::<Lexer, Parser>(html) {
            Ok(dom) => self.transform_nodes(dom.nodes.clone()),

            Err(e) => Err(AdapterError::ParsingError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_conversion() {
        let html = r#"
    <html lang="en">
    <head>
        <title>Document</title>
    </head>
    <body id="container">
        ok ceci est un texte
        <a href="2" id="l1">link1</a>
        <a href="1">link2</a>
    </body>
    </html>"#;
        let adapter = DomTreeAdapter;
        let result = adapter.transform(html);

        assert!(result.is_ok());
    }

    #[test]
    fn unexpected_tag() {
        let html = r#"
    <html lang="en">
    <head>
        <title>Document</title>
    </head>
    <body id="container">
        ok ceci est un texte
        <a href="2" id="l1">link1</a>
        <button href="1">link2</button>
    </body>
    </html>"#;
        let adapter = DomTreeAdapter;
        let result = adapter.transform(html);

        assert!(matches!(result, Err(AdapterError::UnexcepetedTag(tag)) if tag == "button"));
    }
}
