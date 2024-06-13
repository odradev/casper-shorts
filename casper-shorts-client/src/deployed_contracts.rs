use std::{fs::File, io::Write, str::FromStr};

use casper_shorts_contracts::{
    config::Config, events::OnPriceChange, market::MarketHostRef, price_data::PriceData,
    system::MarketState, token_long::TokenLongHostRef, token_short::TokenShortHostRef,
    token_wcspr::TokenWCSPRHostRef,
};
use chrono::{DateTime, SecondsFormat, Utc};
use odra::{
    casper_types::U256,
    contract_def::HasIdent,
    host::{HostEnv, HostRef, HostRefLoader},
    Address, Addressable, EventError,
};
use serde_derive::{Deserialize, Serialize};

use crate::log;

const DEPLOYED_CONTRACTS_FILE: &str = "casper-shorts-client/resources/deployed_contracts.toml";

/// This struct represents a contract in the `deployed_contracts.toml` file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeployedContractsToml {
    pub time: String,
    pub contracts: Vec<Contract>,
}

impl DeployedContractsToml {
    /// Create new instance.
    pub fn new() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            time: now.to_rfc3339_opts(SecondsFormat::Secs, true),
            contracts: Vec::new(),
        }
    }

    /// Return creation time.
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Add contract to the list.
    pub fn add_contract<T: HostRef + HasIdent>(&mut self, contract: &T) {
        self.contracts.push(Contract {
            name: T::ident(),
            package_hash: contract.address().to_string(),
        });
        self.update();
    }

    /// Return contract address.
    pub fn address<T: HasIdent>(&self) -> Option<Address> {
        self.contracts
            .iter()
            .find(|c| c.name == T::ident())
            .map(|c| Address::from_str(&c.package_hash).unwrap())
    }

    /// Update the file.
    pub fn update(&self) {
        self.save_at(DEPLOYED_CONTRACTS_FILE);
    }

    /// Save the file at the given path.
    pub fn save_at(&self, file_name: &str) {
        let content = toml::to_string_pretty(&self).unwrap();
        let mut file = File::create(file_name).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    /// Load from the file.
    pub fn load() -> Option<Self> {
        std::fs::read_to_string(DEPLOYED_CONTRACTS_FILE)
            .ok()
            .map(|s| toml::from_str(&s).unwrap())
    }

    /// Backup previous version of the file.
    pub fn handle_previous_version() {
        if let Some(deployed_contracts) = Self::load() {
            // Build new file name.
            let date = deployed_contracts.time();
            let path = format!("{}.{}", DEPLOYED_CONTRACTS_FILE, date);

            // Store previous version under new file name.
            deployed_contracts.save_at(&path);

            // Remove old file.
            std::fs::remove_file(DEPLOYED_CONTRACTS_FILE).unwrap();
        }
    }
}

/// This struct represents a contract in the `deployed_contracts.toml` file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub package_hash: String,
}

pub struct DeployedContracts {
    env: HostEnv,
    wcspr_token: TokenWCSPRHostRef,
    short_token: TokenShortHostRef,
    long_token: TokenLongHostRef,
    market: MarketHostRef,
}

impl DeployedContracts {
    pub fn new(
        env: &HostEnv,
        wcspr_token: TokenWCSPRHostRef,
        short_token: TokenShortHostRef,
        long_token: TokenLongHostRef,
        market: MarketHostRef,
    ) -> Self {
        Self {
            env: env.clone(),
            wcspr_token,
            short_token,
            long_token,
            market,
        }
    }

    pub fn load(env: &HostEnv) -> Self {
        let contracts = DeployedContractsToml::load().unwrap();
        Self {
            env: env.clone(),
            wcspr_token: TokenWCSPRHostRef::load(
                env,
                contracts.address::<TokenWCSPRHostRef>().unwrap(),
            ),
            short_token: TokenShortHostRef::load(
                env,
                contracts.address::<TokenShortHostRef>().unwrap(),
            ),
            long_token: TokenLongHostRef::load(
                env,
                contracts.address::<TokenLongHostRef>().unwrap(),
            ),
            market: MarketHostRef::load(env, contracts.address::<MarketHostRef>().unwrap()),
        }
    }

    pub fn transfer_long(&mut self, recipient: &Address, amount: &U256) {
        self.env.set_gas(10_000_000_000);
        self.long_token.transfer(recipient, amount);
    }

    pub fn transfer_short(&mut self, recipient: &Address, amount: &U256) {
        self.env.set_gas(10_000_000_000);
        self.short_token.transfer(recipient, amount);
    }

    pub fn transfer_wcspr(&mut self, recipient: &Address, amount: &U256) {
        self.env.set_gas(10_000_000_000);
        self.wcspr_token.transfer(recipient, amount);
    }

    pub fn long_address(&self) -> Address {
        *HostRef::address(&self.long_token)
    }

