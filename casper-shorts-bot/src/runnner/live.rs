use super::step;
use crate::{RunnerContext, Strategy};
use casper_shorts_client::{
    actions, deployed_contracts::DeployedContracts, log, models::SystemStats,
};
use odra::casper_types::U256;
use std::{thread, time::Duration};

pub struct Runner {
    strategy: Box<dyn Strategy>,
    ctx: LiveRunnerContext,
}

impl Runner {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Self {
            strategy,
            ctx: LiveRunnerContext::new(),
        }
    }

    pub fn run_once(&mut self) {
        step(&self.strategy, &mut self.ctx);
    }

    pub fn run_forever(&mut self, interval: Duration) {
        loop {
            self.run_once();
            log::info(format!("Sleeping for {:?}", interval));
            thread::sleep(interval);
        }
    }
}

pub struct LiveRunnerContext {
    stats: SystemStats,
    contracts: DeployedContracts,
    prices: Vec<U256>,
    processed_events_count: u32,
}

impl LiveRunnerContext {
    pub fn new() -> Self {
        let env = odra_casper_livenet_env::env();
        let contracts = DeployedContracts::load(&env);
        let stats = actions::get_stats(&contracts);

        Self {
            stats,
            contracts,
            prices: vec![],
            processed_events_count: 0,
        }
    }
}

impl RunnerContext for LiveRunnerContext {
    fn stats(&self) -> &SystemStats {
        &self.stats
    }

    fn refresh_market_state(&mut self) {
        self.stats = actions::get_stats(&self.contracts);
    }

    fn prices(&self) -> &[U256] {
        &self.prices
    }

    fn refresh_prices(&mut self) {
        let start = self.processed_events_count;
        let (prices, count) = self.contracts.get_prices(start);
        self.prices.extend(prices.iter().map(|p| p.price));
        self.processed_events_count += count;

        dbg!(self.prices.len());
        dbg!(self.processed_events_count);
    }

    fn deployed_contracts(&mut self) -> &mut DeployedContracts {
        &mut self.contracts
    }
}
