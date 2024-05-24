use crate::actions;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "casper-shorts-client")]
#[command(about = "Casper Shorts Client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Deploys all DAO contracts
    DeployContracts,
    /// Configures whitelists of all contracts
    SetConfig,
    /// Update price.
    UpdatePrice {
        #[arg(short, long)]
        dry_run: bool,
    },
    UpdatePriceDeamon {
        interval_mintues: u64,
    },
    PrintBalances,
    GoLong,
}

pub fn parse() {
    match Cli::parse().command {
        Commands::DeployContracts => actions::deploy_all(),
        Commands::SetConfig => actions::set_config(),
        Commands::UpdatePrice { dry_run } => actions::update_price(dry_run),
        Commands::UpdatePriceDeamon { interval_mintues } => {
            actions::update_price_deamon(interval_mintues)
        }
        Commands::PrintBalances => actions::print_balances(),
        Commands::GoLong => actions::go_long(1_000_000_000.into()),
    }
}
