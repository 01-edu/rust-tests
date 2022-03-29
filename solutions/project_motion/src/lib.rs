/*
## project_motion

### Instructions

For this exercise, you will have to create a [projectile motion](https://cimg2.ck12.org/datastreams/f-d%3Abb024be6673110b31e78b46819e792adaed8dc661e082a61f0a6d64e%2BIMAGE%2BIMAGE.1).

Two structures will be provided. A structure called `Object` which will have the corresponding values of X and Y.

A structure called `ThrowObject` that will contain all the variables that are
essential for the projectile physics (initial position, initial velocity, current position, current velocity and time).

You must implement :

- A function `new` that will initialize the ThrowObject with a given initial position and an initial velocity.
- The trait Iterator with the `.next()` in which the position and speed of the object must be calculated after 1 second.
  It will return an `Option` with the ThrowObject, or it will return `None` if the ThrowObject has already reached the floor.

### Notions

- [trait Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [iter](https://doc.rust-lang.org/rust-by-example/trait/iter.html)

*/

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<ThrowObject> {
        self.time += 1.0;
        let actual_distance = Object {
            x: round_two(self.init_position.x + self.init_velocity.x * self.time),
            y: round_two(
                self.init_position.y + self.init_velocity.y * self.time
                    - 9.8 * self.time * self.time / 2.0,
            ),
        };

        let actual_velocity = Object {
            x: round_two(self.init_velocity.x),
            y: round_two(self.init_velocity.y - 9.8 * self.time),
        };

        if actual_distance.y < 0.0 {
            return None;
        } else {
            return Some(ThrowObject {
                init_position: self.init_position.clone(),
                init_velocity: self.init_velocity.clone(),
                actual_position: actual_distance,
                actual_velocity: actual_velocity,
                time: self.time,
            });
        }
    }
}

fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_velocity() {
        let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 45.1 },
                actual_velocity: Object { x: 0.0, y: -9.8 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 30.4 },
                actual_velocity: Object { x: 0.0, y: -19.6 },
                time: 2.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 5.9 },
                actual_velocity: Object { x: 0.0, y: -29.4 },
                time: 3.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_velocity() {
        let mut obj = ThrowObject::new(Object { x: 0.0, y: 50.0 }, Object { x: 10.0, y: 10.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 10.0, y: 55.1 },
                actual_velocity: Object { x: 10.0, y: 0.2 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 20.0, y: 50.4 },
                actual_velocity: Object { x: 10.0, y: -9.6 },
                time: 2.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 30.0, y: 35.9 },
                actual_velocity: Object { x: 10.0, y: -19.4 },
                time: 3.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 40.0, y: 11.6 },
                actual_velocity: Object { x: 10.0, y: -29.2 },
                time: 4.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_negative_velocity() {
        let mut obj = ThrowObject::new(Object { x: -10.0, y: 50.0 }, Object { x: -10.0, y: -10.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: -10.0, y: 50.0 },
                init_velocity: Object { x: -10.0, y: -10.0 },
                actual_position: Object { x: -20.0, y: 35.1 },
                actual_velocity: Object { x: -10.0, y: -19.8 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: -10.0, y: 50.0 },
                init_velocity: Object { x: -10.0, y: -10.0 },
                actual_position: Object { x: -30.0, y: 10.4 },
                actual_velocity: Object { x: -10.0, y: -29.6 },
                time: 2.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_Long_distance() {
        let mut obj = ThrowObject::new(Object { x: 0.0, y: 150.0 }, Object { x: 0.0, y: 10.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 150.0 },
                init_velocity: Object { x: 0.0, y: 10.0 },
                actual_position: Object { x: 0.0, y: 155.1 },
                actual_velocity: Object { x: 0.0, y: 0.2 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 150.0 },
                init_velocity: Object { x: 0.0, y: 10.0 },
                actual_position: Object { x: 0.0, y: 150.4 },
                actual_velocity: Object { x: 0.0, y: -9.6 },
                time: 2.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 150.0 },
                init_velocity: Object { x: 0.0, y: 10.0 },
                actual_position: Object { x: 0.0, y: 135.9 },
                actual_velocity: Object { x: 0.0, y: -19.4 },
                time: 3.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 150.0 },
                init_velocity: Object { x: 0.0, y: 10.0 },
                actual_position: Object { x: 0.0, y: 111.6 },
                actual_velocity: Object { x: 0.0, y: -29.2 },
                time: 4.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 150.0 },
                init_velocity: Object { x: 0.0, y: 10.0 },
                actual_position: Object { x: 0.0, y: 77.5 },
                actual_velocity: Object { x: 0.0, y: -39.0 },
                time: 5.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 150.0 },
                init_velocity: Object { x: 0.0, y: 10.0 },
                actual_position: Object { x: 0.0, y: 33.6 },
                actual_velocity: Object { x: 0.0, y: -48.8 },
                time: 6.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }
}
