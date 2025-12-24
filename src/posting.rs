use serde::Deserialize;
use chrono::NaiveDate;
use regex::RegexBuilder;
use tracing::{debug, info, warn};

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

impl Date {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        let date_str = record[self.column].trim();
        debug!(
            "Processing date string: \"{}\" with regex {}",
            date_str, self.date_regex
        );
        let re = RegexBuilder::new(&format!(
            r"{}",
            self.date_regex
        ))
        .build()
        .unwrap();
        let cleaned_date = re.find(date_str).unwrap();
        debug!("Date matched with regex: {}", cleaned_date.as_str());
        let date = NaiveDate::parse_from_str(
            cleaned_date.as_str(),
            &self.date_format,
        )
        .unwrap();
        return date.to_string();
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct State {
    pub xsv_to_entry: XsvToEntry,
}
impl State {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return "*".to_string();
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Payee {
    pub xsv_to_entry: XsvToEntry,
}
impl Payee {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return "*".to_string();
    }
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
