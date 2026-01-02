use crate::condition_types::ConditionTypes;
use crate::condition_types::eval_conditions;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct InvertSign {
    pub conditions: Vec<ConditionTypes>,
}

impl InvertSign {
    pub fn invert(&self, record: &csv::StringRecord) -> bool {
        return eval_conditions(&self.conditions, record);
    }
}
