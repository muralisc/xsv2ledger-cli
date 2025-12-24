use crate::settings::Settings;
use chrono::NaiveDate;

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
    pub fn new(
        settings: Settings,
        record: csv::StringRecord
    ) -> Self {
        LedgerEntry {
            settings: settings,
            record: record
        }
    }


    fn get_date(&self) -> NaiveDate {
        return NaiveDate::from_ymd_opt(2015, 6, 26).unwrap();
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

        println!("{} {} \"{}\"", self.get_date(), self.get_state(), self.get_payee());

        println!("{}; {}", tab_as_spaces, self.get_comment());
        println!(
            "{}{}",
            tab_as_spaces,
            self.get_target_posting(),
        );
        println!("{}{}", tab_as_spaces, self.get_source_posting());
        println!();
    }
}
