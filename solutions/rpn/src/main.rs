fn main() {
    match std::env::args().collect::<Vec<_>>().as_slice() {
        [_, arg] => rpn(arg.as_str()),
        _ => println!("Error"),
    }
}

#[inline]
pub fn rpn(input: &str) {
    if rpn_wrapper(input).is_err() {
        println!("Error");
    }
}

pub fn rpn_wrapper(input: &str) -> Result<(), ()> {
    let mut values = Vec::<i64>::new();

    for v in input.split_whitespace() {
        if let Ok(x) = v.parse() {
            values.push(x);
        } else {
            let (y, x) = (values.pop().ok_or(())?, values.pop().ok_or(())?);
            match v {
                "+" => values.push(x + y),
                "-" => values.push(x - y),
                "*" => values.push(x * y),
                "/" => values.push(x / y),
                "%" => values.push(x % y),
                _ => return Err(()),
            }
        }
    }

    if values.len() != 1 {
        Err(())
    } else {
        println!("{}", values[0]);
        Ok(())
    }
}
