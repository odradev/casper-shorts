use std::{thread, time::Duration};

use casper_shorts_client::{
    actions, deployed_contracts::DeployedContracts, log, models::SystemStats,
};
use casper_shorts_contracts::market::MarketHostRef;
use odra::host::HostEnv;

use super::strategy::Strategy;

pub struct Runner {
    strategy: Box<dyn Strategy>,
    ctx: RunnerContext,
}

impl Runner {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Self {
            strategy,
            ctx: RunnerContext::new(),
        }
    }

    pub fn run_once(&mut self) {
        self.ctx.refresh();
        log::info(format!("Time: {}", chrono::Utc::now()));
        let action = self.strategy.run_step(&self.ctx);

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
    env: HostEnv,
    stats: SystemStats,
    contracts: DeployedContracts,
}

impl RunnerContext {
    pub fn new() -> Self {
        let env = odra_casper_livenet_env::env();
        let contracts = DeployedContracts::load(&env);
        let stats = actions::get_stats(&env, &contracts);
        Self {
            env,
            stats,
            contracts,
        }
    }

    pub fn refresh(&mut self) {
        self.stats = actions::get_stats(&self.env, &self.contracts);
    }

    pub fn stats(&self) -> &SystemStats {
        &self.stats
    }

    pub fn market_ref(&self) -> &MarketHostRef {
        &self.contracts.market
    }
}
