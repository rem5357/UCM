use serde::Serialize;

use super::DurationBreakdown;

/// Response for ucm_now
#[derive(Debug, Serialize)]
pub struct NowResponse {
    pub iso: String,
    pub unix: i64,
    pub timezone: String,
    pub date: String,
    pub time: String,
    pub day_of_week: String,
    pub day_of_year: u32,
    pub week_of_year: u32,
    pub quarter: u32,
}

/// Response for ucm_parse
#[derive(Debug, Serialize)]
pub struct ParseResponse {
    pub iso: String,
    pub unix: i64,
    pub date: String,
    pub day_of_week: String,
    pub days_from_now: i64,
    pub is_past: bool,
    pub parsed_expression: String,
}

/// Response for ucm_diff
#[derive(Debug, Serialize)]
pub struct DiffResponse {
    pub from_date: String,
    pub to_date: String,
    pub total_days: i64,
    pub total_seconds: i64,
    pub is_future: bool,
    pub breakdown: DurationBreakdown,
    pub alt_breakdowns: AltBreakdowns,
}

#[derive(Debug, Serialize)]
pub struct AltBreakdowns {
    pub total_weeks: f64,
    pub total_months: f64,
    pub total_hours: i64,
}

/// Response for ucm_add
#[derive(Debug, Serialize)]
pub struct AddResponse {
    pub base_date: String,
    pub added: String,
    pub result_date: String,
    pub result_iso: String,
    pub day_of_week: String,
}

/// Response for ucm_convert
#[derive(Debug, Serialize)]
pub struct ConvertResponse {
    pub input_value: f64,
    pub input_unit: String,
    pub conversions: Conversions,
    pub breakdown_string: String,
}

#[derive(Debug, Serialize)]
pub struct Conversions {
    pub years: i32,
    pub months: i32,
    pub weeks: i32,
    pub days: i32,
    pub total_years: f64,
    pub total_months: f64,
    pub total_weeks: f64,
    pub total_days: f64,
}

/// Response for ucm_info
#[derive(Debug, Serialize)]
pub struct InfoResponse {
    pub iso: String,
    pub day_of_week: String,
    pub day_of_month: u32,
    pub day_of_year: u32,
    pub week_of_year: u32,
    pub month: u32,
    pub month_name: String,
    pub year: i32,
    pub quarter: u32,
    pub is_leap_year: bool,
    pub days_in_month: u32,
    pub is_weekend: bool,
    pub days_from_now: i64,
    pub is_past: bool,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct UcmError {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

impl UcmError {
    pub fn parse_error(input: &str, message: &str) -> Self {
        Self {
            error: "parse_error".to_string(),
            message: message.to_string(),
            input: Some(input.to_string()),
        }
    }

    pub fn invalid_unit(input: &str) -> Self {
        Self {
            error: "invalid_unit".to_string(),
            message: format!("Invalid unit: '{}'. Valid units: days, weeks, months, years, hours, minutes, seconds", input),
            input: Some(input.to_string()),
        }
    }
}
