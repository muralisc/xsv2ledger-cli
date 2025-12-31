use crate::xsv_to_entry::XsvToEntry;
use serde::Deserialize;

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
