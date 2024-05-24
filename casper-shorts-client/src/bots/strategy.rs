use super::runnner::RunnerContext;
use crate::models::TradingAction;

pub trait Strategy {
    fn run_step(&self, ctx: RunnerContext) -> Option<TradingAction>;
}
