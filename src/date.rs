use chrono::NaiveDate;
use regex::RegexBuilder;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct Date {
    pub column: usize,
    pub date_format: String,
    pub date_regex: String,
}

impl Date {
    pub fn get_string(&self, record: &csv::StringRecord) -> String {
        let date_str = record[self.column].trim();
        debug!(
            "Processing date string: \"{}\" with regex {}",
            date_str, self.date_regex
        );
        let re = RegexBuilder::new(&format!(r"{}", self.date_regex))
            .build()
            .unwrap();
        let cleaned_date = re.find(date_str).unwrap();
        debug!("Date matched with regex: {}", cleaned_date.as_str());
        let date = NaiveDate::parse_from_str(cleaned_date.as_str(), &self.date_format).unwrap();
        return date.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(format: &str, regex: &str) -> Date {
        Date { column: 0, date_format: format.to_string(), date_regex: regex.to_string() }
    }

    fn rec(field: &str) -> csv::StringRecord {
        csv::StringRecord::from(vec![field])
    }

    #[test]
    fn formats_date() {
        let d = date("%d/%m/%Y", r"\d{2}/\d{2}/\d{4}");
        assert_eq!(d.get_string(&rec("30/10/2022")), "2022-10-30");
    }

    #[test]
    fn extracts_date_from_noisy_string() {
        let d = date("%d/%m/%Y", r"\d{2}/\d{2}/\d{4}");
        assert_eq!(d.get_string(&rec("Date: 30/10/2022 (extra)")), "2022-10-30");
    }
}
