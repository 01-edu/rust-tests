use std::env;

fn match_brackets(s: &str) -> bool {
    let runes: Vec<&str> = s.split("").collect();
    let mut opened: Vec<&str> = vec![];
    let mut ptr: i32 = -1;
    for c in runes.into_iter() {
        if c == "(" || c == "[" || c == "{" {
            opened.push(c);
            ptr += 1;
        } else if c == ")" {
            if ptr < 0 || opened[ptr as usize] != "(" {
                return false;
            }
            let (o, _) = opened.split_at(opened.len() - 1);
            opened = o.to_vec();
            ptr -= 1;
        } else if c == "]" {
            if ptr < 0 || opened[ptr as usize] != "[" {
                return false;
            }
            let (o, _) = opened.split_at(opened.len() - 1);
            opened = o.to_vec();
            ptr -= 1;
        } else if c == "}" {
            if ptr < 0 || opened[ptr as usize] != "{" {
                return false;
            }
            let (o, _) = opened.split_at(opened.len() - 1);
            opened = o.to_vec();
            ptr -= 1;
        }
    }
    return opened.len() == 0;
}

fn main() {
    let arg1: Vec<String> = env::args().collect();
    if arg1.len() == 0 {
        println!("");
    } else {
        for (i, v) in arg1.iter().enumerate() {
            if i != 0 {
                if match_brackets(v) {
                    println!("OK");
                } else {
                    println!("Error");
                }
            }
        }
    }
}
