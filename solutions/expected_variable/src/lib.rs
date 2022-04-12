/*
## expected_variable

### Instructions

Create a function `expected_variable` that receives two strings: one to be evaluated and the other to be compared to (expected) and returns an Option. Every comparison should be case insensitive.

If the evaluated string is not in camel case or in snake case according to the `case` crate that you should use, `expected_variable` returns None.
Otherwise the evaluated string should be compared to the expected string using the `edit_distance` function that you did in one of the previous quests.

If the result of `edit_distance` has more than 50% of 'alikeness' to the expected string, considering the length of the expected string and the result of `edit_distance`, return that same percentage with a '%' symbol in front of the number.
Otherwise `expected_value` should return None.

Example:

```rs
fn main() {
    println!(
        "{} close to it",
        expected_variable("On_Point", "on_point").unwrap()
    ); // -> 100%
    println!(
        "{} close to it",
        expected_variable("soClose", "So_Close").unwrap()
    ); // -> 88%
    println!(
        "{:?}",
        expected_variable("something", "something_completely_different")
    ); // -> None
    println!(
        "{} close to it",
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
    ); // -> 73%
}
```

*/

use case::CaseExt;
use edit_distance::edit_distance;

pub fn expected_variable(evaluated: &str, expected: &str) -> Option<String> {
    let (evaluated1, correct1) = (evaluated.to_lowercase(), expected.to_lowercase());

    if (evaluated == evaluated.to_camel_lowercase()
        || evaluated == evaluated.to_camel()
        || evaluated1 == evaluated1.to_snake())
        && !evaluated1.contains("-")
        && !evaluated1.contains(" ")
    {
        let distance = edit_distance(&evaluated1, &correct1) as i64;

        if distance == 0 {
            return Some("100%".to_string());
        }
        let percentage = 100 - (distance * 100 / expected.len() as i64);
        if percentage >= 50 {
            let mut res = percentage.to_string();
            res.push_str("%");
            return Some(res);
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_variable_case() {
        let mut result = expected_variable("It is simply not a variable case", "gonnaFail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}", result
        );

        result = expected_variable("do-not-use-dashes", "do-not-use-dashes");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}", result
        );
    }

    #[test]
    fn incorrect_names() {
        let mut result = expected_variable("it_is_done", "almost_there");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}", result
        );

        result = expected_variable("frankenstein", "Dracula");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}", result
        );
    }

    #[test]
    fn one_hundred_percent() {
        assert_eq!(
            "100%",
            expected_variable("great_variable", "great_variable").unwrap()
        );
        assert_eq!("100%", expected_variable("SpOtON", "SpotOn").unwrap());
        assert_eq!(
            "100%",
            expected_variable("Another_great_variable", "Another_great_variable").unwrap()
        );
    }

    #[test]
    fn passing_tests() {
        assert_eq!("88%", expected_variable("soClose", "So_Close").unwrap());
        assert_eq!("73%", expected_variable("lets_try", "lets_try_it").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!(
            "67%",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}
