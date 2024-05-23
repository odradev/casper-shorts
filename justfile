test-lib:
    cargo test --lib

test-market:
    ODRA_MODULE=Cep18 cargo odra test -- --test test_casper_shorts

test-market-casper:
    ODRA_MODULE=Cep18 cargo odra test -b casper -- --test test_casper_shorts

clippy:
    cargo clippy --all-targets -- -D warnings

lint:
    cargo fmt

check-lint: clippy
    cargo fmt -- --check

plots:
    cargo run --bin plots -F plots