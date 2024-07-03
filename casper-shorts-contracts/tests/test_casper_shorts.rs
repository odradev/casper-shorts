mod common;
mod steps;

use common::world::CasperShortsWorld;
use odra_test::bdd::run;

fn main() {
    // run::<CasperShortsWorld, _>("tests/features/transfer_interface.feature");
    // run::<CasperShortsWorld, _>("tests/features/setup.feature");
    run::<CasperShortsWorld, _>("tests/features/market.feature");
}
