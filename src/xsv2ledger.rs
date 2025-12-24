use crate::ledger_entry::LedgerEntry;
use crate::settings::{ExcludeCondition, Settings};

use tracing::{debug, info, warn};

pub struct Xsv2Ledger {
    settings: Settings,
    transactions_file_path: String,
}

impl Xsv2Ledger {
    pub fn new(settings: Settings, filepath: String) -> Self {
        Xsv2Ledger {
            settings: settings,
            transactions_file_path: filepath,
        }
    }

    fn should_exclude(&self, record: &csv::StringRecord) -> bool {
        let mut should_exclude: bool = false;
        for exclude_condition in &self.settings.exclude_conditions {
            debug!("Checking Excluding condition: {:?}", exclude_condition);
            match exclude_condition {
                ExcludeCondition::ColumnContainsValue {
                    column,
                    value,
                    operation,
                } => {
                    let column_under_check = &record[*column];
                    if operation == "contains" {
                        if column_under_check.contains(&*value.as_str()) {
                            should_exclude = true;
                        }
                    } else if operation == "equal" {
                        if column_under_check == *value {
                            should_exclude = true;
                        }
                    }
                }
                ExcludeCondition::RecordLen(record_len) => {
                    if *record_len == record.len() {
                        should_exclude = true
                    }
                    debug!(
                        "Excluding condition: {:?}, record len : {}, should_exclude: {}",
                        record_len,
                        record.len(),
                        should_exclude
                    );
                }
            }
            if should_exclude == true {
                break;
            }
        }
        return should_exclude;
    }

    pub fn print(&self) {
        let mut reader = match csv::ReaderBuilder::new()
            .has_headers(self.settings.has_headers)
            .flexible(true)
            .delimiter(self.settings.get_delimiter())
            .from_path(&self.transactions_file_path)
        {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        for record in reader.records() {
            let record = record.unwrap();
            debug!("<========== Starting processsing of new record/row ============>");
            debug!("Record : {:?}!", record);
            debug!("Length of Row/Record {}", record.len());

            if self.should_exclude(&record) {
                info!("Excluding row: {:?}", record);
                continue;
            }

            let le = LedgerEntry::new(self.settings.clone(), record);
            le.print();
        }
    }
}
