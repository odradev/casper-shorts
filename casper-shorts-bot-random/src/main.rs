use casper_shorts_bot::{run_bot, RunnerContext, RunnerMode, Strategy, TradingAction, U256};
use rand::Rng;

pub struct RandomTrader;

impl Strategy for RandomTrader {
    fn run_step(&self, ctx: &dyn RunnerContext) -> Option<TradingAction> {
        let mut options = vec![];
        let stats = ctx.stats();

        // If has wcspr balance, go long or short.
        if !stats.wcspr_balance.is_zero() {
            options.push(TradingAction::GoLong {
                amount: random(stats.wcspr_balance),
            });
            options.push(TradingAction::GoShort {
                amount: random(stats.wcspr_balance),
            });
        }

        // If has long balance, stop long.
        if !stats.long_balance.is_zero() {
            options.push(TradingAction::StopLong {
                amount: random(stats.long_balance),
            });
        }

        // If has short balance, stop short.
        if !stats.short_balance.is_zero() {
            options.push(TradingAction::StopShort {
                amount: random(stats.short_balance),
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

fn interval_arg() -> Option<u64> {
    casper_shorts_bot::arg(1)
}

fn main() {
    run_bot(Box::new(RandomTrader), interval_arg(), RunnerMode::Live)
}
