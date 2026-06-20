use crate::invert_sign::InvertSign;
use crate::xsv_to_entry::XsvToEntry;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct Quantity {
    pub invert_sign: Option<InvertSign>,
    pub xsv_to_entry: XsvToEntry,
}

impl Quantity {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        let mut quantity = self.xsv_to_entry.get_string(&record).replace("$", "");
        if let Some(invert_sign) = &self.invert_sign {
            if invert_sign.invert(record) {
                quantity = if quantity.starts_with('+') {
                    format!("-{}", &quantity[1..])
                } else if quantity.starts_with('-') {
                    format!("{}", &quantity[1..])
                } else {
                    format!("-{}", quantity)
                };
            }
        }
        return quantity;
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commodity {
    pub xsv_to_entry: XsvToEntry,
}

impl Commodity {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        return self.xsv_to_entry.get_string(&record);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Amount {
    pub quantity: Quantity,
    pub commodity: Commodity,
}

impl Amount {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        debug!("Getting Amount string");
        return format!(
            "{} {}",
            self.quantity.get_string(&record),
            self.commodity.get_string(&record)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition_types::ConditionTypes;
    use crate::invert_sign::InvertSign;
    use crate::xsv_to_entry::XsvToEntry;

    fn xsv(col: usize) -> XsvToEntry {
        XsvToEntry { hint_columns: vec![col], hint_mapping: None }
    }

    fn always_invert() -> InvertSign {
        InvertSign { conditions: vec![ConditionTypes::RecordLen(1)] }
    }

    fn rec(field: &str) -> csv::StringRecord {
        csv::StringRecord::from(vec![field])
    }

    #[test]
    fn no_invert_strips_dollar() {
        let q = Quantity { invert_sign: None, xsv_to_entry: xsv(0) };
        assert_eq!(q.get_string(&rec("$100.00")), "100.00");
    }

    #[test]
    fn invert_positive_to_negative() {
        let q = Quantity { invert_sign: Some(always_invert()), xsv_to_entry: xsv(0) };
        assert_eq!(q.get_string(&rec("100.00")), "-100.00");
    }

    #[test]
    fn invert_negative_to_positive() {
        let q = Quantity { invert_sign: Some(always_invert()), xsv_to_entry: xsv(0) };
        assert_eq!(q.get_string(&rec("-100.00")), "100.00");
    }

    #[test]
    fn invert_leading_plus_to_negative() {
        let q = Quantity { invert_sign: Some(always_invert()), xsv_to_entry: xsv(0) };
        assert_eq!(q.get_string(&rec("+100.00")), "-100.00");
    }
}
