pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let digits = nb.checked_ilog10().unwrap_or_default() + 1;

    let armstrong = (0..digits)
        .scan(nb, |state, _| {
            let val = (*state % 10).pow(digits);

            *state /= 10;

            Some(val)
        })
        .sum::<u32>();

    (armstrong == nb).then_some(nb)
}
