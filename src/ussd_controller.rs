use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use reqwest::{
    Error,
    blocking::{Client, Response, get},
};

use crate::{
    adapter::adapter_trait::TagAdapter,
    helper::{handle_result_response, load_file},
    html::{BodyContent, FormMethod, HrefType, InputType},
    renderer::renderer_trait::{RenderParams, Renderer},
    validator_and_transformer::ValidatorAndTransformer,
};

const BACK_KEY: &str = "0";
const HOME_KEY: &str = "00";

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
    pub base_dir: Option<PathBuf>,
}

pub struct NewController<R: Renderer, T: TagAdapter> {
    pub pages: HashMap<String, String>,
    pub main_page: String,
    pub adapter: T,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
    pub base_dir: Option<PathBuf>,
}

pub struct DisplayParams {
    pub html: String,
    pub is_main_page: bool,
    pub is_next_page: bool,
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
            base_dir: params.base_dir,
        }
    }

    pub fn run(&self) {
        self.display(DisplayParams {
            html: self.main_page.clone(),
            is_main_page: true,
            is_next_page: true,
        });
    }
    pub fn display(&self, params: DisplayParams) {
        let DisplayParams {
            html,
            is_main_page,
            is_next_page,
        } = params;

        let tags = match self.adapter.transform(html.as_str()) {
            Ok(tags) => tags,
            Err(e) => {
                eprintln!("Adapter error: {:?}", e);
                return;
            }
        };

        let tree = match self.validator.validate(tags) {
            Ok(tree) => tree,
            Err(e) => {
                eprintln!("Validation error: {:?}", e);
                return;
            }
        };

        if is_next_page && tree.source.history_enabled {
            let mut history = self.history.borrow_mut();
            history.push(HistoryItem {
                page: html.to_string(),
                is_main_page,
            });
            // println!("is_next_page : {:?}", history);
            // println!("is_next_page.len : {:?}", history.len());
            drop(history);
        }

        let body_content = tree.source.body.content.clone();

        self.renderer.render(RenderParams {
            tree,
            is_main_page,
            on_input: Box::new(move |user_input| {
                if user_input == BACK_KEY && !is_main_page {
                    self.go_back();
                    return;
                }

                if user_input == HOME_KEY && !is_main_page {
                    self.go_to_main_page();
                    return;
                }

                match &body_content {
                    BodyContent::Links(links) => {
                        if let Ok(index) = user_input.parse::<usize>() {
                            if index == 0 || index > links.len() {
                                println!("Invalid input: selected link index is out of bounds");
                                return;
                            }
                            let option_next_link = links.get(index - 1);
                            if option_next_link.is_none() {
                                println!("Invalid input: invalid link index");
                                return;
                            }
                            let next_link = option_next_link.unwrap();

                            if next_link.href.href_type == HrefType::File {
                                match self.get_file(&next_link.href.url) {
                                    Ok(next_html) => {
                                        self.display(DisplayParams {
                                            html: next_html.clone(),
                                            is_main_page: false,
                                            is_next_page: true,
                                        });
                                        return;
                                    }
                                    Err(err) => {
                                        println!("{}", err);
                                        return;
                                    }
                                }
                            } else {
                                self.handle_response(get(&next_link.href.url));
                                return;
                            }
                        }

                        println!("Invalid input: expected a numeric value");
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
                            println!("Invalid form input: please enter a valid value");
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
                is_next_page: true,
            });
        } else {
            drop(history);
            // println!("No previous page");
            self.display(DisplayParams {
                html: self.main_page.clone(),
                is_main_page: true,
                is_next_page: true,
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
            is_next_page: true,
        });
    }

    fn handle_response(&self, response: Result<Response, Error>) {
        match handle_result_response(response) {
            Ok(html) => {
                self.display(DisplayParams {
                    html: html.clone(),
                    is_main_page: false,
                    is_next_page: true,
                });
            }
            Err(err) => {
                println!("{:}", err);
            }
        }
    }

    pub fn get_file(&self, file_path: &str) -> Result<String, String> {
        if let Some(next_html) = self.pages.get(file_path) {
            return Ok(next_html.to_string());
        }
        let final_path = if let Some(base) = &self.base_dir {
            base.join(file_path)
        } else {
            PathBuf::from(file_path)
        };
        load_file(final_path.to_str().unwrap())
    }
}
