/*
## rgb_match

### Instructions

Implement the struct `Color` with the function `swap`.
This function must allow you to swap the values of the struct.

### Notions

- https://doc.rust-lang.org/book/ch18-00-patterns.html

### Expected functions

```rust
impl Color {
    fn swap(mut self, first: u8, second: u8) -> Color {}
    }
```

### Usage

Here is a program to test your function.

```rust
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn main() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };

    println!("{:?}", c.swap(c.r, c.b));
    println!("{:?}", c.swap(c.r, c.g));
    println!("{:?}", c.swap(c.r, c.a));
    println!();
    println!("{:?}", c.swap(c.g, c.r));
    println!("{:?}", c.swap(c.g, c.b));
    println!("{:?}", c.swap(c.g, c.a));
    println!();
    println!("{:?}", c.swap(c.b, c.r));
    println!("{:?}", c.swap(c.b, c.g));
    println!("{:?}", c.swap(c.b, c.a));
    println!();
    println!("{:?}", c.swap(c.a, c.r));
    println!("{:?}", c.swap(c.a, c.b));
    println!("{:?}", c.swap(c.a, c.g));
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
Color { r: 10, g: 200, b: 255, a: 30 }
Color { r: 200, g: 255, b: 10, a: 30 }
Color { r: 30, g: 200, b: 10, a: 255 }

Color { r: 200, g: 255, b: 10, a: 30 }
Color { r: 255, g: 10, b: 200, a: 30 }
Color { r: 255, g: 30, b: 10, a: 200 }

Color { r: 10, g: 200, b: 255, a: 30 }
Color { r: 255, g: 10, b: 200, a: 30 }
Color { r: 255, g: 200, b: 30, a: 10 }

Color { r: 30, g: 200, b: 10, a: 255 }
Color { r: 255, g: 200, b: 30, a: 10 }
Color { r: 255, g: 30, b: 10, a: 200 }

student@ubuntu:~/[[ROOT]]/test$
```


*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match first {
            v if v == self.r => match second {
                v if v == self.b => {
                    self.r = second;
                    self.b = first;
                }
                v if v == self.g => {
                    self.r = second;
                    self.g = first;
                }
                v if v == self.a => {
                    self.r = second;
                    self.a = first;
                }
                _ => {}
            },

            v if v == self.g => match second {
                v if v == self.r => {
                    self.g = second;
                    self.r = first;
                }
                v if v == self.b => {
                    self.g = second;
                    self.b = first;
                }
                v if v == self.a => {
                    self.g = second;
                    self.a = first;
                }
                _ => {}
            },

            v if v == self.a => match second {
                v if v == self.r => {
                    self.a = second;
                    self.r = first;
                }
                v if v == self.b => {
                    self.a = second;
                    self.b = first;
                }
                v if v == self.g => {
                    self.a = second;
                    self.g = first;
                }
                _ => {}
            },

            v if v == self.b => match second {
                v if v == self.r => {
                    self.b = second;
                    self.r = first;
                }
                v if v == self.g => {
                    self.b = second;
                    self.g = first;
                }
                v if v == self.a => {
                    self.b = second;
                    self.a = first;
                }
                _ => {}
            },
            _ => {}
        }

        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };
        // swap r
        assert_eq!(
            c.swap(c.r, c.b),
            Color {
                r: 10,
                g: 200,
                b: 255,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.r, c.g),
            Color {
                r: 200,
                g: 255,
                b: 10,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.r, c.a),
            Color {
                r: 30,
                g: 200,
                b: 10,
                a: 255
            }
        );

        // swap g
        assert_eq!(
            c.swap(c.g, c.r),
            Color {
                r: 200,
                g: 255,
                b: 10,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.g, c.b),
            Color {
                r: 255,
                g: 10,
                b: 200,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.g, c.a),
            Color {
                r: 255,
                g: 30,
                b: 10,
                a: 200
            }
        );

        // swap b
        assert_eq!(
            c.swap(c.b, c.r),
            Color {
                r: 10,
                g: 200,
                b: 255,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.b, c.g),
            Color {
                r: 255,
                g: 10,
                b: 200,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.b, c.a),
            Color {
                r: 255,
                g: 200,
                b: 30,
                a: 10
            }
        );

        // swap a
        assert_eq!(
            c.swap(c.a, c.r),
            Color {
                r: 30,
                g: 200,
                b: 10,
                a: 255
            }
        );
        assert_eq!(
            c.swap(c.a, c.b),
            Color {
                r: 255,
                g: 200,
                b: 30,
                a: 10
            }
        );
        assert_eq!(
            c.swap(c.a, c.g),
            Color {
                r: 255,
                g: 30,
                b: 10,
                a: 200
            }
        );
    }
}
