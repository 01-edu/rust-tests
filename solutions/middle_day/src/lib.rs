/*

## middle_day

### Instructions

Use the [`chrono`](https://docs.rs/chrono/0.4.19/chrono/index.html) crate to create a function called `middle_day`, that returns the Weekday of the middle day of the year passed as an argument, wrapped in an Option.
You also should refer to chrono::Weekday as `wd`.

```rs
fn main() {
    let date = Utc.ymd(2011, 12, 2).and_hms(21, 12, 09);

    println!("{:?}", middle_day(1022).unwrap());
}
```
*/

extern crate chrono;
pub use chrono::prelude::*;
pub use chrono::Weekday as wd;

pub fn middle_day(year: usize) -> Option<wd> {
    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        return None;
    }

    Some(Utc.ymd(year as i32, 7, 2).weekday())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn leap_years() {
        println!(
            "{:?}",
            (middle_day(1892), middle_day(1904), middle_day(2012))
        );
        assert!(
            middle_day(1892).is_none(),
            format!("1892 was a leap year!",)
        );
        assert!(
            middle_day(1904).is_none(),
            format!("1904 was a leap year!",)
        );
        assert!(
            middle_day(2012).is_none(),
            format!("2012 was a leap year!",)
        );
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
