use middle_day::*;

fn main() {
    println!("{:?}", middle_day(1022).unwrap());
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
