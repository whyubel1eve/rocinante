mod rocinante;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize};

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
    /// Lookup CSGO statistic
    CS(CS),
    /// Lookup recent trending
    Trending {
        #[clap(long, action)]
        weibo: bool,
        #[clap(long, action)]
        zhihu: bool,
    },
    /// A simple ethereum wallet
    Wallet(Wallet),
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

#[derive(Args)]
#[clap(args_conflicts_with_subcommands = true)]
struct Wallet {
    #[clap(subcommand)]
    command: WalletCommands,
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Lookup balance of specified ethereum address
    Balance {
        #[clap(short, long, value_parser, default_value_t = String::from("0x54A5c80a0a51c2f91241E856D5A8F8F82e3114f1"))]
        address: String,
        #[clap(short, long, value_parser, default_value_t = String::from("main"))]
        network: String,
    },
    /// Create a new account
    New,
    /// Lookup transaction by hash
    Transaction {
        #[clap(short, long, value_parser)]
        id: String,
        #[clap(short, long, value_parser, default_value_t = String::from("main"))]
        network: String,
    }

}

#[derive(Deserialize)]
pub struct Conf {
    eth_network: Api
}
#[derive(Deserialize)]
struct Api {
    main_api: String,
    goerli_api: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufReader::new(OpenOptions::new().read(true).open("config.toml")?);
    let mut conf = String::new();
    buf.read_to_string(&mut conf).unwrap();
    let config: Conf = toml::from_str(&conf).unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Music => {
            rocinante::music::parse_music().await.expect("fail to execute music command")
        }
        Commands::CS(cs) => {
            let cs_cmd = &cs.command;
            match cs_cmd {
                CSCommands::Stat { id } => {
                    rocinante::cs::parse_cs_stat(id).await.expect("fail to execute cs stat command")
                }
                CSCommands::Ranking => {
                    rocinante::cs::parse_cs_ranking().await.expect("fail to execute cs ranking command")
                }
            }
        }
        Commands::Trending {weibo, zhihu}  => {
            rocinante::trending::parse_trending(*weibo, *zhihu).await.expect("fail to execute trending command")
        }
        Commands::Wallet(w) => {
            let w_cmd = &w.command;
            match w_cmd {
                WalletCommands::New => {
                    rocinante::wallet::new().await.expect("fail to execute new wallet command")
                }
                WalletCommands::Balance { address, network} => {
                    rocinante::wallet::balance(address, network, &config).await.expect("fail to execute wallet balance command")
                }
                WalletCommands::Transaction { id, network } => {
                    rocinante::wallet::transaction(id, network, &config).await.expect("fail to execute wallet transaction command")
                }
            }
        }
    }

    Ok(())
}