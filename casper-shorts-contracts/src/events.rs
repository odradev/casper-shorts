use odra::{casper_types::U256, prelude::*, Address};

#[odra::event]
pub struct OnPriceChange {
    pub price: U256,
    pub timestamp: u64,
}

#[odra::event]
pub struct OnDeposit {
    pub depositor: Address,
    pub amount: U256,
    pub is_long: bool,
}

#[odra::event]
pub struct OnWithdrawal {
    pub withdrawer: Address,
    pub amount: U256,
    pub is_long: bool,
}
