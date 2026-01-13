use chrono::{Local, NaiveTime};

use crate::parser::parse_to_date;
use crate::types::{ParseResponse, UcmError};

/// Parse a natural language date expression
pub fn ucm_parse(expression: &str) -> Result<ParseResponse, UcmError> {
    let parsed_date = parse_to_date(expression).map_err(|e| {
        UcmError::parse_error(expression, &e.to_string())
    })?;

    let today = Local::now().naive_local().date();
    let days_from_now = (parsed_date - today).num_days();

    // Create datetime at midnight for unix timestamp
    let datetime = parsed_date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());

    Ok(ParseResponse {
        iso: parsed_date.format("%Y-%m-%d").to_string(),
        unix: datetime.and_local_timezone(Local).unwrap().timestamp(),
        date: parsed_date.format("%Y-%m-%d").to_string(),
        day_of_week: parsed_date.format("%A").to_string(),
        days_from_now,
        is_past: days_from_now < 0,
        parsed_expression: expression.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_today() {
        let result = ucm_parse("today").unwrap();
        assert_eq!(result.days_from_now, 0);
        assert!(!result.is_past);
    }

    #[test]
    fn test_parse_yesterday() {
        let result = ucm_parse("yesterday").unwrap();
        assert_eq!(result.days_from_now, -1);
        assert!(result.is_past);
    }

    #[test]
    fn test_parse_invalid() {
        let result = ucm_parse("flurbnesday");
        assert!(result.is_err());
    }
}
