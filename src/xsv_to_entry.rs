use regex::RegexBuilder;
use serde::Deserialize;
use tracing::debug;

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
                            "Match for value: {:?} hint: {:?}, value: {:?}",
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
