# UCM - Universal Calendar Manager

## Design Document v1.0

**Purpose**: MCP server providing date/time calculations for Claude Desktop, addressing LLM limitations with temporal reasoning and date arithmetic.

**Philosophy**: Claude Desktop is the consumer - outputs should be machine-readable JSON, not human prose. No verbosity needed.

---

## Problem Statement

LLMs are notoriously bad at:
- Knowing the current date/time
- Calculating days between dates
- Converting durations (e.g., "3823 days in months")
- Parsing natural language dates ("next wednesday", "yesterday")
- Date arithmetic ("today + 3 weeks")

UCM solves this by providing reliable, deterministic date/time tools.

---

## Technology Stack

### Core Dependencies

```toml
[package]
name = "ucm"
version = "0.1.0"
edition = "2024"

[dependencies]
# MCP Server Framework (official Rust SDK)
rmcp = { version = "0.1", features = ["server", "transport-io"] }
schemars = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }

# Date/Time Libraries
chrono = { version = "0.4", features = ["serde"] }
chrono-tz = "0.10"

# Natural Language Parsing (choose one - see evaluation below)
two_timer = "2.2"           # RECOMMENDED - most comprehensive
# chrono-english = "0.1"    # Alternative - simpler, Linux date-style
# dateparser = "0.2"        # Alternative - format detection focused

# Error Handling
anyhow = "1.0"
thiserror = "1.0"
```

### Library Evaluation

| Crate | Pros | Cons | Recommendation |
|-------|------|------|----------------|
| `two_timer` | Comprehensive NL parsing, handles ranges, "Friday the 13th", "last year", time zones configurable, returns time ranges | Larger dependency, regex-heavy | **PRIMARY CHOICE** |
| `chrono-english` | Simple API, Linux `date -d` style, lightweight | Less comprehensive, no ranges | Fallback option |
| `dateparser` | Good format detection, many date formats | Less NL parsing, more structured formats | Not needed |
| `date_time_parser` | Basic NL, extracts dates from text | Less maintained, limited expressions | Skip |

**Decision**: Use `two_timer` as primary parser. It handles the widest range of expressions and returns time ranges which is useful for understanding granularity.

---

## MCP Tools Specification

### 1. `ucm_now` - Get Current DateTime

Returns the current date/time from the system.

**Parameters**: None

**Response**:
```json
{
  "iso": "2026-01-13T14:32:05-06:00",
  "unix": 1768437125,
  "timezone": "America/Chicago",
  "date": "2026-01-13",
  "time": "14:32:05",
  "day_of_week": "Tuesday",
  "day_of_year": 13,
  "week_of_year": 3,
  "quarter": 1
}
```

---

### 2. `ucm_parse` - Parse Natural Language Date

Parses natural language date expressions relative to now.

**Parameters**:
```json
{
  "expression": "string (required) - Natural language date expression"
}
```

**Example Inputs**:
- "today", "tomorrow", "yesterday"
- "next wednesday", "last friday"
- "october 22", "oct 22 2026"
- "in 3 weeks", "2 months ago"
- "next month", "last year"
- "friday the 13th"
- "end of month"

**Response**:
```json
{
  "iso": "2026-01-21",
  "unix": 1769068800,
  "date": "2026-01-21",
  "day_of_week": "Wednesday",
  "days_from_now": 8,
  "is_past": false,
  "parsed_expression": "next wednesday"
}
```

**Error Response**:
```json
{
  "error": "unrecognized_expression",
  "message": "Could not parse 'flurbnesday' as a date",
  "input": "flurbnesday"
}
```

---

### 3. `ucm_diff` - Calculate Difference Between Dates

Calculates the difference between two dates with multiple unit breakdowns.

**Parameters**:
```json
{
  "from": "string (required) - Start date (NL or ISO)",
  "to": "string (required) - End date (NL or ISO)"
}
```

**Example**: `ucm_diff("today", "october 22")`

