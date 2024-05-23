mod common;
mod steps;

use common::world::CasperShortsWorld;
use cucumber::World;

fn main() {
    futures::executor::block_on(CasperShortsWorld::run("tests/features/setup.feature"));
    futures::executor::block_on(CasperShortsWorld::run("tests/features/market.feature"));
}
