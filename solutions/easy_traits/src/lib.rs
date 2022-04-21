/*
## easy_traits

### Instructions

Your task is to implement the trait `AppendStr` for the type StringValue.

The trait `AppendStr` has the following functions:

- `append_str`, that appends to the value of the structure a new_str of type String
- `append_number`, that appends to the value of the structure a new_number of type f64
- `remove_punctuation_marks`, that removes from the value of the structure the following punctuation marks `. , ? !`


### Expected Function

```rust
#[derive(Clone)]
struct StringValue {
    value: String,
}

trait AppendStr {
    fn append_str(self, new_str: String) -> Self;

    fn append_number(self, new_number: f64) -> Self;

    fn remove_punctuation_marks(self) -> Self;
}

impl AppendStr for StringValue {
}
```
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, new_str: String) -> Self;

    fn append_number(&mut self, new_number: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, new_str: String) -> Self {
        self.value.push_str(&new_str);
        self.clone()
    }

    fn append_number(&mut self, new_number: f64) -> Self {
        self.value = format!("{}{new_number}", self.value);

        self.clone()
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        let mut new_str = String::from("");
        let chars: Vec<char> = self.value.chars().collect();
        for i in chars {
            if i != '!' && i != '.' && i != ',' && i != '?' {
                new_str.push(i);
            }
        }

        self.value = new_str;

        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_str() {
        let mut str_aux = StringValue {
            value: String::from("hello"),
        };

        assert_eq!(
            String::from("hello there!"),
            str_aux.append_str(String::from(" there!")).value
        );

        assert_eq!(
            String::from("hello there! How are You?"),
            str_aux.append_str(String::from(" How are You?")).value
        );

        assert_eq!(
            String::from("hello there How are You"),
            str_aux.remove_punctuation_marks().value
        );
    }

    #[test]
    fn test_remove_punctuation() {
        let mut str_aux = StringValue {
            value: String::from("!?.,!?.,"),
        };

        assert_eq!(String::from(""), str_aux.remove_punctuation_marks().value);

        assert_eq!(
            String::from("h!e!l?lo. the,.re!"),
            str_aux.append_str(String::from("h!e!l?lo. the,.re!")).value
        );
        assert_eq!(
            String::from("hello there"),
            str_aux.remove_punctuation_marks().value
        );
    }

    #[test]
    fn test_append_number() {
        let mut str_aux = StringValue {
            value: String::from(""),
        };

        assert_eq!(String::from("-1"), str_aux.append_number(-1.0).value);

        assert_eq!(String::from("-15"), str_aux.append_number(5.0).value);

        assert_eq!(String::from("-155.5"), str_aux.append_number(5.5).value);

        assert_eq!(
            String::from("-1555"),
            str_aux.remove_punctuation_marks().value
        );
    }
}
