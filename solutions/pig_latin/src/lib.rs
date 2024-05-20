pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|s| {
            let mut nb_chars_to_move = 0;
            for c in s.chars() {
                if !is_vowel(c) {
                    nb_chars_to_move += 1;
                } else {
                    break;
                }
            }
            if nb_chars_to_move >= 2
                && nb_chars_to_move < s.len()
                && s.chars().nth(nb_chars_to_move - 1) == Some('q')
                && s.chars().nth(nb_chars_to_move) == Some('u')
            {
                nb_chars_to_move += 1;
            }
            format!("{}{}ay", &s[nb_chars_to_move..], &s[0..nb_chars_to_move])
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn is_vowel(mut c: char) -> bool {
    c = lowercase(c);
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn lowercase(c: char) -> char {
    c.to_lowercase().to_string().chars().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_beginning_with_vowel() {
        assert_eq!(pig_latin(&String::from("apple")), "appleay");
        assert_eq!(pig_latin(&String::from("ear")), "earay");
        assert_eq!(pig_latin(&String::from("igloo")), "iglooay");
        assert_eq!(pig_latin(&String::from("object")), "objectay");
        assert_eq!(pig_latin(&String::from("under")), "underay");
    }

    #[test]

    fn test_word_beginning_with_consonant() {
        assert_eq!(pig_latin(&String::from("queen")), "ueenqay");
        assert_eq!(pig_latin(&String::from("square")), "aresquay");
        assert_eq!(pig_latin(&String::from("equal")), "equalay");
        assert_eq!(pig_latin(&String::from("pig")), "igpay");
        assert_eq!(pig_latin(&String::from("koala")), "oalakay");
        assert_eq!(pig_latin(&String::from("yellow")), "ellowyay");
        assert_eq!(pig_latin(&String::from("xenon")), "enonxay");
        assert_eq!(pig_latin(&String::from("qat")), "atqay");
        assert_eq!(pig_latin(&String::from("chair")), "airchay");
        assert_eq!(pig_latin(&String::from("therapy")), "erapythay");
        assert_eq!(pig_latin(&String::from("thrush")), "ushthray");
        assert_eq!(pig_latin(&String::from("school")), "oolschay");
    }
}
