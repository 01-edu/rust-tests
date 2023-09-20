pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let nb_digits = nb_digits(nb);
    let mut new_num = nb;
    let mut armstrong: u32 = 0;

    while new_num != 0 {
        let digit = new_num % 10;
        match armstrong.checked_add(digit.pow(nb_digits)) {
            Some(nb) => armstrong = nb,
            None => return None,
        }

        new_num /= 10;
    }
    (armstrong == nb).then_some(nb)
}

fn nb_digits(mut nb: u32) -> u32 {
    let mut nb_digits = 0;

    while nb != 0 {
        nb_digits += 1;
        nb /= 10;
    }
    nb_digits
}
