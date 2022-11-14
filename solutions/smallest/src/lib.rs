use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    let mut smallest_value = i32::MAX;

    for (_, v) in h.iter() {
        if *v < smallest_value {
            smallest_value = *v;
        }
    }
    return smallest_value;
}
