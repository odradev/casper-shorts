use std::thread;
use std::time::Duration;

use casper_shorts_contracts::address_pack::AddressPack;
use casper_shorts_contracts::market::{MarketHostRef, MarketInitArgs};
use casper_shorts_contracts::price_data::PriceData;
use casper_shorts_contracts::system::{ONE_CENT, ONE_DOLLAR};
use casper_shorts_contracts::token_long::{TokenLongHostRef, TokenLongInitArgs};
use casper_shorts_contracts::token_short::{TokenShortHostRef, TokenShortInitArgs};
use casper_shorts_contracts::token_wcspr::{TokenWCSPRHostRef, TokenWCSPRInitArgs};
use odra::casper_types::U256;
use odra::host::HostRef;
use odra::host::{Deployer, HostEnv};

use crate::bots::runnner::Runner;
use crate::bots::traders::random_trader::RandomTrader;
use crate::deployed_contracts::{DeployedContracts, DeployedContractsToml};
use crate::models::{BotMode, Recipient, SystemStats, Token, TransferOrder};
use crate::{coinmarketcap, log};

pub fn deploy_all() {
    DeployedContractsToml::handle_previous_version();
    let mut contracts = DeployedContractsToml::new();
    let env = odra_casper_livenet_env::env();

    env.set_gas(300_000_000_000);
    let wcspr_token = TokenWCSPRHostRef::deploy(
        &env,
        TokenWCSPRInitArgs {
            name: "002_CS_WCSPR".to_string(),
            symbol: "002_CS_WCSPR".to_string(),
            decimals: 9,
            initial_supply: 1_000_000_000_000_000u64.into(),
        },
    );
    contracts.add_contract("WCSPR", wcspr_token.address());

    env.set_gas(300_000_000_000);
    let short_token = TokenShortHostRef::deploy(
        &env,
        TokenShortInitArgs {
            name: "002_CS_SHORT".to_string(),
            symbol: "002_CS_SHORT".to_string(),
            decimals: 9,
            initial_supply: 0u64.into(),
        },
    );
    contracts.add_contract("SHORT", short_token.address());

    env.set_gas(300_000_000_000);
    let long_token = TokenLongHostRef::deploy(
        &env,
        TokenLongInitArgs {
            name: "002_CS_LONG".to_string(),
            symbol: "002_CS_LONG".to_string(),
            decimals: 9,
            initial_supply: 0u64.into(),
        },
    );
    contracts.add_contract("LONG", long_token.address());

    env.set_gas(300_000_000_000);
    let market = MarketHostRef::deploy(
        &env,
        MarketInitArgs {
            last_price: PriceData {
                price: ONE_CENT.into(),
                timestamp: 0u64.into(),
            },
        },
    );
    contracts.add_contract("Market", market.address());
}

pub fn set_config() {
    let env = odra_casper_livenet_env::env();
    let mut contracts = DeployedContracts::load(env.clone());

    // Make market minter of LONG and SHORT tokens.
    env.set_gas(10_000_000_000);
    contracts
        .short_token
        .change_security(vec![], vec![contracts.market.address().clone()], vec![]);

    env.set_gas(10_000_000_000);
    contracts
        .long_token
        .change_security(vec![], vec![contracts.market.address().clone()], vec![]);

    let address_pack = AddressPack {
        wcspr_token: contracts.wcspr_token.address().clone(),
        short_token: contracts.short_token.address().clone(),
        long_token: contracts.long_token.address().clone(),
        market: contracts.market.address().clone(),
        fee_collector: env.get_account(0),
    };

    contracts.market.set_addres_pack(address_pack.clone());
    contracts.long_token.set_address_pack(address_pack.clone());
    contracts.short_token.set_address_pack(address_pack.clone());
    contracts.wcspr_token.set_address_pack(address_pack.clone());
}

