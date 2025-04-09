use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use crate::{
    adapter::adapter_trait::TagAdapter,
    helper::load_file,
    html::{BodyContent, FormMethod, HrefType, InputType},
    http_client::HttpClient,
    renderer::renderer_trait::{RenderParams, Renderer},
    validator_and_transformer::ValidatorAndTransformer,
};

const BACK_KEY: &str = "0";
const HOME_KEY: &str = "00";

#[derive(Debug, PartialEq, Clone)]
pub struct HistoryItem {
    pub page: String,
    pub is_main_page: bool,
    pub source_url: String,
    pub is_from_form: bool,
}

pub struct UssdController<R: Renderer, T: TagAdapter> {
    pub cache_pages: RefCell<HashMap<String, String>>,
    pub main_page: String,
    pub tag_adapter: T,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
    pub history: RefCell<Vec<HistoryItem>>,
    pub base_dir: Option<PathBuf>,
    pub use_cache: bool,
    pub phone: String,
    pub default_request_data: Vec<(String, String)>,
    pub http_client: HttpClient,
}

pub struct NewController<R: Renderer, T: TagAdapter> {
    pub cache_pages: HashMap<String, String>,
    pub main_page: String,
    pub tag_adapter: T,
    pub validator: ValidatorAndTransformer,
    pub renderer: R,
    pub base_dir: Option<PathBuf>,
    pub use_cache: bool,
    pub phone: String,
    pub default_request_data: Vec<(String, String)>,
    pub http_client: HttpClient,
}

pub struct DisplayParams {
    pub html: String,
    pub is_main_page: bool,
    pub is_next_page: bool,
    pub source_url: String,
    pub is_from_form: bool,
}

impl<R: Renderer, T: TagAdapter> UssdController<R, T> {
    pub fn new(params: NewController<R, T>) -> Self {
        Self {
            cache_pages: RefCell::new(params.cache_pages),
            main_page: params.main_page,
            tag_adapter: params.tag_adapter,
            validator: params.validator,
            renderer: params.renderer,
            history: RefCell::new(vec![]),
            base_dir: params.base_dir,
            use_cache: params.use_cache,
            phone: params.phone,
            default_request_data: params.default_request_data,
            http_client: params.http_client,
        }
    }

    pub fn run(&self) {
        self.display(DisplayParams {
            html: self.main_page.clone(),
            is_main_page: true,
            is_next_page: true,
            source_url: "main".to_string(),
            is_from_form: false,
        });
    }
    pub fn display(&self, params: DisplayParams) {
        let DisplayParams {
            html,
            is_main_page,
            is_next_page,
            source_url,
            is_from_form,
        } = params;

        let tags = match self.tag_adapter.transform(html.as_str()) {
            Ok(tags) => tags,
            Err(e) => {
                self.renderer.render_error(format!("Adapter error: {}", e));
                return;
            }
        };

        let tree = match self.validator.validate(tags) {
            Ok(tree) => tree,
            Err(e) => {
                self.renderer
                    .render_error(format!("Validation error: {}", e));
                return;
            }
        };

        if is_next_page && tree.source.history_enabled {
            let mut history = self.history.borrow_mut();
            history.push(HistoryItem {
                page: html.to_string(),
                is_main_page,
                source_url: source_url.clone(),
                is_from_form,
            });
            // self.renderer.render_text(format!("is_next_page : {:?}", history);
            // self.renderer.render_text(format!("is_next_page.len : {:?}", history.len());
            drop(history);
        }

        let body_content = tree.source.body.content.clone();
        let cache = tree.source.cache;

        if cache && !is_from_form {
            self.set_to_cache(source_url, html.clone());
        }

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
                                self.renderer.render_text(
                                    "Invalid input: selected link index is out of bounds"
                                        .to_string(),
                                );
                                return;
                            }
                            let option_next_link = links.get(index - 1);
                            if option_next_link.is_none() {
                                self.renderer
                                    .render_text("Invalid input: invalid link index".to_string());
                                return;
                            }
                            let next_link = option_next_link.unwrap();

