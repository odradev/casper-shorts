use cucumber::{then, when};

use crate::common::{
    params::{Account, Amount, Price, TokenKind},
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

#[when(expr = "price changes to {price} USD")]
fn set_price(world: &mut CasperShortsWorld, price: Price) {
    world.set_price(price.value());
}

#[then(expr = "price is {price} USD")]
fn check_price(world: &mut CasperShortsWorld, price: Price) {
    let market_state = world.get_market_state();
    assert_eq!(market_state.price, price.value());
}
