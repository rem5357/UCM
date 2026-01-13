use chrono::{Datelike, Local};

use crate::types::NowResponse;

/// Get the current date and time
pub fn ucm_now() -> NowResponse {
    let now = Local::now();

    NowResponse {
        iso: now.to_rfc3339(),
        unix: now.timestamp(),
        timezone: now.format("%Z").to_string(),
        date: now.format("%Y-%m-%d").to_string(),
        time: now.format("%H:%M:%S").to_string(),
        day_of_week: now.format("%A").to_string(),
        day_of_year: now.ordinal(),
        week_of_year: now.iso_week().week(),
        quarter: ((now.month() - 1) / 3) + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_returns_valid_response() {
        let response = ucm_now();
        assert!(!response.iso.is_empty());
        assert!(response.unix > 0);
        assert!(response.day_of_year >= 1 && response.day_of_year <= 366);
        assert!(response.quarter >= 1 && response.quarter <= 4);
    }
}
