pub fn fibonacci(n: u32) -> u32 {
    if matches!(n, 0 | 1) {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
