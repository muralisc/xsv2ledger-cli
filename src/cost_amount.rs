use crate::amount::Amount;
use crate::exclude_condition::Exclude;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CostAmount {
    pub amount: Amount,
    pub exclude: Exclude,
}

impl CostAmount {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        if self.exclude.exclude(&record) {
            return "".to_string();
        }
        return format!(" @ {}", self.amount.get_string(&record));
    }
}
