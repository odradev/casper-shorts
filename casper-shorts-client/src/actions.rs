use std::thread;
use std::time::Duration;

use casper_shorts_contracts::config::Config;
use casper_shorts_contracts::market::{MarketHostRef, MarketInitArgs};
use casper_shorts_contracts::price_data::PriceData;
use casper_shorts_contracts::system::ONE_CENT;
use casper_shorts_contracts::token_long::{TokenLongHostRef, TokenLongInitArgs};
use casper_shorts_contracts::token_short::{TokenShortHostRef, TokenShortInitArgs};
use casper_shorts_contracts::token_wcspr::{TokenWCSPRHostRef, TokenWCSPRInitArgs};
use odra::casper_types::U256;
use odra::contract_def::HasIdent;
use odra::host::{Deployer, HostEnv};
use odra::host::{EntryPointsCallerProvider, HostRef, InitArgs};

use crate::deployed_contracts::{DeployedContracts, DeployedContractsToml};
use crate::log;
use crate::models::{Recipient, SystemStats, Token, TransferOrder};
use crate::price::{HistoricalPriceProvider, PriceProvider};

// Fits the oracle precision.
const PRICE_MULTIPLIER: f64 = 1e9;
const DEFAULT_WASM_DEPLOY_COST: u64 = 300_000_000_000;

pub fn deploy_all(env: &HostEnv) {
    DeployedContractsToml::handle_previous_version();
    let mut contracts = DeployedContractsToml::new();
    deploy_contract::<_, MarketHostRef>(
        env,
        &mut contracts,
        MarketInitArgs {
            last_price: PriceData {
                price: ONE_CENT.into(),
                timestamp: 0u64.into(),
            },
        },
    );
    deploy_contract::<_, TokenWCSPRHostRef>(
        env,
        &mut contracts,
        TokenWCSPRInitArgs {
            name: "004_CS_CSPR".to_string(),
            symbol: "004_CS_CSPR".to_string(),
            decimals: 9,
            initial_supply: 1_000_000_000_000_000u64.into(),
        },
    );
    deploy_contract::<_, TokenShortHostRef>(
        env,
        &mut contracts,
        TokenShortInitArgs {
            name: "004_SHORT".to_string(),
            symbol: "004_SHORT".to_string(),
            decimals: 9,
            initial_supply: 0u64.into(),
        },
    );
    deploy_contract::<_, TokenLongHostRef>(
        env,
        &mut contracts,
        TokenLongInitArgs {
            name: "004_LONG".to_string(),
            symbol: "004_LONG".to_string(),
            decimals: 9,
            initial_supply: 0u64.into(),
        },
    );
}

pub fn set_config(contracts: &mut DeployedContracts) {
    // Make market minter of LONG and SHORT tokens.
    contracts.set_short_minter(contracts.market_address());
    contracts.set_long_minter(contracts.market_address());

    let cfg = Config {
        wcspr_token: contracts.wcspr_address(),
        short_token: contracts.short_address(),
        long_token: contracts.long_address(),
        market: contracts.market_address(),
        fee_collector: contracts.get_account(0),
    };
    contracts.set_config(&cfg);
}

pub fn update_price<T: PriceProvider>(contracts: &mut DeployedContracts, dry_run: bool) {
    // Print time.
    log::info(format!("Time: {}", chrono::Utc::now()));

    let new_price = T::get_cspr_price().unwrap();
    log::info(format!("CMC price: {} CSPR/USD", new_price));

    let current_price = contracts.get_market_state().price;
    log::info(format!("Contract price: 0.0{} CSPR/USD", current_price));

    if dry_run {
        return;
    }

    let integer_value: u64 = (new_price * PRICE_MULTIPLIER).round() as u64;
    let new_price = U256::from(integer_value);

    if new_price == current_price {
        log::info("Price is the same, no need to update.");
        return;
    }
    contracts.set_price(new_price);

    let current_price = contracts.get_market_state().price;
    log::info(format!("New contract price: 0.0{} CSPR/USD", current_price));
}

pub fn update_price_daemon<T: PriceProvider>(
    contracts: &mut DeployedContracts,
    interval: Option<Duration>,
) {
    loop {
        update_price::<T>(contracts, false);
        if let Some(interval) = interval {
            log::info(format!("Sleeping for {:?}", interval));
            thread::sleep(interval);
        } else {
            break;
        }
    }
}

pub fn print_balances(contracts: &mut DeployedContracts) {
    let account = contracts.get_account(0);

    log::info("Balances:");
    log::info(format!("WCSPR: {}", contracts.wcspr_balance(&account)));
    log::info(format!("SHORT: {}", contracts.short_balance(&account)));
    log::info(format!("LONG: {}", contracts.long_balance(&account)));
}

