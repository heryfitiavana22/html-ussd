use std::collections::HashMap;

use parse_html::{dom::dom_tree::DomTree, node::Node};

use crate::{
    adapter::{AdapterError, TagAdapter},
    html::{Tag, TagElement},
};

pub struct DomTreeAdapter {
    pub tree: DomTree,
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

#[cfg(test)]
mod tests {
    use parse_html::node::ElementNode;

    use super::*;

    #[test]
    fn valid_conversion() {
        let dom_tree = DomTree {
            nodes: vec![Node::Element(ElementNode {
                tag_name: "html".to_string(),
                attributes: vec![],
                children: vec![
                    Node::Element(ElementNode {
                        tag_name: "head".to_string(),
                        attributes: vec![],
                        children: vec![Node::Element(ElementNode {
                            tag_name: "title".to_string(),
                            attributes: vec![],
                            children: vec![Node::Text("Test Title".to_string())],
                        })],
                    }),
                    Node::Element(ElementNode {
                        tag_name: "body".to_string(),
                        attributes: vec![],
                        children: vec![Node::Element(ElementNode {
                            tag_name: "p".to_string(),
                            attributes: vec![],
                            children: vec![Node::Text("Hello, USSD!".to_string())],
                        })],
                    }),
                ],
            })],
        };

        let adapter = DomTreeAdapter { tree: dom_tree };
        let result = adapter.transform();

        assert!(result.is_ok());
    }

    #[test]
    fn unexpected_tag() {
        let dom_tree = DomTree {
            nodes: vec![Node::Element(ElementNode {
                tag_name: "html".to_string(),
                attributes: vec![],
                children: vec![
                    Node::Element(ElementNode {
                        tag_name: "head".to_string(),
                        attributes: vec![],
                        children: vec![Node::Element(ElementNode {
                            tag_name: "title".to_string(),
                            attributes: vec![],
                            children: vec![Node::Text("Test Title".to_string())],
                        })],
                    }),
                    Node::Element(ElementNode {
                        tag_name: "body".to_string(),
                        attributes: vec![],
                        children: vec![
                            Node::Element(ElementNode {
                                tag_name: "p".to_string(),
                                attributes: vec![],
                                children: vec![Node::Text("Hello, USSD!".to_string())],
                            }),
                            Node::Element(ElementNode {
                                tag_name: "button".to_string(),
                                attributes: vec![],
                                children: vec![],
                            }),
                        ],
                    }),
                ],
            })],
        };

        let adapter = DomTreeAdapter { tree: dom_tree };
        let result = adapter.transform();

        assert!(matches!(result, Err(AdapterError::UnexcepetedTag(tag)) if tag == "button"));
    }
}
