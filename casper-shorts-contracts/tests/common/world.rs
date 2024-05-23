use std::fmt::{Debug, Formatter};

use casper_shorts_contracts::{
    address_pack::{self, AddressPack},
    market::{MarketHostRef, MarketInitArgs},
    price_data::PriceData,
    system::{MarketState, ONE_CENT},
    token_long::{TokenLongHostRef, TokenLongInitArgs},
    token_short::{TokenShortHostRef, TokenShortInitArgs},
    token_wcspr::{TokenWCSPRHostRef, TokenWCSPRInitArgs},
};
use odra::{
    casper_types::U256,
    host::{Deployer, HostEnv, HostRef},
    Address,
};

use super::params::{Account, TokenKind};

const INITIAL_WCSPR_BALANCE: u64 = 1_000_000_000_000u64; // 1000 CSPR

#[derive(cucumber::World)]
pub struct CasperShortsWorld {
    odra_env: HostEnv,
    wcspr_token: TokenWCSPRHostRef,
    short_token: TokenShortHostRef,
    long_token: TokenLongHostRef,
    pub market: MarketHostRef,
}

impl Default for CasperShortsWorld {
    fn default() -> Self {
        let odra_env = odra_test::env();
        odra_env.advance_block_time(100);

        let mut wcspr_token = TokenWCSPRHostRef::deploy(
            &odra_env,
            TokenWCSPRInitArgs {
                name: "CasperShorts".to_string(),
                symbol: "WCSPR".to_string(),
                decimals: 9,
                initial_supply: 0u64.into(),
            },
        );

        let mut short_token = TokenShortHostRef::deploy(
            &odra_env,
            TokenShortInitArgs {
                name: "CS_SHORT".to_string(),
                symbol: "SHORT".to_string(),
                decimals: 9,
                initial_supply: 0u64.into(),
            },
        );

        let mut long_token = TokenLongHostRef::deploy(
            &odra_env,
            TokenLongInitArgs {
                name: "CS_LONG".to_string(),
                symbol: "LONG".to_string(),
                decimals: 9,
                initial_supply: 0u64.into(),
            },
        );

        let mut market = MarketHostRef::deploy(
            &odra_env,
            MarketInitArgs {
                last_price: PriceData {
                    price: ONE_CENT.into(),
                    timestamp: 0u64.into(),
                },
            },
        );

        // Update addresses.
        let address_pack = AddressPack {
            wcspr_token: wcspr_token.address().clone(),
            short_token: short_token.address().clone(),
            long_token: long_token.address().clone(),
            market: market.address().clone(),
            fee_collector: odra_env.get_account(Account::FeeCollector.index()),
        };

        market.set_addres_pack(address_pack.clone());
        long_token.set_address_pack(address_pack.clone());
        short_token.set_address_pack(address_pack.clone());
        wcspr_token.set_address_pack(address_pack.clone());

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
    pub fn address(&self, account: Account) -> Address {
        match account {
            Account::Market => self.market.address().clone(),
            _ => self.odra_env.get_account(account.index()),
        }
    }

    pub fn balance_of(&self, token: TokenKind, account: Account) -> U256 {
        let address = self.address(account);
        match token {
            TokenKind::WCSPR => self.wcspr_token.balance_of(&address),
            TokenKind::SHORT => self.short_token.balance_of(&address),
            TokenKind::LONG => self.long_token.balance_of(&address),
        }
    }

    pub fn mint(&mut self, token: TokenKind, account: Account, amount: U256) {
        let address = self.address(account);
        match token {
            TokenKind::WCSPR => self.wcspr_token.mint(&address, &amount),
            TokenKind::SHORT => self.short_token.mint(&address, &amount),
            TokenKind::LONG => self.long_token.mint(&address, &amount),
        }
    }

    pub fn go_long(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.wcspr_token.approve(self.market.address(), &amount);
        self.market.deposit_long(amount);
    }

    pub fn go_short(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.wcspr_token.approve(self.market.address(), &amount);
        self.market.deposit_short(amount);
    }

    pub fn withdraw_long(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.long_token.approve(self.market.address(), &amount);
        self.market.withdraw_long(amount);
    }

    pub fn withdraw_short(&mut self, account: Account, amount: U256) {
        let address = self.address(account);
        self.odra_env.set_caller(address);
        self.short_token.approve(self.market.address(), &amount);
        self.market.withdraw_short(amount);
    }

    pub fn set_price(&mut self, price: U256) {
        self.market.set_price(PriceData {
            price,
            timestamp: 0,
        });
    }

    pub fn get_market_state(&self) -> MarketState {
        self.market.get_market_state()
    }
}
