use crate::xsv_to_entry::XsvToEntry;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Payee {
    pub xsv_to_entry: XsvToEntry,
}

impl Payee {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return self.xsv_to_entry.get_string(&record);
    }
}
