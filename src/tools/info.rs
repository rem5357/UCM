use chrono::{Datelike, Local, NaiveDate};

use crate::parser::parse_to_date;
use crate::types::{InfoResponse, UcmError};

/// Get detailed information about a date
pub fn ucm_info(date: &str) -> Result<InfoResponse, UcmError> {
    let parsed_date = parse_to_date(date).map_err(|e| {
        UcmError::parse_error(date, &e.to_string())
    })?;

    let today = Local::now().naive_local().date();
    let days_from_now = (parsed_date - today).num_days();

    let is_leap_year = parsed_date.leap_year();
    let days_in_month = days_in_month(parsed_date.year(), parsed_date.month());
    let is_weekend = matches!(
        parsed_date.weekday(),
        chrono::Weekday::Sat | chrono::Weekday::Sun
    );

    let month_name = parsed_date.format("%B").to_string();

    Ok(InfoResponse {
        iso: parsed_date.format("%Y-%m-%d").to_string(),
        day_of_week: parsed_date.format("%A").to_string(),
        day_of_month: parsed_date.day(),
        day_of_year: parsed_date.ordinal(),
        week_of_year: parsed_date.iso_week().week(),
        month: parsed_date.month(),
        month_name,
        year: parsed_date.year(),
        quarter: ((parsed_date.month() - 1) / 3) + 1,
        is_leap_year,
        days_in_month,
        is_weekend,
        days_from_now,
        is_past: days_from_now < 0,
    })
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };

    next_month
        .and_then(|d| d.pred_opt())
        .map(|d| d.day())
        .unwrap_or(30)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_specific_date() {
        let result = ucm_info("2026-10-22").unwrap();
        assert_eq!(result.day_of_week, "Thursday");
        assert_eq!(result.month, 10);
        assert_eq!(result.month_name, "October");
        assert_eq!(result.quarter, 4);
    }

    #[test]
    fn test_info_leap_year() {
        let result = ucm_info("2024-02-29").unwrap();
        assert!(result.is_leap_year);
        assert_eq!(result.days_in_month, 29);
    }

    #[test]
    fn test_info_weekend() {
        let result = ucm_info("2026-01-17").unwrap(); // Saturday
        assert!(result.is_weekend);
    }
}