pub fn update_price(dry_run: bool) {
    let env = odra_casper_livenet_env::env();
    let mut contracts = DeployedContracts::load(env.clone());

    // Print time.
    log::info(format!("Time: {}", chrono::Utc::now()));

    let new_price = coinmarketcap::get_cspr_price().unwrap();
    log::info(format!("CMC price: {} CSPR/USD", new_price));

    let current_price = contracts.market.get_market_state().price;
    log::info(format!("Contract price: 0.0{} CSPR/USD", current_price));

    if dry_run {
        return;
    }

    let new_price = new_price * ONE_DOLLAR as f64;
    let new_price = new_price.round() as u64;
    let new_price = U256::from(new_price);

    if new_price == current_price {
        log::info("Price is the same, no need to update.");
        return;
    }

    env.set_gas(300_000_000);
    contracts.market.set_price(PriceData {
        price: new_price,
        timestamp: 0,
    });

    let current_price = contracts.market.get_market_state().price;
    log::info(format!("New contract price: 0.0{} CSPR/USD", current_price));
}

pub fn update_price_deamon(interval: Option<Duration>) {
    loop {
        update_price(false);
        if let Some(interval) = interval {
            log::info(format!("Sleeping for {:?}", interval));
            thread::sleep(interval);
        } else {
            break;
        }
    }
}

pub fn print_balances() {
    let env = odra_casper_livenet_env::env();
    let contracts = DeployedContracts::load(env.clone());

    log::info("Balances:");
    log::info(format!(
        "WCSPR: {}",
        contracts.wcspr_token.balance_of(&env.get_account(0))
    ));
    log::info(format!(
        "SHORT: {}",
        contracts.short_token.balance_of(&env.get_account(0))
    ));
    log::info(format!(
        "LONG: {}",
        contracts.long_token.balance_of(&env.get_account(0))
    ));
}

pub fn go_long(amount: U256) {
    let env = odra_casper_livenet_env::env();
    let mut contracts = DeployedContracts::load(env.clone());
    env.set_gas(10_000_000_000);
    contracts
        .wcspr_token
        .transfer(contracts.long_token.address(), &amount)
    // contracts.market.deposit_long(amount);
}

pub fn make_transfer(order: TransferOrder) {
    let env = odra_casper_livenet_env::env();
    let mut contracts = DeployedContracts::load(env.clone());
    let recipient = match order.recipient {
        Recipient::WcsprContract => contracts.wcspr_token.address().clone(),
        Recipient::ShortContract => contracts.short_token.address().clone(),
        Recipient::LongContract => contracts.long_token.address().clone(),
        Recipient::Address(address) => address,
    };
    let amount = order.amount;
    log::info(format!(
        "Transfering {} {:?} to {:?}",
        amount, order.token, recipient
    ));
    env.set_gas(10_000_000_000);
    match order.token {
        Token::Long => contracts.long_token.transfer(&recipient, &amount),
        Token::Short => contracts.short_token.transfer(&recipient, &amount),
        Token::Wcspr => contracts.wcspr_token.transfer(&recipient, &amount),
    }
}

pub fn run_bot(mode: BotMode, interval: Option<Duration>) {
    let mut runner = {
        match mode {
            BotMode::Random => {
                let trader = RandomTrader::new();
                Runner::new(trader)
            }
        }
    };

    if let Some(interval) = interval {
        runner.run_forever(interval);
    } else {
        runner.run_once();
    }
}

pub fn get_stats(env: &HostEnv, contracts: &DeployedContracts) -> SystemStats {
    log::info("Loading stats. Might take a while...");
    let account = env.get_account(0);
    let wcspr_balance = contracts.wcspr_token.balance_of(&account);
    let short_balance = contracts.short_token.balance_of(&account);
    let long_balance = contracts.long_token.balance_of(&account);
    let market_state = contracts.market.get_market_state();

    SystemStats {
        account,
        wcspr_balance,
        short_balance,
        long_balance,
        market_state,
    }
}

pub fn print_stats() {
    let env = odra_casper_livenet_env::env();
    let contracts = DeployedContracts::load(env.clone());
    let stats = get_stats(&env, &contracts);

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
