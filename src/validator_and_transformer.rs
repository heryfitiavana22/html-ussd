use std::fmt;

use crate::{
    helper::is_server_url,
    html::{
        Body, BodyContent, Form, FormMethod, Head, Href, HrefType, Html, HtmlUssdTree, Input,
        InputType, Link, Paragraph, Tag, TagElement, Title,
    },
};

pub struct ValidatorAndTransformer;

impl ValidatorAndTransformer {
    pub fn validate(
        &self,
        tag_elements: Vec<TagElement>,
    ) -> Result<HtmlUssdTree, ValidatorAndTransformerError> {
        if tag_elements.len() != 1 {
            return Err(ValidatorAndTransformerError::OneHtmlRootOnly);
        }

        let option_html: Option<&TagElement> = tag_elements.first();
        if option_html.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Html));
        };
        let html_element = option_html.unwrap();
        if html_element.tag_name != Tag::Html {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Html));
        }
        if html_element.children.len() > 2 {
            return Err(ValidatorAndTransformerError::UnexpectedChilds(
                html_element.clone(),
                html_element.children.clone(),
            ));
        }
        let history_enabled = match html_element.attributes.get("history-enabled") {
            Some(val) => val == "true",
            None => true,
        };

        let cache = match html_element.attributes.get("cache") {
            Some(val) => val == "true",
            None => true,
        };

        // <head>
        let option_head = html_element.children.first();
        if option_head.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Head));
        };
        let head_element = option_head.unwrap();
        if head_element.tag_name != Tag::Head {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Head));
        }
        if head_element.children.len() > 1 {
            return Err(ValidatorAndTransformerError::UnexpectedChilds(
                head_element.clone(),
                head_element.children.clone(),
            ));
        }
        let option_title = head_element.children.first();
        if option_title.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Title));
        };
        let title_element = option_title.unwrap();
        if title_element.tag_name != Tag::Title {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Title));
        }
        if title_element.children.len() > 1 {
            return Err(ValidatorAndTransformerError::UnexpectedChilds(
                title_element.clone(),
                title_element.children.clone(),
            ));
        }
        let option_child_title = title_element.children.first();
        if option_title.is_none() {
            return Err(ValidatorAndTransformerError::MissingTextInTitle);
        };
        let text_title_element = option_child_title.unwrap();
        let title = match &text_title_element.tag_name {
            Tag::Text(content_text) => content_text.clone(),
            _ => {
                return Err(ValidatorAndTransformerError::UnexpectedTag(
                    text_title_element.tag_name.clone(),
                ));
            }
        };

        // <body>
        let option_body = html_element.children.get(1);
        if option_body.is_none() {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Body));
        };
        let body_element = option_body.unwrap();
        if body_element.tag_name != Tag::Body {
            return Err(ValidatorAndTransformerError::TagNotFound(Tag::Body));
        }
        if body_element.children.is_empty() {
            return Err(ValidatorAndTransformerError::EmptyBody);
        }

        let mut body_paragraphs: Vec<Paragraph> = vec![];
        let mut body_content: BodyContent = BodyContent::Empty;
        let mut links: Vec<Link> = vec![];
        let mut form_found = false;
        let mut link_found = false;

        for child_body_element in &body_element.children {
            match &child_body_element.tag_name {
                Tag::Text(_) | Tag::P => {
                    if form_found || link_found {
                        return Err(ValidatorAndTransformerError::TextMustBeBeforeLinkOrForm);
                    }

                    let text_link = self.get_text_with_paragraph(child_body_element.clone())?;
                    body_paragraphs.push(Paragraph {
                        text: text_link,
                        attributes: child_body_element.attributes.clone(),
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

                    if child_body_element.children.len() > 1 {
                        return Err(ValidatorAndTransformerError::UnexpectedChilds(
                            child_body_element.clone(),
                            child_body_element.children.clone(),
                        ));
                    }

                    let mut form_method: FormMethod = FormMethod::Get;
                    let option_method_attr = child_body_element.attributes.get_key_value("method");
                    if let Some((_, value)) = option_method_attr {
                        if value.to_lowercase() != "get" && value.to_lowercase() != "post" {
                            return Err(ValidatorAndTransformerError::InvalidFormMethod(
                                value.clone(),
                            ));
                        }
                        if value.to_lowercase() == "post" {
                            form_method = FormMethod::Post
                        }
                    } else {
                        return Err(ValidatorAndTransformerError::MissingFormMethod);
                    }
                    let option_action_attr = child_body_element.attributes.get_key_value("action");
                    if option_action_attr.is_none() {
                        return Err(ValidatorAndTransformerError::MissingFormAction);
                    }
                    let form_action = option_action_attr.unwrap().1;
                    if !is_server_url(form_action) {
                        return Err(ValidatorAndTransformerError::FormActionMustBeServerUrl);
                    }

                    // <input />
                    let option_input = child_body_element.children.first();
                    if option_input.is_none() {
                        return Err(ValidatorAndTransformerError::FormMustHaveOneInput);
                    };
                    let input_element = option_input.unwrap();
                    if input_element.tag_name != Tag::Input {
                        return Err(ValidatorAndTransformerError::UnexpectedChilds(
                            child_body_element.clone(),
                            child_body_element.children.clone(),
                        ));
                    }

                    // <input type="..." />
                    let mut input_type: InputType = InputType::Text;
                    let option_type_attr = input_element.attributes.get_key_value("type");
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

                    let option_placeholder_attr =
                        input_element.attributes.get_key_value("placeholder");
                    if option_placeholder_attr.is_none() {
                        return Err(ValidatorAndTransformerError::MissingInputPlaceholder);
                    }

                    let option_name_attr = input_element.attributes.get_key_value("name");
                    if option_name_attr.is_none() {
                        return Err(ValidatorAndTransformerError::MissingInputName);
                    }

                    if !input_element.children.is_empty() {
                        return Err(ValidatorAndTransformerError::UnexpectedChilds(
                            input_element.clone(),
                            input_element.children.clone(),
                        ));
                    }

                    body_content = BodyContent::Form(Form {
                        attributes: child_body_element.attributes.clone(),
                        method: form_method,
                        action: option_action_attr.unwrap().1.to_string(),
                        input: Input {
                            attributes: input_element.attributes.clone(),
                            name: option_name_attr.unwrap().1.to_string(),
                            input_type,
                            placeholder: option_placeholder_attr.unwrap().1.to_string(),
                        },
                    })
                }
                Tag::Link => {
                    if form_found {
                        return Err(ValidatorAndTransformerError::FormAndLinkTogether);
                    }
                    link_found = true;
                    let option_href = child_body_element.attributes.get_key_value("href");
                    if option_href.is_none() {
                        return Err(ValidatorAndTransformerError::MissingHref);
                    }
                    let href = option_href.unwrap().1;

                    let option_text = child_body_element.children.first();
                    if option_text.is_none() {
                        return Err(ValidatorAndTransformerError::MissingTextInLink);
                    };
                    let text_element = option_text.unwrap();
                    let text_link = self.get_text(text_element.clone())?;

                    let href_type = if is_server_url(href) {
                        HrefType::Server
                    } else if !href.is_empty() {
                        HrefType::File
                    } else {
                        return Err(ValidatorAndTransformerError::InvalidHref(href.to_string()));
                    };

                    links.push(Link {
                        attributes: child_body_element.attributes.clone(),
                        text: text_link,
                        href: Href {
                            url: href.to_string(),
                            href_type,
                        },
                    });
                }
                _ => {
                    return Err(ValidatorAndTransformerError::UnexpectedTag(
                        child_body_element.tag_name.clone(),
                    ));
                }
            }
        }

        if link_found {
            body_content = BodyContent::Links(links);
        }

        let html_tree = {
            HtmlUssdTree {
                source: Html {
                    attributes: html_element.attributes.clone(),
                    history_enabled,
                    cache,
                    head: Head {
                        attributes: head_element.attributes.clone(),
                        title: Title {
                            attributes: title_element.attributes.clone(),
                            text: title,
                        },
                    },
                    body: Body {
                        attributes: body_element.attributes.clone(),
                        paragraphs: body_paragraphs,
                        content: body_content,
                    },
                },
            }
        };

        Ok(html_tree)
    }

    fn get_text(&self, text_element: TagElement) -> Result<String, ValidatorAndTransformerError> {
        let text_link = match &text_element.tag_name {
            Tag::Text(content_text) => content_text.clone(),
            _ => {
                return Err(ValidatorAndTransformerError::TextExpected(
                    text_element.tag_name.clone(),
                ));
            }
        };
        Ok(text_link)
    }

    fn get_text_with_paragraph(
        &self,
        text_element: TagElement,
    ) -> Result<String, ValidatorAndTransformerError> {
        let mut text_link = String::new();
        match &text_element.tag_name {
            Tag::Text(content_text) => {
                text_link = content_text.clone();
            }
            Tag::P => {
                if text_element.children.len() > 1 {
                    return Err(ValidatorAndTransformerError::UnexpectedChilds(
                        text_element.clone(),
                        text_element.children.clone(),
                    ));
                }
                if let Some(child_paragraph) = text_element.children.first() {
                    text_link = self.get_text(child_paragraph.clone())? + "\n";
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

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatorAndTransformerError {
    OneHtmlRootOnly,
    TagNotFound(Tag),
    FormAndLinkTogether,
    UnexpectedTag(Tag),
    TextExpected(Tag),
    UnexpectedChilds(TagElement, Vec<TagElement>),
    FormMustHaveOneInput,
    InvalidInputType(String),
    InvalidFormMethod(String),
    MissingFormAction,
    FormActionMustBeServerUrl,
    MissingFormMethod,
    EmptyBody,
    MissingInputType,
    MissingInputPlaceholder,
    MissingInputName,
    MutlipleForm,
    MissingHref,
    MissingTextInLink,
    MissingTextInTitle,
    InvalidHref(String),
    TextMustBeBeforeLinkOrForm,
}

impl fmt::Display for ValidatorAndTransformerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidatorAndTransformerError::OneHtmlRootOnly => {
                write!(f, "Only one <html> root element is allowed.")
            }
            ValidatorAndTransformerError::TagNotFound(tag) => write!(f, "Tag not found: {}", tag),
            ValidatorAndTransformerError::FormAndLinkTogether => {
                write!(f, "A <form> and a <link> cannot be siblings.")
            }
            ValidatorAndTransformerError::UnexpectedTag(tag) => {
                write!(f, "Unexpected tag : {}", tag)
            }
            ValidatorAndTransformerError::TextExpected(tag) => {
                write!(f, "Text was expected : {}", tag)
            }
            ValidatorAndTransformerError::UnexpectedChilds(parent, children) => {
                write!(f, "Unexpected children in <{}>: ", parent.tag_name)?;
                for child in children {
                    write!(f, "<{}> ", child.tag_name)?;
                }
                Ok(())
            }
            ValidatorAndTransformerError::FormMustHaveOneInput => {
                write!(f, "Form must have exactly one <input> field.")
            }
            ValidatorAndTransformerError::InvalidInputType(input_type) => {
                write!(f, "Invalid input type: '{}'.", input_type)
            }
            ValidatorAndTransformerError::InvalidFormMethod(method) => write!(
                f,
                "Invalid form method: '{}'. Expected 'GET' or 'POST'.",
                method
            ),
            ValidatorAndTransformerError::MissingFormAction => {
                write!(f, "Missing 'action' attribute in <form>.")
            }
            ValidatorAndTransformerError::FormActionMustBeServerUrl => {
                write!(f, "Form action must be a server URL.")
            }
            ValidatorAndTransformerError::MissingFormMethod => {
                write!(f, "Missing 'method' attribute in <form>.")
            }
            ValidatorAndTransformerError::EmptyBody => write!(f, "The <body> tag cannot be empty."),
            ValidatorAndTransformerError::MissingInputType => {
                write!(f, "Missing 'type' attribute in <input>.")
            }
            ValidatorAndTransformerError::MissingInputPlaceholder => {
                write!(f, "Missing 'placeholder' attribute in <input>.")
            }
            ValidatorAndTransformerError::MissingInputName => {
                write!(f, "Missing 'name' attribute in <input>.")
            }
            ValidatorAndTransformerError::MutlipleForm => {
                write!(f, "Multiple <form> tags found. Only one is allowed.")
            }
            ValidatorAndTransformerError::MissingHref => {
                write!(f, "Missing 'href' attribute in <link>.")
            }
            ValidatorAndTransformerError::MissingTextInLink => {
                write!(f, "Missing text inside <link>.")
            }
            ValidatorAndTransformerError::MissingTextInTitle => {
                write!(f, "Missing text inside <title>.")
            }
            ValidatorAndTransformerError::InvalidHref(href) => {
                write!(f, "Invalid href value: '{}'.", href)
            }
            ValidatorAndTransformerError::TextMustBeBeforeLinkOrForm => {
                write!(f, "Text must come before a <link> or <form>.")
            }
        }
    }
}
