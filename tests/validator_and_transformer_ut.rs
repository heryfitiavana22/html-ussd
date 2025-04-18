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
fn valid_html() {
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
fn missing_html_tag() {
    let html_tree = vec![tag_element(Tag::Body, &[], vec![])];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::TagNotFound(Tag::Html))
    ));
}

#[test]
fn empty_body() {
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
fn form_and_link_together() {
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
                        &[("method", "get"), ("action", "http://localhost:8080")],
                        vec![tag_element(
                            Tag::Input,
                            &[
                                ("type", "text"),
                                ("placeholder", "text"),
                                ("name", "exampe"),
                            ],
                            vec![],
                        )],
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
fn invalid_form_method() {
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
                    &[("method", "gett"), ("action", "http://localhost:8080")],
                    vec![tag_element(Tag::Input, &[("type", "text")], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::InvalidFormMethod(_))
    ));
}

#[test]
fn missing_input_in_form() {
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
                    &[("method", "get"), ("action", "http://localhost:8080")],
                    vec![],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::FormMustHaveOneInput)
    ));
}

#[test]
fn invalid_child_form() {
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
                    &[("method", "get"), ("action", "http://localhost:8080")],
                    vec![tag_element(
                        Tag::Text("ok".to_string()),
                        &[("type", "text")],
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
        Err(ValidatorAndTransformerError::UnexpectedChilds(_, _))
    ));
}

#[test]
fn missing_form_action() {
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
                    &[("method", "get")],
                    vec![tag_element(Tag::Input, &[("type", "text")], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::MissingFormAction)
    ));
}

#[test]
fn invalid_form_action() {
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
                    &[("method", "get"), ("action", "/")],
                    vec![tag_element(Tag::Input, &[("type", "text")], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::FormActionMustBeServerUrl)
    ));
}


#[test]
fn invalid_input_type() {
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
                    &[("method", "get"), ("action", "http://localhost:8080")],
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
fn missing_input_type() {
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
                    &[("method", "get"), ("action", "http://localhost:8080")],
                    vec![tag_element(Tag::Input, &[], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::MissingInputType)
    ));
}

#[test]
fn missing_input_placeholder() {
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
                    &[("method", "get"), ("action", "http://localhost:8080")],
                    vec![tag_element(Tag::Input, &[("type", "text")], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::MissingInputPlaceholder)
    ));
}

#[test]
fn missing_input_name() {
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
                    &[("method", "get"), ("action", "http://localhost:8080")],
                    vec![tag_element(
                        Tag::Input,
                        &[("type", "text"), ("placeholder", "exam")],
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
        Err(ValidatorAndTransformerError::MissingInputName)
    ));
}

#[test]
fn invalid_input_child() {
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
                    &[("method", "get"), ("action", "http://localhost:8080")],
                    vec![tag_element(
                        Tag::Input,
                        &[("type", "text"), ("placeholder", "exam"), ("name", "examp")],
                        vec![tag_element(Tag::Text("Title".to_string()), &[], vec![])],
                    )],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::UnexpectedChilds(_, _))
    ));
}

#[test]
fn link_without_href() {
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
fn link_with_invalid_child() {
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
                    &[("href", "test.html")],
                    vec![tag_element(Tag::P, &[], vec![])],
                )],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::TextExpected(_))
    ));
}

#[test]
fn text_after_link_or_form() {
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
                        Tag::Link,
                        &[("href", "test.html")],
                        vec![tag_element(Tag::Text("ok".to_string()), &[], vec![])],
                    ),
                    tag_element(Tag::Text("ok".to_string()), &[], vec![]),
                ],
            ),
        ],
    )];

    let validator = ValidatorAndTransformer;
    let result = validator.validate(html_tree);

    assert!(matches!(
        result,
        Err(ValidatorAndTransformerError::TextMustBeBeforeLinkOrForm)
    ));
}
