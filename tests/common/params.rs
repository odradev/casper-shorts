use std::str::FromStr;

use cucumber::Parameter;
use odra::casper_types::U256;

#[derive(Debug, Parameter, Clone, Copy)]
#[param(name = "account", regex = ".+")]
pub enum Account {
    Alice = 1,
    Bob = 2,
}

impl FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Alice" => Ok(Account::Alice),
            "Bob" => Ok(Account::Bob),
            _ => Err(format!("Invalid account: {}", s)),
        }
    }
}

impl Account {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Parameter, Clone, Copy)]
#[param(name = "token_kind", regex = ".+")]
pub enum TokenKind {
    ShortToken,
    LongToken,
    WCSPR,
}

impl FromStr for TokenKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ShortToken" => Ok(TokenKind::ShortToken),
            "LongToken" => Ok(TokenKind::LongToken),
            "WCSPR" => Ok(TokenKind::WCSPR),
            _ => Err(format!("Invalid token kind: {}", s)),
        }
    }
}

#[derive(Debug, Parameter, Clone, Copy)]
#[param(name = "amount", regex = r"\d+")]
pub struct Amount(U256);

impl FromStr for Amount {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = (s.parse::<f64>().unwrap() * 1_000_000_000f64) as u64;
        let num = U256::from(num);
        Ok(Self(num))
    }
}

impl Amount {
    pub fn value(&self) -> U256 {
        self.0
    }
}
