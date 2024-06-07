mod runnner;
mod strategy;

use runnner::Runner;

// re-export
pub use casper_shorts_client::models::TradingAction;
pub use odra::casper_types::U256;
pub use runnner::RunnerContext;
pub use strategy::Strategy;

pub fn run_bot(strategy: Box<dyn Strategy>, interval_seconds: Option<u64>) {
    let interval = duration(interval_seconds);
    let mut runner = Runner::new(strategy);
    if let Some(interval) = interval {
        runner.run_forever(interval);
    } else {
        runner.run_once();
    }
}

fn duration(seconds: Option<u64>) -> Option<std::time::Duration> {
    seconds.map(|s| std::time::Duration::from_secs(s))
}
