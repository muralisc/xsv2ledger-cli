use crate::invert_sign::InvertSign;
use crate::xsv_to_entry::XsvToEntry;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct Quantity {
    pub invert_sign: Option<InvertSign>,
    pub xsv_to_entry: XsvToEntry,
}

impl Quantity {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        let mut quantity = self.xsv_to_entry.get_string(&record).replace("$", "");
        if let Some(invert_sign) = &self.invert_sign {
            if invert_sign.invert(record) {
                quantity = if quantity.starts_with('+') {
                    format!("-{}", &quantity[1..])
                } else if quantity.starts_with('-') {
                    format!("+{}", &quantity[1..])
                } else {
                    format!("-{}", quantity)
                };
            }
        }
        return quantity;
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
