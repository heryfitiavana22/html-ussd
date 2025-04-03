use html_ussd::{
    adapter::TagAdapter,
    html::{BodyContent, InputType},
    renderer::Renderer,
    validator_and_transformer::ValidatorAndTransformer,
};

pub struct Screen<R: Renderer> {
    pub html: String,
    pub adapter: Box<dyn TagAdapter>,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
}

impl<R: Renderer> Screen<R> {
    pub fn run(&self) {
        let tags = match self.adapter.transform(&self.html) {
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
            match &tree.source.body.content {
                BodyContent::Links(links) => {
                    if let Ok(index) = user_input.parse::<usize>() {
                        if index > 0 && index <= links.len() {
                            println!("navigate to index : {}", index - 1);
                            return;
                        }
                    }
                    // invalid
                    println!("invalid input links");
                }
                BodyContent::Form(form) => {
                    let valid = match form.input.input_type {
                        InputType::Text => true,
                        InputType::Number => user_input.parse::<f64>().is_ok(),
                    };

                    if valid {
                        println!("form data : {}", user_input);
                    } else {
                        println!("invalid form dats");
                    }
                }
                BodyContent::Empty => {}
            }
        });
    }
}
