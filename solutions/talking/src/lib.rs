pub fn talking(text: &str) -> &str {
    if text.chars().all(|c| !c.is_ascii_lowercase()) && text.chars().any(|c| c.is_ascii_uppercase())
    {
        if text.ends_with('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else if text.ends_with('?') {
        "Sure."
    } else if text.trim().is_empty() {
        "Just say something!"
    } else {
        "Interesting"
    }
}
