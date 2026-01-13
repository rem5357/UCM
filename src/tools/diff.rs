use crate::parser::parse_to_date;
use crate::types::{AltBreakdowns, DiffResponse, DurationBreakdown, UcmError};

/// Calculate the difference between two dates
pub fn ucm_diff(from: &str, to: &str) -> Result<DiffResponse, UcmError> {
    let from_date = parse_to_date(from).map_err(|e| {
        UcmError::parse_error(from, &e.to_string())
    })?;

    let to_date = parse_to_date(to).map_err(|e| {
        UcmError::parse_error(to, &e.to_string())
    })?;

    let total_days = (to_date - from_date).num_days();
    let total_seconds = total_days * 86400;
    let is_future = total_days > 0;

    let breakdown = DurationBreakdown::between_dates(from_date, to_date);

    let total_days_abs = total_days.abs() as f64;
    let alt_breakdowns = AltBreakdowns {
        total_weeks: (total_days_abs / 7.0 * 100.0).round() / 100.0,
        total_months: (total_days_abs / 30.44 * 100.0).round() / 100.0,
        total_hours: total_days * 24,
    };

    Ok(DiffResponse {
        from_date: from_date.format("%Y-%m-%d").to_string(),
        to_date: to_date.format("%Y-%m-%d").to_string(),
        total_days,
        total_seconds,
        is_future,
        breakdown,
        alt_breakdowns,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_same_date() {
        let result = ucm_diff("2026-01-13", "2026-01-13").unwrap();
        assert_eq!(result.total_days, 0);
    }

    #[test]
    fn test_diff_future() {
        let result = ucm_diff("2026-01-13", "2026-10-22").unwrap();
        assert!(result.total_days > 0);
        assert!(result.is_future);
    }

    #[test]
    fn test_diff_past() {
        let result = ucm_diff("2026-10-22", "2026-01-13").unwrap();
        assert!(result.total_days < 0);
        assert!(!result.is_future);
    }
}
