use std::fmt::{Debug, Formatter};

use casper_shorts_contracts::{
    config::Config,
    market::{Market, MarketHostRef, MarketInitArgs},
    price_data::PriceData,
    system::{MarketState, ONE_CENT},
    token_long::{TokenLong, TokenLongHostRef, TokenLongInitArgs},
    token_short::{TokenShort, TokenShortHostRef, TokenShortInitArgs},
    token_wcspr::{TokenWCSPR, TokenWCSPRHostRef, TokenWCSPRInitArgs},
};
use odra::{
    casper_types::U256,
    host::{Deployer, HostRef},
    Address,
};
use odra_test::bdd::{
    param::{Account, Amount},
    OdraWorld,
};

use super::params::{Price, TokenKind};

const INITIAL_WCSPR_BALANCE: u64 = 1_000_000_000_000u64; // 1000 CSPR

#[derive(cucumber::World)]
pub struct CasperShortsWorld {
    pub odra_world: OdraWorld,
}

impl CasperShortsWorld {
    pub fn long_token(&mut self) -> &mut TokenLongHostRef {
        self.odra_world.get_contract()
    }

    pub fn short_token(&mut self) -> &mut TokenShortHostRef {
        self.odra_world.get_contract()
    }

    pub fn wcspr_token(&mut self) -> &mut TokenWCSPRHostRef {
        self.odra_world.get_contract()
    }

    pub fn market(&mut self) -> &mut MarketHostRef {
        self.odra_world.get_contract()
    }

    pub fn token(&mut self, kind: TokenKind) -> TokenHostRef {
        let address = match kind {
            TokenKind::WCSPR => *self.wcspr_token().address(),
            TokenKind::SHORT => *self.short_token().address(),
            TokenKind::LONG => *self.long_token().address(),
        };
        let env = self.odra_world.env();
        TokenHostRef::new(address, env.clone())
    }
}

impl Default for CasperShortsWorld {
    fn default() -> Self {
        let mut odra_world = OdraWorld::default();
        odra_world.advance_block_time(100);
        odra_world.add_contract::<TokenWCSPR, _>(|env| {
            Box::new(TokenWCSPRHostRef::deploy(
                env,
                TokenWCSPRInitArgs {
                    name: "CasperShorts".to_string(),
                    symbol: "WCSPR".to_string(),
                    decimals: 9,
                    initial_supply: 0u64.into(),
                },
            ))
        });

        odra_world.add_contract::<TokenWCSPR, _>(|env| {
            Box::new(TokenWCSPRHostRef::deploy(
                env,
                TokenWCSPRInitArgs {
                    name: "CasperShorts".to_string(),
                    symbol: "WCSPR".to_string(),
                    decimals: 9,
                    initial_supply: 0u64.into(),
                },
            ))
        });

        odra_world.add_contract::<TokenShort, _>(|env| {
            Box::new(TokenShortHostRef::deploy(
                env,
                TokenShortInitArgs {
                    name: "CS_SHORT".to_string(),
                    symbol: "SHORT".to_string(),
                    decimals: 9,
                    initial_supply: 0u64.into(),
                },
            ))
        });

        odra_world.add_contract::<TokenLong, _>(|env| {
            Box::new(TokenLongHostRef::deploy(
                env,
                TokenLongInitArgs {
                    name: "CS_LONG".to_string(),
                    symbol: "LONG".to_string(),
                    decimals: 9,
                    initial_supply: 0u64.into(),
                },
            ))
        });

        odra_world.add_contract::<Market, _>(|env| {
            Box::new(MarketHostRef::deploy(
                env,
                MarketInitArgs {
                    last_price: PriceData {
                        price: ONE_CENT.into(),
                        timestamp: 0u64.into(),
                    },
                },
            ))
        });

        let market_address = *odra_world.get_contract::<MarketHostRef>().address();

        // Update addresses.
        let cfg = Config {
            wcspr_token: *odra_world.get_contract::<TokenWCSPRHostRef>().address(),
            short_token: *odra_world.get_contract::<TokenShortHostRef>().address(),
            long_token: *odra_world.get_contract::<TokenLongHostRef>().address(),
            market: market_address,
            fee_collector: odra_world.get_address(Account::CustomRole("FeeCollector".to_string())),
        };

        odra_world.get_contract::<MarketHostRef>().set_config(&cfg);
        odra_world
            .get_contract::<TokenShortHostRef>()
            .set_config(&cfg);
        odra_world
            .get_contract::<TokenLongHostRef>()
            .set_config(&cfg);
        odra_world
            .get_contract::<TokenWCSPRHostRef>()
            .set_config(&cfg);

        // Make market minter of LONG and SHORT tokens.
        odra_world
            .get_contract::<TokenShortHostRef>()
            .change_security(vec![], vec![market_address], vec![]);
        odra_world
            .get_contract::<TokenLongHostRef>()
            .change_security(vec![], vec![market_address], vec![]);

        let mut world = CasperShortsWorld { odra_world };
        world.mint(
            TokenKind::WCSPR,
            Account::Alice,
            Amount::from(INITIAL_WCSPR_BALANCE),
        );
        world.mint(
            TokenKind::WCSPR,
            Account::Bob,
            Amount::from(INITIAL_WCSPR_BALANCE),
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
    pub fn balance_of(&mut self, token: TokenKind, account: Account) -> U256 {
        let address = self.odra_world.get_address(account);
        self.token(token).balance_of(&address)
    }

    pub fn mint(&mut self, token: TokenKind, account: Account, amount: Amount) {
        let owner = self.odra_world.get_address(account);
        self.token(token).mint(&owner, &amount)
    }

    pub fn go_long(&mut self, account: Account, amount: Amount) {
        self.odra_world.set_caller(account);
        let market_address = *self.market().address();
        self.wcspr_token().approve(&market_address, &amount);
        self.market().deposit_long(*amount);
    }

    pub fn go_short(&mut self, account: Account, amount: Amount) {
        self.odra_world.set_caller(account);
        let market_address = *self.market().address();
        self.wcspr_token().approve(&market_address, &amount);
        self.market().deposit_short(*amount);
    }

    pub fn withdraw_long(&mut self, account: Account, amount: Amount) {
        self.odra_world.set_caller(account);
        let market_address = *self.market().address();
        self.long_token().approve(&market_address, &amount);
        self.market().withdraw_long(*amount);
    }

    pub fn withdraw_short(&mut self, account: Account, amount: Amount) {
        self.odra_world.set_caller(account);
        let market_address = *self.market().address();
        self.short_token().approve(&market_address, &amount);
        self.market().withdraw_short(*amount);
    }

    pub fn set_price(&mut self, price: Price) {
        let market = self.market();
        market.set_price(PriceData {
            price: *price,
            timestamp: 0,
        });
    }

    pub fn get_market_state(&mut self) -> MarketState {
        self.market().get_market_state()
    }

    pub fn transfer(
        &mut self,
        kind: TokenKind,
        sender: Account,
        amount: Amount,
        receiver: Account,
    ) {
        let receiver = self.odra_world.get_address(receiver);
        let amount = *amount;
        self.odra_world.set_caller(sender);
        self.token(kind).transfer(&receiver, &amount);
    }
}

#[odra::external_contract]
pub trait Token {
    fn balance_of(&self, address: &Address) -> U256;
    fn transfer(&mut self, recipient: &Address, amount: &U256);
    fn mint(&mut self, owner: &Address, amount: &U256);
}
