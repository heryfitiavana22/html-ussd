# Extending interface

**HTML-USSD** is designed to be modular and extensible. Although it primarily uses `HTML` as input, the system is built to support other formats — such as `JSON`, `XML`, or even domain-specific languages — thanks to the `TagAdapter` trait.

This guide explains how to create your own adapter.

## What is a TagAdapter?

A `TagAdapter` is a trait responsible for **parsing raw content** (usually HTML) into a list of tags that the system can validate and render.

The default adapter in HTML-USSD is `DomTreeAdapter`, which uses [parse-html](https://github.com/heryfitiavana22/parse-html) under the hood.

## Trait Definition

Here’s what the `TagAdapter` trait looks like:

```rust
pub trait TagAdapter {
    fn transform(&self, content: &str) -> Result<Vec<TagElement>, AdapterError>;
}

pub enum AdapterError {
    TagNotFound(Tag),
    UnexcepetedTag(String),
    ParsingError(String),
}


pub struct TagElement {
    pub tag_name: Tag,
    pub attributes: HashMap<String, String>,
    pub children: Vec<TagElement>,
}

pub enum Tag {
    Html,
    Head,
    Title,
    Body,
    P,
    Text(String),
    Link,
    Form,
    Input,
}

```

## Plugging in Your Adapter

Once you've created your custom adapter, pass it when building the controller:

```rust
let controller = UssdController::new(NewController {
    adapter: JsonAdapter,
    // other fields...
});
```