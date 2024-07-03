use crate::common::{params::TokenKind, utils::BalanceCheck, world::CasperShortsWorld};
use cucumber::then;
use odra_test::bdd::param::{Account, Amount};

#[then(expr = "{account} has {amount} {token_kind}")]
fn balance_check(
    world: &mut CasperShortsWorld,
    account: Account,
    amount: Amount,
    token_kind: TokenKind,
) {
    let balance = world.balance_of(token_kind, account.clone());
    let diff = balance.abs_diff(*amount);

    let error_msg = format!(
        "{:?} has {} {:?} but expected {} {:?}. Diff: {}",
        account,
        Amount::from(balance),
        token_kind,
        amount,
        token_kind,
        Amount::from(diff),
    );

    assert!(balance.is_close_to(&amount), "{}", error_msg);
}
