use std::{collections::HashMap, hash::Hash};

#[inline]
pub fn slices_to_map<T: Eq + Hash + Copy, U: Copy>(keys: &[T], values: &[U]) -> HashMap<T, U> {
    keys.iter().copied().zip(values.iter().copied()).collect()
}