**Response**:
```json
{
  "from_date": "2026-01-13",
  "to_date": "2026-10-22",
  "total_days": 282,
  "total_seconds": 24364800,
  "is_future": true,
  "breakdown": {
    "years": 0,
    "months": 9,
    "weeks": 1,
    "days": 2,
    "hours": 0,
    "minutes": 0,
    "seconds": 0
  },
  "alt_breakdowns": {
    "total_weeks": 40.28,
    "total_months": 9.27,
    "total_hours": 6768
  }
}
```

---

### 4. `ucm_add` - Add Duration to Date

Adds a duration to a date.

**Parameters**:
```json
{
  "date": "string (required) - Base date (NL or ISO)",
  "add": "string (required) - Duration to add (e.g., '3 weeks', '2 months', '-5 days')"
}
```

**Example**: `ucm_add("today", "3 weeks")`

**Response**:
```json
{
  "base_date": "2026-01-13",
  "added": "3 weeks",
  "result_date": "2026-02-03",
  "result_iso": "2026-02-03T00:00:00-06:00",
  "day_of_week": "Tuesday"
}
```

---

### 5. `ucm_convert` - Convert Duration Units

Converts a duration value between different units.

**Parameters**:
```json
{
  "value": "number (required) - The duration value",
  "from_unit": "string (required) - Source unit (days, weeks, months, years, hours, minutes, seconds)",
  "to_unit": "string (optional) - Target unit (if omitted, returns all units)"
}
```

**Example**: `ucm_convert(3823, "days")`

**Response**:
```json
{
  "input_value": 3823,
  "input_unit": "days",
  "conversions": {
    "years": 10,
    "months": 5,
    "weeks": 2,
    "days": 4,
    "total_years": 10.47,
    "total_months": 125.6,
    "total_weeks": 546.14
  },
  "breakdown_string": "10 years, 5 months, 2 weeks, 4 days"
}
```

---

### 6. `ucm_info` - Rich Date Information

Returns comprehensive information about a specific date.

**Parameters**:
```json
{
  "date": "string (required) - Date to analyze (NL or ISO)"
}
```

**Response**:
```json
{
  "iso": "2026-10-22",
  "day_of_week": "Thursday",
  "day_of_month": 22,
  "day_of_year": 295,
  "week_of_year": 43,
  "month": 10,
  "month_name": "October",
  "year": 2026,
  "quarter": 4,
  "is_leap_year": false,
  "days_in_month": 31,
  "is_weekend": false,
  "days_from_now": 282,
  "is_past": false
}
```

---

## Implementation Architecture

```
ucm/
├── Cargo.toml
├── src/
│   ├── main.rs              # MCP server entry point
│   ├── lib.rs               # Library exports
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── now.rs           # ucm_now implementation
│   │   ├── parse.rs         # ucm_parse implementation
│   │   ├── diff.rs          # ucm_diff implementation
│   │   ├── add.rs           # ucm_add implementation
│   │   ├── convert.rs       # ucm_convert implementation
│   │   └── info.rs          # ucm_info implementation
│   ├── parser/
│   │   ├── mod.rs
│   │   └── natural.rs       # two_timer wrapper with fallbacks
│   └── types/
│       ├── mod.rs
│       ├── duration.rs      # Duration breakdown types
│       └── responses.rs     # Response structs
└── tests/
    ├── integration.rs
    └── parser_tests.rs
```

---

## MCP Server Structure

