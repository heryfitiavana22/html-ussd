use std::collections::HashMap;

use parse_html::{dom::dom_tree::DomTree, node::Node};

use crate::{
    adapter::{AdapterError, TagAdapter},
    html::{Tag, TagElement},
};

pub struct DomTreeAdapter {
    tree: DomTree,
}

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
    fn transform(&self) -> Result<Vec<TagElement>, AdapterError> {
        self.transform_nodes(self.tree.nodes.clone())
    }
}
