use crate::condition_types::eval_conditions;
use crate::condition_types::ConditionTypes;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Exclude {
    pub conditions: Vec<ConditionTypes>,
}

impl Exclude {
    pub fn exclude(&self, record: &csv::StringRecord) -> bool {
        return eval_conditions(&self.conditions, record);
    }
}
