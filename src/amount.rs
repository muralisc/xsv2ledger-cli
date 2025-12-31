use crate::xsv_to_entry::XsvToEntry;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct Quantity {
    pub xsv_to_entry: XsvToEntry,
}

impl Quantity {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return self.xsv_to_entry.get_string(&record).replace("$", "");
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
        debug!("Getting Amount string");
        return format!(
            "{} {}",
            self.quantity.get_string(&record),
            self.commodity.get_string(&record)
        );
    }
}
