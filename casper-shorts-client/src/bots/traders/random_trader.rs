use odra::casper_types::U256;

use crate::{
    bots::{runnner::RunnerContext, strategy::Strategy},
    models::TradingAction,
};
use rand::Rng;
pub struct RandomTrader;

impl RandomTrader {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for RandomTrader {
    fn run_step(&self, ctx: RunnerContext) -> Option<TradingAction> {
        let mut options = vec![];

        // If has wcspr balance, go long or short.
        if !ctx.stats.wcspr_balance.is_zero() {
            options.push(TradingAction::GoLong {
                amount: random(ctx.stats.wcspr_balance),
            });
            options.push(TradingAction::GoShort {
                amount: random(ctx.stats.wcspr_balance),
            });
        }

        // If has long balance, stop long.
        if !ctx.stats.long_balance.is_zero() {
            options.push(TradingAction::StopLong {
                amount: random(ctx.stats.long_balance),
            });
        }

        // If has short balance, stop short.
        if !ctx.stats.short_balance.is_zero() {
            options.push(TradingAction::StopShort {
                amount: random(ctx.stats.short_balance),
            });
        }

        if options.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let number = rng.gen_range(0..options.len());
            Some(options[number].clone())
        }
    }
}

fn random(max: U256) -> U256 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..max.as_u128());
    U256::from(number)
}
