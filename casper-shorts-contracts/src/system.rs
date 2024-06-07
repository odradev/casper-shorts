/// Responible for calculations and state of the Casper Shorts.
/// It realizes the design explained in `README.md`.
use odra::casper_types::U256;

// ## State Definition
//
// The system is defined as:
//
// $State = (P, L_{Long}, L_{Short}, T_{Long}, T_{Short})$
//
// where:
// - $P$ is the current price of the asset,
// - $L_{Long}$ is the total amount of the asset for the long position,
// - $L_{Short}$ is the total amount of the asset for the short position,
// - $T_{Long}$ is the total supply for the long token,
// - $T_{Short}$ is the total supply for the short token.
//
// In addition:
//
// - $D$ - liquidity deposit for the long or short position,
// - $W$ - tokens amount to be exchanges for the asset.
// - $L$ - the amount of the asset for the long or short position.
// - $S$ - the total supply of the long or short token.
//
// ## Mechanics
//
// Whenever new price $Price_{i+1}$ is received the system updates the $State$,
// by rebalancing the liquidity between long and short positions.
//
// If the new price is lower then the previous price, first the short position
// is reduced and then the long position is increased with the same amount.
// And vice versa for the price increase.
//
// Then the system updates the total supply of the long or short token.

pub static ONE_DOLLAR: u64 = 10_000;
pub static ONE_CENT: u64 = 100;
pub static ONE_TENTH_CENT: u64 = 10;

#[odra::odra_type]
pub struct MarketState {
    pub long_total_supply: U256,
    pub short_total_supply: U256,
    pub long_liquidity: U256,
    pub short_liquidity: U256,
    pub price: U256,
}

impl MarketState {
    pub fn new(price: U256) -> Self {
        MarketState {
            long_total_supply: U256::zero(),
            short_total_supply: U256::zero(),
            long_liquidity: U256::zero(),
            short_liquidity: U256::zero(),
            price: price,
        }
    }

    pub fn on_price_change(&mut self, new_price: U256) {
        if new_price > self.price {
            self.on_price_goes_up(new_price);
        } else if new_price < self.price {
            self.on_price_goes_down(new_price);
        }
        // Do nothing when price is the same.
    }

    // ## Price goes up
    //
    // Short position is losing liquidity and it is transferred to the long position.
    // The formula is intended to cap the adjustment to the total available liquidity,
    // using the `min` function.
    //
    // $\Delta L_{Short}(i + 1) = L_{Short}(i) \times min(1, \frac{P(i+1)}{P(i)} - 1)$
    // $L_{Short}(i+1) = L_{Short}(i) - \Delta L_{Short}$
    // $L_{Long}(i+1) = L_{Long}(i) + \Delta L_{Short}$
    pub fn on_price_goes_up(&mut self, new_price: U256) {
        let delta = self.short_liquidity * new_price / self.price - self.short_liquidity;
        let delta = self.short_liquidity.min(delta);

        self.short_liquidity -= delta;
        self.long_liquidity += delta;
        self.price = new_price;
    }

    // ## Price goes down
    //
    // Long position is losing liquidity and it is transferred to the short position.
    //
    // $\Delta L_{Long}(i + 1) = L_{Long}(i) \times min(1, 1 - \frac{P(i)}{P(i+1)})$
    // $L_{Short}(i+1) = L_{Short}(i) + \Delta L_{Long}$
    // $L_{Long}(i+1) = L_{Long}(i) - \Delta L_{Long}$
    pub fn on_price_goes_down(&mut self, new_price: U256) {
        let delta = self.long_liquidity * self.price / new_price - self.long_liquidity;

        self.long_liquidity -= delta;
        self.short_liquidity += delta;
        self.price = new_price;
    }

    // ## Token deposit
    //
    // When user deposits funds $D$ into the short or long position, the system
    // calculates the new amount of the position and the new total supply of the long
    // or short token.
    //
    // $L(i+1) = L(i) + D$
    // $\Delta T(i+1) = T(i) \times (\frac{L(i+1)}{L(i)} - 1)$
    // $T(i+1) = T(i) + \Delta S$
    pub fn on_deposit(&mut self, side: Side, amount: U256) -> U256 {
        let (old_liquidity, old_token_supply) = match side {
            Side::Long => (self.long_liquidity, self.long_total_supply),
            Side::Short => (self.short_liquidity, self.short_total_supply),
        };
        let new_liqidity = old_liquidity + amount;

        let delta_token_supply = if old_token_supply.is_zero() {
            amount
        } else {
            old_token_supply * new_liqidity / old_liquidity - old_token_supply
        };

        let new_token_supply = old_token_supply + delta_token_supply;

        match side {
            Side::Long => {
                self.long_liquidity = new_liqidity;
                self.long_total_supply = new_token_supply;
            }
            Side::Short => {
                self.short_liquidity = new_liqidity;
                self.short_total_supply = new_token_supply;
            }
        }

        delta_token_supply
    }

