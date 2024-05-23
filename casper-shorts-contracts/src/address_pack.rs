use odra::{module::Module, Address, ContractRef, UnwrapOrRevert, Var};
use odra_modules::cep18_token::Cep18ContractRef;

use crate::{market::MarketContractRef, token_long::TokenLongContractRef};

#[odra::odra_type]
pub struct AddressPack {
    pub long_token: Address,
    pub short_token: Address,
    pub wcspr_token: Address,
    pub fee_collector: Address,
    pub market: Address,
}

#[odra::module]
pub struct AddressPackModule {
    state: Var<AddressPack>,
}

impl AddressPackModule {
    pub fn init(&mut self, state: AddressPack) {
        self.state.set(state);
    }

    pub fn set(&mut self, state: AddressPack) {
        self.state.set(state);
    }

    pub fn long_token(&self) -> TokenLongContractRef {
        let addr = self.addresses().long_token;
        TokenLongContractRef::new(self.env(), addr)
    }

    pub fn long_token_cep18(&self) -> Cep18ContractRef {
        let addr = self.addresses().long_token;
        Cep18ContractRef::new(self.env(), addr)
    }

    pub fn short_token(&self) -> TokenLongContractRef {
        let addr = self.addresses().short_token;
        TokenLongContractRef::new(self.env(), addr)
    }

    pub fn short_token_cep18(&self) -> Cep18ContractRef {
        let addr = self.addresses().short_token;
        Cep18ContractRef::new(self.env(), addr)
    }

    pub fn wcspr_token(&self) -> TokenLongContractRef {
        let addr = self.addresses().wcspr_token;
        TokenLongContractRef::new(self.env(), addr)
    }

    pub fn wcspr_token_cep18(&self) -> Cep18ContractRef {
        let addr = self.addresses().wcspr_token;
        Cep18ContractRef::new(self.env(), addr)
    }

    pub fn market(&self) -> MarketContractRef {
        let addr = self.addresses().market;
        MarketContractRef::new(self.env(), addr)
    }

    pub fn fee_collector(&self) -> Address {
        self.addresses().fee_collector
    }

    pub fn addresses(&self) -> AddressPack {
        self.state.get().unwrap_or_revert(&self.env())
    }
}
