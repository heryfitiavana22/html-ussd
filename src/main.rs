use std::{collections::HashMap, path::PathBuf};

use helper::{fetch_html, load_file};
use html_ussd::{
    adapter::dom_tree_adapter::DomTreeAdapter,
    i18n::{Lang, init_i18n},
    renderer::terminal_renderer::TerminalRenderer,
    ussd_controller::{NewController, UssdController},
    validator_and_transformer::ValidatorAndTransformer,
};

use clap::{Parser, Subcommand};

pub mod adapter;
pub mod helper;
pub mod html;
pub mod i18n;
pub mod renderer;
pub mod ussd_controller;
pub mod validator_and_transformer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Language to use (en, fr, mg)
    #[arg(short, long, default_value = "en")]
    lang: String,

    /// Disable cache when loading files from local or server
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    no_cache: bool,

    #[command(subcommand)]
    mode: StartMode,
}

#[derive(Subcommand)]
enum StartMode {
    /// Start with a URL
    Url {
        /// The starting URL
        #[arg()]
        url: String,
    },
    /// Start with local files
    Files {
        /// The name of the main file (entry point)
        #[arg(short, long)]
        main: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Langue
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

    match cli.mode {
        StartMode::Url { url } => match fetch_html(url.as_str()) {
            Ok(html) => {
                main_page = html;
                base_dir = None;
            }
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        },

        StartMode::Files { main } => {
            main_page = match load_file(&main) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };

            base_dir = PathBuf::from(&main)
                .parent()
                .map(|p| p.to_path_buf())
                .filter(|p| !p.as_os_str().is_empty());
        }
    }
    // println!("Base dir: {:?}", base_dir);

    let controller = UssdController::new(NewController {
        main_page,
        cache_pages: HashMap::new(),
        adapter: DomTreeAdapter,
        renderer: TerminalRenderer,
        validator: ValidatorAndTransformer,
        base_dir,
        use_cache: !cli.no_cache,
    });
    controller.run();
}
