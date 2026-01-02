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
                if operation == "contains" {
                    if column_under_check.contains(&*value.as_str()) {
                        should_exclude = true;
                    }
                } else if operation == "equal" {
                    if column_under_check == *value {
                        should_exclude = true;
                    }
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
        }
        if should_exclude == true {
            break;
        }
    }
    return should_exclude;
}
