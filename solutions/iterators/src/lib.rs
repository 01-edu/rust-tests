use std::iter::successors;

pub fn collatz(n: u64) -> Option<usize> {
    if n == 0 {
        return None;
    }

    let mut seq = successors(Some(Ok(n)), |&val| {
        val.ok().and_then(|x| {
            if x == 1 {
                None
            } else {
                Some(match x % 2 {
                    0 => Ok(x / 2),
                    1 => x.checked_mul(3).and_then(|y| y.checked_add(1)).ok_or(()),
                    _ => unreachable!(),
                })
            }
        })
    });

    seq.try_fold(0, |count, step| step.map(|_| count + 1))
        .ok()
        .map(|count| count - 1)
}
