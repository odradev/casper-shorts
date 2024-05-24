use crate::common::{
    params::{Account, Amount, TokenKind},
    world::CasperShortsWorld,
};
use cucumber::then;
use odra::casper_types::U256;

#[then(expr = "{account} has {amount} {token_kind}")]
fn balance_check(
    world: &mut CasperShortsWorld,
    account: Account,
    amount: Amount,
    token_kind: TokenKind,
) {
    let balance = world.balance_of(token_kind, account);
    let diff = balance.abs_diff(amount.value());

    let error_msg = format!(
        "{:?} has {} {:?} but expected {} {:?}. Diff: {}",
        account,
        Amount(balance),
        token_kind,
        amount,
        token_kind,
        Amount(diff)
    );

    assert!(diff < U256::from(10_000), "{}", error_msg);
}
