
# xsv2ledger

Convert bank,stock broker csv(tsv) files into [ledger-cli](https://ledger-cli.org/) format.

From [ledger-cli.org](https://ledger-cli.org/),

> Ledger is a powerful, double-entry accounting system that is accessed from the
UNIX command-line.

However to use it we need out transactions to be encoded in ledger format. 
This tool lets us do that for any bank / finance handlers (Insurance, Stock
Brokers) who can furnish a csv of transactions

### How does it work

For a transaction snippet from Monzo csv:
```
tx_0000AZPPhBZWW3Pq4dXGYW,04/09/2023,02:02:01,Direct Debit,Hyperoptic,,Bills,-26.50,GBP,-26.50,GBP,HYP000000630597,,,HYP000000630597,,-26.50,
```
to
```
2023-09-04 * "Hyperoptic"
        Assets:Bank:Monzo        -26.50 GBP
        Expenses:Utilities
```

Exmaple csv:
https://github.com/muralisc/xsv2ledger-cli/blob/c9b2205812c0247b571a5ead73baaf681c687531/test/test_xsv/monzo.csv?plain=1#L3-L6
Example snippet:
https://github.com/muralisc/xsv2ledger-cli/blob/c9b2205812c0247b571a5ead73baaf681c687531/test/expected/monzo.ledger#L7-L21

### How to run
```
RUST_BACKTRACE=1 cargo run -- \
    --config $PATH_TO_XSV2LEDGER_CLI/config/monzo.toml \
    --transactions-csv $PATH_TO_CSV/Monzo\ Data\ Export\ -\ July.csv \
    > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_monzo.ledger
```

### Pre-existing config files
- Monzo
- HSBC
- Fidelity UK
- Charles Schwab
- Amex
- SBI (State Bank of India)

Details on how to obtain CSV: [supported-institutions.md](https://github.com/muralisc/xsv2ledger-cli/blob/main/docs/supported-institutions.md)