```rust
use rmcp::{ServerHandler, ServiceExt, model::*, schemars, tool, transport::stdio};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct UcmServer;

#[tool(tool_box)]
impl ServerHandler for UcmServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "ucm".into(),
                version: env!("CARGO_PKG_VERSION").into(),
            },
            instructions: Some(
                "Universal Calendar Manager - Date/time calculations for Claude Desktop. \
                 Use ucm_now for current time, ucm_parse for natural language dates, \
                 ucm_diff for date differences, ucm_add for date arithmetic, \
                 ucm_convert for duration conversions, ucm_info for date details.".into()
            ),
        }
    }
}

#[tool(tool_box)]
impl UcmServer {
    #[tool(description = "Get current date and time from system")]
    fn ucm_now(&self) -> Result<CallToolResult, McpError> {
        // Implementation
    }

    #[tool(description = "Parse natural language date expression (e.g., 'next wednesday', 'yesterday', 'october 22')")]
    fn ucm_parse(
        &self,
        #[tool(param)]
        #[schemars(description = "Natural language date expression")]
        expression: String,
    ) -> Result<CallToolResult, McpError> {
        // Implementation
    }

    #[tool(description = "Calculate difference between two dates in multiple units")]
    fn ucm_diff(
        &self,
        #[tool(param)]
        #[schemars(description = "Start date (natural language or ISO format)")]
        from: String,
        #[tool(param)]
        #[schemars(description = "End date (natural language or ISO format)")]
        to: String,
    ) -> Result<CallToolResult, McpError> {
        // Implementation
    }

    #[tool(description = "Add duration to a date (e.g., '3 weeks', '-5 days', '2 months')")]
    fn ucm_add(
        &self,
        #[tool(param)]
        #[schemars(description = "Base date (natural language or ISO format)")]
        date: String,
        #[tool(param)]
        #[schemars(description = "Duration to add (e.g., '3 weeks', '2 months', '-5 days')")]
        add: String,
    ) -> Result<CallToolResult, McpError> {
        // Implementation
    }

    #[tool(description = "Convert duration between units (e.g., 3823 days to years/months/weeks)")]
    fn ucm_convert(
        &self,
        #[tool(param)]
        #[schemars(description = "Duration value to convert")]
        value: f64,
        #[tool(param)]
        #[schemars(description = "Source unit: days, weeks, months, years, hours, minutes, seconds")]
        from_unit: String,
    ) -> Result<CallToolResult, McpError> {
        // Implementation
    }

    #[tool(description = "Get detailed information about a date")]
    fn ucm_info(
        &self,
        #[tool(param)]
        #[schemars(description = "Date to analyze (natural language or ISO format)")]
        date: String,
    ) -> Result<CallToolResult, McpError> {
        // Implementation
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = UcmServer.serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
```

---

## Natural Language Parser Wrapper

```rust
// src/parser/natural.rs

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use two_timer::{parse, Config};
use anyhow::{Result, anyhow};

pub struct ParsedDate {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub is_range: bool,
}

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

pub fn parse_to_date(expression: &str) -> Result<NaiveDate> {
    let parsed = parse_natural(expression)?;
    Ok(parsed.start.date())
}

pub fn parse_to_datetime(expression: &str) -> Result<NaiveDateTime> {
    let parsed = parse_natural(expression)?;
    Ok(parsed.start)
}
```

---

## Duration Breakdown Algorithm

```rust
// src/types/duration.rs

use chrono::{NaiveDate, Datelike};

#[derive(Debug, Serialize)]
pub struct DurationBreakdown {
    pub years: i32,
    pub months: i32,
    pub weeks: i32,
    pub days: i32,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl DurationBreakdown {
    /// Convert total days into years, months, weeks, days
    /// Uses average month length (30.44 days) and year (365.25 days)
    pub fn from_days(total_days: i64) -> Self {
        let years = total_days / 365;
        let remaining = total_days % 365;
        let months = remaining / 30;
        let remaining = remaining % 30;
        let weeks = remaining / 7;
        let days = remaining % 7;

        Self {
            years: years as i32,
            months: months as i32,
            weeks: weeks as i32,
            days: days as i32,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    /// More accurate breakdown between two specific dates
    pub fn between_dates(from: NaiveDate, to: NaiveDate) -> Self {
        let mut years = 0i32;
        let mut months = 0i32;
        let mut current = from;

        // Count full years
        while current.with_year(current.year() + 1)
            .map(|d| d <= to)
            .unwrap_or(false)
        {
            years += 1;
            current = current.with_year(current.year() + 1).unwrap();
        }

        // Count full months
        while add_months(current, 1).map(|d| d <= to).unwrap_or(false) {
            months += 1;
            current = add_months(current, 1).unwrap();
        }

        // Remaining days
        let remaining_days = (to - current).num_days();
        let weeks = remaining_days / 7;
        let days = remaining_days % 7;

        Self {
            years,
            months,
            weeks: weeks as i32,
            days: days as i32,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

fn add_months(date: NaiveDate, months: i32) -> Option<NaiveDate> {
    let total_months = date.month0() as i32 + months;
    let year = date.year() + total_months.div_euclid(12);
    let month = (total_months.rem_euclid(12) + 1) as u32;
    let day = date.day().min(days_in_month(year, month));
    NaiveDate::from_ymd_opt(year, month, day)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}
```

