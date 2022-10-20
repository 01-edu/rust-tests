pub fn delete_and_backspace(s: &mut String) {
    let s_copy = s.clone();
    s.clear();

    let mut skip_next = 0;
    for v in s_copy.chars() {
        if v == '-' {
            s.pop();
        } else if v == '+' {
            skip_next += 1;
        } else if skip_next > 0 {
            skip_next -= 1;
        } else {
            s.push(v);
        }
    }
}

pub fn do_operations(v: &mut Vec<String>) {
    for (i, equation) in v.clone().iter().enumerate() {
        let numbers: Vec<&str> = equation.split(|c| c == '+' || c == '-').collect();
        let left_number = numbers[0].parse::<i32>().unwrap();
        let right_number = numbers[1].parse::<i32>().unwrap();

        let result = if equation.contains('+') {
            left_number + right_number
        } else {
            left_number - right_number
        };
        v[i] = result.to_string();
    }
}
