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
        interval_seconds: Option<u64>,
    },
    PrintBalances,
    GoLong,
    PrintStats,
    TransferWCSPR {
        amount: f64,
        recipient: String,
    },
    // #[clap(name = "run-bot", about = "Run bot in a specific mode")]
    // RunBot(RunBot),
}

#[derive(Debug, Parser)]
pub struct RunBot {
    #[structopt(subcommand)]
    pub run_bots_commands: RunBotCommands,
}

#[derive(Debug, Subcommand)]
pub enum RunBotCommands {
    Random {
        #[arg(short, long)]
        interval_seconds: Option<u64>,
    },
}

pub fn parse() {
    match Cli::parse().command {
        Commands::DeployContracts => actions::deploy_all(),
        Commands::SetConfig => actions::set_config(),
        Commands::UpdatePrice { dry_run } => actions::update_price(dry_run),
        Commands::UpdatePriceDeamon { interval_seconds } => {
            actions::update_price_deamon(duration(interval_seconds))
        }
        Commands::PrintBalances => actions::print_balances(),
        Commands::GoLong => actions::go_long(1_000_000_000.into()),
        Commands::TransferWCSPR { .. } => {
            panic!("Not implemented")
        }
        Commands::PrintStats => actions::print_stats(),
    }
}

fn duration(seconds: Option<u64>) -> Option<std::time::Duration> {
    seconds.map(|s| std::time::Duration::from_secs(s))
}
