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
    cargo run -p casper-shorts-charts

deploy-contracts:
    cargo run -p casper-shorts-client deploy-contracts
    cargo run -p casper-shorts-client set-security

update-price:
    cargo run -p casper-shorts-client update-price

update-price-deamon:
    cargo run -p casper-shorts-client update-price-deamon 1