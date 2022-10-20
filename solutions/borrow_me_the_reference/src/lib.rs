use meval::eval_str;

fn d_a_b(s: String) -> String {
    let mut new_string = String::new();
    let mut count = 0;
    for v in s.chars() {
        if count != 0 && v != '+' {
            count -= 1;
            continue;
        }
        if v == '+' {
            count += 1;
            continue;
        }
        new_string.push(v);
        if v == '-' {
            new_string.pop();
            new_string.pop();
        }
    }
    new_string
}
// - If a value does **not implement Copy**, it must be **borrowed** and so will be passed by **reference**.
// this is not good in rust because strings are not to be manipulated like this
// but its a good view for the students to see how memory works in rust
pub fn delete_and_backspace(s: &mut String) {
    let a = d_a_b(s.clone());
    s.clear();
    s.push_str(&a);
}

// - If a value does **not implement Copy**, it must be **borrowed** and so will be passed by **reference**.
pub fn is_correct(v: &mut Vec<&str>) -> usize {
    let mut percentage = 0;
    for (i, equation) in v.clone().iter().enumerate() {
        let a: Vec<&str> = equation.split('=').collect();
        if eval_str(a[0]).unwrap() == a[1].parse::<f64>().unwrap() {
            v[i] = "✔";
            percentage += 1;
        } else {
            v[i] = "✘";
        }
    }
    (percentage * 100) / v.len()
}
