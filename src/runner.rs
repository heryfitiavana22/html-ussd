use std::{collections::HashMap, path::PathBuf, str::FromStr};

use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::{
    adapter::adapter_trait::TagAdapter,
    command::{Cli, Commands},
    helper::{is_server_url, load_file, parse_key_value_safe, uninstall_self},
    http_client::HttpClient,
    i18n::{Lang, init_i18n},
    renderer::renderer_trait::Renderer,
    ussd_controller::{NewController, UssdController},
    validator_and_transformer::ValidatorAndTransformer,
};

pub struct Runner<R: Renderer + Clone, T: TagAdapter + Clone> {
    tag_adapter: T,
    renderer: R,
}

impl<R: Renderer + Clone, T: TagAdapter + Clone> Runner<R, T> {
    pub fn new(tag_adapter: T, renderer: R) -> Self {
        Self {
            tag_adapter,
            renderer,
        }
    }

    pub fn run(&self) {
        let cli = Cli::parse();

        match cli.command {
            Commands::Run {
                lang,
                no_cache,
                main,
                phone,
                query,
                header,
                access_token,
            } => {
                // println!("Queries: {:?}", query);
                // println!("Queries: {:?}", header);

                let default_query = match parse_key_value_safe(&query) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Error parsing queries: {}", e);
                        std::process::exit(1);
                    }
                };

                let mut mut_header = header;

                if let Some(token) = &access_token {
                    mut_header.push(format!("Authorization=Bearer {}", token));
                }

                let default_header = match parse_key_value_safe(&mut_header) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Error parsing headers: {}", e);
                        std::process::exit(1);
                    }
                };

                let mut header = HeaderMap::new();

                for (key, value) in default_header {
                    if let (Ok(header_name), Ok(header_value)) = (
                        HeaderName::from_str(key.as_str()),
                        HeaderValue::from_str(value.as_str()),
                    ) {
                        header.insert(header_name, header_value);
                    } else {
                        panic!("Invalid header key or value: {} -> {}", key, value);
                    }
                }

                let lang = match lang.as_str() {
                    "fr" => Lang::Fr,
                    "mg" => Lang::Mg,
                    "en" => Lang::En,
                    _ => {
                        eprintln!("Invalid language. Supported languages: fr, mg, en");
                        return;
                    }
                };
                init_i18n(lang);

                let main_page;
                let base_dir;

                let param_phone = "phone".to_string();
                let param_phone_value = phone.clone();
                let default_request_data = vec![(param_phone, param_phone_value)];
                let http_client = HttpClient::new(default_query, header);

                let main_str = main.as_str();
                if is_server_url(main_str) {
                    match http_client.get(main_str, default_request_data.clone()) {
                        Ok(html) => {
                            main_page = html;
                            base_dir = None;
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                } else {
                    main_page = match load_file(main_str) {
                        Ok(content) => content,
                        Err(err) => {
                            eprintln!("{}", err);
                            return;
                        }
                    };

                    base_dir = PathBuf::from(main_str)
                        .parent()
                        .map(|p| p.to_path_buf())
                        .filter(|p| !p.as_os_str().is_empty());
                }

                // println!("Base dir: {:?}", base_dir);

                let controller = UssdController::new(NewController {
                    main_page,
                    cache_pages: HashMap::new(),
                    tag_adapter: self.tag_adapter.clone(),
                    renderer: self.renderer.clone(),
                    validator: ValidatorAndTransformer,
                    base_dir,
                    use_cache: !no_cache,
                    phone: phone,
                    default_request_data,
                    http_client,
                });
                controller.run();
            }
            Commands::Uninstall {} => {
                uninstall_self();
            }
        }
    }
}
