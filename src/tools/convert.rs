use crate::types::{Conversions, ConvertResponse, DurationBreakdown, UcmError};

/// Convert a duration value between units
pub fn ucm_convert(value: f64, from_unit: &str) -> Result<ConvertResponse, UcmError> {
    let unit = from_unit.to_lowercase();
    let unit = unit.trim_end_matches('s'); // Normalize plural

    // Convert everything to days first (as base unit)
    let total_days = match unit {
        "day" => value,
        "week" => value * 7.0,
        "month" => value * 30.44,
        "year" => value * 365.25,
        "hour" => value / 24.0,
        "minute" => value / 1440.0,
        "second" => value / 86400.0,
        _ => return Err(UcmError::invalid_unit(from_unit)),
    };

    let breakdown = DurationBreakdown::from_days(total_days.round() as i64);

    let conversions = Conversions {
        years: breakdown.years,
        months: breakdown.months,
        weeks: breakdown.weeks,
        days: breakdown.days,
        total_years: (total_days / 365.25 * 100.0).round() / 100.0,
        total_months: (total_days / 30.44 * 100.0).round() / 100.0,
        total_weeks: (total_days / 7.0 * 100.0).round() / 100.0,
        total_days: (total_days * 100.0).round() / 100.0,
    };

    Ok(ConvertResponse {
        input_value: value,
        input_unit: from_unit.to_string(),
        conversions,
        breakdown_string: breakdown.to_string_breakdown(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_days() {
        let result = ucm_convert(3823.0, "days").unwrap();
        assert_eq!(result.conversions.years, 10);
    }

    #[test]
    fn test_convert_weeks() {
        let result = ucm_convert(52.0, "weeks").unwrap();
        assert!(result.conversions.total_days > 360.0);
    }

    #[test]
    fn test_convert_invalid_unit() {
        let result = ucm_convert(100.0, "fortnights");
        assert!(result.is_err());
    }
}
