const WORD: &str = "stupid";

#[inline]
pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.contains(WORD) {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
