pub fn brainfuck(pattern: &str) {
    let mut ds = vec![0 as i8; 5000];
    let mut dp: usize = 0;

    let pattern_arr = pattern.chars().collect::<Vec<char>>();

    let mut i: i32 = 0;

    if pattern_arr.len() == 0 {
        return;
    }
    while i as usize <= (&pattern_arr).len() - 1 {
        match &pattern_arr[i as usize] {
            '>' => dp += 1,
            '<' => dp -= 1,
            '+' => ds[dp] = ds[dp] + 1,
            '-' => ds[dp] = ds[dp] - 1,
            '.' => {
                print!("{}", ds[dp] as u8 as char);
            }
            '[' => {
                if ds[dp] == 0 {
                    let mut nc = 1;
                    while nc > 0 {
                        i += 1;
                        if pattern_arr[i as usize] == '[' {
                            nc += 1;
                        } else if pattern_arr[i as usize] == ']' {
                            nc -= 1;
                        }
                    }
                }
            }
            ']' => {
                if ds[dp] != 0 {
                    let mut nc = 1;
                    while nc > 0 {
                        i -= 1;
                        if pattern_arr[i as usize] == ']' {
                            nc += 1;
                        } else if pattern_arr[i as usize] == '[' {
                            nc -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    brainfuck(&args[1]);
}
