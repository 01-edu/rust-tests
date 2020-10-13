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

### Notions

- https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/question_mark.html

*/
#[derive(Clone, Copy)]
struct One {
    first_layer: Option<Two>
}

#[derive(Clone, Copy)]
struct Two {
    second_layer: Option<Three>
}

#[derive(Clone, Copy)]
struct Three {
    third_layer: Option<Four>
}

#[derive(Clone, Copy)]
struct Four {
    fourth_layer: Option<u16>
}

impl One {
    fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}

/*
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
    println!("{:?}", match a.get_third_layer() {
        Some(e) => e,
        None => 0
    })
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_value() {
        let a = One {
            first_layer : Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(1000)
                    })
                })
            })
        };
        let b = One {
            first_layer : Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(3)
                    })
                })
            })
        };
        assert_eq!(a.get_fourth_layer(), Some(1000));
        assert_eq!(b.get_fourth_layer(), Some(3));
    }
}
