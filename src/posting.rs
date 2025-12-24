use chrono::NaiveDate;
use crate::exclude_condition::Exclude;
use regex::RegexBuilder;
use serde::Deserialize;
use tracing::{debug, info, warn};

#[derive(Debug, Deserialize, Clone)]
pub struct Mapping {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct XsvToEntry {
    // Columns which help identify this entry
    pub hint_columns: Vec<usize>,
    pub hint_mapping: Option<Vec<Mapping>>,
}

impl XsvToEntry {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        let hint_string = self
            .hint_columns
            .iter()
            .map(|i| record[*i].to_string())
            .collect::<Vec<String>>()
            .join(" | ");
        if let Some(mapping) = &self.hint_mapping {
            for item in mapping {
                let re = RegexBuilder::new(&format!(r"{}", item.key))
                    .case_insensitive(true)
                    .build()
                    .unwrap();
                match re.find(&hint_string) {
                    Some(mat) => {
                        debug!(
                            "Match for first account: {:?} hint: {:?}, value: {:?}",
                            mat, item.key, item.value
                        );
                        return item.value.to_string();
                    }
                    None => debug!(
                        "First account mapped to None for hint {:?} for regex {:?}",
                        hint_string, item.key
                    ),
                }
            }
        }
        return hint_string;
    }
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
        return self.xsv_to_entry.get_string(&record);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Quantity {
    pub xsv_to_entry: XsvToEntry,
}

impl Quantity {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return self.xsv_to_entry.get_string(&record);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commodity {
    pub xsv_to_entry: XsvToEntry,
}

impl Commodity {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return self.xsv_to_entry.get_string(&record);
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Amount {
    pub quantity: Quantity,
    pub commodity: Commodity,
}

impl Amount {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return format!(
            "{} {}",
            self.quantity.get_string(&record),
            self.commodity.get_string(&record)
        );
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CostAmount {
    pub amount: Amount,
    pub exclude: Exclude
}

impl CostAmount {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        if self.exclude.exclude(&record) {
            return "".to_string();
        }
        return format!("@ {}", self.amount.get_string(&record));
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    pub amount: Amount,
    pub cost_amount: Option<CostAmount>,
}

impl Price {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        let amount_str = self.amount.get_string(&record);
        if let Some(cost_amount) = &self.cost_amount {
            return format!("{} {}", amount_str, cost_amount.get_string(&record));
        }
        return amount_str;
    }
}

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
