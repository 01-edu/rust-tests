/*
## Scaler
### Instructions
Create the following functions, that receives two parameters:
- sum, that returns the sum between two values from 0 to 255
- diff, that returns the difference between two values from -32768 to 32767
- pro, that returns the product of the multiplication between two values from -128 to 127
- quo, that returns the quotient of the division between two values
- rem, that returns the remainder of the division between two values
### Notions
- https://doc.rust-lang.org/book/ch03-02-data-types.html
*/

fn main() {
    // sum
    println!("sum : {}", sum(234, 2));
    println!("sum : {}", sum(1, 255)); // 'ERROR: attempt to add with overflow'
    // diff
    println!("diff : {}", diff(234, 2));
    println!("diff : {}", diff(-32768, 32766)); // 'ERROR: attempt to subtract with overflow'
    // product
    println!("pro : {}", pro(23, 2));
    println!("pro : {}", pro(-128, 2)); // 'ERROR: attempt to multiply with overflow'
    // quotient
    println!("quo : {}", quo(22.0, 2.0));
    println!("quo : {}", quo(-128.23, 2.0));
    // remainder
    println!("rem : {}", rem(22.0, 2.0));
    println!("rem : {}", rem(-128.23, 2.0));
}

fn sum(a:u8, b: u8) -> u8 {
    a+b
}
fn diff(a:i16, b:i16) -> i16 {
    a-b
}
fn pro(a:i8, b:i8) -> i8 {
    a*b
}
fn quo(a:f32, b:f32) -> f32 {
    a/b
}
fn rem(a:f32, b:f32) -> f32{
    a%b
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_sum() {
        sum(25, 255);
    }
    #[test]
    #[should_panic]
    fn test_panic_diff() {
        diff(-32768, 32766);
    }
    #[test]
    #[should_panic]
    fn test_panic_pro() {
        pro(-128, 2);
    }

    #[test]
    fn pass() {
        assert_eq!(sum(1, 2), 3);
        assert_eq!(diff(1, 2), -1);
        assert_eq!(pro(1, 2), 2);
        assert_eq!(quo(1.0, 2.0), 0.5);
        assert_eq!(rem(1.0, 2.0), 1.0);
    }
}