use std::env;

fn match_brackets(s: String) -> bool {
    let mut opened = vec![];

    let mut ptr = -1;
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            opened.push(c);
            ptr += 1;
        } else if c == ')' {
            if ptr < 0 || opened[ptr as usize] != '(' {
                return false;
            }
            let (o, _) = opened.split_at(opened.len() - 1);
            opened = o.to_vec();
            ptr -= 1;
        } else if c == ']' {
            if ptr < 0 || opened[ptr as usize] != '[' {
                return false;
            }
            let (o, _) = opened.split_at(opened.len() - 1);
            opened = o.to_vec();
            ptr -= 1;
        } else if c == '}' {
            if ptr < 0 || opened[ptr as usize] != '{' {
                return false;
            }
            let (o, _) = opened.split_at(opened.len() - 1);
            opened = o.to_vec();
            ptr -= 1;
        }
    }

    opened.is_empty()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    for (i, v) in args.into_iter().enumerate() {
        if i != 0 {
            if match_brackets(v) {
                println!("OK");
            } else {
                println!("Error");
            }
        }
    }
}
