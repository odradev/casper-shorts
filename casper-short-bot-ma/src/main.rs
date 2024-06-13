use casper_shorts_bot::{RunnerContext, RunnerMode, Strategy, TradingAction, U256};

const SHORT_WINDOW: usize = 10;
const LONG_WINDOW: usize = 50;

pub struct MovingAverageStrategy {
    short_window: usize,
    long_window: usize,
}

impl MovingAverageStrategy {
    pub fn new(short_window: usize, long_window: usize) -> Self {
        Self {
            short_window,
            long_window,
        }
    }

    fn calculate_moving_average(prices: &[U256], window: usize) -> U256 {
        prices.iter().rev().take(window).cloned().sum::<U256>() / U256::from(window)
    }
}

impl Strategy for MovingAverageStrategy {
    fn run_step(&self, ctx: &dyn RunnerContext) -> Option<TradingAction> {
        let prices = ctx.prices();
        if prices.len() < self.long_window {
            return None;
        }

        let short_ma = Self::calculate_moving_average(&prices, self.short_window);
        let long_ma = Self::calculate_moving_average(&prices, self.long_window);

        if short_ma > long_ma {
            Some(TradingAction::GoLong {
                amount: U256::from(1_000_000),
            })
        } else if short_ma < long_ma {
            Some(TradingAction::GoShort {
                amount: U256::from(1_000_000),
            })
        } else {
            None
        }
    }
}

fn interval_arg() -> Option<u64> {
    casper_shorts_bot::arg(1)
}

fn short_window() -> usize {
    casper_shorts_bot::arg(2).unwrap_or(SHORT_WINDOW)
}

fn long_window() -> usize {
    casper_shorts_bot::arg(3).unwrap_or(LONG_WINDOW)
}

fn mode() -> RunnerMode {
    match casper_shorts_bot::arg::<String>(4) {
        Some(mode) if mode == "backtesting" => RunnerMode::Backtesting,
        _ => RunnerMode::Live,
    }
}

fn main() {
    let strategy = Box::new(MovingAverageStrategy::new(short_window(), long_window()));
    casper_shorts_bot::run_bot(strategy, interval_arg(), mode());
}
