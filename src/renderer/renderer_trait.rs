use crate::html::HtmlUssdTree;

pub struct RenderParams<'a> {
    pub tree: HtmlUssdTree,
    pub on_input: Box<dyn Fn(String) + 'a>,
}

pub trait Renderer {
    fn render<'a>(&self, params: RenderParams<'a>);
}
