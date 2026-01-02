use serde::Deserialize;

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
