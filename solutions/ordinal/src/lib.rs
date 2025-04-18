/*
 Complete the function "num_to_ordinal" that receives a cardinal number and returns its ordinal number.

fn main() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
}
*/

pub fn num_to_ordinal(x: u32) -> String {
    format!(
        "{}{}",
        x,
        match (x % 10, x % 100) {
            (1, 1) | (1, 21..=91) => "st",
            (2, 2) | (2, 22..=92) => "nd",
            (3, 3) | (3, 23..=93) => "rd",
            _ => "th",
        }
    )
}

#[test]
fn test_num_to_ordinal() {
    assert_eq!(num_to_ordinal(0), "0th");
    assert_eq!(num_to_ordinal(1), "1st");
    assert_eq!(num_to_ordinal(12), "12th");
    assert_eq!(num_to_ordinal(22), "22nd");
    assert_eq!(num_to_ordinal(43), "43rd");
    assert_eq!(num_to_ordinal(67), "67th");
    assert_eq!(num_to_ordinal(1901), "1901st");
}
