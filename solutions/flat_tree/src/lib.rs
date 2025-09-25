use std::collections::BTreeSet;

#[inline]
pub fn flatten_tree<T: ToOwned<Owned = T>>(tree: &BTreeSet<T>) -> Vec<T> {
    tree.iter().map(T::to_owned).collect()
}
