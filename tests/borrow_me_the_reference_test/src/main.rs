use borrow_me_the_reference::*;

fn main() {
}

#[test]
fn test_delete_and_backspace() {
    let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
    let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
    let mut a_3 = String::from("pad-rtic+eulqw--+rar");
    delete_and_backspace(&mut a_1);
    delete_and_backspace(&mut a_2);
    delete_and_backspace(&mut a_3);
    assert_eq!(a_1, "borrow".to_string());
    assert_eq!(a_2, "help".to_string());
    assert_eq!(a_3, "particular".to_string());
}
#[test]
fn test_is_correct() {
    let mut b_1: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];
    let mut b_2: Vec<&str> = vec!["1+2=4", "0+2=5", "10-3=3", "41+5=10"];
    let mut b_3: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=7", "5+5=10"];
    let result_1 = is_correct(&mut b_1);
    let result_2 = is_correct(&mut b_2);
    let result_3 = is_correct(&mut b_3);
    assert_eq!(result_1, 75);
    assert_eq!(result_2, 0);
    assert_eq!(result_3, 100);

    assert_eq!(b_1, vec!["✔", "✔", "✘", "✔"]);
    assert_eq!(b_2, vec!["✘", "✘", "✘", "✘"]);
    assert_eq!(b_3, vec!["✔", "✔", "✔", "✔"]);
}
