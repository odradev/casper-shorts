use crate::common::{
    params::{Account, Amount, TokenKind},
    world::CasperShortsWorld,
};
use cucumber::then;

#[then(expr = "{account} has {amount} {token_kind}")]
fn balance_check(
    world: &mut CasperShortsWorld,
    account: Account,
    amount: Amount,
    token_kind: TokenKind,
) {
    let balance = world.balance_of(token_kind, account);
    let error_msg = format!(
        "{:?} has {} {:?} but expected {}",
        account,
        balance,
        token_kind,
        amount.value()
    );
    assert_eq!(amount.value(), balance, "{}", error_msg);
}
