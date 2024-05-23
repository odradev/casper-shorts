use casper_shorts_contracts::cep18::{Cep18HostRef, Cep18InitArgs, Cep18Modality};
use casper_shorts_contracts::market::{MarketHostRef, MarketInitArgs};
use casper_shorts_contracts::price_data::PriceData;
use casper_shorts_contracts::system::{ONE_CENT, ONE_DOLLAR};
use odra::casper_types::U256;
use odra::host::Deployer;
use odra::host::HostRef;

use crate::{coinmarketcap, log};
use crate::deployed_contracts::{DeployedContracts, DeployedContractsToml};

pub fn deploy_all() {
    DeployedContractsToml::handle_previous_version();
    let mut contracts = DeployedContractsToml::new();
    let env = odra_casper_livenet_env::env();

    env.set_gas(300_000_000_000);
    let wcspr_token = Cep18HostRef::deploy(
        &env,
        Cep18InitArgs {
            name: "CS_WCSPR".to_string(),
            symbol: "CS_WCSPR".to_string(),
            decimals: 9,
            initial_supply: 1_000_000_000_000_000u64.into(),
            minter_list: vec![],
            admin_list: vec![],
            modality: Some(Cep18Modality::MintAndBurn),
        },
    );
    contracts.add_contract("WCSPR", wcspr_token.address());

    env.set_gas(300_000_000_000);
    let short_token = Cep18HostRef::deploy(
        &env,
        Cep18InitArgs {
            name: "CS_SHORT".to_string(),
            symbol: "SHORT".to_string(),
            decimals: 9,
            initial_supply: 0u64.into(),
            minter_list: vec![],
            admin_list: vec![],
            modality: Some(Cep18Modality::MintAndBurn),
        },
    );
    contracts.add_contract("SHORT", short_token.address());

    env.set_gas(300_000_000_000);
    let long_token = Cep18HostRef::deploy(
        &env,
        Cep18InitArgs {
            name: "CS_LONG".to_string(),
            symbol: "LONG".to_string(),
            decimals: 9,
            initial_supply: 0u64.into(),
            minter_list: vec![],
            admin_list: vec![],
            modality: Some(Cep18Modality::MintAndBurn),
        },
    );
    contracts.add_contract("LONG", long_token.address());

    env.set_gas(300_000_000_000);
    let market = MarketHostRef::deploy(
        &env,
        MarketInitArgs {
            long_token: long_token.address().clone(),
            short_token: short_token.address().clone(),
            wcspr_token: wcspr_token.address().clone(),
            fee_collector: env.get_account(0),
            last_price: PriceData {
                price: ONE_CENT.into(),
                timestamp: 0u64.into(),
            },
        },
    );
    contracts.add_contract("Market", market.address());
   
}

pub fn set_security() {
    let env = odra_casper_livenet_env::env();
    let mut contracts = DeployedContracts::load(env.clone());

    // Make market minter of LONG and SHORT tokens.
    env.set_gas(10_000_000_000);
    contracts.short_token.change_security(vec![], vec![contracts.market.address().clone()], vec![]);

    env.set_gas(10_000_000_000);
    contracts.long_token.change_security(vec![], vec![contracts.market.address().clone()], vec![]);    
}

pub fn update_price(dry_run: bool) {
    let env = odra_casper_livenet_env::env();
    let mut contracts = DeployedContracts::load(env.clone());

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