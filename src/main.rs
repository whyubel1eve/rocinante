mod crawl;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lookup this week's European and American music charts
    Music,
    /// Lookup CSGO statistic by steam_id
    CS(CS),
    /// Lookup recent trending
    Trending {
        #[clap(long, action)]
        weibo: bool,
        #[clap(long, action)]
        zhihu: bool,
    }
}

#[derive(Args)]
#[clap(args_conflicts_with_subcommands = true)]
struct CS {
    #[clap(subcommand)]
    command: CSCommands,
}

#[derive(Subcommand)]
enum CSCommands {
    /// Lookup your csgo official statistic
    Stat {
        #[clap(short, long, value_parser, default_value_t = String::from("76561198446269449"))]
        id: String,
    },
    /// CSGO World ranking
    Ranking,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Music => {
            crawl::music::parse_music().await?
        }
        Commands::CS(cs) => {
            let cs_cmd = &cs.command;
            match cs_cmd {
                CSCommands::Stat { id } => {
                    crawl::cs::parse_cs_stat(id).await?
                }
                CSCommands::Ranking => {
                    crawl::cs::parse_cs_ranking().await?
                }
            }
        }
        Commands::Trending {weibo, zhihu}  => {
            crawl::trending::parse_trending(*weibo, *zhihu).await?
        }
    };

    Ok(())
}