// Structs and enums used in the client.

use casper_shorts_contracts::system::MarketState;
use odra::{casper_types::U256, Address};

#[derive(Debug)]
pub enum Token {
    Long,
    Short,
    Wcspr,
}

#[derive(Debug)]
pub enum Recipient {
    LongContract,
    ShortContract,
    WcsprContract,
    Address(Address),
}

#[derive(Debug)]
pub struct TransferOrder {
    pub token: Token,
    pub recipient: Recipient,
    pub amount: U256,
}

#[derive(Debug, Clone)]
pub enum TradingAction {
    GoLong { amount: U256 },
    GoShort { amount: U256 },
    StopLong { amount: U256 },
    StopShort { amount: U256 },
}

impl TradingAction {
    pub fn to_transfer_order(&self) -> TransferOrder {
        match self {
            TradingAction::GoLong { amount } => TransferOrder {
                token: Token::Wcspr,
                recipient: Recipient::LongContract,
                amount: *amount,
            },
            TradingAction::GoShort { amount } => TransferOrder {
                token: Token::Wcspr,
                recipient: Recipient::ShortContract,
                amount: *amount,
            },
            TradingAction::StopLong { amount } => TransferOrder {
                token: Token::Long,
                recipient: Recipient::WcsprContract,
                amount: *amount,
            },
            TradingAction::StopShort { amount } => TransferOrder {
                token: Token::Short,
                recipient: Recipient::WcsprContract,
                amount: *amount,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum BotMode {
    Random,
}

#[derive(Debug)]
pub struct SystemStats {
    pub account: Address,
    pub wcspr_balance: U256,
    pub short_balance: U256,
    pub long_balance: U256,
    pub market_state: MarketState,
}
