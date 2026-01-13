use chrono::{Local, NaiveTime};

use crate::parser::{parse_duration, parse_to_date};
use crate::types::{AddResponse, UcmError};

/// Add a duration to a date
pub fn ucm_add(date: &str, add: &str) -> Result<AddResponse, UcmError> {
    let base_date = parse_to_date(date).map_err(|e| {
        UcmError::parse_error(date, &e.to_string())
    })?;

    let duration = parse_duration(add).map_err(|e| {
        UcmError::parse_error(add, &e.to_string())
    })?;

    let result_date = base_date + duration;
    let result_datetime = result_date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());

    Ok(AddResponse {
        base_date: base_date.format("%Y-%m-%d").to_string(),
        added: add.to_string(),
        result_date: result_date.format("%Y-%m-%d").to_string(),
        result_iso: result_datetime
            .and_local_timezone(Local)
            .unwrap()
            .to_rfc3339(),
        day_of_week: result_date.format("%A").to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_days() {
        let result = ucm_add("2026-01-13", "10 days").unwrap();
        assert_eq!(result.result_date, "2026-01-23");
    }

    #[test]
    fn test_add_weeks() {
        let result = ucm_add("2026-01-13", "3 weeks").unwrap();
        assert_eq!(result.result_date, "2026-02-03");
    }

    #[test]
    fn test_add_negative() {
        let result = ucm_add("2026-01-13", "-5 days").unwrap();
        assert_eq!(result.result_date, "2026-01-08");
    }
}
