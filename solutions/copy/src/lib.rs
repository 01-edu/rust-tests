pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    (
        a.clone(),
        a.split_ascii_whitespace()
            .map(|n| n.parse::<f64>().unwrap().exp().to_string())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    (
        b.clone(),
        b.into_iter().map(|n| (n as f64).abs().ln()).collect(),
    )
}
