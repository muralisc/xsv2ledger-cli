use crate::settings::Settings;
use chrono::NaiveDate;
use regex::RegexBuilder;
use tracing::{debug, info, warn};

pub struct LedgerEntry {
    settings: Settings,
    record: csv::StringRecord,
    // date: NaiveDate,
    // state: String,
    // payee: String,
    // comment: Option<String>,
    // target_posting: String,
    // source_posting: String,
}

impl LedgerEntry {
    pub fn new(settings: Settings, record: csv::StringRecord) -> Self {
        LedgerEntry {
            settings: settings,
            record: record,
        }
    }

    fn get_date(&self) -> NaiveDate {
        // let date_str = self.record[self.settings.xsv_to_ledger_record.date.xsv_to_entry.hint_columns].trim();
        let date_str = self.record[self.settings.xsv_to_ledger_record.date.column].trim();
        debug!(
            "Processing date string: \"{}\" with regex {}",
            date_str, self.settings.xsv_to_ledger_record.date.date_regex
        );
        let re = RegexBuilder::new(&format!(
            r"{}",
            self.settings.xsv_to_ledger_record.date.date_regex
        ))
        .build()
        .unwrap();
        let cleaned_date = re.find(date_str).unwrap();
        debug!("Date matched with regex: {}", cleaned_date.as_str());
        return NaiveDate::parse_from_str(
            cleaned_date.as_str(),
            &self.settings.xsv_to_ledger_record.date.date_format,
        )
        .unwrap();
    }

    fn get_state(&self) -> String {
        return "*".to_string();
    }

    fn get_payee(&self) -> String {
        return "".to_string();
    }

    fn get_target_posting(&self) -> String {
        return "".to_string();
    }

    fn get_source_posting(&self) -> String {
        return "".to_string();
    }

    fn get_comment(&self) -> String {
        return "".to_string();
    }

    pub fn print(&self) {
        let tab_as_spaces = "        ";

        println!(
            "{} {} \"{}\"",
            self.get_date(),
            self.get_state(),
            self.get_payee()
        );

        println!("{}; {}", tab_as_spaces, self.get_comment());
        println!("{}{}", tab_as_spaces, self.get_target_posting(),);
        println!("{}{}", tab_as_spaces, self.get_source_posting());
        println!();
    }
}
