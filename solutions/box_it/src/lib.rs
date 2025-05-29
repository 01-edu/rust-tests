pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_ascii_whitespace()
        .map(|v| {
            if let Some(v) = v.strip_suffix('k') {
                Box::new((v.parse::<f64>().unwrap() * 1000.) as _)
            } else {
                Box::new(v.parse().unwrap())
            }
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}
