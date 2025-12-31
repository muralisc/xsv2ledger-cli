use crate::amount::Amount;
use crate::cost_amount::CostAmount;
use crate::exclude_condition::Exclude;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    pub amount: Amount,
    pub cost_amount: Option<CostAmount>,
    pub exclude: Option<Exclude>,
}

impl Price {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        debug!("Getting Price string");
        if let Some(exclude) = &self.exclude {
            if exclude.exclude(&record) {
                return "".to_string();
            }
        }
        let amount_str = self.amount.get_string(&record);
        if let Some(cost_amount) = &self.cost_amount {
            return format!("{}{}", amount_str, cost_amount.get_string(&record));
        }
        return amount_str;
    }
}
