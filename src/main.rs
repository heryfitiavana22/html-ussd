use std::{collections::HashMap, fs};

use helper::fetch_html;
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

        /// List of file names (e.g. page2.html page3.html)
        #[arg()]
        files: Vec<String>,
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

    let mut pages = HashMap::new();
    let main_page;

    match cli.mode {
        StartMode::Url { url } => match fetch_html(url.as_str()) {
            Ok(html) => {
                main_page = html;
            }
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        },

        StartMode::Files { main, files } => {
            for file in &files {
                if let Ok(content) = fs::read_to_string(file) {
                    pages.insert(file.to_string(), content);
                } else {
                    eprintln!("Failed to read file: {}", file);
                    return;
                }
            }

            main_page = fs::read_to_string(&main).expect("Cannot read main file");
        }
    }

    let controller = UssdController::new(NewController {
        main_page,
        pages,
        adapter: DomTreeAdapter,
        renderer: TerminalRenderer,
        validator: ValidatorAndTransformer,
    });
    controller.run();
}
