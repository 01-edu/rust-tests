use invert_sentence::*;

fn main() {
    println!("{}", invert_sentence("Rust is Awesome"));
    println!("{}", invert_sentence("   word1     word2  "));
    println!("{}", invert_sentence("Hello, World!"));
}


#[cfg(test)]
mod tests {
    use invert_sentence::*;

    #[test]
    fn single_word_sentence() {
        let sentence = "word";
        assert!(invert_sentence(sentence) == "word");
    }

    #[test]
    fn multiple_words_sentence() {
        let sentence = "Rust is Awesome";
        assert!(invert_sentence(sentence) == "Awesome is Rust");
    }

    #[test]
    fn multiple_leading_and_trailing_whitespaces() {
        let sentence = "   word1     word2  ";
        assert!(invert_sentence(sentence) == "word2 word1");
    }

    #[test]
    fn multiple_words_sentence_with_punctuation() {
        let sentence = "Hello, World!";
        assert!(invert_sentence(sentence) == "World! Hello,");
    }

    #[test]
    fn empty_sentence() {
        let sentence = "";
        assert!(invert_sentence(sentence) == "");
    }
}