    pub fn short_address(&self) -> Address {
        *HostRef::address(&self.short_token)
    }

    pub fn wcspr_address(&self) -> Address {
        *HostRef::address(&self.wcspr_token)
    }

    pub fn market_address(&self) -> Address {
        *HostRef::address(&self.market)
    }

    pub fn wcspr_balance(&self, account: &Address) -> U256 {
        self.wcspr_token.balance_of(account)
    }

    pub fn short_balance(&self, account: &Address) -> U256 {
        self.short_token.balance_of(account)
    }

    pub fn long_balance(&self, account: &Address) -> U256 {
        self.long_token.balance_of(account)
    }

    pub fn get_market_state(&self) -> MarketState {
        self.market.get_market_state()
    }

    pub fn set_short_minter(&mut self, minter: Address) {
        self.env.set_gas(10_000_000_000);
        self.short_token
            .change_security(vec![], vec![minter], vec![]);
    }

    pub fn set_long_minter(&mut self, minter: Address) {
        self.env.set_gas(10_000_000_000);
        self.long_token
            .change_security(vec![], vec![minter], vec![]);
    }

    pub fn set_config(&mut self, cfg: &Config) {
        self.market.set_config(cfg);
        self.long_token.set_config(cfg);
        self.short_token.set_config(cfg);
        self.wcspr_token.set_config(cfg);
    }

    pub fn set_price(&mut self, price: U256) {
        let price_data = PriceData {
            price,
            timestamp: self.env.block_time(),
        };
        self.env.set_gas(400_000_000);
        self.market.set_price(price_data);
    }

    pub fn get_account(&self, index: usize) -> Address {
        self.env.get_account(index)
    }

    pub fn get_prices(&self, start: u32) -> (Vec<PriceData>, u32) {
        log::info("Loading prices...");
        let end = self.env.events_count(&self.market);

        let count = end - start;
        (get_prices(&self.env, &self.market, start, end), count)
    }
}

trait PriceDataProvider {
    fn get_price_data<T: Addressable>(
        &self,
        addressable: &T,
        idx: i32,
    ) -> Result<PriceData, EventError>;
}

impl PriceDataProvider for HostEnv {
    fn get_price_data<T: Addressable>(
        &self,
        addressable: &T,
        idx: i32,
    ) -> Result<PriceData, EventError> {
        self.get_event::<OnPriceChange, _>(addressable, idx)
            .map(|e| PriceData {
                price: e.price,
                timestamp: e.timestamp,
            })
    }
}

fn get_prices<T: PriceDataProvider, A: Addressable>(
    provider: &T,
    addressable: &A,
    start: u32,
    end: u32,
) -> Vec<PriceData> {
    (start..end)
        .rev()
        .map(|i| provider.get_price_data(addressable, i as i32))
        .filter_map(|r| r.ok())
        .collect()
}

#[cfg(test)]
mod test {
    use casper_shorts_contracts::price_data::PriceData;
    use odra::{Address, Addressable, EventError, OdraError};

    use super::{get_prices, PriceDataProvider};

    const ADDR: Result<Address, OdraError> =
        Address::new("hash-0000000000000000000000000000000000000000000000000000000000000000");

    #[test]
    fn get_price_data() {
        struct TestProvider;

        impl PriceDataProvider for TestProvider {
            fn get_price_data<T: Addressable>(
                &self,
                _addressable: &T,
                idx: i32,
            ) -> Result<PriceData, EventError> {
                Ok(PriceData {
                    price: idx.into(),
                    timestamp: 0,
                })
            }
        }
        let mut acc_prices = vec![];
        acc_prices.append(&mut get_prices(
            &TestProvider,
            ADDR.as_ref().unwrap(),
            0,
            100,
        ));

        assert!(acc_prices.len() == 100);

        acc_prices.append(&mut get_prices(
            &TestProvider,
            ADDR.as_ref().unwrap(),
            100,
            150,
        ));
        assert!(acc_prices.len() == 150);
    }

    #[test]
    fn get_price_data_with_error() {
        struct TestProvider;

        impl PriceDataProvider for TestProvider {
            fn get_price_data<T: Addressable>(
                &self,
                _addressable: &T,
                idx: i32,
            ) -> Result<PriceData, EventError> {
                if idx % 2 == 0 {
                    Err(EventError::Parsing)
                } else {
                    Ok(PriceData {
                        price: idx.into(),
                        timestamp: 0,
                    })
                }
            }
        }

        let mut acc_prices = vec![];
        acc_prices.append(&mut get_prices(
            &TestProvider,
            ADDR.as_ref().unwrap(),
            100,
            150,
        ));
        acc_prices.append(&mut get_prices(
            &TestProvider,
            ADDR.as_ref().unwrap(),
            0,
            100,
        ));

        assert_eq!(acc_prices.len(), 75);
    }
}
