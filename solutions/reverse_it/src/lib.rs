pub fn reverse_it(nbr: i32) -> String {
    let s = nbr.to_string();
    let abs_s = s.trim_start_matches('-');
    let rev = abs_s.chars().rev().collect::<String>();
    format!("{}{}{}", if nbr < 0 { "-" } else { "" }, rev, abs_s)
}
