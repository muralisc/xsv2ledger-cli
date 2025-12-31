use crate::price::Price;
use crate::xsv_to_entry::XsvToEntry;
use chrono::NaiveDate;
use regex::RegexBuilder;
use serde::Deserialize;
use tracing::debug;

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
        let re = RegexBuilder::new(&format!(r"{}", self.date_regex))
            .build()
            .unwrap();
        let cleaned_date = re.find(date_str).unwrap();
        debug!("Date matched with regex: {}", cleaned_date.as_str());
        let date = NaiveDate::parse_from_str(cleaned_date.as_str(), &self.date_format).unwrap();
        return date.to_string();
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct State {}
impl State {
    pub fn get_string(&self, _record: &csv::StringRecord) -> String {
        return "*".to_string();
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Payee {
    pub xsv_to_entry: XsvToEntry,
}
impl Payee {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return self.xsv_to_entry.get_string(&record);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Note {
    pub xsv_to_entry: XsvToEntry,
}

impl Note {
    pub fn get_string(&self, record: &csv::StringRecord) -> Option<String> {
        let note_string = self
            .xsv_to_entry
            .hint_columns
            .iter()
            .map(|i| record[*i].to_string())
            .collect::<Vec<String>>()
            .join(" | ");
        if note_string.len() == 0 {
            return None;
        }
        return Some(note_string);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    pub xsv_to_entry: XsvToEntry,
}

impl Account {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        debug!("Getting Account string");
        return self.xsv_to_entry.get_string(&record);
    }
}

// Posting:
//      -> Account [Price]
//      -> Account [Amount [CostAmount]]
//      -> Account [Amount [@ Amount]]
//      -> Account [Quantity Commodity [@ Quantity Commodity]]
#[derive(Debug, Deserialize, Clone)]
pub struct Posting {
    pub account: Account,
    pub price: Option<Price>,
}

impl Posting {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        let account_str = self.account.get_string(&record);
        if let Some(price) = &self.price {
            return format!("{}        {}", account_str, price.get_string(&record));
        }
        return account_str;
    }
}
