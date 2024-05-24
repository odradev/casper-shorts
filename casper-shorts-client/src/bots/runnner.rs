use std::{thread, time::Duration};

use odra::host::HostEnv;

use crate::{actions, deployed_contracts::DeployedContracts, log, models::SystemStats};

use super::strategy::Strategy;

pub struct Runner<T: Strategy> {
    strategy: T,
}

impl<T: Strategy> Runner<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }

    pub fn run_once(&mut self) {
        log::info(format!("Time: {}", chrono::Utc::now()));
        let context = RunnerContext::new();
        let action = self.strategy.run_step(context);

        if action.is_none() {
            log::info("No action.");
            return;
        }

        let action = action.unwrap();
        log::info(format!("Action: {:?}", action));
        let transfer = action.to_transfer_order();
        actions::make_transfer(transfer);
    }

    pub fn run_forever(&mut self, interval: Duration) {
        loop {
            self.run_once();
            log::info(format!("Sleeping for {:?}", interval));
            thread::sleep(interval);
        }
    }
}

pub struct RunnerContext {
    pub stats: SystemStats,
    pub contracts: DeployedContracts,
    pub env: HostEnv,
}

impl RunnerContext {
    pub fn new() -> Self {
        let env = odra_casper_livenet_env::env();
        let contracts = DeployedContracts::load(env.clone());
        let stats = actions::get_stats(&env, &contracts);
        Self {
            stats,
            contracts,
            env,
        }
    }
}
