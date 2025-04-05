use rustyline::DefaultEditor;

use crate::{
    html::{BodyContent, InputType},
    i18n::t,
};

use super::renderer_trait::{RenderParams, Renderer};

pub struct TerminalRenderer;

impl Renderer for TerminalRenderer {
    fn render(&self, params: RenderParams) {
        let RenderParams {
            tree,
            on_input,
            is_main_page,
        } = params;
        println!("\n=== {} ===\n", tree.source.head.title.text);
        for paragraph in &tree.source.body.paragraphs {
            println!("{}", paragraph.text);
        }

        let mut is_empty = false;
        let mut input_hint = String::new();

        match &tree.source.body.content {
            BodyContent::Form(form) => {
                println!("{}", form.input.placeholder);
                input_hint = match form.input.input_type {
                    InputType::Text => "[abc]".to_string(),
                    InputType::Number => "[123]".to_string(),
                };
            }
            BodyContent::Links(links) => {
                for (index, link) in links.iter().enumerate() {
                    println!("{}. {}", index + 1, link.text);
                }
                input_hint = "[#]".to_string()
            }
            BodyContent::Empty => {
                is_empty = true;
            }
        }
        if is_empty {
            return;
        };
        if !is_main_page {
            println!("----");
            println!("0. {}", t("back"));
            println!("00. {}", t("home"));
        }
        let mut rl = match DefaultEditor::new() {
            Ok(editor) => editor,
            Err(err) => {
                eprintln!("Failed to create editor: {:?}", err);
                return;
            }
        };
        let readline = rl.readline(format!("{} > ", input_hint).as_str());
        match readline {
            Ok(line) => {
                on_input(line);
            }
            Err(_) => println!("{}", t("no_input")),
        }
    }
}
