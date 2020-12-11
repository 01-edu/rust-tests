/*
## RefCell

### Instructions

### 1º part

Create a module called `messenger`. This module will be able to inform a user of how much references of a given value he is using.
The main objective for this module is to limit how many times a value is referenced.

For the module you must create the following:

- A trait `Logger` that implements three functions: `warning`, `info`, `error`. All function should receive a reference to themselves and a string literal.

```rust
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
```

- A structure called `Tracker`, that must have the fields: `logger` being a reference to the Logger, `value` being the count of how many times the value was referenced,
  `max` being the max count of references the actual value can achieve.

- An implementation of three functions that are associated to the `Tracker` structure:
  - `new` that will initialize the structure
  - `set_value` that sets the value to the `Tracker` structure and writes to the trait functions. This should be done comparing the **max** and the number of referenced of the actual value.
    If the percentage is equal or greater to 100% of the limit usage, it should write "Error: you are over your quota!" to the `error` function
    If the percentage is equal or greater to 70% of the limit usage, it should write ("Warning: you have used up over {}% of your quota! Proceeds with precaution", <calculated_percentage>)" to the `warning` function
  - `peek` that will take a peek of how much usage the variable already has. It should write ("Info: you are using up too {}% of your quote", <calculated_percentage>) to the `info` function

### 2ª part

Afterwards you must use the module `messenger` and create the following:

- A structure `Worker` that has the fields:
  -`track_value` this will be the value that will be tracked by the tracker.
  - `mapped_messages` that will have the latest messages. This must be a HashMap with the key being the type of message
  sent by the logger (info, error or warning) and the value being the message
  - `all_messages` that will be a vector of all messages sent.

- A `new` function that initializes the structure `Worker`

- To use the trait `Logger` you must implement it for the Worker structure. Each function (warning, error and info) must insert the message to the
  respective fields of the structure Worker.

You must use **interior mutability**, this means it must be able to mutate data even when there are immutable references to that data.

So the user doesn't need to use the keyword `mut` (tip: RefCell)

### Example

```rust
fn main() {
    // initialize the worker
    let Logger = Worker::new(1);

    // initialize the tracker, with the max number of
    // called references as 10
    let track = Tracker::new(&Logger, 10);

    let _a = Logger.track_value.clone();    // |\
    let _a1 = Logger.track_value.clone();   // | -> increase the Rc to 4 references
    let _a2 = Logger.track_value.clone();   // |/

    // take a peek of how much we already used from our quota
    track.peek(&Logger.track_value);

    let _b = Logger.track_value.clone();  // |\
    let _b1 = Logger.track_value.clone(); // |  -> increase the Rc to 8 references
    let _b2 = Logger.track_value.clone(); // | /
    let _b3 = Logger.track_value.clone(); // |/

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&Logger.track_value);

    let _c = Logger.track_value.clone(); // | -> increase the Rc to 9 references

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&Logger.track_value);

    let _c1 = Logger.track_value.clone(); // | -> increase the Rc to 10 references, this will be the limit

    track.set_value(&Logger.track_value);

    for (k ,v) in Logger.mapped_messages.into_inner() {
        println!("{:?}", (k ,v));
    }
    // output:
    // ("Info", "you are using up too 40% of your quote")
    // ("Warning", "you have used up over 90% of your quota! Proceeds with precaution")
    // ("Error", "you are over your quota!")

    println!("{:?}", Logger.all_messages.into_inner());
    // [
    //   "Info: you are using up too 40% of your quote",
    //   "Warning: you have used up over 80% of your quota! Proceeds with precaution",
    //   "Warning: you have used up over 90% of your quota! Proceeds with precaution",
    //   "Error: you are over your quota!"
    // ]

}
```
### Notions

- https://doc.rust-lang.org/std/cell/struct.RefCell.html
- https://doc.rust-lang.org/std/rc/struct.Rc.html

*/

mod messenger;
use messenger::{Logger, Tracker};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Worker {
    track_value: Rc<usize>,
    mapped_messages: RefCell<HashMap<String, String>>,
    all_messages: RefCell<Vec<String>>,
}

impl Worker {
    fn new(s: usize) -> Worker {
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
        self.mapped_messages.borrow_mut().insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
    fn info(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages.borrow_mut().insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
    fn error(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages.borrow_mut().insert(v[0].to_string(), v[1].to_string());
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
              String::from("Info: you are using up too 40% of your quote"),
              String::from("Warning: you have used up over 80% of your quota! Proceeds with precaution"),
              String::from("Error: you are over your quota!")
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
        assert_eq!(log.mapped_messages.borrow().get("Warning").unwrap(), "you have used up over 75% of your quota! Proceeds with precaution");

        let _clone_test8 = log.track_value.clone();

        // warning: 83% of the quota <- most resent of the messages last onw to be added to the hashmap
        track.set_value(&log.track_value);
        assert_eq!(log.mapped_messages.borrow().get("Warning").unwrap(), "you have used up over 83% of your quota! Proceeds with precaution");

        // info: 83%
        track.peek(&log.track_value);
        assert_eq!(log.mapped_messages.borrow().get("Info").unwrap(), "you are using up too 83% of your quote");

        let _clone_test9 = log.track_value.clone();
        // info: 91%
        track.peek(&log.track_value);
        assert_eq!(log.mapped_messages.borrow().get("Info").unwrap(), "you are using up too 91% of your quote");

        let _clone_test10 = log.track_value.clone();
        // error: passed the quota
        track.set_value(&log.track_value);
        assert_eq!(log.mapped_messages.borrow().get("Error").unwrap(), "you are over your quota!");
    }

    #[test]
    fn test_module_usage_vector() {
        let correct = vec![
            "Info: you are using up too 40% of your quote",
            "Warning: you have used up over 80% of your quota! Proceeds with precaution",
            "Info: you are using up too 80% of your quote", "Error: you are over your quota!"
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
