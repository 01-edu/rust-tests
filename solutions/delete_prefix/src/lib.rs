// Define the function `delete_prefix(prefix: &str, s: &str) -> Option<&str>`
// That takes 2 slices of string and returns the string of slice s
// with the `prefix` removed wrapped in Some
// If `prefix ` is not contained in `s` return None

// Example:
// delete_prefix("hello, ", "hello, world")? == "world"
// delete_prefix("not", "win");

pub fn delete_prefix<'a, 'b>(prefix: &'b str, s: &'a str) -> Option<&'a str> {
    if prefix.len() > s.len() {
        return None;
    }
    let mut char_s = s.chars();

    for char_prefix in prefix.chars() {
        if char_prefix != char_s.next()? {
            return None;
        }
    }
    Some(&s[prefix.len()..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_prefix() {
        assert_eq!(
            delete_prefix("augusto", "augusto ornelas"),
            Some(" ornelas")
        );

        assert_eq!(delete_prefix("ab", "b"), None);

        assert_eq!(delete_prefix("aa", "ab"), None);

        assert_eq!(delete_prefix("á©", "á©ab"), Some("ab"));
    }
}
