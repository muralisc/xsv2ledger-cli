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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xsv_to_entry::XsvToEntry;

    fn note(cols: Vec<usize>) -> Note {
        Note { xsv_to_entry: XsvToEntry { hint_columns: cols, hint_mapping: None } }
    }

    fn rec(fields: &[&str]) -> csv::StringRecord {
        csv::StringRecord::from(fields.to_vec())
    }

    #[test]
    fn single_column() {
        assert_eq!(note(vec![0]).get_string(&rec(&["hello"])), Some("hello".to_string()));
    }

    #[test]
    fn multiple_columns_joined() {
        assert_eq!(
            note(vec![0, 1]).get_string(&rec(&["hello", "world"])),
            Some("hello | world".to_string())
        );
    }

    #[test]
    fn empty_single_column_returns_none() {
        assert_eq!(note(vec![0]).get_string(&rec(&[""])), None);
    }
}
