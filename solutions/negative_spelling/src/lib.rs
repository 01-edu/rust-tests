#[inline]
pub fn negative_spell(n: i64) -> String {
    match n {
        1.. => "error: positive number".to_owned(),
        0 => "zero".to_owned(),
        _ => format!("minus {}", spell(-n as u64)),
    }
}

#[inline]
pub fn spell(n: u64) -> String {
    match n {
        0..=99 => spells_below_100(n),
        100..=999 => spells_hundreds(n),
        _ => spells_bignum(n),
    }
}

#[inline]
pub fn spells_below_100(n: u64) -> String {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => {
            let rem = n % 10;
            return format!("{}-{}", spells_below_100(n - rem), spells_below_100(rem));
        }
    }
    .to_owned()
}

pub fn spells_hundreds(n: u64) -> String {
    let div = n / 100;
    let rem = n % 100;
    let mut enc_str = format!("{} hundred", spells_below_100(div));
    if rem != 0 {
        enc_str = format!("{} {}", enc_str, spells_below_100(rem));
    }
    enc_str
}

pub fn spells_bignum(n: u64) -> String {
    let chunks = [0; 7].into_iter().scan(n, |acc, _| {
        let rem = *acc % 1_000;
        *acc /= 1_000;
        Some(rem)
    });

    let mut s = chunks
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let substr = match i {
                0 => "",
                1 => "thousand",
                2 => "million",
                3 => "billion",
                4 => "trillion",
                5 => "quadrillion",
                _ => "quintillion",
            };
            if chunk != 0 {
                Some(format!("{} {}", spell(chunk), substr))
            } else {
                None
            }
        })
        .flat_map(|v| v)
        .collect::<Vec<_>>();

    s.reverse();

    s.join(" ").trim().to_owned()
}
