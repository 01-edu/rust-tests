// Write a function `acronym` that turns a phrase into an acronym.
// Example:
//          "HyperText Markup Language" -> HTML
//          "Something - I made up" -> "SIMU"

pub fn acronym(s: &str) -> String {
    s.split(|c| c == ' ' || c == '-' || c == '_')
        .filter(|e| !e.is_empty())
        .flat_map(|str_s| {
            let chars: Vec<char> = str_s.chars().collect();
            chars
                .iter()
                .enumerate()
                .filter_map(|(i, e)| {
                    if i == 0 || (e.is_uppercase() && !chars[i - 1].is_uppercase()) {
                        Some(e.to_uppercase().to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<String>()
}

fn main() {
    println!("{}", acronym("GNU Image Manipulation Program"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_cases() {
        assert_eq!(acronym(""), "");
        assert_eq!(acronym("Portable Network Graphics"), "PNG");
        assert_eq!(acronym("HyperText Markup Language"), "HTML");
        assert_eq!(acronym("Ruby on Rails"), "ROR");
        assert_eq!(acronym("First In, First Out"), "FIFO");
        assert_eq!(acronym("GNU Image Manipulation Program"), "GIMP");
        assert_eq!(acronym("PHP: Hypertext Preprocessor"), "PHP");
        assert_eq!(acronym("Complementary metal-oxide semiconductor"), "CMOS");
        assert_eq!(
            acronym("Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"),
            "ROTFLSHTMDCOALM"
        );
        assert_eq!(acronym("Something - I made up from thin air"), "SIMUFTA");
        assert_eq!(acronym("The Road _Not_ Taken"), "TRNT");
        assert_eq!(acronym("Halley's Comet"), "HC");
    }
}
