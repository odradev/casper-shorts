use casper_shorts_client::{actions, deployed_contracts::DeployedContracts, models::SystemStats, price::FilePriceProvider};
use odra::casper_types::U256;

use crate::{RunnerContext, Strategy};

use super::{live::LiveRunnerContext, step};

pub struct BacktestingRunnerContext {
    ctx: LiveRunnerContext,
    prices: Vec<U256>,
    current_index: usize,
}

impl BacktestingRunnerContext {
    pub fn new() -> Self {
        Self {
            ctx: LiveRunnerContext::new(),
            prices: actions::get_historical_cspr_prices::<FilePriceProvider>(),
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> bool {
        if self.current_index < self.prices.len() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }
}

impl RunnerContext for BacktestingRunnerContext {
    fn stats(&self) -> &SystemStats {
        self.ctx.stats()
    }

    fn refresh_market_state(&mut self) {
        self.ctx.refresh_market_state();
    }

    fn prices(&self) -> &[U256] {
        &self.prices
    }
    
    fn refresh_prices(&mut self) {
        // do nothing
    }

    fn deployed_contracts(&mut self) -> &mut DeployedContracts {
        self.ctx.deployed_contracts()
    }
}

pub struct BacktestingRunner {
    strategy: Box<dyn Strategy>,
    ctx: BacktestingRunnerContext,
}

impl BacktestingRunner {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        let ctx = BacktestingRunnerContext::new();
        Self { strategy, ctx }
    }

    pub fn run(&mut self) {
        while self.ctx.next() {

            step(&self.strategy, &mut self.ctx);
        }
    }
}