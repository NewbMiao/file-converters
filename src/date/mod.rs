use chrono::{Datelike, Duration, Local, NaiveDate};

use std::fmt;

pub enum PastDateRangeType {
    Week,
    Month,
    ThreeMonth,
}
impl PastDateRangeType {
    pub fn format((start, end): (NaiveDate, NaiveDate)) -> String {
        if start.year() == end.year() {
            format!("{} - {}", start.format("%-d %b"), end.format("%-d %b %Y"))
        } else {
            format!(
                "{} - {}",
                start.format("%-d %b %Y"),
                end.format("%-d %b %Y")
            )
        }
    }

    pub fn get_local_date(today: Option<NaiveDate>) -> NaiveDate {
        today.map_or(Local::now().date_naive(), |v| v)
    }

    // Get the date range for the previous week
    pub fn prev_week(today: Option<NaiveDate>) -> (NaiveDate, NaiveDate) {
        let today = Self::get_local_date(today);
        let start_of_week = today
            - Duration::days(today.weekday().num_days_from_monday() as i64)
            - Duration::days(7);
        let end_of_week = start_of_week + Duration::days(6);
        (start_of_week, end_of_week)
    }

    // Get the date range for the previous month
    pub fn prev_month(today: Option<NaiveDate>) -> (NaiveDate, NaiveDate) {
        let today = Self::get_local_date(today);
        let prev_month = today.with_day(1).unwrap() - Duration::days(1);
        let start_of_pre_month = prev_month.with_day(1).unwrap();
        (start_of_pre_month, prev_month)
    }

    // Get the date range for the previous quarter
    pub fn prev_three_month(today: Option<NaiveDate>) -> (NaiveDate, NaiveDate) {
        let (start_of_prev_month, prev_month) = Self::prev_month(today);

        // Subtract three months
        let mut month = start_of_prev_month.month() as i32 - 2;
        let mut year = start_of_prev_month.year();
        if month < 1 {
            month += 12;
            year -= 1;
        }
        let start_of_prev_three_month = NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap();
        (start_of_prev_three_month, prev_month)
    }
    #[allow(dead_code)]
    pub fn prev_quarter(today: Option<NaiveDate>) -> (NaiveDate, NaiveDate) {
        let today = Self::get_local_date(today);
        let month = today.month();

        let start_month = match month {
            1..=3 => 10,
            4..=6 => 1,
            7..=9 => 4,
            10..=12 => 7,
            _ => unreachable!(),
        };

        let start = NaiveDate::from_ymd_opt(today.year(), start_month, 1).unwrap();
        let end = match start_month {
            10 => NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap(),
            1 => NaiveDate::from_ymd_opt(today.year(), 3, 31).unwrap(),
            4 => NaiveDate::from_ymd_opt(today.year(), 6, 30).unwrap(),
            7 => NaiveDate::from_ymd_opt(today.year(), 9, 30).unwrap(),
            _ => unreachable!(),
        };

        (start, end)
    }
}
impl fmt::Display for PastDateRangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Week => write!(f, "{}", Self::format(Self::prev_week(None))),
            Self::Month => write!(f, "{}", Self::format(Self::prev_month(None))),
            Self::ThreeMonth => {
                write!(f, "{}", Self::format(Self::prev_three_month(None)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::date::PastDateRangeType;

    fn with_fixed_today() -> Option<NaiveDate> {
        Some(NaiveDate::from_ymd_opt(2023, 3, 9).unwrap())
    }
    #[test]
    fn test_prev_week_should_work() {
        assert_eq!(
            "27 Feb - 5 Mar 2023",
            PastDateRangeType::format(PastDateRangeType::prev_week(with_fixed_today()))
        );
    }

    #[test]
    fn test_prev_month_should_work() {
        assert_eq!(
            "1 Feb - 28 Feb 2023",
            PastDateRangeType::format(PastDateRangeType::prev_month(with_fixed_today()))
        );
    }

    #[test]
    fn test_prev_three_month_should_work() {
        assert_eq!(
            "1 Dec 2022 - 28 Feb 2023",
            PastDateRangeType::format(PastDateRangeType::prev_three_month(with_fixed_today()))
        );
    }

    #[test]
    fn test_prev_quarter_should_work() {
        assert_eq!(
            "1 Oct - 31 Dec 2023",
            PastDateRangeType::format(PastDateRangeType::prev_quarter(with_fixed_today()))
        );
    }
}
