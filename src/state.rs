use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct State {}

impl State {
    pub fn get_string(&self, _record: &csv::StringRecord) -> String {
        return "*".to_string();
    }
}
