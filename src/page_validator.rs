use crate::html::{Tag, TagElement};

pub struct PageValidator;

impl PageValidator {
    pub fn validate(tag_elements: &[TagElement]) -> Result<(), PageValidatorError> {
        let mut title_found = false;
        let mut form_found = false;
        let mut link_found = false;

        for tag in tag_elements {
            match &tag.tag_name {
                &Tag::Head => {
                    let title_count = tag
                        .children
                        .iter()
                        .filter(|t| matches!(t.tag_name, Tag::Title))
                        .count();
                    if title_count != 1 {
                        return Err(PageValidatorError::TitleMissingOrMultiple);
                    }
                    title_found = true;
                }
                Tag::Body => {
                    for child in &tag.children {
                        match &child.tag_name {
                            Tag::Text(_) | Tag::P | Tag::Link | Tag::Form => {}
                            _ => {
                                return Err(PageValidatorError::UnexpectedTag(child.tag_name.clone()));
                            }
                        }

                        // <a> ou un <form>, mais pas les deux
                        match &child.tag_name {
                            Tag::Form => {
                                if form_found || link_found {
                                    return Err(PageValidatorError::FormAndLinkTogether);
                                }
                                form_found = true;

                                // Vérifier que le form a un seul input
                                let input_count = child
                                    .children
                                    .iter()
                                    .filter(|t| matches!(t.tag_name, Tag::Input))
                                    .count();
                                if input_count != 1 {
                                    return Err(PageValidatorError::FormMustHaveOneInput);
                                }

                                // Vérifier le type de l'input
                                if let Some(input) = child
                                    .children
                                    .iter()
                                    .find(|t| matches!(t.tag_name, Tag::Input))
                                {
                                    let type_attr =
                                        input.attributes.iter().find(|(k, _)| k == "type");
                                    if let Some((_, value)) = type_attr {
                                        if value != "text" && value != "number" {
                                            return Err(PageValidatorError::InvalidInputType(
                                                value.clone(),
                                            ));
                                        }
                                    } else {
                                        return Err(PageValidatorError::MissingInputType);
                                    }
                                }
                            }
                            Tag::Link => {
                                if form_found {
                                    return Err(PageValidatorError::FormAndLinkTogether);
                                }
                                link_found = true;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if !title_found {
            return Err(PageValidatorError::TitleMissingOrMultiple);
        }

        if !form_found && !link_found {
            return Err(PageValidatorError::NoLinksOrForm);
        }

        Ok(())
    }
}

pub enum PageValidatorError {
    TitleMissingOrMultiple,
    FormAndLinkTogether,
    UnexpectedTag(Tag),
    FormMustHaveOneInput,
    InvalidInputType(String),
    MissingInputType,
    NoLinksOrForm,
}
