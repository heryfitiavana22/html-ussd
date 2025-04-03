use std::collections::HashMap;

use html_ussd::{
    dom_tree_adapter::DomTreeAdapter, terminal_renderer::TerminalRenderer,
    ussd_controller::UssdController, validator_and_transformer::ValidatorAndTransformer,
};

pub mod adapter;
pub mod dom_tree_adapter;
pub mod html;
pub mod renderer;
pub mod terminal_renderer;
pub mod ussd_controller;
pub mod validator_and_transformer;

fn main() {
    // println!("Hello, world!");
    let main_page = r#"
    <html lang="en">
    <head>
        <title>Titre main page</title>
    </head>
    <body id="container">
        bievenue
        <a href="http://localhost:8888/main-page" id="l1">server</a>
        <a href="page2.html" id="l1">page 2</a>
        <a href="page3.html">page 3</a>
    </body>
    </html>"#;

    let page2 = r#"
    <html lang="en">
    <head>
        <title>Titre page 2</title>
    </head>
    <body id="container">
        oui miova e
        <a href="page3.html">link2</a>
    </body>
    </html>"#;

    let page3 = r#"
    <html lang="en">
    <head>
        <title>Titre page 3</title>
    </head>
    <body id="container">
        farany
    </body>
    </html>"#;

    let mut pages: HashMap<String, String> = HashMap::new();
    pages.insert("page2.html".to_string(), page2.to_string());
    pages.insert("page3.html".to_string(), page3.to_string());

    let ussd_controller = UssdController {
        main_page: main_page.to_string(),
        pages,
        adapter: DomTreeAdapter,
        renderer: TerminalRenderer,
        validator: ValidatorAndTransformer,
    };
    ussd_controller.run();
}
