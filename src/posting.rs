use crate::price::Price;
use crate::xsv_to_entry::XsvToEntry;
use serde::Deserialize;
use tracing::debug;

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
