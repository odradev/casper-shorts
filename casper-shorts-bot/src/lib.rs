mod runnner;
mod strategy;

use runnner::{backtesting::BacktestingRunner, live::Runner};

// re-export
pub use casper_shorts_client::models::TradingAction;
pub use odra::casper_types::U256;
pub use runnner::{RunnerContext, RunnerMode};
pub use strategy::Strategy;

pub fn run_bot(strategy: Box<dyn Strategy>, interval_seconds: Option<u64>, mode: RunnerMode) {
    match mode {
        RunnerMode::Live => {
            let mut runner = Runner::new(strategy);
            match duration(interval_seconds) {
                Some(interval) => runner.run_forever(interval),
                None => runner.run_once(),
            }
        }
        RunnerMode::Backtesting => {
            let mut runner = BacktestingRunner::new(strategy);
            runner.run();
        }
    }
}

pub fn arg<T>(n: usize) -> Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    std::env::args().nth(n).and_then(|s| s.parse().ok())
}

fn duration(seconds: Option<u64>) -> Option<std::time::Duration> {
    seconds.map(|s| std::time::Duration::from_secs(s))
}
