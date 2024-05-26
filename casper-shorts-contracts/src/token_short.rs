use odra::{casper_types::U256, prelude::*, Address, SubModule};
use odra_modules::{access::Ownable, cep18::{errors::Error as Cep18Error, utils::Cep18Modality}, cep18_token::Cep18};

use crate::config::{Config, ConfigModule};

/// A module definition. Each module struct consists of Vars and Mappings
/// or/and other modules.
#[odra::module]
pub struct TokenShort {
    /// A submodule that implements the CEP-18 token standard.
    token: SubModule<Cep18>,
    cfg: SubModule<ConfigModule>,
    ownable: SubModule<Ownable>,
}

/// Module implementation.
///
/// To generate entrypoints,
/// an implementation block must be marked as #[odra::module].
#[odra::module]
impl TokenShort {
    /// Initializes the contract with the given metadata and initial supply.
    pub fn init(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        self.token.init(
            symbol,
            name,
            decimals,
            initial_supply,
            vec![],
            vec![],
            Some(Cep18Modality::MintAndBurn),
        );
        self.ownable.init();
    }

    pub fn set_config(&mut self, cfg: Config) {
        self.ownable.assert_owner(&self.env().caller());
        self.cfg.set(cfg);
    }

    pub fn transfer(&mut self, recipient: &Address, amount: &U256) {
        let sender = self.env().caller();
        let pack = self.cfg.get();
        if pack.is_wcspr_token(&recipient) {
            self.cfg
                .market()
                .withdraw_short_from(&sender, *amount);
        } else {
            self.token.raw_transfer(&sender, &recipient, &amount);
        }
    }

    pub fn transfer_from(&mut self, owner: &Address, recipient: &Address, amount: &U256) {
        let sender = self.env().caller();
        let pack = self.cfg.get();
        if pack.is_market(&sender) {
            self.token.raw_transfer(owner, recipient, amount);
        } else {
            self.token.transfer_from(owner, recipient, amount);
        }
    }

    /// Burns the given amount of tokens from the given address.
    pub fn burn(&mut self, owner: &Address, amount: &U256) {
        // self.assert_burn_and_mint_enabled();

        let caller = self.env().caller();
        if self.cfg.get().is_market(&caller) {
            if self.balance_of(owner) < *amount {
                self.env().revert(Cep18Error::InsufficientBalance);
            }
            self.token.raw_burn(owner, amount);
        } else {
            self.token.burn(owner, amount);
        }
    }

    // Delegate all Cep18 functions to the token submodule.
    delegate! {
        to self.token {
            /// Admin EntryPoint to manipulate the security access granted to users.
            /// One user can only possess one access group badge.
            /// Change strength: None > Admin > Minter
            /// Change strength meaning by example: If a user is added to both Minter and Admin, they will be an
            /// Admin, also if a user is added to Admin and None then they will be removed from having rights.
            /// Beware: do not remove the last Admin because that will lock out all admin functionality.
            fn change_security(
                &mut self,
                admin_list: Vec<Address>,
                minter_list: Vec<Address>,
                none_list: Vec<Address>
            );

            /// Returns the name of the token.
            fn name(&self) -> String;

            /// Returns the symbol of the token.
            fn symbol(&self) -> String;

            /// Returns the number of decimals the token uses.
            fn decimals(&self) -> u8;

            /// Returns the total supply of the token.
            fn total_supply(&self) -> U256;

            /// Returns the balance of the given address.
            fn balance_of(&self, address: &Address) -> U256;

            /// Returns the amount of tokens the owner has allowed the spender to spend.
            fn allowance(&self, owner: &Address, spender: &Address) -> U256;

            /// Approves the spender to spend the given amount of tokens on behalf of the caller.
            fn approve(&mut self, spender: &Address, amount: &U256);

            /// Decreases the allowance of the spender by the given amount.
            fn decrease_allowance(&mut self, spender: &Address, decr_by: &U256);

            /// Increases the allowance of the spender by the given amount.
            fn increase_allowance(&mut self, spender: &Address, inc_by: &U256);

            /// Mints new tokens and assigns them to the given address.
            fn mint(&mut self, owner: &Address, amount: &U256);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::Deployer;

    #[test]
    fn it_works() {
        let env = odra_test::env();
        // To test a module, we need to deploy it.
        // Autogenerated `TokenShortInitArgs` implements `InitArgs` trait
        // and `TokenShortHostRef` implements `Deployer` trait,
        // so we can use it to deploy the module.
        let init_args = TokenShortInitArgs {
            name: "TokenShort".to_string(),
            symbol: "TS".to_string(),
            decimals: 10,
            initial_supply: U256::from(1_000_000_000_000u64),
        };
        assert!(TokenShortHostRef::try_deploy(&env, init_args).is_ok());
    }
}
