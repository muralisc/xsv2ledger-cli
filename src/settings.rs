use crate::date::Date;
use crate::exclude_condition::Exclude;
use crate::note::Note;
use crate::payee::Payee;
use crate::posting::Posting;
use crate::state::State;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::collections::HashMap;
use tracing::debug;

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

impl XsvToLedgerRecord {
    pub fn print(&self, record: csv::StringRecord) {
        let tab_as_spaces = "        ".to_string();
        println!(
            "{} {} \"{}\"",
            self.date.get_string(&record),
            match &self.state {
                None => "*".to_string(),
                Some(state) => state.get_string(&record),
            },
            self.payee.get_string(&record)
        );
        if let Some(notes) = &self.notes {
            if let Some(note_str) = notes.get_string(&record) {
                println!("{}; {}", tab_as_spaces, note_str);
            }
        }
        debug!("Getting Target Posing string");
        println!(
            "{}{}",
            tab_as_spaces,
            self.target_posting.get_string(&record)
        );
        debug!("Getting Source Posing string");
        println!(
            "{}{}",
            tab_as_spaces,
            self.source_posting.get_string(&record)
        );
        println!();
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub delimiter: Option<String>,
    pub has_headers: bool,
    pub xsv_to_ledger_record: XsvToLedgerRecord,
    pub exclude: Exclude,
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
