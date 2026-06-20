use regex::RegexBuilder;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct Mapping {
    pub key_regex: Option<String>,
    pub key_regex_list: Option<Vec<String>>,
    pub key_regex_file: Option<String>,
    pub value: String,
}

impl Mapping {
    fn build_pattern(&self) -> String {
        if let Some(ref r) = self.key_regex {
            return r.clone();
        }
        if let Some(ref list) = self.key_regex_list {
            return list.join("|");
        }
        if let Some(ref path) = self.key_regex_file {
            let content = std::fs::read_to_string(path)
                .unwrap_or_else(|e| panic!("Cannot read key_regex_file {path}: {e}"));
            return content
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect::<Vec<_>>()
                .join("|");
        }
        panic!("Mapping must specify one of: key_regex, key_regex_list, key_regex_file");
    }
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
            .map(|i| {
                // Remove any newlines in column
                record[*i]
                    .to_string()
                    .split('\n')
                    .map(|l| l.trim().to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" | ");
        if let Some(mapping) = &self.hint_mapping {
            for item in mapping {
                let pattern = item.build_pattern();
                let re = RegexBuilder::new(&pattern)
                    .case_insensitive(true)
                    .build()
                    .unwrap();
                match re.find(&hint_string) {
                    Some(mat) => {
                        debug!(
                            "Match for value: {:?} hint: {:?}, value: {:?}",
                            mat, pattern, item.value
                        );
                        return item.value.to_string();
                    }
                    None => debug!(
                        "First account mapped to None for hint {:?} for regex {:?}",
                        hint_string, pattern
                    ),
                }
            }
        }
        return hint_string;
    }
}
