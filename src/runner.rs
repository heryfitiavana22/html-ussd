use std::{collections::HashMap, path::PathBuf};

use crate::{
    adapter::adapter_trait::TagAdapter,
    helper::{fetch_html, is_server_url, load_file},
    i18n::{Lang, init_i18n},
    renderer::renderer_trait::Renderer,
    ussd_controller::{NewController, UssdController},
    validator_and_transformer::ValidatorAndTransformer,
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Language to use (en, fr, mg)
    #[arg(short, long, default_value = "en")]
    lang: String,

    /// Disable caching when loading files from disk or server.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_cache: bool,

    /// The starting URL or main file name (e.g. http://localhost:8888 or index.html)
    #[arg(short, long)]
    main: String,
}

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

        let lang = match cli.lang.as_str() {
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

        let main_str = cli.main.as_str();
        if is_server_url(main_str) {
            match fetch_html(main_str) {
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
            adapter: self.tag_adapter.clone(),
            renderer: self.renderer.clone(),
            validator: ValidatorAndTransformer,
            base_dir,
            use_cache: !cli.no_cache,
        });
        controller.run();
    }
}
