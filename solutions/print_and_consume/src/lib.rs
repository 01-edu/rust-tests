#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_consume() {
        let to_consume = "Hello".to_string();
        print_and_consume(to_consume);
        println!("{}", to_consume);
    }
    #[test]
    fn test_only_print() {
        let only_to_print = "Hello".to_string();
        only_print(&only_to_print);
        println!("{}", only_to_print);
    }
}

pub fn print_and_consume(to_consume: String) {
    println!("{}", to_consume);
}

pub fn only_print(only_to_print: &String) {
    println!("{}", only_to_print)
}
