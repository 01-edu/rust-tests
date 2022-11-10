use count_factorial_steps::*;

fn main() {
    println!("The factorial step to 0: {}", count_factorial_steps(0));
    println!("The factorial step to 6: {}", count_factorial_steps(6));
    println!("The factorial step to 123: {}", count_factorial_steps(123));
    println!("The factorial step to 720: {}", count_factorial_steps(720));
    println!(
        "The factorial step to 3628800: {}",
        count_factorial_steps(3628800)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_factorial_steps_1() {
        assert_eq!(0, count_factorial_steps(0));
        assert_eq!(0, count_factorial_steps(1));
        assert_eq!(0, count_factorial_steps(123));
        assert_eq!(6, count_factorial_steps(720));
        assert_eq!(10, count_factorial_steps(3628800));
    }
}
