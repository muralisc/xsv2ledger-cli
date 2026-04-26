use regex::RegexBuilder;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum ConditionTypes {
    ColumnContainsValue {
        column: usize,
        value: String,
        operation: String,
    },
    RecordLen(usize),
    ColumnDoNotMatchRegex {
        column: usize,
        regex_value: String,
    },
}

pub fn eval_conditions(conditions: &Vec<ConditionTypes>, record: &csv::StringRecord) -> bool {
    let mut should_exclude: bool = false;
    for exclude_condition in conditions {
        debug!("Checking Excluding condition: {:?}", exclude_condition);
        match exclude_condition {
            ConditionTypes::ColumnContainsValue {
                column,
                value,
                operation,
            } => {
                let column_under_check = &record[*column];
                debug!("Column value under check: -->{:?}<--", column_under_check);
                if operation == "contains" {
                    if column_under_check.contains(&*value.as_str()) {
                        should_exclude = true;
                    }
                } else if operation == "equal" {
                    if column_under_check == *value {
                        should_exclude = true;
                    }
                } else {
                    panic!("Panic: === Invalid ConditionTypes operation === ");
                }
            }
            ConditionTypes::RecordLen(record_len) => {
                if *record_len == record.len() {
                    should_exclude = true
                }
                debug!(
                    "Excluding condition: {:?}, record len : {}, should_exclude: {}",
                    record_len,
                    record.len(),
                    should_exclude
                );
            }
            ConditionTypes::ColumnDoNotMatchRegex {
                column,
                regex_value,
            } => {
                let column_under_check = &record[*column];
                debug!(
                    "ColumnDoNotMatchRegex Col: {:?} regex_value: {:?}, column_under_check: {:?}",
                    column, regex_value, column_under_check
                );
                let re = RegexBuilder::new(&format!(r"{}", regex_value))
                    .build()
                    .unwrap();
                match re.find(column_under_check) {
                    Some(matched) => {
                        debug!(
                            "ColumnDoNotMatchRegex Col: {:?} regex_value: {:?} matched: {:?}",
                            column, regex_value, matched
                        );
                        should_exclude = false;
                    }
                    None => {
                        should_exclude = true;
                    }
                }
            }
        }
        if should_exclude == true {
            break;
        }
    }
    return should_exclude;
}
