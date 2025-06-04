pub fn get_diamond(c: char) -> Vec<String> {
    let n = ((c as u8 - b'A') * 2 + 1) as _;
    let mid = (n + 1) / 2;

    let ladder = (0..mid).map(|i| {
        let mut s = vec![' '; n];
        let c = (b'A' + i as u8) as _;
        s[mid - 1 - i] = c;
        s[mid - 1 + i] = c;

        String::from_iter(s)
    });

    ladder.clone().chain(ladder.rev().skip(1)).collect()
}
