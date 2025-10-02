const STACK_MEM: usize = 2048;

pub fn brainfuck(code: &str) {
    if code.is_empty() {
        return;
    }

    let mut mem = [0i8; STACK_MEM];
    let mut mp = 0;
    let code = code.as_bytes();
    let jumps = build_jumps(code);
    let mut ip = 0;

    while ip < code.len() {
        match code[ip] {
            b'>' => mp = (mp + 1) % STACK_MEM,
            b'<' => mp = (mp + STACK_MEM - 1) % STACK_MEM,
            b'+' => mem[mp] = mem[mp].wrapping_add(1),
            b'-' => mem[mp] = mem[mp].wrapping_sub(1),
            b'.' => print!("{}", mem[mp] as u8 as char),
            b'[' => {
                if mem[mp] == 0 {
                    ip = jumps[ip]
                }
            }
            b']' => {
                if mem[mp] != 0 {
                    ip = jumps[ip]
                }
            }
            _ => (),
        }
        ip += 1;
    }
}

fn build_jumps(code: &[u8]) -> Vec<usize> {
    let mut jumps = vec![0; code.len()];
    let mut stack = Vec::new();

    for (i, &b) in code.iter().enumerate() {
        match b {
            b'[' => stack.push(i),
            b']' => {
                if let Some(start) = stack.pop() {
                    jumps[start] = i;
                    jumps[i] = start;
                }
            }
            _ => (),
        }
    }

    jumps
}

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        brainfuck(&arg);
    }
}
