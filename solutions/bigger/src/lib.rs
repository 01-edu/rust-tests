use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.into_values().max().unwrap()
}
