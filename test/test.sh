#!/bin/sh
cargo run -- --transactions-csv test/test_xsv/fidelity-1999.csv --config config/fidelity.toml | diff test/expected/fidelity.ledger -

cargo run -- --transactions-csv test/test_xsv/monzo.csv --config config/monzo.toml | diff test/expected/monzo.ledger -


cargo run -- --config config/schwab-trades.toml --transactions-csv test/test_xsv/schwab-trades.csv | diff test/expected/schwab-trades.ledger -

