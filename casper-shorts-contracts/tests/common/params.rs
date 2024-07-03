use std::{fmt::Display, ops::Deref, str::FromStr};

use casper_shorts_contracts::system::ONE_DOLLAR;
use cucumber::Parameter;
use odra::casper_types::U256;

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
#[param(name = "price", regex = ".+")]
pub struct Price(U256);

impl FromStr for Price {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse::<f64>().expect("Should be a number");
        let num = U256::from((num * ONE_DOLLAR as f64).round() as u64);
        Ok(Self(num))
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

impl Price {
    pub fn as_f64(&self) -> f64 {
        let num = self.0.as_u64();
        num as f64 / ONE_DOLLAR as f64
    }
}

impl Deref for Price {
    type Target = U256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<U256> for Price {
    fn eq(&self, other: &U256) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Price> for U256 {
    fn eq(&self, other: &Price) -> bool {
        *self == other.0
    }
}