pub fn go_long(contracts: &mut DeployedContracts, amount: U256) {
    contracts.transfer_wcspr(&contracts.long_address(), &amount);
    // contracts.market.deposit_long(amount);
}

pub fn make_transfer(order: TransferOrder, contracts: &mut DeployedContracts) {
    let recipient = match order.recipient {
        Recipient::WcsprContract => contracts.wcspr_address(),
        Recipient::ShortContract => contracts.short_address(),
        Recipient::LongContract => contracts.long_address(),
        Recipient::Address(address) => address,
    };
    let amount = order.amount;
    log::info(format!(
        "Transferring {} {:?} to {:?}",
        amount, order.token, recipient
    ));
    match order.token {
        Token::Long => contracts.transfer_long(&recipient, &amount),
        Token::Short => contracts.transfer_short(&recipient, &amount),
        Token::Wcspr => contracts.transfer_wcspr(&recipient, &amount),
    }
}

pub fn get_historical_cspr_prices<T: HistoricalPriceProvider>() -> Vec<U256> {
    T::get_historical_cspr_price()
        .unwrap_or_default()
        .iter()
        .map(|v| U256::from((v * PRICE_MULTIPLIER).round() as u64))
        .collect()
}

pub fn get_stats(contracts: &DeployedContracts) -> SystemStats {
    log::info("Loading stats. Might take a while...");
    let account = contracts.get_account(0);
    let wcspr_balance = contracts.wcspr_balance(&account);
    let short_balance = contracts.short_balance(&account);
    let long_balance = contracts.long_balance(&account);
    let market_state = contracts.get_market_state();

    SystemStats {
        account,
        wcspr_balance,
        short_balance,
        long_balance,
        market_state,
    }
}

pub fn print_stats(contracts: &DeployedContracts) {
    let stats = get_stats(&contracts);

    log::info("Account Info:");
    log::info(format!("Account: {:?}", stats.account));
    log::info(format!("WCSPR: {}", stats.wcspr_balance));
    log::info(format!("SHORT: {}", stats.short_balance));
    log::info(format!("LONG: {}", stats.long_balance));

    log::info("Market Conditions:");
    log::info(format!("Price: 0.0{} CSPR/USD", stats.market_state.price));
    log::info(format!(
        "Total LONG: {}",
        stats.market_state.long_total_supply
    ));
    log::info(format!(
        "Total SHORT: {}",
        stats.market_state.short_total_supply
    ));
    log::info(format!(
        "Long liquidity: {} WCSPR",
        stats.market_state.long_liquidity
    ));
    log::info(format!(
        "Short liquidity: {} WCSPR",
        stats.market_state.short_liquidity
    ));
}

fn deploy_contract<T: InitArgs, R: HostRef + HasIdent + EntryPointsCallerProvider>(
    env: &HostEnv,
    contracts: &mut DeployedContractsToml,
    init_args: T,
) {
    env.set_gas(DEFAULT_WASM_DEPLOY_COST);
    let contract = R::deploy(env, init_args);
    contracts.add_contract(&contract);
}

pub fn deploy_market_contract(env: &HostEnv, init_args: Option<MarketInitArgs>) -> MarketHostRef {
    let init_args = init_args.unwrap_or_else(|| MarketInitArgs {
        last_price: PriceData {
            price: ONE_CENT.into(),
            timestamp: 0u64.into(),
        },
    });
    MarketHostRef::deploy(env, init_args)
}

pub fn deploy_wcspr_contract(
    env: &HostEnv,
    init_args: Option<TokenWCSPRInitArgs>,
) -> TokenWCSPRHostRef {
    let init_args = init_args.unwrap_or_else(|| TokenWCSPRInitArgs {
        name: "004_CS_CSPR".to_string(),
        symbol: "004_CS_CSPR".to_string(),
        decimals: 9,
        initial_supply: 1_000_000_000_000_000u64.into(),
    });
    TokenWCSPRHostRef::deploy(env, init_args)
}

pub fn deploy_long_token_contract(
    env: &HostEnv,
    init_args: Option<TokenLongInitArgs>,
) -> TokenLongHostRef {
    let init_args = init_args.unwrap_or_else(|| TokenLongInitArgs {
        name: "004_LONG".to_string(),
        symbol: "004_LONG".to_string(),
        decimals: 9,
        initial_supply: 0u64.into(),
    });
    TokenLongHostRef::deploy(env, init_args)
}

pub fn deploy_short_token_contract(
    env: &HostEnv,
    init_args: Option<TokenShortInitArgs>,
) -> TokenShortHostRef {
    let init_args = init_args.unwrap_or_else(|| TokenShortInitArgs {
        name: "004_SHORT".to_string(),
        symbol: "004_SHORT".to_string(),
        decimals: 9,
        initial_supply: 0u64.into(),
    });
    TokenShortHostRef::deploy(env, init_args)
}
