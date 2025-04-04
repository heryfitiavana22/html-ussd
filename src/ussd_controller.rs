use std::collections::HashMap;

use reqwest::{
    Error,
    blocking::{Client, Response, get},
};

use crate::{
    adapter::adapter_trait::TagAdapter,
    html::{BodyContent, FormMethod, HrefType, InputType},
    renderer::renderer_trait::Renderer,
    validator_and_transformer::ValidatorAndTransformer,
};

pub struct UssdController<R: Renderer, T: TagAdapter> {
    pub pages: HashMap<String, String>,
    pub main_page: String,
    pub adapter: T,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
}

impl<R: Renderer, T: TagAdapter> UssdController<R, T> {
    pub fn run(&self) {
        self.display(&self.main_page);
    }
    pub fn display(&self, html: &str) {
        let tags = match self.adapter.transform(html) {
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

        self.renderer
            .render(&tree, |user_input| match &tree.source.body.content {
                BodyContent::Links(links) => {
                    if let Ok(index) = user_input.parse::<usize>() {
                        if index == 0 || index > links.len() {
                            println!("invalid input links, index out of bounds");
                            return;
                        }
                        let option_next_link = links.get(index - 1);
                        if option_next_link.is_none() {
                            println!("invalid input links, invalid index");
                            return;
                        }
                        let next_link = option_next_link.unwrap();

                        if next_link.href.href_type == HrefType::File {
                            if let Some(next_html) = self.pages.get(&next_link.href.url) {
                                // println!("navigate to : {}", next_link.href.url);
                                self.display(&next_html);
                                return;
                            } else {
                                println!("page not found : {}", next_link.href.url);
                                return;
                            }
                        } else {
                            self.handle_response(get(&next_link.href.url));
                            return;
                        }
                    }

                    println!("invalid input links expected number");
                }
                BodyContent::Form(form) => {
                    let valid = match form.input.input_type {
                        InputType::Text => true,
                        InputType::Number => user_input.parse::<f64>().is_ok(),
                    };

                    if valid {
                        // println!("form data : {}", user_input);
                        let url = &form.action;
                        let param_name = &form.input.name;
                        let client = Client::new();

                        let response_result = match form.method {
                            FormMethod::Get => {
                                client.get(url).query(&[(param_name, &user_input)]).send()
                            }
                            FormMethod::Post => {
                                client.post(url).form(&[(param_name, &user_input)]).send()
                            }
                        };
                        self.handle_response(response_result);
                    } else {
                        println!("Invalid form data");
                    }
                }
                BodyContent::Empty => {}
            });
    }

    fn handle_response(&self, response: Result<Response, Error>) {
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(html) = response.text() {
                        self.display(&html);
                    } else {
                        println!("Failed to read response text");
                    }
                } else {
                    println!("Request failed with status: {}", response.status());
                }
            }
            Err(err) => {
                println!("Failed to fetch page: {:?}", err);
            }
        }
    }
}
