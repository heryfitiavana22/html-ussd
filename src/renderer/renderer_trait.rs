use crate::html::HtmlUssdTree;

pub struct RenderParams<'a> {
    pub tree: HtmlUssdTree,
    pub on_input: Box<dyn Fn(String) + 'a>,
    pub is_main_page: bool,
}

pub trait Renderer {
    #[allow(clippy::needless_lifetimes)]
    fn render<'a>(&self, params: RenderParams<'a>);
}
