use chrono::{Datelike, NaiveDate, Weekday};

const MIDDLE_OF_YEAR: (u32, u32) = (7, 2);

pub fn middle_day(year: u32) -> Option<Weekday> {
    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        None
    } else {
        Some(
            NaiveDate::from_ymd_opt(year.try_into().unwrap(), MIDDLE_OF_YEAR.0, MIDDLE_OF_YEAR.1)
                .unwrap()
                .weekday(),
        )
    }
}
