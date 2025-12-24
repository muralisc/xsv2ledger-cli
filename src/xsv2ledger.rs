use crate::settings::{Settings};
use crate::exclude_condition::Exclude;
use crate::exclude_condition::ConditionTypes;

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
        return self.settings.exclude.exclude(&record);
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

            self.settings.xsv_to_ledger_record.print(record);
        }
    }
}
