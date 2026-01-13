use chrono::{Datelike, NaiveDate};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
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
    /// Uses average month length (30 days) and year (365 days)
    pub fn from_days(total_days: i64) -> Self {
        let sign = if total_days < 0 { -1 } else { 1 };
        let total_days = total_days.abs();

        let years = total_days / 365;
        let remaining = total_days % 365;
        let months = remaining / 30;
        let remaining = remaining % 30;
        let weeks = remaining / 7;
        let days = remaining % 7;

        Self {
            years: (years as i32) * sign,
            months: (months as i32) * sign,
            weeks: (weeks as i32) * sign,
            days: (days as i32) * sign,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    /// More accurate breakdown between two specific dates
    pub fn between_dates(from: NaiveDate, to: NaiveDate) -> Self {
        let (from, to, sign) = if from <= to {
            (from, to, 1)
        } else {
            (to, from, -1)
        };

        let mut years = 0i32;
        let mut months = 0i32;
        let mut current = from;

        // Count full years
        while let Some(next_year) = current.with_year(current.year() + 1) {
            if next_year <= to {
                years += 1;
                current = next_year;
            } else {
                break;
            }
        }

        // Count full months
        while let Some(next_month) = add_months(current, 1) {
            if next_month <= to {
                months += 1;
                current = next_month;
            } else {
                break;
            }
        }

        // Remaining days
        let remaining_days = (to - current).num_days();
        let weeks = remaining_days / 7;
        let days = remaining_days % 7;

        Self {
            years: years * sign,
            months: months * sign,
            weeks: (weeks as i32) * sign,
            days: (days as i32) * sign,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    /// Format as human-readable string
    pub fn to_string_breakdown(&self) -> String {
        let mut parts = Vec::new();

        if self.years != 0 {
            parts.push(format!(
                "{} {}",
                self.years.abs(),
                if self.years.abs() == 1 { "year" } else { "years" }
            ));
        }
        if self.months != 0 {
            parts.push(format!(
                "{} {}",
                self.months.abs(),
                if self.months.abs() == 1 { "month" } else { "months" }
            ));
        }
        if self.weeks != 0 {
            parts.push(format!(
                "{} {}",
                self.weeks.abs(),
                if self.weeks.abs() == 1 { "week" } else { "weeks" }
            ));
        }
        if self.days != 0 {
            parts.push(format!(
                "{} {}",
                self.days.abs(),
                if self.days.abs() == 1 { "day" } else { "days" }
            ));
        }

        if parts.is_empty() {
            "0 days".to_string()
        } else {
            parts.join(", ")
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
    fn test_duration_from_days() {
        let breakdown = DurationBreakdown::from_days(3823);
        assert_eq!(breakdown.years, 10);
        assert_eq!(breakdown.months, 5);
        assert_eq!(breakdown.weeks, 3);
        assert_eq!(breakdown.days, 2); // 3823 = 10*365 + 5*30 + 3*7 + 2
    }

    #[test]
    fn test_duration_between_dates() {
        let from = NaiveDate::from_ymd_opt(2026, 1, 13).unwrap();
        let to = NaiveDate::from_ymd_opt(2026, 10, 22).unwrap();
        let breakdown = DurationBreakdown::between_dates(from, to);
        assert_eq!(breakdown.years, 0);
        assert_eq!(breakdown.months, 9);
    }

    #[test]
    fn test_duration_negative() {
        let from = NaiveDate::from_ymd_opt(2026, 10, 22).unwrap();
        let to = NaiveDate::from_ymd_opt(2026, 1, 13).unwrap();
        let breakdown = DurationBreakdown::between_dates(from, to);
        assert_eq!(breakdown.years, 0);
        assert_eq!(breakdown.months, -9);
    }
}
