use odra::casper_types::U256;
use odra_test::bdd::param::Amount;

pub trait BalanceCheck {
    type Target;

    fn is_close_to(&self, other: &Self::Target) -> bool;
}

impl BalanceCheck for U256 {
    type Target = Amount;

    fn is_close_to(&self, other: &Self::Target) -> bool {
        let diff = self.abs_diff(**other);
        diff < U256::from(10_000)
    }
}
