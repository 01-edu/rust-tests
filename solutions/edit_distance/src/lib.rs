// Levenshtein's distance using two matrix rows

use std::mem;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    let mut v0 = (0..=n).collect::<Vec<_>>();
    let mut v1 = vec![0; n + 1];

    for i in 0..m {
        v1[0] = i + 1;

        for j in 0..n {
            let deletion_cost = v0[j + 1] + 1;
            let insertion_cost = v1[j] + 1;
            let substitution_cost = v0[j]
                + if source.chars().nth(i) == target.chars().nth(j) {
                    0
                } else {
                    1
                };

            v1[j + 1] = [deletion_cost, insertion_cost, substitution_cost]
                .into_iter()
                .min()
                .unwrap();
        }

        mem::swap(&mut v0, &mut v1);
    }

    v0[n]
}
