

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        print_and_consume("Hello".to_string())
    }
}

pub fn print_and_consume(to_consume: String) {
    println!("{}", to_consume);
}