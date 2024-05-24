use casper_shorts_contracts::address_pack::AddressPack;
use casper_shorts_contracts::market::{MarketHostRef, MarketInitArgs};
use casper_shorts_contracts::price_data::PriceData;
use casper_shorts_contracts::system::{ONE_CENT, ONE_DOLLAR};
use casper_shorts_contracts::token_long::{TokenLongHostRef, TokenLongInitArgs};
use casper_shorts_contracts::token_short::{TokenShortHostRef, TokenShortInitArgs};
use casper_shorts_contracts::token_wcspr::{TokenWCSPRHostRef, TokenWCSPRInitArgs};
use odra::casper_types::U256;
use odra::host::Deployer;
use odra::host::HostRef;

use crate::deployed_contracts::{DeployedContracts, DeployedContractsToml};
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

pub fn update_price_deamon(interval_minutes: u64) {
    loop {
        update_price(false);
        std::thread::sleep(std::time::Duration::from_secs(interval_minutes * 60));
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
    // contracts
    //     .wcspr_token
    //     .transfer(contracts.long_token.address(), &amount)
    contracts.market.deposit_long(amount);
}
