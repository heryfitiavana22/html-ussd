use html_ussd::{
    adapter::TagAdapter, renderer::{Renderer, UserInput}, validator_and_transformer::ValidatorAndTransformer,
};

pub struct Screen<R: Renderer> {
    pub adapter: Box<dyn TagAdapter>,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
}

impl<R: Renderer> Screen<R> {
    pub fn run(&self) {
        let tags = match self.adapter.transform() {
            Ok(tags) => tags,
            Err(e) => {
                eprintln!("Adapter error : {:?}", e);
                return;
            }
        };

        let tree = match self.validator.validate(tags) {
            Ok(tree) => tree,
            Err(e) => {
                eprintln!("Validation error : {:?}", e);
                return;
            }
        };

        self.renderer.render(&tree, |user_input| {
            match user_input {
                UserInput::Navigation(index) => { /* ... */ }
                UserInput::FormData(data) => { /* ... */ }
                UserInput::Exit => { /* ... */ }
                UserInput::Back => { /* ... */ }
            }
        });
    }
}
