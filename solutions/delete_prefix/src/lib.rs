#[inline]
pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    s.strip_prefix(prefix)
}
