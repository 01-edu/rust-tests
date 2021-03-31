fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

pub fn rpn(input: &str) {
    let mut values: Vec<i64> = Vec::new();
    let op = input.split_whitespace();
    let mut err = true;

    for v in op {
        if let Ok(x) = v.parse() {
            values.push(x);
        } else {
            if is_op(v) && values.len() < 2 {
                err = false;
                break;
            }
            let (y, x) = (values.pop().unwrap(), values.pop().unwrap());
            match v {
                "+" => values.push(x + y),
                "-" => values.push(x - y),
                "*" => values.push(x * y),
                "/" => values.push(x / y),
                "%" => values.push(x % y),
                _ => {
                    err = false;
                    break;
                }
            }
        }
    }

    if values.len() == 1 && err {
        println!("{}", values[0]);
    } else {
        println!("Error");
    }
}

fn is_op(s: &str) -> bool {
    s == "+" || s == "-" || s == "*" || s == "/" || s == "%"
}
