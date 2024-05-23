use odra::{
    casper_types::U256, module::Module, prelude::*, Address, ContractRef, SubModule, UnwrapOrRevert, Var
};
use odra_modules::{access::Ownable, cep18_token::Cep18ContractRef};

use crate::{
    price_data::PriceData,
    system::{MarketState, Side},
};

#[odra::module]
pub struct Market {
    admin: SubModule<Ownable>,
    long_token: Var<Address>,
    short_token: Var<Address>,
    wcspr_token: Var<Address>,
    fee_collector: Var<Address>,
    state: Var<MarketState>,
}

#[odra::module]
impl Market {
    pub fn init(
        &mut self,
        long_token: Address,
        short_token: Address,
        wcspr_token: Address,
        fee_collector: Address,
        last_price: PriceData,
    ) {
        // TODO: Long and Short tokens should have total supply of 0.
        self.long_token.set(long_token);
        self.short_token.set(short_token);
        self.wcspr_token.set(wcspr_token);
        self.fee_collector.set(fee_collector);
        self.state.set(MarketState::new(last_price.price));
        self.admin.init();
    }

    pub fn deposit_long(&mut self, amount: U256, price_data: Option<PriceData>) {
        self.deposit_unchecked(Side::Long, amount, price_data);
    }

    pub fn deposit_short(&mut self, amount: U256, price_data: Option<PriceData>) {
        self.deposit_unchecked(Side::Short, amount, price_data);
    }

    pub fn withdraw_long(&mut self, amount: U256, price_data: Option<PriceData>) {
        self.withdrawal_unchecked(Side::Long, amount, price_data);
    }

    pub fn withdraw_short(&mut self, amount: U256, price_data: Option<PriceData>) {
        self.withdrawal_unchecked(Side::Short, amount, price_data);
    }

    pub fn set_price(&mut self, price_data: PriceData) {
        self.admin.assert_owner(&self.env().caller());
        let mut state = self.get_state();
        state.on_price_change(price_data.price);
        self.set_state(state);
    }

    pub fn get_market_state(&self) -> MarketState {
        self.get_state()
    }
}

impl Market {
    fn wcspr_token(&self) -> Cep18ContractRef {
        let addr = self.wcspr_token.get().unwrap_or_revert(&self.env());
        Cep18ContractRef::new(self.env(), addr)
    }

    fn long_token(&self) -> Cep18ContractRef {
        let addr = self.long_token.get().unwrap_or_revert(&self.env());
        Cep18ContractRef::new(self.env(), addr)
    }

    fn short_token(&self) -> Cep18ContractRef {
        let addr = self.short_token.get().unwrap_or_revert(&self.env());
        Cep18ContractRef::new(self.env(), addr)
    }

    fn fee_collector(&self) -> Address {
        self.fee_collector.get().unwrap_or_revert(&self.env())
    }

    fn get_state(&self) -> MarketState {
        self.state.get().unwrap_or_revert(&self.env())
    }

    fn set_state(&mut self, state: MarketState) {
        self.state.set(state);
    }

    fn deposit_unchecked(&mut self, side: Side, amount: U256, price_data: Option<PriceData>) {
        self.collect_deposit(&amount);
        let (amount, fee) = split_fee(amount);
        self.collect_fee(&fee);

        let mut state = self.get_state();

        if let Some(price_data) = price_data {
            state.on_price_change(price_data.price);
        }
        let new_tokens = state.on_deposit(side, amount);
        match side {
            Side::Long => self.long_token().mint(&self.env().caller(), &new_tokens),
            Side::Short => self.short_token().mint(&self.env().caller(), &new_tokens),
        }

        self.set_state(state);
    }

    pub fn withdrawal_unchecked(&mut self, side: Side, amount: U256, price_data: Option<PriceData>) {
        let mut state = self.get_state();
        if let Some(price_data) = price_data {
            state.on_price_change(price_data.price);
        }
        let withdraw_amount = state.on_withdraw(side, amount);

        let (withdraw_amount, fee) = split_fee(withdraw_amount);
        self.collect_fee(&fee);
        self.withdraw_deposit(&withdraw_amount);

        let self_address = self.env().self_address();
        let caller = self.env().caller();
        let mut token = match side {
            Side::Long => self.long_token(),
            Side::Short => self.short_token(),
        };
        token.transfer_from(&caller, &self_address, &amount);
        token.burn(&self_address, &amount);

        self.set_state(state);
    }

    // Check if the new price is in fact newer and if so, update the last price.
    // fn handle_and_validate_new_price(&mut self, new: PriceData) {
    //     let current = self.last_price.get_or_revert_with(MarketError::LastPriceNotSet);
    //     if current.timestamp > new.timestamp {
    //         self.env().revert(MarketError::NewPriceIsTooOld);
    //     }
    //     if new.timestamp > self.env().get_block_time() {
    //         self.env().revert(MarketError::NewPriceIsFromTheFuture);
    //     }
    //     self.last_price.set(new);
    // }

    fn collect_fee(&mut self, amount: &U256) {
        self.wcspr_token().transfer(&self.fee_collector(), amount);
    }

    fn collect_deposit(&mut self, amount: &U256) {
        self.wcspr_token()
            .transfer_from(&self.env().caller(), &self.env().self_address(), amount);
    }

    fn withdraw_deposit(&mut self, amount: &U256) {
        self.wcspr_token().transfer(&self.env().caller(), amount);
    }
}

pub fn split_fee(amount: U256) -> (U256, U256) {
    let fee = amount / U256::from(200);
    let amount = amount - fee;
    (amount, fee)
}

#[odra::odra_error]
pub enum MarketError {
    LastPriceNotSet = 8001,
    NewPriceIsTooOld = 8002,
    NewPriceIsFromTheFuture = 8003,
    LongShareNotSet = 8004,
    TotalDepositNotSet = 8005,
}
