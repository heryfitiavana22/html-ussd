use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the main program
    Run {
        /// Language to use (en, fr, mg)
        #[arg(short, long, default_value = "en")]
        lang: String,

        /// Disable caching when loading files from disk or server.
        #[arg(long, action = clap::ArgAction::SetTrue)]
        no_cache: bool,

        /// The starting URL or main file name (e.g. http://localhost:8888 or index.html)
        #[arg(short, long)]
        main: String,

        /// Phone number to use for the USSD session
        #[arg(long, default_value = "0312345678")]
        phone: String,

        /// Query parameters, e.g. id=1,q=dfa
        #[arg(long, value_delimiter = ',', num_args = 0..)]
        query: Vec<String>,

        /// Headers, e.g. Authorization=Bearer123
        #[arg(long, value_delimiter = ',', num_args = 0..)]
        header: Vec<String>,

        /// Access token to add as Bearer in Authorization header
        #[arg(long)]
        access_token: Option<String>,
    },

    /// Uninstall the html-ussd CLI
    Uninstall,
}
