
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


