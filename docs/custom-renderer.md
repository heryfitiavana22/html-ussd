# Creating a Custom Renderer

**HTML-USSD** supports custom renderers, giving you full control over how USSD screens are displayed.

This guide shows you how to implement your own renderer.

## Trait Definition

To create a renderer, you need to implement the `Renderer` trait:

```rust
pub trait Renderer {
    fn render<'a>(&self, params: RenderParams<'a>);
    fn render_text(&self, text: String);
    fn render_error(&self, error: String);
}

pub struct RenderParams<'a> {
    pub tree: HtmlUssdTree,
    pub on_input: Box<dyn Fn(String) + 'a>,
    pub is_main_page: bool,
}
```

## Example: TuiRenderer

Here's a basic with `TuiRenderer`:

```rust
pub struct TuiRenderer;

impl Renderer for TuiRenderer {
    fn render(&self, params: RenderParams) {
        todo!()
    }

    fn render_text(&self, text: String) {
        todo!()
    }

    fn render_error(&self, error: String) {
        todo!()
    }
}
```

## Plugging in Your Renderer

To use your custom renderer, you need to pass it to the `Runner` when initializing your application. Here's how you can plug in your renderer:

```rust
fn main() {
    let runner = Runner::new(DomTreeAdapter, TuiRenderer);
    runner.run();
}
```

## Running the app

**From a local file:**

```bash
cargo run --main index.html
```

**From a remote server:**

```bash
cargo run --main http://localhost:8888/
```
