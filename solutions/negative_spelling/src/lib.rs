pub fn negative_spell(n: i64) -> String {
    match n {
        1.. => "error: positive number".to_string(),
        0 => "zero".to_string(),
        _ => format!("minus {}", spell(-n as u64)),
    }
}

pub fn spell(n: u64) -> String {
    match n {
        0..=99 => spells_below_100(n),
        100..=999 => spells_hundreds(n),
        _ => spells_bignum(n),
    }
}

pub fn spells_below_100(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "fifteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        _ => {
            let rem = n % 10;
            format!("{}-{}", spells_below_100(n - rem), spells_below_100(rem))
        }
    }
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
    let mut enc_chunks: Vec<String> = vec![];
    let mut chunks: Vec<u64> = vec![0; 7];
    let mut m = n;
    for e in chunks.iter_mut() {
        let rem = m % 1_000;
        m /= 1_000;
        *e += rem;
    }
    for (idx, chunk) in chunks.into_iter().enumerate() {
        let substr = match idx {
            0 => "",
            1 => "thousand",
            2 => "million",
            3 => "billion",
            4 => "trillion",
            5 => "quadrillion",
            _ => "quintillion",
        };
        if chunk != 0 {
            enc_chunks.push(format!("{} {}", spell(chunk), substr).trim().to_string());
        }
    }
    enc_chunks.reverse();
    enc_chunks.join(" ")
}
