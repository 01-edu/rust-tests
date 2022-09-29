// convert a binary search tree into a sorted array

use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn flatten_tree<T: ToOwned<Owned = T>>(tree: &BTreeSet<T>) -> Vec<T> {
    let mut flat = Vec::new();
    for v in tree {
        flat.push(v.to_owned());
    }
    flat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = BTreeSet::new();
        tree.insert(3);
        tree.insert(0);
        tree.insert(9);
        tree.insert(10);
        assert_eq!(flatten_tree(&tree), &[0, 3, 9, 10]);
    }
}
