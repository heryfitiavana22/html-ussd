use crate::{
    html::{BodyContent, HtmlUssdTree},
    renderer::{Renderer, UserInput},
};

pub struct TerminalRenderer;

impl Renderer for TerminalRenderer {
    fn render<F>(&self, tree: &HtmlUssdTree, on_input: F)
    where
        F: Fn(UserInput),
    {
        println!("\n=== {} ===\n", tree.source.head.title.text);
        for paragraph in &tree.source.body.paragraphs {
            println!("{}", paragraph.text);
        }

        match &tree.source.body.content {
            BodyContent::Form(form) => {
                // TODO: input placeholder
                println!("Saisissez une valeur:");
            }
            BodyContent::Links(links) => {
                for (index, link) in links.iter().enumerate() {
                    println!("{}. {} \n", index + 1, link.text);
                }
            }
            BodyContent::Empty => {
                println!("Fin de l'application.");
            }
        }

        // TODO: handle on_input
    }
}
