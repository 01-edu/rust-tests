/*
## ref_cell

### Instructions

#### First part (messenger.rs)

Create a module named `messenger`. This module will be able to inform a user of how many references of a given value they are using.
The main objective of this module is to limit how many times a value is referenced.

For this module the following must be created:

Implement `Logger`: a trait which implements the following three functions:

```rust
fn warning(&self, msg: &str);
fn info(&self, msg: &str);
fn error(&self, msg: &str);
```

Implement the `Tracker` structure with the following fields:

- `logger`: a reference to `Logger`.
- `value`: the count of how many times the value was referenced. It should not exceed `max`.
- `max`: the max count of references.

Add the following associated functions to `Tracker`:

- `new`: that initializes the structure.
- `set_value`: that sets the `value`. It should compare the number of references to `value` and `max` to work out the percentage used. It should write to the following traits if it exceeds the specified usage percentage:
  - percentage >= 100%: `"Error: you are over your quota!"` should be written to `error`.
  - percentage >= 70% and percentage < 100%: `"Warning: you have used up over X% of your quota! Proceeds with precaution"` should be written to `warning`, where `X` should be replaced with the calculated percentage.
- `peek`: that will take a peek at how much usage the variable already has. It should write `"Info: you are using up to X% of your quota"` to the `info` trait function. `X` should be replaced with the calculated percentage.

### Second part (lib.rs)

Now that you've created `messenger`, you can now create the following:

Create the `Worker` structure with the following fields:

- `track_value`: which is the value that will be tracked by the tracker.
- `mapped_messages`: that will store the latest messages from the `Logger` trait functions. This will be a HashMap. The key will represent the type of message (`info`, `error` or `warning`), and the value will be the actual message.
- `all_messages`: that will be a vector of **all** messages sent.

Create the following associated functions for `Worker`:

- `new`: that initializes a `Worker` structure.
- `Logger`: to use the trait `Logger`, you must implement it for the `Worker` structure. Each function (`warning`, `error` and `info`) must insert the message to the respective field of the `Worker` structure.

You must use **interior mutability**, this means it must be possible to mutate data, even when there are immutable references to that data. Consequently, the user will not need to use the keyword `mut`. _tip:_ RefCell.

### Usage

Here is a program to test your function,

```rust
use ref_cell::*;

fn main() {
    // initialize the worker
    let logger = Worker::new(1);

    // initialize the tracker, with the max number of
    // called references as 10
    let track = Tracker::new(&logger, 10);

    let _a = logger.track_value.clone();    // |\
    let _a1 = logger.track_value.clone();   // | -> increase the Rc to 4 references
    let _a2 = logger.track_value.clone();   // |/

    // take a peek of how much we already used from our quota
    track.peek(&logger.track_value);

    let _b = logger.track_value.clone();  // |\
    let _b1 = logger.track_value.clone(); // |  -> increase the Rc to 8 references
    let _b2 = logger.track_value.clone(); // | /
    let _b3 = logger.track_value.clone(); // |/

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);

    let _c = logger.track_value.clone(); // | -> increase the Rc to 9 references

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);

    let _c1 = logger.track_value.clone(); // | -> increase the Rc to 10 references, this will be the limit

    track.set_value(&logger.track_value);

    for (k ,v) in logger.mapped_messages.into_inner() {
        println!("{:?}", (k ,v));
    }
    println!("{:?}", logger.all_messages.into_inner());
}
```

And its output:

```console
$ cargo run
("Info", "you are using up to 40% of your quota")
("Warning", "you have used up over 90% of your quota! Proceeds with precaution")
("Error", "you are over your quota!")
[
  "Info: you are using up to 40% of your quota",
  "Warning: you have used up over 80% of your quota! Proceeds with precaution",
  "Warning: you have used up over 90% of your quota! Proceeds with precaution",
  "Error: you are over your quota!"
]
$
```

### Notions

- [std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- [Struct std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)


*/

mod messenger;
pub use messenger::*;
pub use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(s: usize) -> Worker {
        Worker {
            track_value: Rc::new(s),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(vec![]),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
    fn info(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
    fn error(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        #[derive(Clone, Debug)]
        struct TestMs {
            value: Rc<usize>,
            ms: RefCell<Vec<String>>,
            correct: Vec<String>,
        }

        impl Logger for TestMs {
            fn warning(&self, message: &str) {
                self.ms.borrow_mut().push(message.to_string());
            }
            fn info(&self, message: &str) {
                self.ms.borrow_mut().push(message.to_string());
            }
            fn error(&self, message: &str) {
                self.ms.borrow_mut().push(message.to_string());
            }
        }

        let l = TestMs {
            value: Rc::new(115),
            ms: RefCell::new(vec![]),
            correct: vec![
                String::from("Info: you are using up to 40% of your quota"),
                String::from(
                    "Warning: you have used up over 80% of your quota! Proceeds with precaution",
                ),
                String::from("Error: you are over your quota!"),
            ],
        };

        let track = Tracker::new(&l, 5);
        let _a = l.value.clone();
        track.peek(&l.value); // 40%
        let _a1 = l.value.clone();
        let _a2 = l.value.clone();
        track.set_value(&l.value); // 80%
        let _a3 = l.value.clone();
        track.set_value(&l.value); // 100%

        for (i, v) in l.ms.into_inner().iter().enumerate() {
            assert_eq!(v, &l.correct[i])
        }
    }

    #[test]
    fn test_module_usage_hasmap() {
        let log = Worker::new(1000);
        let track = Tracker::new(&log, 12);
        let _clone_test = log.track_value.clone();
        let _clone_test1 = log.track_value.clone();
        let _clone_test2 = log.track_value.clone();
        let _clone_test3 = log.track_value.clone();
        let _clone_test4 = log.track_value.clone();
        let _clone_test5 = log.track_value.clone();
        let _clone_test6 = log.track_value.clone();
        let _clone_test7 = log.track_value.clone();

        // warning: 75% of the quota
        track.set_value(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Warning").unwrap(),
            "you have used up over 75% of your quota! Proceeds with precaution"
        );

        let _clone_test8 = log.track_value.clone();

        // warning: 83% of the quota <- most resent of the messages last onw to be added to the hashmap
        track.set_value(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Warning").unwrap(),
            "you have used up over 83% of your quota! Proceeds with precaution"
        );

        // info: 83%
        track.peek(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Info").unwrap(),
            "you are using up to 83% of your quota"
        );

        let _clone_test9 = log.track_value.clone();
        // info: 91%
        track.peek(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Info").unwrap(),
            "you are using up to 91% of your quota"
        );

        let _clone_test10 = log.track_value.clone();
        // error: passed the quota
        track.set_value(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Error").unwrap(),
            "you are over your quota!"
        );
    }

    #[test]
    fn test_module_usage_vector() {
        let correct = vec![
            "Info: you are using up to 40% of your quota",
            "Warning: you have used up over 80% of your quota! Proceeds with precaution",
            "Info: you are using up to 80% of your quota",
            "Error: you are over your quota!",
        ];
        let log = Worker::new(1);
        let track = Tracker::new(&log, 5);
        let _a = log.track_value.clone();
        // info: 40%
        track.peek(&log.track_value);
        let _a1 = log.track_value.clone();
        let _a2 = log.track_value.clone();

        // warning: 80%
        track.set_value(&log.track_value);
        // info: 80%
        track.peek(&log.track_value);
        let _a3 = log.track_value.clone();

        // error: passed the quota
        track.set_value(&log.track_value);

        for (i, v) in log.all_messages.into_inner().iter().enumerate() {
            assert_eq!(v, correct[i]);
        }
    }
}
