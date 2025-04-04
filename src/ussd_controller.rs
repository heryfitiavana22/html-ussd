use std::{cell::RefCell, collections::HashMap};

use reqwest::{
    Error,
    blocking::{Client, Response, get},
};

use crate::{
    adapter::adapter_trait::TagAdapter,
    html::{BodyContent, FormMethod, HrefType, InputType},
    renderer::renderer_trait::{RenderParams, Renderer},
    validator_and_transformer::ValidatorAndTransformer,
};

#[derive(Debug, PartialEq, Clone)]
pub struct HistoryItem {
    pub page: String,
    pub is_main_page: bool,
}

pub struct UssdController<R: Renderer, T: TagAdapter> {
    pub pages: HashMap<String, String>,
    pub main_page: String,
    pub adapter: T,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
    pub history: RefCell<Vec<HistoryItem>>,
}

pub struct NewController<R: Renderer, T: TagAdapter> {
    pub pages: HashMap<String, String>,
    pub main_page: String,
    pub adapter: T,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
}

pub struct DisplayParams {
    pub html: String,
    pub is_main_page: bool,
    pub push_to_history: bool,
}

impl<R: Renderer, T: TagAdapter> UssdController<R, T> {
    pub fn new(params: NewController<R, T>) -> Self {
        Self {
            pages: params.pages,
            main_page: params.main_page,
            adapter: params.adapter,
            validator: params.validator,
            renderer: params.renderer,
            history: RefCell::new(vec![]),
        }
    }

    pub fn run(&self) {
        self.display(DisplayParams {
            html: self.main_page.clone(),
            is_main_page: true,
            push_to_history: true,
        });
    }
    pub fn display(&self, params: DisplayParams) {
        let DisplayParams {
            html,
            is_main_page,
            push_to_history,
        } = params;

        if push_to_history {
            let mut history = self.history.borrow_mut();
            history.push(HistoryItem {
                page: html.to_string(),
                is_main_page,
            });
            // println!("push_to_history : {:?}", history);
            // println!("push_to_history.len : {:?}", history.len());
            drop(history);
        }

        let tags = match self.adapter.transform(html.as_str()) {
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

        let body_content = tree.source.body.content.clone();

        self.renderer.render(RenderParams {
            tree,
            is_main_page,
            on_input: Box::new(move |user_input| {
                if user_input == "0" && !is_main_page {
                    self.go_back();
                    return;
                }

                if user_input == "00" && !is_main_page {
                    self.go_to_main_page();
                    return;
                }

                match &body_content {
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
                                    self.display(DisplayParams {
                                        html: next_html.clone(),
                                        is_main_page: false,
                                        push_to_history: true,
                                    });
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
                }
            }),
        });
    }

    fn go_back(&self) {
        let mut history = self.history.borrow_mut();
        history.pop();
        // println!("go_back : {:?}", history);
        // println!("go_back.len : {:?}", history.len());

        if let Some(previous) = history.pop() {
            drop(history);
            self.display(DisplayParams {
                html: previous.page,
                is_main_page: previous.is_main_page,
                push_to_history: true,
            });
        } else {
            drop(history);
            // println!("No previous page");
            self.display(DisplayParams {
                html: self.main_page.clone(),
                is_main_page: true,
                push_to_history: true,
            });
        }
    }

    fn go_to_main_page(&self) {
        let mut history = self.history.borrow_mut();
        history.clear();
        drop(history);

        self.display(DisplayParams {
            html: self.main_page.clone(),
            is_main_page: true,
            push_to_history: true,
        });
    }

    fn handle_response(&self, response: Result<Response, Error>) {
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(html) = response.text() {
                        self.display(DisplayParams {
                            html: html.clone(),
                            is_main_page: false,
                            push_to_history: true,
                        });
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
