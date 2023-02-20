extern crate chrono;
pub use chrono::prelude::*;
pub use chrono::Weekday as wd;

pub fn middle_day(year: usize) -> Option<wd> {
    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        return None;
    }

    Some(
        Utc.with_ymd_and_hms(year as i32, 7, 2, 0, 0, 0)
            .unwrap()
            .weekday(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        assert!(middle_day(1892).is_none(), "1892 was a leap year!");
        assert!(middle_day(1904).is_none(), "1904 was a leap year!");
        assert!(middle_day(2012).is_none(), "2012 was a leap year!");
    }

    #[test]
    fn weekdays() {
        assert_eq!(wd::Tue, middle_day(2019).unwrap());
        assert_eq!(wd::Wed, middle_day(1997).unwrap());
        assert_eq!(wd::Mon, middle_day(1663).unwrap());
        assert_eq!(wd::Wed, middle_day(1873).unwrap());
        assert_eq!(wd::Thu, middle_day(1953).unwrap());
        assert_eq!(wd::Wed, middle_day(1879).unwrap());
    }
}
