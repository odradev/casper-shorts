use crate::{actions, price::CoinmarketcapProvider};
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
    UpdatePriceDaemon {
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
    let env = odra_casper_livenet_env::env();
    match Cli::parse().command {
        Commands::DeployContracts => actions::deploy_all(&env),
        Commands::SetConfig => actions::set_config(&env),
        Commands::UpdatePrice { dry_run } => {
            actions::update_price::<CoinmarketcapProvider>(&env, dry_run)
        }
        Commands::UpdatePriceDaemon { interval_seconds } => {
            actions::update_price_daemon::<CoinmarketcapProvider>(&env, duration(interval_seconds))
        }
        Commands::PrintBalances => actions::print_balances(&env),
        Commands::GoLong => actions::go_long(&env, 1_000_000_000.into()),
        Commands::TransferWCSPR { .. } => {
            panic!("Not implemented")
        }
        Commands::PrintStats => actions::print_stats(&env),
    }
}

fn duration(seconds: Option<u64>) -> Option<std::time::Duration> {
    seconds.map(|s| std::time::Duration::from_secs(s))
}
