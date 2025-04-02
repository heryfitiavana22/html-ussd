use crate::html::{
    Body, BodyContent, Form, FormMethod, Head, Href, HrefType, Html, HtmlUssdTree, Input,
    InputType, Link, Paragraph, Tag, TagElement, Title,
};

pub struct ValidatorAndTransformer;

impl ValidatorAndTransformer {
    pub fn validate(
        &self,
        tag_elements: Vec<TagElement>,
    ) -> Result<HtmlUssdTree, ValidatorAndTransformerError> {
        let mut form_found = false;
        let mut link_found = false;

        let option_html: Option<&TagElement> = tag_elements.get(0);
        if option_html.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Html));
        };
        let html = option_html.unwrap();
        if html.tag_name != Tag::Html {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Html));
        }
        if html.children.len() > 2 {
            return Err(ValidatorAndTransformerError::UnexpectedChilds(
                html.children.clone(),
            ));
        }

        // <head>
        let option_head = html.children.get(0);
        if option_head.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Head));
        };
        let head = option_head.unwrap();
        if head.tag_name != Tag::Head {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Head));
        }
        if head.children.len() > 1 {
            return Err(ValidatorAndTransformerError::UnexpectedChilds(
                head.children.clone(),
            ));
        }
        let option_title = head.children.get(0);
        if option_title.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Title));
        };
        let title_element = option_title.unwrap();
        if title_element.tag_name != Tag::Title {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Title));
        }
        if title_element.children.len() > 0 {
            return Err(ValidatorAndTransformerError::UnexpectedChilds(
                title_element.children.clone(),
            ));
        }
        let title = self.get_text(title_element.clone())?;

        // <body>
        let option_body = html.children.get(1);
        if option_body.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Body));
        };
        let body = option_body.unwrap();
        if body.tag_name != Tag::Body {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Body));
        }
        if body.children.len() == 0 {
            return Err(ValidatorAndTransformerError::EmptyBody);
        }

        let mut body_paragraphs: Vec<Paragraph> = vec![];
        let mut body_content: BodyContent = BodyContent::Empty;
        let mut links: Vec<Link> = vec![];

        for child_body in &body.children {
            match &child_body.tag_name {
                Tag::Text(_) | Tag::P => {
                    let text_link = self.get_text(child_body.clone())?;
                    body_paragraphs.push(Paragraph {
                        text: text_link,
                        attributes: child_body.attributes.clone(),
                    });
                }
                Tag::Form => {
                    if form_found {
                        return Err(ValidatorAndTransformerError::MutlipleForm);
                    }
                    if link_found {
                        return Err(ValidatorAndTransformerError::FormAndLinkTogether);
                    }
                    form_found = true;

                    if child_body.children.len() > 1 {
                        return Err(ValidatorAndTransformerError::UnexpectedChilds(
                            child_body.children.clone(),
                        ));
                    }

                    let option_input = child_body.children.get(0);
                    if option_input.is_none() {
                        return Err(ValidatorAndTransformerError::FormMustHaveOneInput);
                    };
                    let input = option_input.unwrap();
                    if input.tag_name != Tag::Input {
                        return Err(ValidatorAndTransformerError::UnexpectedTag(
                            input.tag_name.clone(),
                        ));
                    }

                    // <input type="..." />
                    let mut input_type: InputType = InputType::Text;
                    let option_type_attr = input.attributes.get_key_value("type");
                    if let Some((_, value)) = option_type_attr {
                        if value != "text" && value != "number" {
                            return Err(ValidatorAndTransformerError::InvalidInputType(
                                value.clone(),
                            ));
                        }
                        if value == "number" {
                            input_type = InputType::Number
                        }
                    } else {
                        return Err(ValidatorAndTransformerError::MissingInputType);
                    }

                    if input.children.len() > 0 {
                        return Err(ValidatorAndTransformerError::UnexpectedChilds(
                            input.children.clone(),
                        ));
                    }

                    body_content = BodyContent::Form(Form {
                        attributes: child_body.attributes.clone(),
                        method: FormMethod::Get,
                        input: Input {
                            attributes: input.attributes.clone(),
                            name: "name".to_string(),
                            input_type,
                        },
                    })
                }
                Tag::Link => {
                    if form_found {
                        return Err(ValidatorAndTransformerError::FormAndLinkTogether);
                    }
                    link_found = true;
                    let option_href = child_body.attributes.get_key_value("href");
                    if option_href.is_none() {
                        return Err(ValidatorAndTransformerError::MissingHref);
                    }
                    let href = option_href.unwrap().1;

                    let option_text = child_body.children.get(0);
                    if option_text.is_none() {
                        return Err(ValidatorAndTransformerError::MissingTextInLink);
                    };
                    let text_element = option_text.unwrap();
                    let text_link = self.get_text(text_element.clone())?;

                    links.push(Link {
                        attributes: child_body.attributes.clone(),
                        text: text_link,
                        href: Href {
                            url: href.to_string(),
                            href_type: HrefType::File,
                        },
                    });
                }
                _ => {
                    return Err(ValidatorAndTransformerError::UnexpectedTag(
                        child_body.tag_name.clone(),
                    ));
                }
            }
        }

        let html_tree = {
            HtmlUssdTree {
                source: Html {
                    attributes: html.attributes.clone(),
                    head: Head {
                        attributes: head.attributes.clone(),
                        title: Title {
                            attributes: title_element.attributes.clone(),
                            text: title,
                        },
                    },
                    body: Body {
                        attributes: body.attributes.clone(),
                        paragraphs: body_paragraphs,
                        content: body_content,
                    },
                },
            }
        };

        Ok(html_tree)
    }

    fn get_text(&self, text_element: TagElement) -> Result<String, ValidatorAndTransformerError> {
        let mut text_link = String::new();
        match &text_element.tag_name {
            Tag::Text(content_text) => {
                text_link = content_text.clone();
            }
            Tag::P => {
                if text_element.children.len() > 0 {
                    return Err(ValidatorAndTransformerError::UnexpectedChilds(
                        text_element.children.clone(),
                    ));
                }
                if let Some(child_paragraph) = text_element.children.get(0) {
                    match &child_paragraph.tag_name {
                        Tag::Text(content_text) => {
                            text_link = content_text.clone() + "\n";
                        }
                        _ => {
                            return Err(ValidatorAndTransformerError::UnexpectedTag(
                                child_paragraph.tag_name.clone(),
                            ));
                        }
                    }
                }
            }
            _ => {
                return Err(ValidatorAndTransformerError::UnexpectedTag(
                    text_element.tag_name.clone(),
                ));
            }
        }
        Ok(text_link)
    }
}

pub enum ValidatorAndTransformerError {
    TagNotFound(Tag),
    FormAndLinkTogether,
    UnexpectedTag(Tag),
    UnexpectedChilds(Vec<TagElement>),
    FormMustHaveOneInput,
    InvalidInputType(String),
    EmptyBody,
    MissingInputType,
    MutlipleForm,
    MissingHref,
    MissingTextInLink,
}
