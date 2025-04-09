# Validator and Transformer

This module is responsible for validating and transforming the parsed `TagElement`s from the [`TagAdapter`](./supporting-custom-input-formats).

It checks the structure of your page and converts it into a strongly typed `HtmlUssdTree` that the `UssdController` use.

## Validation Rules

- Must have one `<html>` tag
- `<html>` must contain exactly one `<head>` and one `<body>`
- `<head>` must have one `<title>` with text
- `<body>` must contain only:
  - `<p>` or text before any `<form>` or `<a>`
  - Either multiple `<a>` tags _or_ a single `<form>`
- `<form>` must:
  - Have one `<input>`
  - Use `method="get"` or `method="post"`
  - Point to a server URL in the `action` attribute
  - Include `name` and `placeholder` in `<input>`
- `<a>` must:
  - Have an `href` attribute (either file or server URL)
  - Contain link text

## Cache and History

Two optional attributes can be added to the `<html>` tag:

- `cache="false"` → disables caching **only for that screen**.
- `history-enabled="false"` → disables navigation history.

If not specified, both are enabled by default.

## Transformation

Once validated, the content is transformed into a `HtmlUssdTree`

::: details Code

```rust
pub struct HtmlUssdTree {
    pub source: Html,
}

pub struct Html {
    pub attributes: HashMap<String, String>,
    pub head: Head,
    pub body: Body,
    pub history_enabled: bool,
    pub cache: bool
}

pub struct Head {
    pub attributes: HashMap<String, String>,
    pub title: Title,
}

pub struct Title {
    pub attributes: HashMap<String, String>,
    pub text: String,
}

pub struct Body {
    pub attributes: HashMap<String, String>,
    pub paragraphs: Vec<Paragraph>,
    pub content: BodyContent,
}

pub enum BodyContent {
    Form(Form),
    Links(Vec<Link>),
    Empty,
}

pub struct Form {
    pub attributes: HashMap<String, String>,
    pub input: Input,
    pub method: FormMethod,
    pub action: String,
}

pub enum FormMethod {
    Get,
    Post,
}

pub struct Input {
    pub attributes: HashMap<String, String>,
    pub input_type: InputType,
    pub name: String,
    pub placeholder: String,
}

pub enum InputType {
    Text,
    Number,
}

pub struct Paragraph {
    pub attributes: HashMap<String, String>,
    pub text: String,
}

pub struct Link {
    pub attributes: HashMap<String, String>,
    pub text: String,
    pub href: Href,
}

pub struct Href {
    pub url: String,
    pub href_type: HrefType,
}

pub enum HrefType {
    File,
    Server,
}
```

:::

## Errors

The validator can raise several detailed errors

::: details Code

```rust
pub enum ValidatorAndTransformerError {
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
```

:::
