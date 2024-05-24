use std::{fmt::Display, str::FromStr};

use casper_shorts_contracts::system::ONE_DOLLAR;
use cucumber::Parameter;
use odra::casper_types::U256;

#[derive(Debug, Parameter, Clone, Copy)]
#[param(name = "account", regex = ".+")]
pub enum Account {
    Alice = 1,
    Bob = 2,
    Charlie = 3,
    FeeCollector = 4,
    MarketContract = 100,
    LongContract = 101,
    ShortContract = 102,
    WCSPRContract = 103,
}

impl FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Alice" => Ok(Account::Alice),
            "Bob" => Ok(Account::Bob),
            "Charlie" => Ok(Account::Charlie),
            "FeeCollector" => Ok(Account::FeeCollector),
            "LongContract" => Ok(Account::LongContract),
            "ShortContract" => Ok(Account::ShortContract),
            "WCSPRContract" => Ok(Account::WCSPRContract),
            "MarketContract" => Ok(Account::MarketContract),
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
    SHORT,
    LONG,
    WCSPR,
}

impl FromStr for TokenKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SHORT" => Ok(TokenKind::SHORT),
            "LONG" => Ok(TokenKind::LONG),
            "WCSPR" => Ok(TokenKind::WCSPR),
            _ => Err(format!("Invalid token kind: {}", s)),
        }
    }
}

#[derive(Debug, Parameter, Clone, Copy)]
#[param(name = "amount", regex = ".+")]
pub struct Amount(pub U256);

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

    pub fn as_f64(&self) -> f64 {
        let num = self.0.as_u64();
        num as f64 / 1_000_000_000f64
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

#[derive(Debug, Parameter, Clone, Copy)]
#[param(name = "price", regex = ".+")]
pub struct Price(pub U256);

impl FromStr for Price {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = (s.parse::<f64>().unwrap() * ONE_DOLLAR as f64) as u64;
        let num = U256::from(num);
        Ok(Self(num))
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

impl Price {
    pub fn value(&self) -> U256 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        let num = self.0.as_u64();
        num as f64 / ONE_DOLLAR as f64
    }
}
