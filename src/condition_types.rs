use regex::RegexBuilder;
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
    ColumnDoNotMatchRegex {
        column: usize,
        regex_value: String,
    },
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
                debug!("Column value under check: -->{:?}<--", column_under_check);
                if operation == "contains" {
                    if column_under_check.contains(&*value.as_str()) {
                        should_exclude = true;
                    }
                } else if operation == "equal" {
                    if column_under_check == *value {
                        should_exclude = true;
                    }
                } else {
                    panic!("Panic: === Invalid ConditionTypes operation === ");
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
            ConditionTypes::ColumnDoNotMatchRegex {
                column,
                regex_value,
            } => {
                let column_under_check = &record[*column];
                debug!(
                    "ColumnDoNotMatchRegex Col: {:?} regex_value: {:?}, column_under_check: {:?}",
                    column, regex_value, column_under_check
                );
                let re = RegexBuilder::new(&format!(r"{}", regex_value))
                    .build()
                    .unwrap();
                match re.find(column_under_check) {
                    Some(matched) => {
                        debug!(
                            "ColumnDoNotMatchRegex Col: {:?} regex_value: {:?} matched: {:?}",
                            column, regex_value, matched
                        );
                        should_exclude = false;
                    }
                    None => {
                        should_exclude = true;
                    }
                }
            }
        }
        if should_exclude == true {
            break;
        }
    }
    return should_exclude;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(fields: &[&str]) -> csv::StringRecord {
        csv::StringRecord::from(fields.to_vec())
    }

    fn col_contains(column: usize, value: &str, operation: &str) -> ConditionTypes {
        ConditionTypes::ColumnContainsValue {
            column,
            value: value.to_string(),
            operation: operation.to_string(),
        }
    }

    #[test]
    fn contains_match() {
        assert!(eval_conditions(&vec![col_contains(0, "foo", "contains")], &rec(&["foobar"])));
    }

    #[test]
    fn contains_no_match() {
        assert!(!eval_conditions(&vec![col_contains(0, "baz", "contains")], &rec(&["foobar"])));
    }

    #[test]
    fn equal_match() {
        assert!(eval_conditions(&vec![col_contains(0, "foo", "equal")], &rec(&["foo"])));
    }

    #[test]
    fn equal_no_match() {
        assert!(!eval_conditions(&vec![col_contains(0, "foo", "equal")], &rec(&["foobar"])));
    }

    #[test]
    fn record_len_match() {
        assert!(eval_conditions(&vec![ConditionTypes::RecordLen(2)], &rec(&["a", "b"])));
    }

    #[test]
    fn record_len_no_match() {
        assert!(!eval_conditions(&vec![ConditionTypes::RecordLen(3)], &rec(&["a", "b"])));
    }

    #[test]
    fn do_not_match_regex_regex_found() {
        let cond = vec![ConditionTypes::ColumnDoNotMatchRegex {
            column: 0,
            regex_value: r"\d+".to_string(),
        }];
        // regex matches → should NOT exclude
        assert!(!eval_conditions(&cond, &rec(&["abc123"])));
    }

    #[test]
    fn do_not_match_regex_regex_not_found() {
        let cond = vec![ConditionTypes::ColumnDoNotMatchRegex {
            column: 0,
            regex_value: r"\d+".to_string(),
        }];
        // no regex match → should exclude
        assert!(eval_conditions(&cond, &rec(&["abc"])));
    }

    #[test]
    fn early_exit_stops_at_first_match() {
        // RecordLen(2) matches and sets should_exclude=true, breaking before the
        // second condition which would panic on an out-of-bounds column index.
        let cond = vec![
            ConditionTypes::RecordLen(2),
            ConditionTypes::ColumnContainsValue {
                column: 99,
                value: "x".to_string(),
                operation: "contains".to_string(),
            },
        ];
        assert!(eval_conditions(&cond, &rec(&["a", "b"])));
    }
}
