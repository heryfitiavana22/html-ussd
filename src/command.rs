use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Language to use (en, fr, mg)
    #[arg(short, long, default_value = "en")]
    pub lang: String,

    /// Disable caching when loading files from disk or server.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_cache: bool,

    /// The starting URL or main file name (e.g. http://localhost:8888 or index.html)
    #[arg(short, long)]
    pub main: String,

    /// Phone number to use for the USSD session
    #[arg(long, default_value = "0312345678")]
    pub phone: String,
}
