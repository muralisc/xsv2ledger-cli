use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct XsvToEntry {
    // Columns which help identify this entry
    pub hint_columns: Vec<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Date {
    pub column: usize,
    pub date_format: String,
    pub date_regex: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct State {
    pub xsv_to_entry: XsvToEntry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Payee {
    pub xsv_to_entry: XsvToEntry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Note {
    pub xsv_to_entry: XsvToEntry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    pub xsv_to_entry: XsvToEntry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Quantity {
    pub xsv_to_entry: XsvToEntry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commodity {
    pub xsv_to_entry: XsvToEntry,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Amount {
    pub quantity: Quantity,
    pub commodity: Commodity,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<Amount>,
    pub cost_amount: Option<Amount>,
}

// Date State Payee
//      ; Note
//      Posting (Account Amount @ CostAmount)
//      Posting (Account )
//
#[derive(Debug, Deserialize, Clone)]
pub struct XsvToLedgerRecord {
    pub date: Date,
    pub state: Option<State>,
    pub payee: Payee,
    pub notes: Option<Note>,
    // From manual:
    // It is a general convention within Ledger that the “top” postings
    // in a transaction contain the target accounts,
    // while the final posting contains the source account.
    pub target_posting: Posting,
    pub source_posting: Posting,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum ExcludeCondition {
    ColumnContainsValue {
        column: usize,
        value: String,
        operation: String,
    },
    RecordLen(usize),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub delimiter: Option<String>,
    pub has_headers: bool,
    pub xsv_to_ledger_record: XsvToLedgerRecord,
    pub exclude_conditions: Vec<ExcludeCondition>,
}

impl Settings {
    pub fn new(config_file: &String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&config_file))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }

    pub fn get_delimiter(&self) -> u8 {
        let delimiter_map: HashMap<&str, u8> =
            HashMap::from([(",", b','), ("comma", b','), ("tab", b'\t')]);
        match &self.delimiter {
            None => delimiter_map["comma"],
            Some(delimiter_string) => delimiter_map[&delimiter_string as &str],
        }
    }
}
