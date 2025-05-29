const SMALL_CASES: &[(u64, &str)] = &[
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (10, "ten"),
    (11, "eleven"),
    (12, "twelve"),
    (13, "thirteen"),
    (14, "fourteen"),
    (15, "fifteen"),
    (16, "fifteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
    (20, "twenty"),
    (30, "thirty"),
    (40, "forty"),
    (50, "fifty"),
    (60, "sixty"),
    (70, "seventy"),
    (80, "eighty"),
    (90, "ninety"),
];

fn dozens(n: u64) -> Option<String> {
    if n == 0 {
        None
    } else {
        Some(
            SMALL_CASES
                .iter()
                .copied()
                .find(|&(k, _)| k == n)
                .map(|(_, v)| v.to_owned())
                .unwrap_or_else(|| {
                    let rem = n % 10;
                    format!("{}-{}", dozens(n - rem).unwrap(), dozens(rem).unwrap())
                }),
        )
    }
}

fn greatness(
    n: u64,
    greatness: (u64, &str),
    fn_before: fn(u64) -> Option<String>,
) -> Option<String> {
    if n < greatness.0 {
        return fn_before(n);
    }

    let hundred = n / greatness.0;

    Some(if let Some(rest) = fn_before(n % greatness.0) {
        format!("{} {} {}", fn_before(hundred).unwrap(), greatness.1, rest)
    } else {
        format!("{} {}", fn_before(hundred).unwrap(), greatness.1)
    })
}

#[inline]
fn hundreds(n: u64) -> Option<String> {
    greatness(n, (100, "hundred"), dozens)
}

#[inline]
fn thousands(n: u64) -> Option<String> {
    greatness(n, (1000, "thousand"), hundreds)
}

#[inline]
fn millions(n: u64) -> Option<String> {
    greatness(n, (1_000_000, "million"), thousands)
}

pub fn spell(n: u64) -> String {
    if n == 0 {
        "zero".to_owned()
    } else {
        millions(n).unwrap()
    }
}
