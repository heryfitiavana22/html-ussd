use std::io::{self, Write};

use crate::html::{BodyContent, HtmlUssdTree};

use super::renderer_trait::Renderer;

pub struct TerminalRenderer;

impl Renderer for TerminalRenderer {
    fn render<F>(&self, tree: &HtmlUssdTree, on_input: F)
    where
        F: Fn(String),
    {
        println!("\n=== {} ===\n", tree.source.head.title.text);
        for paragraph in &tree.source.body.paragraphs {
            println!("{}", paragraph.text);
        }

        let mut is_empty = false;
        match &tree.source.body.content {
            BodyContent::Form(form) => {
                println!("{}", form.input.placeholder);
            }
            BodyContent::Links(links) => {
                for (index, link) in links.iter().enumerate() {
                    println!("{}. {}", index + 1, link.text);
                }
            }
            BodyContent::Empty => {
                is_empty = true;
            }
        }
        if is_empty {
            return;
        };
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        on_input(input.to_string())
    }
}
