use std::collections::HashMap;

#[inline]
pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    h.values().min().copied().unwrap_or(i32::MAX)
}
