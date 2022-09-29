/*
## question_mark

### Instructions

You will have to create 3 structures:

- `One`, that contains one element called `first_layer` it should be an `Option` for the structure `Two`.
- `Two`, that contains one element called `second_layer` it should be an `Option` for the structure `Three`.
- `Three`, that contains one element called `third_layer` it should be an `Option` for the structure `Four`.
- `Four`, that contains one element called `fourth_layer` it should be an `u16` that is an `Option`.

Beside the structure you must create a function named `get_fourth_layer` that is associated to the `One` structure.
This function should return the `Option` value in the `Four` structure.

### Example

```rust
fn main() {
    let a = One {
        first_layer : Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four {
                    fourth_layer: Some(1000)
                })
            })
        })
    };

    // output: 1000
    println!("{:?}", match a.get_fourth_layer() {
        Some(e) => e,
        None => 0
    })
}
```

### Notions

- https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/question_mark.html

*/
#[derive(Clone, Copy)]
pub struct One {
    pub first_layer: Option<Two>,
}

#[derive(Clone, Copy)]
pub struct Two {
    pub second_layer: Option<Three>,
}

#[derive(Clone, Copy)]
pub struct Three {
    pub third_layer: Option<Four>,
}

#[derive(Clone, Copy)]
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_value() {
        let a = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(1000),
                    }),
                }),
            }),
        };
        let b = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(3),
                    }),
                }),
            }),
        };
        assert_eq!(a.get_fourth_layer(), Some(1000));
        assert_eq!(b.get_fourth_layer(), Some(3));
    }
}
