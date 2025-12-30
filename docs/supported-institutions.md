# Supported Institutions

## Charles Schwab

### How to Get CSV file ?

Schwab > Accounts > History > Export > Filter Transcation Types {trades, non-trades}

### Command to run
Trades:
```
RUST_BACKTRACE=1 cargo run -- \
  --config $PATH_TO_XSV2LEDGER_CLI/config/schwab-trades.toml \
  --transactions-csv $PATH_TO_CSV/schwab_Transactions_20230409-103308.csv \
  > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_schwab.ledger
```

```
RUST_BACKTRACE=1 cargo run -- \
  --config $PATH_TO_XSV2LEDGER_CLI/config/schwab-non-trades.toml \
  --transactions-csv $PATH_TO_CSV/schwab_Transactions_20230409-103308.csv \
  > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_schwab.ledger
```

## HSBC

### How to get TrasactionHistory file for HSBC ?

From web portal
1. [Click](assets/hsbc/1_click.png)
2. [Filter](assets/hsbc/2_Filter_for_dates.png)
3. [Download](assets/hsbc/3_download.png)


### Command to run
```
RUST_BACKTRACE=1 cargo run -- \
    --config $PATH_TO_XSV2LEDGER_CLI/config/hsbc.toml \
    --transactions-csv $PATH_TO_CSV/hsbc_TrasactionHistory_09_April_2023.csv \
    > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_hsbc.ledger
```



## Amex


### How to get TrasactionHistory file for AMEX ?

- [Statements & Activity] > Previous Billing Periods > choose window (11 Jul to 10 Aug) > CSV (Check for all details)

### Command to run
```
RUST_BACKTRACE=1 cargo run -- \
    --config $PATH_TO_XSV2LEDGER_CLI/config/amex.toml \
    --transactions-csv $PATH_TO_CSV/amex-dec.csv \
    > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_amex.ledger
```

## Monzo

### Command to run
```
RUST_BACKTRACE=1 cargo run -- \
    --config $PATH_TO_XSV2LEDGER_CLI/config/monzo.toml \
    --transactions-csv $PATH_TO_CSV/Monzo\ Data\ Export\ -\ July.csv \
    > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_monzo.ledger
```


## Identify new Account mappings

grep for WARN in log file
```
grep "WARN" xsv2ledger.debug.log
```

## SBI

Export as XlS, no need to convert it as CSV, XLS == TSV
```
RUST_BACKTRACE=1 cargo run -- \
    --config ~/src/bank2ledger-cli/config/sbi.toml \
    --transactions-csv ~/shared_folders/transfer_work/sbi-12dec.xls > ledger_2023_12Dec_sbi.txt
```
