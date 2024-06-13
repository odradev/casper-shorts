use super::strategy::Strategy;
use casper_shorts_client::{
    actions, deployed_contracts::DeployedContracts, log, models::SystemStats,
};
use odra::casper_types::U256;

pub(crate) mod backtesting;
pub(crate) mod live;

pub trait RunnerContext {
    fn stats(&self) -> &SystemStats;
    fn refresh_market_state(&mut self);
    fn refresh_prices(&mut self);
    fn prices(&self) -> &[U256];
    fn deployed_contracts(&mut self) -> &mut DeployedContracts;
}

pub enum RunnerMode {
    Live,
    Backtesting,
}

fn step<T: RunnerContext>(strategy: &Box<dyn Strategy>, ctx: &mut T) {
    ctx.refresh_market_state();
    ctx.refresh_prices();
    log::info(format!("Time: {}", chrono::Utc::now()));
    match strategy.run_step(ctx) {
        Some(action) => {
            log::info(format!("Action: {:?}", action));
            let transfer = action.to_transfer_order();
            actions::make_transfer(transfer, ctx.deployed_contracts());
        }
        None => log::info("No action."),
    }
}
