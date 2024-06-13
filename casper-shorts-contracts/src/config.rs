use odra::{module::Module, Address, ContractRef, UnwrapOrRevert, Var};
use odra_modules::cep18_token::Cep18ContractRef;

use crate::{market::MarketContractRef, token_long::TokenLongContractRef};

#[odra::odra_type]
pub struct Config {
    pub long_token: Address,
    pub short_token: Address,
    pub wcspr_token: Address,
    pub fee_collector: Address,
    pub market: Address,
}

impl Config {
    pub fn new(
        long_token: Address,
        short_token: Address,
        wcspr_token: Address,
        fee_collector: Address,
        market: Address,
    ) -> Self {
        Self {
            long_token,
            short_token,
            wcspr_token,
            fee_collector,
            market,
        }
    }

    pub fn is_long_token(&self, addr: &Address) -> bool {
        &self.long_token == addr
    }

    pub fn is_short_token(&self, addr: &Address) -> bool {
        &self.short_token == addr
    }

    pub fn is_wcspr_token(&self, addr: &Address) -> bool {
        &self.wcspr_token == addr
    }

    pub fn is_fee_collector(&self, addr: &Address) -> bool {
        &self.fee_collector == addr
    }

    pub fn is_market(&self, addr: &Address) -> bool {
        &self.market == addr
    }
}

#[odra::module]
pub struct ConfigModule {
    state: Var<Config>,
}

impl ConfigModule {
    pub fn init(&mut self, state: Config) {
        self.state.set(state);
    }

    pub fn set(&mut self, state: &Config) {
        self.state.set(state.clone());
    }

    pub fn get(&self) -> Config {
        self.state.get().unwrap_or_revert(&self.env())
    }

    pub fn long_token(&self) -> TokenLongContractRef {
        let addr = self.get().long_token;
        TokenLongContractRef::new(self.env(), addr)
    }

    pub fn long_token_cep18(&self) -> Cep18ContractRef {
        let addr = self.get().long_token;
        Cep18ContractRef::new(self.env(), addr)
    }

    pub fn short_token(&self) -> TokenLongContractRef {
        let addr = self.get().short_token;
        TokenLongContractRef::new(self.env(), addr)
    }

    pub fn short_token_cep18(&self) -> Cep18ContractRef {
        let addr = self.get().short_token;
        Cep18ContractRef::new(self.env(), addr)
    }

    pub fn wcspr_token(&self) -> TokenLongContractRef {
        let addr = self.get().wcspr_token;
        TokenLongContractRef::new(self.env(), addr)
    }

    pub fn wcspr_token_cep18(&self) -> Cep18ContractRef {
        let addr = self.get().wcspr_token;
        Cep18ContractRef::new(self.env(), addr)
    }

    pub fn market(&self) -> MarketContractRef {
        let addr = self.get().market;
        MarketContractRef::new(self.env(), addr)
    }

    pub fn fee_collector(&self) -> Address {
        self.get().fee_collector
    }
}
