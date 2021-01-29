pub fn inv_pyramid(st: String, i: u32) -> Vec<String> {
    let mut vec = vec![];
    for v in 1..i {
        let mut s = String::new();
        for _ in 0..v {
            s.push_str(" ")
        }
        for _ in 0..v {
            s.push_str(&st)
        }
        vec.push(s);
    }
    for v in 0..i {
        let mut s = String::new();

        for _ in 0..(i - v) {
            s.push_str(" ")
        }
        for _ in 0..(i - v) {
            s.push_str(&st)
        }
        vec.push(s);
    }
    return vec;
}
