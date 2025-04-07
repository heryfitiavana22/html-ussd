# Supporting Custom Input Formats

**HTML-USSD** is designed to be modular and extensible. While it comes with built-in support for `HTML`, it does not require HTML specifically.

Thanks to the `TagAdapter` trait, you can plug in any custom input format — such as `JSON`, `XML`, or even a completely different DSL — and still benefit from the full USSD simulation engine.

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

### Example: Custom JSON Adapter

You could, for example, design a JSON adapter that takes this:

```json
{
  "type": "menu",
  "title": "My USSD Menu",
  "options": [
    { "label": "Option 1", "href": "option1.html" },
    { "label": "Option 2", "href": "option2.html" }
  ]
}
```

Implement `TagAdapter` :

```rust
pub struct JsonAdapter;

impl TagAdapter for JsonAdapter {
    fn transform(&self, content: &str) -> Result<Vec<RawTag>, AdapterError> {
        todo!()
    }
}
```

## Plugging in Your Adapter

Once you've created your custom adapter, pass it when building the controller:

```rust
fn main() {
    let runner = Runner::new(JsonAdapter, TerminalRenderer);
    runner.run();
}
```