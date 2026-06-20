#!/bin/bash
set -e

cargo run -- --transactions-csv test/test_xsv/fidelity-1999.csv --config config/fidelity.toml | diff test/expected/fidelity.ledger -
\ledger b --permissive -f test/expected/fidelity.ledger | diff test/expected/fidelity.balance -

cargo run -- --transactions-csv test/test_xsv/monzo.csv --config config/monzo.toml | diff test/expected/monzo.ledger -
\ledger b --permissive -f test/expected/monzo.ledger 2>&1 1>/dev/null | sed "s|$(pwd)/||g" | diff test/expected/monzo.balance-err -

cargo run -- --config config/schwab-trades-and-security_transfers.toml --transactions-csv test/test_xsv/schwab-trades.csv | diff test/expected/schwab-trades.ledger -
\ledger b --permissive -f test/expected/schwab-trades.ledger | diff test/expected/schwab-trades.balance -

cargo run -- --config config/amex.toml --transactions-csv test/test_xsv/amex.csv | diff test/expected/amex.ledger -
\ledger b --permissive -f test/expected/amex.ledger | diff test/expected/amex.balance -

cargo run -- --config config/hsbc.toml --transactions-csv test/test_xsv/hsbc.csv | diff test/expected/hsbc.ledger -
\ledger b --permissive -f test/expected/hsbc.ledger 2>&1 1>/dev/null | sed "s|$(pwd)/||g" | diff test/expected/hsbc.balance-err -

# --- mapping tests (use isolated test configs, not prod configs) ---

# All transactions have a mapping: full output matches golden file
cargo run -- --config test/config/test-complete.toml --transactions-csv test/test_xsv/test-simple.csv | diff test/expected/test-complete.ledger -
\ledger b --permissive -f test/expected/test-complete.ledger | diff test/expected/test-complete.balance -

# One transaction has no mapping: partial output matches golden, exit is non-zero
set +e
cargo run -- --config test/config/test-partial.toml --transactions-csv test/test_xsv/test-simple.csv > /tmp/test-partial-actual.ledger 2>/dev/null
partial_exit=$?
set -e
diff test/expected/test-partial.ledger /tmp/test-partial-actual.ledger
[ "$partial_exit" -ne 0 ] || { echo "FAIL: expected non-zero exit for partial config" >&2; exit 1; }
