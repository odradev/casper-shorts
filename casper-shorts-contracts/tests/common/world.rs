use std::fmt::{Debug, Formatter};

use casper_shorts_contracts::{
    market::{MarketHostRef, MarketInitArgs},
    price_data::PriceData, system::{MarketState, ONE_CENT},
};
use odra::{
    casper_types::U256,
    host::{Deployer, HostEnv, HostRef},
    Address,
};
use odra_modules::{
    cep18::utils::Cep18Modality,
    cep18_token::{Cep18HostRef, Cep18InitArgs},
};

use super::params::{Account, TokenKind};

const INITIAL_WCSPR_BALANCE: u64 = 1_000_000_000_000u64; // 1000 CSPR

#[derive(cucumber::World)]
pub struct CasperShortsWorld {
    odra_env: HostEnv,
    wcspr_token: Cep18HostRef,
    short_token: Cep18HostRef,
    long_token: Cep18HostRef,
    pub market: MarketHostRef,
}

impl Default for CasperShortsWorld {
    fn default() -> Self {
        let odra_env = odra_test::env();
        odra_env.advance_block_time(100);

        let wcspr_token = Cep18HostRef::deploy(
            &odra_env,
            Cep18InitArgs {
                name: "CasperShorts".to_string(),
                symbol: "WCSPR".to_string(),
                decimals: 9,
                initial_supply: 0u64.into(),
                minter_list: vec![],
                admin_list: vec![],
                modality: Some(Cep18Modality::MintAndBurn),
            },
        );

        let mut short_token = Cep18HostRef::deploy(
            &odra_env,
            Cep18InitArgs {
                name: "CS_SHORT".to_string(),
                symbol: "SHORT".to_string(),
                decimals: 9,
                initial_supply: 0u64.into(),
                minter_list: vec![],
                admin_list: vec![],
                modality: Some(Cep18Modality::MintAndBurn),
            },
        );

        let mut long_token = Cep18HostRef::deploy(
            &odra_env,
            Cep18InitArgs {
                name: "CS_LONG".to_string(),
                symbol: "LONG".to_string(),
                decimals: 9,
                initial_supply: 0u64.into(),
                minter_list: vec![],
                admin_list: vec![],
                modality: Some(Cep18Modality::MintAndBurn),
            },
        );

        let market = MarketHostRef::deploy(
            &odra_env,
            MarketInitArgs {
                long_token: long_token.address().clone(),
                short_token: short_token.address().clone(),
                wcspr_token: wcspr_token.address().clone(),
                fee_collector: odra_env.get_account(Account::FeeCollector.index()),
                last_price: PriceData {
                    price: ONE_CENT.into(),
                    timestamp: 0u64.into(),
                },
            },
        );

        // Make market minter of LONG and SHORT tokens.
        short_token.change_security(vec![], vec![market.address().clone()], vec![]);
        long_token.change_security(vec![], vec![market.address().clone()], vec![]);

        let mut world = CasperShortsWorld {
            wcspr_token,
            odra_env,
            short_token,
            long_token,
            market,
        };
        world.mint(
            TokenKind::WCSPR,
            Account::Alice,
            U256::from(INITIAL_WCSPR_BALANCE),
        );
        world.mint(
            TokenKind::WCSPR,
            Account::Bob,
            U256::from(INITIAL_WCSPR_BALANCE),
        );

        world
    }
}

impl Debug for CasperShortsWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "CasperShortsWorld")
    }
}

impl CasperShortsWorld {
    fn token(&self, token: TokenKind) -> &Cep18HostRef {
        match token {
            TokenKind::WCSPR => &self.wcspr_token,
            TokenKind::SHORT => &self.short_token,
            TokenKind::LONG => &self.long_token,
        }
    }

    fn token_mut(&mut self, token: TokenKind) -> &mut Cep18HostRef {
        match token {
            TokenKind::WCSPR => &mut self.wcspr_token,
            TokenKind::SHORT => &mut self.short_token,
            TokenKind::LONG => &mut self.long_token,
        }
    }

    pub fn address(&self, account: Account) -> Address {
        match account {
            Account::Market => self.market.address().clone(),
            _ => self.odra_env.get_account(account.index()),
        }
    }

    pub fn balance_of(&self, token: TokenKind, account: Account) -> U256 {
        let address = self.address(account);
        self.token(token).balance_of(&address)
    }

    pub fn mint(&mut self, token: TokenKind, account: Account, amount: U256) {
        let address = self.address(account);
        self.token_mut(token).mint(&address, &amount);
    }

    pub fn go_long(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.wcspr_token.approve(self.market.address(), &amount);
        self.market.deposit_long(
            amount,
            None
        );
    }

    pub fn go_short(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.wcspr_token.approve(self.market.address(), &amount);
        self.market.deposit_short(
            amount,
            None
        );
    }

    pub fn withdraw_long(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.long_token.approve(self.market.address(), &amount);
        self.market.withdraw_long(
            amount,
            None
        );
    }

    pub fn withdraw_short(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.short_token.approve(self.market.address(), &amount);
        self.market.withdraw_short(
            amount,
            None
        );
    }

    pub fn set_price(&mut self, price: U256) {
        self.market.set_price(PriceData {
            price,
            timestamp: 0
        });
    }

    pub fn get_market_state(&self) -> MarketState {
        self.market.get_market_state()
    }
}
