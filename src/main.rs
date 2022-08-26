mod crawl;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lookup International BBC news
    News,
    /// Lookup this week's European and American music charts
    Music,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let _cli = Cli::parse();

    match &cli.command {
        Commands::News => {
            crawl::news::parse_news()
        }
        Commands::Music => {
            crawl::music::parse_music()
        }
    }

    Ok(())
}