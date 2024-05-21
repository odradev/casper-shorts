use std::fmt::{Debug, Formatter};

use odra::{
    casper_types::U256,
    host::{Deployer, HostEnv},
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
}

impl Default for CasperShortsWorld {
    fn default() -> Self {
        let odra_env = odra_test::env();

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

        let mut world = CasperShortsWorld {
            wcspr_token,
            odra_env,
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
            _ => panic!("Unsupported token kind"),
        }
    }

    fn token_mut(&mut self, token: TokenKind) -> &mut Cep18HostRef {
        match token {
            TokenKind::WCSPR => &mut self.wcspr_token,
            _ => panic!("Unsupported token kind"),
        }
    }

    pub fn address(&self, account: Account) -> Address {
        self.odra_env.get_account(account.index())
    }

    pub fn balance_of(&self, token: TokenKind, account: Account) -> U256 {
        let address = self.address(account);
        self.token(token).balance_of(&address)
    }

    pub fn mint(&mut self, token: TokenKind, account: Account, amount: U256) {
        let address = self.address(account);
        self.token_mut(token).mint(&address, &amount);
    }
}
