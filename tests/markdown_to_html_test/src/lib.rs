#[cfg(test)]
mod tests {
    use markdown_to_html::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_subject_example() {
        assert_eq!(
            markdown_to_html(include_str!("./tests/example.md")),
            include_str!("./tests/example.html")
        );
    }

    #[test]
    fn test_titles() {
        assert_eq!(
            markdown_to_html(include_str!("./tests/titles.md")),
            include_str!("./tests/titles.html")
        );
    }

    #[test]
    fn test_bold_italic() {
        assert_eq!(
            markdown_to_html(include_str!("./tests/bold_italic.md")),
            include_str!("./tests/bold_italic.html")
        );
    }

    #[test]
    fn test_blockquotes() {
        assert_eq!(
            markdown_to_html(include_str!("./tests/blockquotes.md")),
            include_str!("./tests/blockquotes.html")
        );
    }
}
