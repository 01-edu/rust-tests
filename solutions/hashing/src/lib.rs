use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();

    sorted.sort_unstable();

    if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2]
    } else {
        let (l, r) = (sorted[sorted.len() / 2 - 1], sorted[sorted.len() / 2]);
        (l + r) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    list.iter()
        .fold(
            HashMap::<_, i32>::with_capacity(list.len() / 2),
            |mut m, &v| {
                *m.entry(v).or_default() += 1;
                m
            },
        )
        .into_iter()
        .max_by_key(|&(_, v)| v)
        .unwrap()
        .0
}
