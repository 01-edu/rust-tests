/*
## adding_twice

### Instructions

In this exercise you will have to reuse your `add_curry` function
Then you have to complete the function `twice` using closures, this function will
take a function f(x) as parameter and return a function f(f(x))
So, the purpose of this function is to add two times the value in `add_curry` to the original value.

### Notions

- https://doc.rust-lang.org/rust-by-example/fn/hof.html#higher-order-functions

### Expected functions

The type of the arguments are missing use the example `main` function to determine the correct type.

```rust
fn twice<T>(F: _) -> _{}
```

### Usage

Here is a program to test your function.

```rust
fn main() {
    let add10 = add_curry(10);
    let value = twice(add10);
    println!("The value is {}", value(7));

    let add20 = add_curry(20);
    let value = twice(add20);
    println!("The value is {}", value(7));

    let add30 = add_curry(30);
    let value = twice(add30);
    println!("The value is {}", value(7));

    let neg = add_curry(-32);
    let value = twice(neg);
    println!("The value is {}", value(7));
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
The value is 27
The value is 47
The value is 67
The value is -57
student@ubuntu:~/[[ROOT]]/test$
```

*/

pub fn twice<T>(function: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |a| function(function(a))
}

pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    return move |y| -> i32 { x + y };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_twice() {
        let z = twice(add_curry(0));
        assert_eq!(z(1902), 1902);
    }

    #[test]
    fn test_negative_twice() {
        let neg = twice(add_curry(-32));
        assert_eq!(neg(6), -58);
        assert_eq!(neg(10), -54);
        assert_eq!(neg(660), 596);
        assert_eq!(neg(1902), 1838);
        assert_eq!(neg(463), 399);
        assert_eq!(neg(400000000), 399999936);
    }

    #[test]
    fn test_add10_twice() {
        let value = twice(add_curry(10));
        assert_eq!(value(6), 26);
        assert_eq!(value(10), 30);
        assert_eq!(value(600), 620);
        assert_eq!(value(1000), 1020);
        assert_eq!(value(463), 483);
        assert_eq!(value(400000000), 400000020);
    }
    #[test]
    fn test_add20_twice() {
        let value = twice(add_curry(20));
        assert_eq!(value(6), 46);
        assert_eq!(value(10), 50);
        assert_eq!(value(600), 640);
        assert_eq!(value(1000), 1040);
        assert_eq!(value(463), 503);
        assert_eq!(value(400000000), 400000040);
    }
    #[test]
    fn test_add30_twice() {
        let value = twice(add_curry(30));
        assert_eq!(value(6), 66);
        assert_eq!(value(10), 70);
        assert_eq!(value(600), 660);
        assert_eq!(value(1000), 1060);
        assert_eq!(value(463), 523);
        assert_eq!(value(400000000), 400000060);
    }
}
