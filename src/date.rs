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
