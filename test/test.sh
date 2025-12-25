#!/bin/sh
cargo run -- --transactions-csv test/test_xsv/fidelity-1999.csv --config config/fidelity.toml | diff test/expected/fidelity.ledger -
