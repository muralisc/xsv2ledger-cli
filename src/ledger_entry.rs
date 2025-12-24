use crate::posting::Posting;
use crate::settings::Settings;
use chrono::NaiveDate;
use regex::RegexBuilder;
use tracing::{debug, info, warn};

pub struct LedgerEntry {
    tab_as_spaces: String,
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
            tab_as_spaces: "        ".to_string(),
            settings: settings,
            record: record,
        }
    }


    fn get_payee(&self) -> String {
        let payee_string = self
            .settings
            .xsv_to_ledger_record
            .payee
            .xsv_to_entry
            .hint_columns
            .iter()
            .map(|i| self.record[*i].to_string())
            .collect::<Vec<String>>()
            .join(" | ");
        return payee_string;
    }

    fn get_posting(&self, posting: &Posting) -> String {
        return "".to_string();
    }

    fn get_target_posting(&self) -> String {
        return self.get_posting(&self.settings.xsv_to_ledger_record.target_posting)
    }

    fn get_source_posting(&self) -> String {
        return "".to_string();
    }

    fn get_notes(&self) -> Option<String> {
        if let Some(notes) = &self.settings.xsv_to_ledger_record.notes {
            let note_string = notes
                .xsv_to_entry
                .hint_columns
                .iter()
                .map(|i| self.record[*i].to_string())
                .collect::<Vec<String>>()
                .join(" | ");
            if note_string.len() == 0 {
                return None;
            }
            return Some(note_string);
        }
        return None;
    }

    pub fn print(&self) {

        // println!(
        //     "{} {} \"{}\"",
        //     self.get_date(),
        //     self.get_state(),
        //     self.get_payee()
        // );

        if let Some(notes) = self.get_notes() {
            println!("{}; {}", self.tab_as_spaces, notes);
        }
        println!("{}{}", self.tab_as_spaces, self.get_target_posting(),);
        println!("{}{}", self.tab_as_spaces, self.get_source_posting());
        println!();
    }
}
