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

fn main() {
    let _cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    // match &cli.command {
    //     Commands::News => {
    //
    //     }
    // }
}