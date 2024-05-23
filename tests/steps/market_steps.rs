use cucumber::when;

use crate::common::{
    params::{Account, Amount, TokenKind},
    world::CasperShortsWorld,
};

#[when(expr = "{account} goes long with {amount} WCSPR")]
fn go_long(world: &mut CasperShortsWorld, account: Account, amount: Amount) {
    world.go_long(account, amount.value());
}

#[when(expr = "{account} goes short with {amount} WCSPR")]
fn go_short(world: &mut CasperShortsWorld, account: Account, amount: Amount) {
    world.go_short(account, amount.value());
}

// When Alice withdraws 50 LONG
#[when(expr = "{account} withdraws {amount} {token_kind}")]
fn withdraw_long(
    world: &mut CasperShortsWorld,
    account: Account,
    amount: Amount,
    token: TokenKind,
) {
    match token {
        TokenKind::LONG => world.withdraw_long(account, amount.value()),
        TokenKind::SHORT => world.withdraw_short(account, amount.value()),
        TokenKind::WCSPR => panic!("Cannot withdraw using WCSPR"),
    }
}
