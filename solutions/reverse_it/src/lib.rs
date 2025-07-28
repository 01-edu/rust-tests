pub fn reverse_it(nbr: i32) -> String {
    let s = nbr.to_string();
    let str_nbr = s.trim_start_matches('-');
    let rev: String = str_nbr.chars().rev().collect();
    if nbr >= 0 {
        return format!("{}{}", rev, str_nbr);
    }
    return format!("-{}{}", rev, str_nbr);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_it_test() {
        assert_eq!("321123", &reverse_it(123));
        assert_eq!("987654321123456789", &reverse_it(123456789));
        assert_eq!("00", &reverse_it(0));
        assert_eq!("-321123", &reverse_it(-123));
        assert_eq!("11", &reverse_it(1));
    }
}
