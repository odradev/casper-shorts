test:
    ODRA_MODULE=Cep18 cargo odra test -- --test test_casper_shorts

test-casper:
    ODRA_MODULE=Cep18 cargo odra test -b casper -- --test test_casper_shorts

clippy:
    cargo clippy --all-targets -- -D warnings

lint:
    cargo fmt

check-lint: clippy
    cargo fmt -- --check
