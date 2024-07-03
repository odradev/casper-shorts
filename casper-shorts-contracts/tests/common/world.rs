use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

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
    contract_def::HasIdent,
    host::{Deployer, HostRef},
    Address,
};
use odra_test::bdd::{
    param::{Account, Amount},
    refs::Cep18TokenHostRef,
    OdraWorld,
};

use super::params::{Price, TokenKind};

const INITIAL_WCSPR_BALANCE: u64 = 1_000_000_000_000u64; // 1000 CSPR
const FEE_COLLECTOR: &str = "FeeCollector";
const MARKET_CONTRACT: &str = "MarketContract";
const SHORT_CONTRACT: &str = "TokenShortContract";
const LONG_CONTRACT: &str = "TokenLongContract";
const WCSPR_CONTRACT: &str = "TokenWCSPRContract";

const STATE_MINT_CALL_COUNT: &str = "mint_call_count";

#[derive(cucumber::World)]
pub struct CasperShortsWorld {
    odra_world: OdraWorld,
}

impl CasperShortsWorld {
    pub fn market(&mut self) -> &mut MarketHostRef {
        self.odra_world.get_contract::<MarketHostRef>()
    }

    pub fn set_config<I: HasIdent>(&mut self, cfg: &Config) {
        let addr = self.odra_world.get_contract_address::<I>();
        ConfigurableHostRef::new(addr, self.odra_world.env().clone()).set_config(cfg);
    }

    fn address(&mut self, account: &str) -> Address {
        self.odra_world
            .get_address(Account::from_str(account).unwrap())
    }

    fn market_account() -> Account {
        Account::from_str(MARKET_CONTRACT).unwrap()
    }

    pub fn cep18(&mut self, kind: TokenKind) -> Cep18TokenHostRef {
        match kind {
            TokenKind::WCSPR => self.odra_world.cep18::<TokenWCSPRHostRef>(),
            TokenKind::SHORT => self.odra_world.cep18::<TokenShortHostRef>(),
            TokenKind::LONG => self.odra_world.cep18::<TokenLongHostRef>(),
        }
    }

    pub fn set_call_count(&mut self, count: u64) {
        self.odra_world
            .set_state(STATE_MINT_CALL_COUNT.to_string(), count);
    }

    pub fn get_call_count(&mut self) -> &u64 {
        self.odra_world.get_state::<u64>(STATE_MINT_CALL_COUNT)
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

        let mut world = CasperShortsWorld { odra_world };
        world.set_call_count(0);
        // Update addresses.
        let cfg = Config {
            wcspr_token: world.address(WCSPR_CONTRACT),
            short_token: world.address(SHORT_CONTRACT),
            long_token: world.address(LONG_CONTRACT),
            market: world.address(MARKET_CONTRACT),
            fee_collector: world.address(FEE_COLLECTOR),
        };

        world.set_config::<MarketHostRef>(&cfg);
        world.set_config::<TokenShortHostRef>(&cfg);
        world.set_config::<TokenLongHostRef>(&cfg);
        world.set_config::<TokenWCSPRHostRef>(&cfg);

        let market_account = Account::from_str(MARKET_CONTRACT).unwrap();
        // Make market minter of LONG and SHORT tokens.
        world.change_security(TokenKind::SHORT, market_account.clone());
        world.change_security(TokenKind::LONG, market_account.clone());

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
        self.cep18(token).balance_of(&account)
    }

    pub fn mint(&mut self, token: TokenKind, account: Account, amount: Amount) {
        let s = *self.get_call_count();
        self.set_call_count(s + 1);
        self.cep18(token).mint(&account, &amount);
    }

    pub fn change_security(&mut self, token: TokenKind, account: Account) {
        self.cep18(token)
            .change_security(vec![], vec![account], vec![]);
    }

    pub fn go_long(&mut self, account: Account, amount: Amount) {
        let spender = Self::market_account();
        self.odra_world
            .with_caller(account)
            .cep18::<TokenWCSPRHostRef>()
            .approve(&spender, &amount);
        self.market().deposit_long(*amount);
    }

    pub fn go_short(&mut self, account: Account, amount: Amount) {
        let spender = Self::market_account();
        self.odra_world
            .with_caller(account)
            .cep18::<TokenWCSPRHostRef>()
            .approve(&spender, &amount);
        self.market().deposit_short(*amount);
    }

    pub fn withdraw(&mut self, kind: TokenKind, account: Account, amount: Amount) {
        let spender = Self::market_account();
        self.odra_world
            .with_caller(account)
            .cep18::<TokenLongHostRef>()
            .approve(&spender, &amount);
        match kind {
            TokenKind::LONG => self.market().withdraw_long(*amount),
            TokenKind::SHORT => self.market().withdraw_short(*amount),
            TokenKind::WCSPR => panic!("Cannot withdraw using WCSPR"),
        }
    }

    pub fn set_price(&mut self, price: Price) {
        self.market().set_price(PriceData {
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
        self.odra_world.set_caller(sender);
        self.cep18(kind).transfer(&receiver, &amount);
    }
}

#[odra::external_contract]
trait Configurable {
    fn set_config(&mut self, cfg: &Config);
}
