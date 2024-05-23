use std::{fs::File, io::Write, str::FromStr};

use casper_shorts_contracts::{cep18::Cep18HostRef, market::MarketHostRef};
use chrono::{DateTime, SecondsFormat, Utc};
use odra::{host::{HostEnv, HostRefLoader}, Address};
use serde_derive::{Deserialize, Serialize};

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
    pub fn add_contract(&mut self, name: &str, contract: &Address) {
        self.contracts.push(Contract {
            name: name.to_string(),
            package_hash: contract.to_string(),
        });
        self.update();
    }

    /// Return contract address.
    pub fn address(&self, name: &str) -> Option<Address> {
        self.contracts
            .iter()
            .find(|c| c.name == name)
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
    pub wcspr_token: Cep18HostRef,
    pub short_token: Cep18HostRef,
    pub long_token: Cep18HostRef,
    pub market: MarketHostRef,
}

impl DeployedContracts {
    pub fn load(env: HostEnv) -> Self {
        let contracts = DeployedContractsToml::load().unwrap();
        Self {
            wcspr_token: Cep18HostRef::load(&env, contracts.address("WCSPR").unwrap()),
            short_token: Cep18HostRef::load(&env, contracts.address("SHORT").unwrap()),
            long_token: Cep18HostRef::load(&env, contracts.address("LONG").unwrap()),
            market: MarketHostRef::load(&env,contracts.address("Market").unwrap()),
        }
    }
}