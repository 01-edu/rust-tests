pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let transform = a
        .split_ascii_whitespace()
        .map(|n| n.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    (a, transform)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let transform = b.iter().copied().map(|n| (n as f64).abs().ln()).collect();
    (b, transform)
}
