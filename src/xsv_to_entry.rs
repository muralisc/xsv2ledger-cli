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

#[cfg(test)]
mod tests {
    use super::*;

    fn mapping(key_regex: &str, value: &str) -> Mapping {
        Mapping {
            key_regex: Some(key_regex.to_string()),
            key_regex_list: None,
            key_regex_file: None,
            value: value.to_string(),
        }
    }

    fn entry(cols: Vec<usize>, maps: Option<Vec<Mapping>>) -> XsvToEntry {
        XsvToEntry { hint_columns: cols, hint_mapping: maps }
    }

    fn rec(fields: &[&str]) -> csv::StringRecord {
        csv::StringRecord::from(fields.to_vec())
    }

    #[test]
    fn build_pattern_key_regex() {
        let m = Mapping {
            key_regex: Some("foo|bar".to_string()),
            key_regex_list: None,
            key_regex_file: None,
            value: "v".to_string(),
        };
        assert_eq!(m.build_pattern(), "foo|bar");
    }

    #[test]
    fn build_pattern_key_regex_list() {
        let m = Mapping {
            key_regex: None,
            key_regex_list: Some(vec!["foo".to_string(), "bar".to_string()]),
            key_regex_file: None,
            value: "v".to_string(),
        };
        assert_eq!(m.build_pattern(), "foo|bar");
    }

    #[test]
    fn build_pattern_key_regex_file() {
        let path = std::env::temp_dir().join("xsv2ledger_test_key_regex_file.txt");
        std::fs::write(&path, "# comment\nfoo\n\nbar\n").unwrap();
        let m = Mapping {
            key_regex: None,
            key_regex_list: None,
            key_regex_file: Some(path.to_str().unwrap().to_string()),
            value: "v".to_string(),
        };
        assert_eq!(m.build_pattern(), "foo|bar");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn get_string_no_mapping_returns_hint() {
        let e = entry(vec![0, 1], None);
        assert_eq!(e.get_string(&rec(&["hello", "world"])), "hello | world");
    }

    #[test]
    fn get_string_mapping_hit_returns_value() {
        let e = entry(vec![0], Some(vec![mapping("hello", "Expenses:Food")]));
        assert_eq!(e.get_string(&rec(&["hello"])), "Expenses:Food");
    }

    #[test]
    fn get_string_mapping_miss_returns_hint() {
        let e = entry(vec![0], Some(vec![mapping("xyz", "Expenses:Food")]));
        assert_eq!(e.get_string(&rec(&["hello"])), "hello");
    }

    #[test]
    fn get_string_strips_newlines_in_column() {
        let e = entry(vec![0], None);
        assert_eq!(e.get_string(&rec(&["hello\nworld"])), "helloworld");
    }
}
