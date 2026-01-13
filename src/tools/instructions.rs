use crate::types::{InstructionsResponse, ToolInstruction};

/// Get instructions on how to use UCM tools
pub fn ucm_instructions() -> InstructionsResponse {
    InstructionsResponse {
        overview: "UCM (Universal Calendar Manager) provides reliable date/time calculations. \
            All responses are JSON for easy parsing. Use these tools whenever you need to \
            work with dates, times, durations, or temporal calculations.".to_string(),

        tools: vec![
            ToolInstruction {
                name: "ucm_now".to_string(),
                description: "Get the current date and time from the system.".to_string(),
                parameters: "None".to_string(),
                example: "Returns current datetime with ISO format, unix timestamp, timezone, \
                    day of week, day of year, week number, and quarter.".to_string(),
            },
            ToolInstruction {
                name: "ucm_parse".to_string(),
                description: "Parse natural language date expressions into structured data.".to_string(),
                parameters: "expression: String - e.g., 'today', 'tomorrow', 'next wednesday', \
                    'october 22', 'in 3 weeks', '2 months ago'".to_string(),
                example: "ucm_parse('next friday') returns the date of next Friday with \
                    days_from_now and is_past indicators.".to_string(),
            },
            ToolInstruction {
                name: "ucm_diff".to_string(),
                description: "Calculate the difference between two dates.".to_string(),
                parameters: "from: String, to: String - Both accept natural language or ISO dates".to_string(),
                example: "ucm_diff('today', 'december 25') returns total_days, breakdown \
                    (years/months/weeks/days), and alternative representations.".to_string(),
            },
            ToolInstruction {
                name: "ucm_add".to_string(),
                description: "Add a duration to a date.".to_string(),
                parameters: "date: String, add: String - e.g., '3 weeks', '-5 days', '2 months'".to_string(),
                example: "ucm_add('today', '3 weeks') returns the date 3 weeks from now.".to_string(),
            },
            ToolInstruction {
                name: "ucm_convert".to_string(),
                description: "Convert a duration value between different units.".to_string(),
                parameters: "value: f64, from_unit: String (days, weeks, months, years, hours, minutes, seconds)".to_string(),
                example: "ucm_convert(100, 'days') returns breakdown as years/months/weeks/days \
                    plus total conversions to other units.".to_string(),
            },
            ToolInstruction {
                name: "ucm_info".to_string(),
                description: "Get detailed information about a specific date.".to_string(),
                parameters: "date: String - natural language or ISO format".to_string(),
                example: "ucm_info('2026-07-04') returns day_of_week, quarter, is_leap_year, \
                    days_in_month, is_weekend, and days_from_now.".to_string(),
            },
            ToolInstruction {
                name: "ucm_status".to_string(),
                description: "Get UCM server version, build number, and available tools.".to_string(),
                parameters: "None".to_string(),
                example: "Returns name, version, build number, and list of all tools.".to_string(),
            },
            ToolInstruction {
                name: "ucm_instructions".to_string(),
                description: "Get these instructions on how to use UCM.".to_string(),
                parameters: "None".to_string(),
                example: "Returns this help information.".to_string(),
            },
        ],

        response_format: "All tools return JSON responses. Success responses contain the \
            requested data. Error responses contain 'error' (code), 'message' (description), \
            and optionally 'input' (the problematic value).".to_string(),

        tips: vec![
            "Use ucm_now first to establish the current date context.".to_string(),
            "Natural language parsing supports: today, tomorrow, yesterday, next/last weekday, \
                month names, relative expressions (in X days, X ago).".to_string(),
            "ISO dates (YYYY-MM-DD) are always accepted and recommended for precision.".to_string(),
            "Duration units can be singular or plural (day/days, week/weeks).".to_string(),
            "Negative durations work with ucm_add: '-5 days' subtracts 5 days.".to_string(),
            "ucm_diff returns both exact total_days and human-friendly breakdown.".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions_returns_valid_response() {
        let response = ucm_instructions();
        assert!(!response.overview.is_empty());
        assert_eq!(response.tools.len(), 8);
        assert!(!response.tips.is_empty());
    }
}
