use html_ussd::{
    html::{Tag, TagElement},
    validator_and_transformer::{ValidatorAndTransformer, ValidatorAndTransformerError},
};

fn tag_element(
    tag_name: Tag,
    attributes: &[(&str, &str)],
    children: Vec<TagElement>,
) -> TagElement {
    TagElement {
        tag_name,
        attributes: attributes
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        children,
    }
}

#[test]
fn test_validator_valid_html() {
    let html_tree = vec![tag_element(
        Tag::Html,
        &[],
        vec![
            tag_element(
                Tag::Head,
                &[],
                vec![tag_element(
                    Tag::Title,
                    &[],
                    vec![tag_element(Tag::Text("Test".to_string()), &[], vec![])],
                )],
            ),
            tag_element(
                Tag::Body,
                &[],
                vec![tag_element(
                    Tag::P,
                    &[],
                    vec![tag_element(Tag::Text("Hello".to_string()), &[], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);
    assert!(result.is_ok());
}

#[test]
fn test_validator_missing_html_tag() {
    let html_tree = vec![tag_element(Tag::Body, &[], vec![])];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::TagNotFound(Tag::Html))
    ));
}

#[test]
fn test_validator_empty_body() {
    let html_tree = vec![tag_element(
        Tag::Html,
        &[],
        vec![
            tag_element(
                Tag::Head,
                &[],
                vec![tag_element(
                    Tag::Title,
                    &[],
                    vec![tag_element(Tag::Text("Title".to_string()), &[], vec![])],
                )],
            ),
            tag_element(Tag::Body, &[], vec![]),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::EmptyBody)
    ));
}

#[test]
fn test_validator_form_and_link_together() {
    let html_tree = vec![tag_element(
        Tag::Html,
        &[],
        vec![
            tag_element(
                Tag::Head,
                &[],
                vec![tag_element(
                    Tag::Title,
                    &[],
                    vec![tag_element(Tag::Text("Title".to_string()), &[], vec![])],
                )],
            ),
            tag_element(
                Tag::Body,
                &[],
                vec![
                    tag_element(
                        Tag::Form,
                        &[],
                        vec![tag_element(Tag::Input, &[("type", "text")], vec![])],
                    ),
                    tag_element(
                        Tag::Link,
                        &[("href", "test.html")],
                        vec![tag_element(Tag::Text("Go".to_string()), &[], vec![])],
                    ),
                ],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::FormAndLinkTogether)
    ));
}

#[test]
fn test_validator_invalid_input_type() {
    let html_tree = vec![tag_element(
        Tag::Html,
        &[],
        vec![
            tag_element(
                Tag::Head,
                &[],
                vec![tag_element(
                    Tag::Title,
                    &[],
                    vec![tag_element(Tag::Text("Title".to_string()), &[], vec![])],
                )],
            ),
            tag_element(
                Tag::Body,
                &[],
                vec![tag_element(
                    Tag::Form,
                    &[],
                    vec![tag_element(Tag::Input, &[("type", "email")], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::InvalidInputType(_))
    ));
}

#[test]
fn test_validator_link_without_href() {
    let html_tree = vec![tag_element(
        Tag::Html,
        &[],
        vec![
            tag_element(
                Tag::Head,
                &[],
                vec![tag_element(
                    Tag::Title,
                    &[],
                    vec![tag_element(Tag::Text("Title".to_string()), &[], vec![])],
                )],
            ),
            tag_element(
                Tag::Body,
                &[],
                vec![tag_element(
                    Tag::Link,
                    &[],
                    vec![tag_element(
                        Tag::Text("Click here".to_string()),
                        &[],
                        vec![],
                    )],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::MissingHref)
    ));
}

#[test]
fn test_validator_link_without_text() {
    let html_tree = vec![tag_element(
        Tag::Html,
        &[],
        vec![
            tag_element(
                Tag::Head,
                &[],
                vec![tag_element(
                    Tag::Title,
                    &[],
                    vec![tag_element(Tag::Text("Title".to_string()), &[], vec![])],
                )],
            ),
            tag_element(
                Tag::Body,
                &[],
                vec![tag_element(Tag::Link, &[("href", "test.html")], vec![])],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::MissingTextInLink)
    ));
}
