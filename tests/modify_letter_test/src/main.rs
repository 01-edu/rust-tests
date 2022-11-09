use modify_letter::*;

fn main() {
    println!("{}", remove_letter_sensitive("hEey hEey", 'e'));
    println!("{}", remove_letter_insensitive("hEye", 'e'));
    println!("{}", swap_letter_case("BbBb", 'b'));
    println!("{}", remove_letter_sensitive("", 'a'));
    println!("{}", remove_letter_sensitive("a", '\0'));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_letter_sensitive() {
        assert_eq!(remove_letter_sensitive("Joje jis mijssjing", 'j'), "Joe is missing")
    }
    
    #[test]
    fn test_remove_letter_insensitive() {
        assert_eq!(remove_letter_insensitive("JaillA ais swiaAmmingA", 'A'), "Jill is swimming")
    }
    
    #[test]
    fn test_swap_letter_case() {
        assert_eq!(swap_letter_case("hEllo therE", 'e'), "hello thEre")
    }
    
    #[test]
    fn test_empty_arguments() {
        assert_eq!(remove_letter_sensitive("", '\0'), "");
        assert_eq!(remove_letter_insensitive("", '\0'), "");
        assert_eq!(swap_letter_case("", '\0'), "");
        assert_eq!(remove_letter_sensitive("a", '\0'), "a");
        assert_eq!(remove_letter_insensitive("a", '\0'), "a");
        assert_eq!(swap_letter_case("a", '\0'), "a");
    }
}