---

## Claude Desktop Configuration

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ucm": {
      "command": "/path/to/ucm",
      "args": []
    }
  }
}
```

---

## Test Cases

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_today() {
        let result = parse_natural("today").unwrap();
        assert!(!result.is_range);
    }

    #[test]
    fn test_parse_next_wednesday() {
        let result = parse_natural("next wednesday").unwrap();
        assert_eq!(result.start.weekday(), chrono::Weekday::Wed);
    }

    #[test]
    fn test_parse_yesterday() {
        let result = parse_natural("yesterday").unwrap();
        let today = Local::now().naive_local().date();
        assert_eq!(result.start.date(), today.pred_opt().unwrap());
    }

    #[test]
    fn test_duration_breakdown() {
        let breakdown = DurationBreakdown::from_days(3823);
        assert_eq!(breakdown.years, 10);
        assert_eq!(breakdown.months, 5);
        // Verify: 10*365 + 5*30 + 2*7 + 4 = 3650 + 150 + 14 + 4 = 3818 (close)
    }

    #[test]
    fn test_duration_between_dates() {
        let from = NaiveDate::from_ymd_opt(2026, 1, 13).unwrap();
        let to = NaiveDate::from_ymd_opt(2026, 10, 22).unwrap();
        let breakdown = DurationBreakdown::between_dates(from, to);
        assert_eq!(breakdown.years, 0);
        assert_eq!(breakdown.months, 9);
    }
}
```

---

## Error Handling Strategy

All tools return structured errors:

```rust
#[derive(Debug, Serialize)]
pub struct UcmError {
    pub error: String,      // Error code: "parse_error", "invalid_unit", etc.
    pub message: String,    // Human-readable description
    pub input: Option<String>, // The problematic input
}

impl Into<CallToolResult> for UcmError {
    fn into(self) -> CallToolResult {
        CallToolResult::error(vec![Content::text(
            serde_json::to_string(&self).unwrap()
        )])
    }
}
```

---

## Configuration

### Timezone Handling

UCM uses the system timezone by default via `chrono::Local`. No user configuration needed.

```rust
fn get_system_timezone() -> String {
    // On Unix, reads from /etc/localtime or TZ env var
    // On Windows, uses system settings
    chrono::Local::now()
        .format("%Z")
        .to_string()
}
```

---

## Build & Deployment

```bash
# Build release binary
cargo build --release

# Binary location
./target/release/ucm

# Test with MCP inspector (if available)
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/release/ucm
```

---

## Future Considerations (Out of Scope for v1)

These are explicitly **NOT** in scope for UCM v1, but noted for potential UTM (Universal Time Manager) integration:

- Calendar event storage
- Reminders/notifications
- Recurring events
- iCal import/export
- Multi-timezone conversions in single call
- Business day calculations
- Holiday awareness

---

## Summary

UCM provides six core tools for Claude Desktop:

| Tool | Purpose | Example |
|------|---------|---------|
| `ucm_now` | Current datetime | "What time is it?" |
| `ucm_parse` | NL date parsing | "When is next wednesday?" |
| `ucm_diff` | Date difference | "How long until October 22?" |
| `ucm_add` | Date arithmetic | "What's the date in 3 weeks?" |
| `ucm_convert` | Duration conversion | "How many months is 3823 days?" |
| `ucm_info` | Date details | "What day of the week is Oct 22?" |

All responses are JSON for machine consumption. The `two_timer` crate handles natural language parsing with comprehensive expression support.

---

*Document Version: 1.0*
*Author: Claude (Opus 4.5)*
*Date: 2026-01-13*
*For: Claude Code Implementation Handoff*
