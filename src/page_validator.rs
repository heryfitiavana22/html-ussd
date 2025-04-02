use crate::html::{Tag, TagElement};

pub struct PageValidator;

impl PageValidator {
    pub fn validate(tag_elements: Vec<TagElement>) -> Result<(), PageValidatorError> {
        let mut form_found = false;
        let mut link_found = false;

        let option_html: Option<&TagElement> = tag_elements.get(0);
        if option_html.is_none() {
            return Err(PageValidatorError::TagNotFound(Tag::Html));
        };
        let html = option_html.unwrap();
        if html.tag_name != Tag::Html {
            return Err(PageValidatorError::TagNotFound(Tag::Html));
        }
        if html.children.len() > 2 {
            return Err(PageValidatorError::UnexpectedChilds(html.children.clone()));
        }

        // <head>
        let option_head = html.children.get(0);
        if option_head.is_none() {
            return Err(PageValidatorError::TagNotFound(Tag::Head));
        };
        let head = option_head.unwrap();
        if head.tag_name != Tag::Head {
            return Err(PageValidatorError::TagNotFound(Tag::Head));
        }
        if head.children.len() > 1 {
            return Err(PageValidatorError::UnexpectedChilds(head.children.clone()));
        }
        let option_title = head.children.get(0);
        if option_title.is_none() {
            return Err(PageValidatorError::TagNotFound(Tag::Title));
        };
        let title = option_title.unwrap();
        if title.tag_name != Tag::Title {
            return Err(PageValidatorError::TagNotFound(Tag::Title));
        }

        // <body>
        let option_body = html.children.get(1);
        if option_body.is_none() {
            return Err(PageValidatorError::TagNotFound(Tag::Body));
        };
        let body = option_body.unwrap();
        if body.tag_name != Tag::Body {
            return Err(PageValidatorError::TagNotFound(Tag::Body));
        }
        if body.children.len() == 0 {
            return Err(PageValidatorError::EmptyBody);
        }

        for child_body in &body.children {
            match &child_body.tag_name {
                Tag::Text(_) | Tag::P => {}
                Tag::Form => {
                    if form_found {
                        return Err(PageValidatorError::MutlipleForm);
                    }
                    if link_found {
                        return Err(PageValidatorError::FormAndLinkTogether);
                    }
                    form_found = true;

                    if child_body.children.len() > 1 {
                        return Err(PageValidatorError::UnexpectedChilds(
                            child_body.children.clone(),
                        ));
                    }

                    let option_input = child_body.children.get(0);
                    if option_input.is_none() {
                        return Err(PageValidatorError::FormMustHaveOneInput);
                    };
                    let input = option_input.unwrap();
                    if input.tag_name != Tag::Input {
                        return Err(PageValidatorError::UnexpectedTag(input.tag_name.clone()));
                    }

                    // <input type="..." />
                    let type_attr = input.attributes.iter().find(|(k, _)| k == "type");
                    if let Some((_, value)) = type_attr {
                        if value != "text" && value != "number" {
                            return Err(PageValidatorError::InvalidInputType(value.clone()));
                        }
                    } else {
                        return Err(PageValidatorError::MissingInputType);
                    }

                    if input.children.len() > 0 {
                        return Err(PageValidatorError::UnexpectedChilds(input.children.clone()));
                    }
                }
                Tag::Link => {
                    if form_found {
                        return Err(PageValidatorError::FormAndLinkTogether);
                    }
                    link_found = true;
                }
                _ => {
                    return Err(PageValidatorError::UnexpectedTag(
                        child_body.tag_name.clone(),
                    ));
                }
            }
        }

        Ok(())
    }
}

pub enum PageValidatorError {
    TagNotFound(Tag),
    FormAndLinkTogether,
    UnexpectedTag(Tag),
    UnexpectedChilds(Vec<TagElement>),
    FormMustHaveOneInput,
    InvalidInputType(String),
    EmptyBody,
    MissingInputType,
    MutlipleForm,
}