                            if next_link.href.href_type == HrefType::File {
                                self.display_from_file(&next_link.href.url);
                                return;
                            } else {
                                self.display_from_server_url(&next_link.href.url, index);
                                return;
                            }
                        }

                        self.renderer
                            .render_text("Invalid input: expected a numeric value".to_string());
                    }
                    BodyContent::Form(form) => {
                        let valid = match form.input.input_type {
                            InputType::Text => true,
                            InputType::Number => user_input.parse::<f64>().is_ok(),
                        };

                        if valid {
                            // self.renderer.render_text(format!("form data : {}", user_input);
                            let url = &form.action;
                            let param_name = form.input.name.clone();
                            let mut data = vec![(param_name, user_input)];
                            data.extend(self.default_request_data.clone());
                            let get_query = data.clone();

                            let response_result = match form.method {
                                FormMethod::Get => self.http_client.get(url, get_query),
                                FormMethod::Post => self.http_client.post(url, data),
                            };
                            self.display_from_request_result(response_result, url, true);
                        } else {
                            self.renderer.render_text(
                                "Invalid form input: please enter a valid value".to_string(),
                            );
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
        // self.renderer.render_text(format!("go_back : {:?}", history);
        // self.renderer.render_text(format!("go_back.len : {:?}", history.len());

        if let Some(previous) = history.pop() {
            drop(history);
            self.display(DisplayParams {
                html: previous.page,
                is_main_page: previous.is_main_page,
                is_next_page: true,
                source_url: previous.source_url,
                is_from_form: previous.is_from_form,
            });
        } else {
            drop(history);
            // self.renderer.render_text(format!("No previous page");
            self.display(DisplayParams {
                html: self.main_page.clone(),
                is_main_page: true,
                is_next_page: true,
                source_url: "main".to_string(),
                is_from_form: false,
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
            source_url: "main".to_string(),
            is_from_form: false,
        });
    }

    pub fn get_from_cache(&self, key: &str) -> Option<String> {
        let caches = self.cache_pages.borrow();
        let html = caches.get(key);
        html.cloned()
    }

    pub fn set_to_cache(&self, key: String, value: String) {
        if !self.use_cache {
            return;
        }
        // self.renderer.render_text(format!("set_to_cache : {:?}", key));
        let mut caches = self.cache_pages.borrow_mut();
        caches.insert(key, value);
        drop(caches);
    }

    pub fn display_from_request_result(
        &self,
        result: Result<String, String>,
        url: &str,
        is_from_form: bool,
    ) {
        match result {
            Ok(html) => {
                self.display(DisplayParams {
                    html,
                    is_main_page: false,
                    is_next_page: true,
                    source_url: url.to_string(),
                    is_from_form,
                });
            }
            Err(err) => {
                self.renderer.render_text(err);
            }
        }
    }

    pub fn display_from_server_url(&self, url: &str, user_entry: usize) {
        if let Some(cached_html) = self.get_from_cache(url) {
            // self.renderer
            //     .render_text(format!("from cache in display_from_server_url"));

            self.display(DisplayParams {
                html: cached_html.clone(),
                is_main_page: false,
                is_next_page: true,
                source_url: url.to_string(),
                is_from_form: false,
            });
            return;
        }
        let query = vec![("user_entry".to_string(), user_entry.to_string())];
        let result = self.http_client.get(url, query);

        self.display_from_request_result(result, url, false)
    }

    pub fn display_from_file(&self, file_path: &str) {
        if let Some(cached_html) = self.get_from_cache(file_path) {
            // self.renderer.render_text(format!("from cache in display_from_file"));

            self.display(DisplayParams {
                html: cached_html.clone(),
                is_main_page: false,
                is_next_page: true,
                source_url: file_path.to_string(),
                is_from_form: false,
            });
            return;
        }
        match self.get_file(file_path) {
            Ok(html) => {
                self.display(DisplayParams {
                    html,
                    is_main_page: false,
                    is_next_page: true,
                    source_url: file_path.to_string(),
                    is_from_form: false,
                });
            }
            Err(err) => {
                self.renderer.render_text(err);
            }
        }
    }

    pub fn get_file(&self, file_path: &str) -> Result<String, String> {
        let final_path = if let Some(base) = &self.base_dir {
            base.join(file_path)
        } else {
            PathBuf::from(file_path)
        };
        load_file(final_path.to_str().unwrap())
    }
}
