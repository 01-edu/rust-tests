pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    match s.len() {
        0 => s.to_string(),
        _ => match letter {
            '\0' => s.to_string(),
            _ => s.replace(letter, "")
        }
    }
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    match s.len() {
        0 => s.to_string(),
        _ => match letter {
            '\0' => s.to_string(),
            _ => {
                let lower_letter = letter.to_lowercase().to_string();
                let upper_letter = letter.to_uppercase().to_string();
                s
                    .replace(&upper_letter, &lower_letter)
                    .replace(&lower_letter, "")
            }
        }
    }
    
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    match s.len() {
        0 => s.to_string(),
        _ => s.split("")
            .into_iter()
            .map(|s| {
                s.chars()
                    .filter_map(|c| {
                        if c.to_string() == letter.to_lowercase().to_string() {
                            c.to_uppercase().next()
                        } else if c.to_string() == letter.to_uppercase().to_string() {
                            c.to_lowercase().next()
                        } else {
                            Some(c)
                        }
                    })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result0 = "hEy hEy";
        let result1 = "hy";
        let result2 = "bBbB";
        assert_eq!(result0, remove_letter_sensitive("hEey hEey", 'e'));
        assert_eq!(result1, remove_letter_insensitive("hEye", 'e'));
        assert_eq!(result2, swap_letter_case("BbBb", 'b'))
    }
}
