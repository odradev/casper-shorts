use odra::{
    casper_types::U256, module::Module, prelude::*, Address, SubModule, UnwrapOrRevert, Var,
};
use odra_modules::access::Ownable;

use crate::{
    config::{Config, ConfigModule},
    events::{OnDeposit, OnPriceChange},
    price_data::PriceData,
    system::{MarketState, Side},
};

#[odra::module]
pub struct Market {
    admin: SubModule<Ownable>,
    cfg: SubModule<ConfigModule>,
    state: Var<MarketState>,
}

#[odra::module]
impl Market {
    pub fn init(&mut self, last_price: PriceData) {
        self.state.set(MarketState::new(last_price.price));
        self.admin.init();
    }

    pub fn deposit_long(&mut self, amount: U256) {
        self.deposit_unchecked(&self.env().caller(), Side::Long, amount);
    }

    pub fn deposit_long_from(&mut self, sender: &Address, amount: U256) {
        if self.cfg.get().is_long_token(&self.env().caller()) {
            self.env()
                .revert(MarketError::LongTokenContractNotACallerOnDeposit);
        }
        self.deposit_unchecked(sender, Side::Long, amount);
    }

    pub fn deposit_short(&mut self, amount: U256) {
        self.deposit_unchecked(&self.env().caller(), Side::Short, amount);
    }

    pub fn deposit_short_from(&mut self, sender: &Address, amount: U256) {
        if self.cfg.get().is_short_token(&self.env().caller()) {
            self.env()
                .revert(MarketError::ShortTokenContractNotACallerOnDeposit);
        }
        self.deposit_unchecked(sender, Side::Short, amount);
    }

    pub fn withdraw_long(&mut self, amount: U256) {
        self.withdrawal_unchecked(&self.env().caller(), Side::Long, amount);
    }

    pub fn withdraw_long_from(&mut self, sender: &Address, amount: U256) {
        if self.cfg.get().is_wcspr_token(&self.env().caller()) {
            self.env()
                .revert(MarketError::LongTokenContractNotACallerOnWithdrawal);
        }
        self.withdrawal_unchecked(sender, Side::Long, amount);
    }

    pub fn withdraw_short(&mut self, amount: U256) {
        self.withdrawal_unchecked(&self.env().caller(), Side::Short, amount);
    }

    pub fn withdraw_short_from(&mut self, sender: &Address, amount: U256) {
        if self.cfg.get().is_wcspr_token(&self.env().caller()) {
            self.env()
                .revert(MarketError::ShortTokenContractNotACallerOnWithdrawal);
        }
        self.withdrawal_unchecked(sender, Side::Short, amount);
    }

    pub fn set_price(&mut self, price_data: PriceData) {
        self.admin.assert_owner(&self.env().caller());
        let mut state = self.get_state();
        state.on_price_change(price_data.price);
        self.set_state(state);
        self.env().emit_event(OnPriceChange {
            price: price_data.price,
            timestamp: price_data.timestamp,
        });
    }

    pub fn get_market_state(&self) -> MarketState {
        self.get_state()
    }

    pub fn set_config(&mut self, cfg: &Config) {
        self.admin.assert_owner(&self.env().caller());
        self.cfg.set(cfg);
    }
}

impl Market {
    fn get_state(&self) -> MarketState {
        self.state.get().unwrap_or_revert(&self.env())
    }

    fn set_state(&mut self, state: MarketState) {
        self.state.set(state);
    }

    fn deposit_unchecked(&mut self, sender: &Address, side: Side, amount: U256) {
        self.collect_deposit(sender, &amount);
        let (amount, fee) = split_fee(amount);
        self.collect_fee(&fee);

        let mut state = self.get_state();
        let new_tokens = state.on_deposit(side, amount);
        self.set_state(state);

        // Mint new tokens to the caller.
        match side {
            Side::Long => self.cfg.long_token().mint(sender, &new_tokens),
            Side::Short => self.cfg.short_token().mint(sender, &new_tokens),
        };

        // Emit the deposit event.
        self.env().emit_event(OnDeposit {
            depositor: *sender,
            amount,
            is_long: side == Side::Long,
        });
    }

    pub fn withdrawal_unchecked(&mut self, receiver: &Address, side: Side, amount: U256) {
        // Update the state and get the amount that can be withdrawn.
        let mut state = self.get_state();
        let withdraw_amount = state.on_withdraw(side, amount);
        self.set_state(state);

        // Withdraw the deposit and fee.
        let (withdraw_amount, fee) = split_fee(withdraw_amount);
        self.collect_fee(&fee);
        self.withdraw_deposit(receiver, &withdraw_amount);

        // Burn the tokens.
        match side {
            Side::Long => self.cfg.long_token().burn(receiver, &amount),
            Side::Short => self.cfg.short_token().burn(receiver, &amount),
        };

        // Emit the withdrawal event.
        self.env().emit_event(OnDeposit {
            depositor: *receiver,
            amount,
            is_long: side == Side::Long,
        });
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
        let fee_collector = self.cfg.fee_collector();
        self.cfg.wcspr_token().transfer(&fee_collector, amount);
    }

    fn collect_deposit(&mut self, sender: &Address, amount: &U256) {
        self.cfg
            .wcspr_token()
            .transfer_from(&sender, &self.env().self_address(), amount);
    }

    fn withdraw_deposit(&mut self, recipient: &Address, amount: &U256) {
        self.cfg.wcspr_token().transfer(recipient, amount);
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
    LongTokenContractNotACallerOnDeposit = 8006,
    ShortTokenContractNotACallerOnDeposit = 8007,
    LongTokenContractNotACallerOnWithdrawal = 8008,
    ShortTokenContractNotACallerOnWithdrawal = 8009,
}