    //## Token withdraw
    //
    // When user withdraws $W$ of $LONG$ or $SHORT$ tokens, the system calculates the
    // new amount of the position and the new total supply of the token.
    //
    // $T(i+1) = T(i) - W$
    // $\Delta L(i+1) = L(i) \times (1 - \frac{T(i)}{T(i+1)})$
    // $L(i+1) = L(i) - \Delta L$
    pub fn on_withdraw(&mut self, side: Side, amount: U256) -> U256 {
        let (old_liquidity, old_token_supply) = match side {
            Side::Long => (self.long_liquidity, self.long_total_supply),
            Side::Short => (self.short_liquidity, self.short_total_supply),
        };
        let new_token_supply = old_token_supply - amount;
        let delta_liquidity = old_liquidity - old_liquidity * new_token_supply / old_token_supply;
        let new_liquidity = old_liquidity - delta_liquidity;

        match side {
            Side::Long => {
                self.long_liquidity = new_liquidity;
                self.long_total_supply = new_token_supply;
            }
            Side::Short => {
                self.short_liquidity = new_liquidity;
                self.short_total_supply = new_token_supply;
            }
        }

        delta_liquidity
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Long,
    Short,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ## Example 1: Price goes up by more then 100%
    // Given:
    // $P(i) = 0.01 \text{USD/CSPR}$
    // $P(i+1) = 0.03 \text{ USD/CSPR}$
    // $L_{Short}(i) = 100 \text { CSPR}$
    // $L_{Long}(i) = 200 \text { CSPR}$
    //
    // Then:
    // $\Delta L_{Short}(i+1) = 100 \times min(1, \frac{0.03}{0.01} - 1) = 100 \times 1 = 100 \text{ CSPR}$
    // $L_{Short}(i+1) = 100 - 100 = 0 \text{ CSPR}$
    // $L_{Long}(i+1) = 200 + 100 = 300 \text{ CSPR}$
    #[test]
    fn example_1_when_price_goes_up_by_more_then_100_percent() {
        let mut state = MarketState {
            long_total_supply: U256::zero(),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(200),
            short_liquidity: U256::from(100),
            price: U256::from(ONE_CENT),
        };
        state.on_price_goes_up(U256::from(3 * ONE_CENT));

        let expected = MarketState {
            long_total_supply: U256::zero(),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(300),
            short_liquidity: U256::from(0),
            price: U256::from(3 * ONE_CENT),
        };
        assert_eq!(state, expected);
    }

    // ## Example 2: Price goes up by less then 100%
    //
    // Given:
    // $P(i) = 0.01 \text{USD/CSPR}$
    // $P(i+1) = 0.014 \text{ USD/CSPR}$
    // $L_{Short}(i) = 100 \text { CSPR}$
    // $L_{Long}(i) = 200 \text { CSPR}$
    //
    // Then:
    // $\Delta L_{Short}(i+1) = 100 \times min(1, \frac{0.014}{0.01} - 1) = 100 \times 0.4 = 40 \text { CSPR}$
    // $L_{Short}(i+1) = 100 - 40 = 60 \text { CSPR}$
    // $L_{Long}(i+1) = 200 + 40 = 240 \text { CSPR}$
    #[test]
    fn example_2_when_price_goes_up_by_less_then_100_percent() {
        let mut state = MarketState {
            long_total_supply: U256::zero(),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(200),
            short_liquidity: U256::from(100),
            price: U256::from(ONE_CENT),
        };
        state.on_price_goes_up(U256::from(14 * ONE_TENTH_CENT));

        let expected = MarketState {
            long_total_supply: U256::zero(),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(240),
            short_liquidity: U256::from(60),
            price: U256::from(14 * ONE_TENTH_CENT),
        };
        assert_eq!(state, expected);
    }

    // ## Example 4: Liquidity deposit
    //
    // Given:
    // $L(i) = 200 \text { CSPR}$
    // $D = 100 \text { CSPR}$
    // $T(i) = 1000 \text{ LONG}$
    //
    // Then:
    // $L(i+1) = 200 + 100 = 300 \text{ CSPR}$
    // $\Delta T(i+1) = 1000 \times (\frac{300}{200} - 1) = 1000 \times 0.5 = 500 \text{ LONG}$
    // $T_{Long}(i+1) = 1000 + 500 = 1500 \text{ LONG}$
    #[test]
    fn example_4_liquidity_deposit() {
        let mut state = MarketState {
            long_total_supply: U256::from(1000),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(200),
            short_liquidity: U256::zero(),
            price: U256::from(ONE_CENT),
        };
        state.on_deposit(Side::Long, U256::from(100));

        let expected = MarketState {
            long_total_supply: U256::from(1500),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(300),
            short_liquidity: U256::zero(),
            price: U256::from(ONE_CENT),
        };
        assert_eq!(state, expected);
    }

    // ## Example 5: Liquidity withdraw
    //
    // Given:
    // $L(i) = 400 \text { CSPR}$
    // $W = 100 \text { LONG}$
    // $T(i) = 1000 \text{ LONG}$
    //
    // Then:
    // $T(i+1) = 1000 - 100 = 900 \text{ LONG}$
    // $\Delta L(i+1) = 400 \times (1 - \frac{900}{1000}) = 400 \times 0.1 = 40 \text{ CSPR}$
    // $L(i+1) = 400 - 40 = 360 \text{ CSPR}$
    #[test]
    fn example_5_liquidity_withdraw() {
        let mut state = MarketState {
            long_total_supply: U256::from(1000),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(400),
            short_liquidity: U256::zero(),
            price: U256::from(ONE_CENT),
        };
        state.on_withdraw(Side::Long, U256::from(100));

        let expected = MarketState {
            long_total_supply: U256::from(900),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(360),
            short_liquidity: U256::zero(),
            price: U256::from(ONE_CENT),
        };
        assert_eq!(state, expected);
    }

    #[test]
    fn example_6_first_deposit() {
        let mut state = MarketState {
            long_total_supply: U256::zero(),
            short_total_supply: U256::zero(),
            long_liquidity: U256::zero(),
            short_liquidity: U256::zero(),
            price: U256::from(ONE_CENT),
        };
        state.on_deposit(Side::Long, U256::from(100));

        let expected = MarketState {
            long_total_supply: U256::from(100),
            short_total_supply: U256::zero(),
            long_liquidity: U256::from(100),
            short_liquidity: U256::zero(),
            price: U256::from(ONE_CENT),
        };
        assert_eq!(state, expected);
    }
}
