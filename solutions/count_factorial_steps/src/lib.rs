pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut step = 0;
    let mut multiplier = 1;
    let mut curr_val = factorial;

    loop {
        if curr_val < 2 {
            return step;
        } else if !curr_val.is_multiple_of(multiplier) {
            return 0;
        }
        curr_val /= multiplier;
        step += 1;
        multiplier += 1;
    }
}
