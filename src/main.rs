use html_ussd::{
    adapter::dom_tree_adapter::DomTreeAdapter, renderer::terminal_renderer::TerminalRenderer,
    runner::Runner,
};

pub mod adapter;
pub mod helper;
pub mod html;
pub mod i18n;
pub mod renderer;
pub mod runner;
pub mod ussd_controller;
pub mod validator_and_transformer;
pub mod command;

fn main() {
    let runner = Runner::new(DomTreeAdapter, TerminalRenderer);
    runner.run();
}
