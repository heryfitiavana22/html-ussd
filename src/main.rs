use html_ussd::{
    dom_tree_adapter::DomTreeAdapter, terminal_renderer::TerminalRenderer,
    validator_and_transformer::ValidatorAndTransformer,
};
use screen::Screen;

pub mod adapter;
pub mod dom_tree_adapter;
pub mod html;
pub mod renderer;
pub mod screen;
pub mod terminal_renderer;
pub mod validator_and_transformer;

fn main() {
    // println!("Hello, world!");
    let html = r#"
    <html lang="en">
    <head>
        <title>Document</title>
    </head>
    <body id="container">
        ok ceci est un texte
        <a href="2" id="l1">link1</a>
        <a href="1">link2</a>
    </body>
    </html>"#;

    let screen = Screen {
        html: html.to_string(),
        adapter: Box::new(DomTreeAdapter),
        renderer: TerminalRenderer,
        validator: ValidatorAndTransformer,
    };
    screen.run();
}
