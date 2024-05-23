use odra::casper_types::U256;

#[odra::odra_type]
pub struct PriceData {
    pub price: U256,
    pub timestamp: u64,
}
