use cucumber::{then, when};
use odra_test::bdd::param::{Account, Amount};

use crate::common::{
    params::{Price, TokenKind},
    world::CasperShortsWorld,
};

#[when(expr = "{account} goes long with {amount} WCSPR")]
fn go_long(world: &mut CasperShortsWorld, account: Account, amount: Amount) {
    world.go_long(account, amount);
}

#[when(expr = "{account} goes short with {amount} WCSPR")]
fn go_short(world: &mut CasperShortsWorld, account: Account, amount: Amount) {
    world.go_short(account, amount);
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
        TokenKind::LONG => world.withdraw_long(account, amount),
        TokenKind::SHORT => world.withdraw_short(account, amount),
        TokenKind::WCSPR => panic!("Cannot withdraw using WCSPR"),
    }
}

#[when(expr = "price changes to {price} USD")]
fn set_price(world: &mut CasperShortsWorld, price: Price) {
    world.set_price(price);
}

#[then(expr = "price is {price} USD")]
fn check_price(world: &mut CasperShortsWorld, price: Price) {
    let market_state = world.get_market_state();
    assert_eq!(market_state.price, *price);
}

#[when(expr = "{account} transfers {amount} {token_kind} to {account}")]
fn transfer(
    world: &mut CasperShortsWorld,
    sender: Account,
    amount: Amount,
    token: TokenKind,
    receiver: Account,
) {
    world.transfer(token, sender, amount, receiver);
}
