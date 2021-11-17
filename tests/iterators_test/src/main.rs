use iterators::*;

fn main() {
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(1000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        assert_eq!(Some(0), collatz(1));
        assert_eq!(Some(4), collatz(16));
        assert_eq!(Some(9), collatz(12));
        assert_eq!(Some(152), collatz(1000000));
    }

    #[test]
    fn test_none() {
        assert_eq!(None, collatz(0));
    }
}
