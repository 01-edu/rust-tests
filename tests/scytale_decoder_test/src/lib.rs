#[cfg(test)]
mod tests {
    use scytale_decoder::*;

    #[test]
    fn test_empty_args() {
        assert_eq!(scytale_decoder("".to_owned(), 5), None);
        assert_eq!(scytale_decoder("empty test".to_owned(), 0), None);
        assert_eq!(scytale_decoder("".to_owned(), 0), None);
    }

    #[test]
    fn test_short_nb_letters() {
        assert_eq!(
            scytale_decoder("This is already decoded".to_owned(), 100),
            Some("This is already decoded".to_owned())
        );
    }

    #[test]
    fn test_short_sentence() {
        assert_eq!(
            scytale_decoder("aebfcgd".to_owned(), 2),
            Some("abcdefg".to_owned())
        );
    }

    #[test]
    fn test_medium_sentence() {
        assert_eq!(
            scytale_decoder("oenset  daa yt hirne et hfea lflosr".to_owned(), 2),
            Some("one day in the forest a three falls".to_owned())
        );
    }

    #[test]
    fn test_long_sentence() {
        assert_eq!(
            scytale_decoder(
                "dbtheouoevyigleolepnudtmmwhheaaoegnnurigtsavoteneeosdss".to_owned(),
                5
            ),
            Some("doyouwanttobuildhousestogetherandhelpmegivesevenmangoes".to_owned())
        );
    }
}
