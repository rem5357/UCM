use anyhow::{anyhow, Result};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use two_timer::{parse, Config};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedDate {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub is_range: bool,
}

/// Parse a natural language date expression
pub fn parse_natural(expression: &str) -> Result<ParsedDate> {
    let now = Local::now().naive_local();
    let config = Config::new().now(now);

    match parse(expression, Some(config)) {
        Ok((start, end, is_range)) => Ok(ParsedDate {
            start,
            end,
            is_range,
        }),
        Err(e) => Err(anyhow!("Could not parse '{}': {:?}", expression, e)),
    }
}

/// Parse expression to just a date
pub fn parse_to_date(expression: &str) -> Result<NaiveDate> {
    // First try ISO format directly
    if let Ok(date) = NaiveDate::parse_from_str(expression, "%Y-%m-%d") {
        return Ok(date);
    }

    let parsed = parse_natural(expression)?;
    Ok(parsed.start.date())
}

/// Parse expression to datetime
#[allow(dead_code)]
pub fn parse_to_datetime(expression: &str) -> Result<NaiveDateTime> {
    // Try ISO format directly first
    if let Ok(dt) = NaiveDateTime::parse_from_str(expression, "%Y-%m-%dT%H:%M:%S") {
        return Ok(dt);
    }
    if let Ok(date) = NaiveDate::parse_from_str(expression, "%Y-%m-%d") {
        return Ok(date.and_hms_opt(0, 0, 0).unwrap());
    }

    let parsed = parse_natural(expression)?;
    Ok(parsed.start)
}

/// Parse a duration string like "3 weeks", "2 months", "-5 days"
pub fn parse_duration(expression: &str) -> Result<Duration> {
    let expr = expression.trim().to_lowercase();

    // Handle negative durations
    let (is_negative, expr) = if expr.starts_with('-') {
        (true, expr[1..].trim())
    } else {
        (false, expr.as_str())
    };

    // Parse "N unit" format
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(anyhow!(
            "Invalid duration format '{}'. Expected format: 'N unit' (e.g., '3 weeks')",
            expression
        ));
    }

    let value: i64 = parts[0]
        .parse()
        .map_err(|_| anyhow!("Invalid number: '{}'", parts[0]))?;

    let unit = parts[1].trim_end_matches('s'); // Normalize plural

    let days = match unit {
        "day" => value,
        "week" => value * 7,
        "month" => value * 30, // Approximate
        "year" => value * 365, // Approximate
        "hour" => return Ok(apply_sign(Duration::hours(value), is_negative)),
        "minute" => return Ok(apply_sign(Duration::minutes(value), is_negative)),
        "second" => return Ok(apply_sign(Duration::seconds(value), is_negative)),
        _ => {
            return Err(anyhow!(
                "Unknown duration unit: '{}'. Valid units: days, weeks, months, years, hours, minutes, seconds",
                parts[1]
            ))
        }
    };

    Ok(apply_sign(Duration::days(days), is_negative))
}

fn apply_sign(duration: Duration, is_negative: bool) -> Duration {
    if is_negative {
        -duration
    } else {
        duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_today() {
        let result = parse_natural("today").unwrap();
        let today = Local::now().naive_local().date();
        assert_eq!(result.start.date(), today);
    }

    #[test]
    fn test_parse_tomorrow() {
        let result = parse_natural("tomorrow").unwrap();
        let tomorrow = Local::now().naive_local().date().succ_opt().unwrap();
        assert_eq!(result.start.date(), tomorrow);
    }

    #[test]
    fn test_parse_iso_date() {
        let result = parse_to_date("2026-10-22").unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 10, 22).unwrap());
    }

    #[test]
    fn test_parse_duration_weeks() {
        let dur = parse_duration("3 weeks").unwrap();
        assert_eq!(dur.num_days(), 21);
    }

    #[test]
    fn test_parse_duration_negative() {
        let dur = parse_duration("-5 days").unwrap();
        assert_eq!(dur.num_days(), -5);
    }
}
