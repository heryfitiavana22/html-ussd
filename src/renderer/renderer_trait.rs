use crate::html::HtmlUssdTree;

pub trait Renderer {
    fn render<F>(&self, tree: &HtmlUssdTree, on_input: F)
    where
        F: Fn(String);
}

pub enum UserInput {
    Navigation(usize),
    FormData(String),
    Exit,
    Back,
    Invalid,
}
