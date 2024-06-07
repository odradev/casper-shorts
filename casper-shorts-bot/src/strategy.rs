use super::runnner::RunnerContext;
use casper_shorts_client::models::TradingAction;

pub trait Strategy {
    fn run_step(&self, ctx: &RunnerContext) -> Option<TradingAction>;
}
