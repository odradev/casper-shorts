use casper_shorts::system::{MarketState, ONE_CENT, ONE_DOLLAR, ONE_TENTH_CENT};
use odra::casper_types::U256;
use plotly::{self, common::Mode, Plot, Scatter};

pub fn main() {
    draw_plot_1_price_goes_up();
}

// Draw short and long liquidity over time when price changes.
pub fn draw_plot_1_price_goes_up() {
    let (prices, long_liquidity_inc, short_liquidity_inc) = symulate(true);
    let (prices, long_liquidity, short_liquidity) = symulate(false);

    // Create a new plot
    let mut plot = Plot::new();

    let trace = Scatter::new(prices.clone(), long_liquidity_inc)
        .mode(Mode::LinesMarkers)
        .name("long_liquidity_inc");
    plot.add_trace(trace);

    let trace = Scatter::new(prices.clone(), short_liquidity_inc)
        .mode(Mode::LinesMarkers)
        .name("short_liquidity_inc");
    plot.add_trace(trace);

    let trace = Scatter::new(prices.clone(), long_liquidity)
        .mode(Mode::LinesMarkers)
        .name("long_liquidity");
    plot.add_trace(trace);

    let trace = Scatter::new(prices, short_liquidity)
        .mode(Mode::LinesMarkers)
        .name("short_liquidity");
    plot.add_trace(trace);

    // let long_percent_diff = percent_diff(&long_liquidity);
    // let trace = Scatter::new(prices.clone(), long_percent_diff.clone()).mode(Mode::LinesMarkers);
    // plot.add_trace(trace);

    // let shorts_percent_diff = percent_diff(&short_liquidity);
    // let trace = Scatter::new(prices.clone(), shorts_percent_diff.clone()).mode(Mode::LinesMarkers);
    // plot.add_trace(trace);

    // let diff = shorts_percent_diff.iter().zip(long_percent_diff.iter()).map(|(a, b)| (a - b).abs()).collect::<Vec<f64>>();
    // let trace = Scatter::new(prices, diff).mode(Mode::LinesMarkers);
    // plot.add_trace(trace);

    // Display the plot in a browser window
    plot.show();
}

fn to_usd(value: U256) -> f64 {
    value.as_u64() as f64 / ONE_DOLLAR as f64
}

fn percent_diff(data: &[f64]) -> Vec<f64> {
    let mut result = vec![];
    for i in 1..data.len() {
        let diff = (data[i] - data[i - 1]) / data[i - 1];
        result.push(diff * 100.0);
    }
    result
}

fn symulate(incremental_mode: bool) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut state = MarketState {
        long_total_supply: U256::zero(),
        short_total_supply: U256::zero(),
        long_liquidity: U256::from(20_000),
        short_liquidity: U256::from(10_000),
        price: U256::from(ONE_CENT),
    };

    let price_list_steps = 60;
    let mut prices = vec![to_usd(state.price)];
    let mut long_liquidity = vec![state.long_liquidity.as_u64() as f64];
    let mut short_liquidity = vec![state.short_liquidity.as_u64() as f64];
    for i in 0..price_list_steps {
        if incremental_mode {
            let old_price = state.price;
            let new_price = old_price + U256::from(ONE_TENTH_CENT);
            state.on_price_goes_up(new_price);

            prices.push(to_usd(new_price));
            long_liquidity.push(state.long_liquidity.as_u64() as f64);
            short_liquidity.push(state.short_liquidity.as_u64() as f64);
        } else {
            let mut state = state.clone();
            let old_price = state.price;
            let new_price = old_price + U256::from(ONE_TENTH_CENT) * U256::from(i + 1);
            state.on_price_goes_up(new_price);

            prices.push(to_usd(new_price));
            long_liquidity.push(state.long_liquidity.as_u64() as f64);
            short_liquidity.push(state.short_liquidity.as_u64() as f64);
        }
    }

    (prices, long_liquidity, short_liquidity)
}
