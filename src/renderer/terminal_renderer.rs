use rustyline::DefaultEditor;

use crate::{
    helper::muted_text,
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

        let raw_title = &tree.source.head.title.text;
        let width = raw_title.chars().count() + 4;
        let border = format!("+{:=^width$}+", "", width = width);
        let centered_title = format!("|{:^width$}|", raw_title, width = width);

        println!("\n{}", border);
        println!("{}", centered_title);
        println!("{}\n", border);

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

        println!("\n------------------------");
        if is_empty {
            return;
        };
        if !is_main_page {
            println!("0. {}", t("back"));
            println!("00. {}", t("home"));
        }

        let mut rl = match DefaultEditor::new() {
            Ok(editor) => editor,
            Err(err) => {
                eprintln!("Failed to create input editor: {:?}", err);
                return;
            }
        };

        let readline = rl.readline(muted_text(input_hint.as_str()).as_str());
        match readline {
            Ok(line) => {
                on_input(line);
            }
            Err(_) => println!("{}", t("no_input")),
        }
    }
}
