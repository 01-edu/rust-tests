/*
## options

### Instructions

Sometimes it is more desirable to catch the failure of some parts of a program instead
of just calling panic

For this exercise you will have to create a structure called `Message`, this structure
must have the following elements:

- content: String
- user: String
- time_sent: String

The struct must also have a implementation of 2 functions associated to it:

- `new`, that initializes the structure
- `send_ms`, that only has its **self** as argument and returns an option. This function must
  return `None` if the content of the message is either **empty** or contains the word **stupid**.
  other wise it returns the content of the message.

You will have to create two more functions that aren't associated to any structure:

- `check_ms` that receives as parameters the reference to the structure `Message` and returns a tuple,
containing a `bool` and a `string`. This function will execute the function `send_ms` and if the result
of the option is `None` it should return (false, "ERROR: illegal"). Other wise it returns `true` and the
content of the message sent.
- `date_format` that creates and formats the date and time that the message was sent, the format should
look like this: **Mon Oct  5 10:22:19 2020**

### Notions

- https://docs.rs/chrono/0.4.19/chrono/
- https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html?highlight=option#the-option-enum-and-its-advantages-over-null-values

*/
use chrono::prelude::Utc;

#[allow(dead_code)]
struct Message {
  content: String,
  user: String,
  time_sent: String,
}

#[allow(dead_code)]
impl Message {
  fn new_ms(ms: String, u: String, t: String) -> Message {
    Message {
      content: ms,
      user: u,
      time_sent: t,
    }
  }
  // allows the student to use options
  #[allow(dead_code)]
  fn send_ms(&self) -> Option<&str> {
    match (&self.content[..], "stupid") {
      (x, p) if x.contains(p) || x == "" => None,
      _ => Some(&self.content),
    }
  }
}

// the student can catch the None and return it as an error
#[allow(dead_code)]
fn check_ms(ms: &Message) -> (bool, &str) {
  match ms.send_ms() {
    Some(e) => (true, e),
    None => (false, "ERROR: illegal"),
  }
}

#[allow(dead_code)]
fn format_date() -> String {
  Utc::now().format("%a %b %e %T %Y").to_string()
}

/*
// example
fn main() {
  let m0 = Message::new_ms("hello there".to_string(), "toby".to_string(), format_date());
  println!("{:?}", check_ms(&m0));

  let m1 = Message::new_ms("".to_string(), "toby".to_string(), format_date());
  println!("{:?}", check_ms(&m1));

  let m2 = Message::new_ms(
    "you are stupid".to_string(),
    "toby".to_string(),
    format_date(),
  );
  println!("{:?}", check_ms(&m2));

  let m3 = Message::new_ms(
    "stupid".to_string(),
    "toby".to_string(),
    format_date(),
  );
  println!("{:?}", check_ms(&m3));
}
*/
#[cfg(test)]
mod tests {
  use super::*;
  fn test_format() -> String {
    Utc::now().format("%a %b %e %T %Y").to_string()
  }

  #[test]
  fn test_time_format() {
    assert_eq!(format_date(), test_format());
  }

  #[test]
  fn test_error_ms() {
    let v = vec![
      Message::new_ms("".to_string(), "toby".to_string(), format_date()),
      Message::new_ms("stupid".to_string(), "jack".to_string(), format_date()),
      Message::new_ms(
        "you are stupid".to_string(),
        "jacob".to_string(),
        format_date(),
      ),
    ];
    for value in v {
      let (t, _) = check_ms(&value);
      assert!(!t);
    }
  }

  #[test]
  fn test_ok_ms() {
    let v = vec![
      Message::new_ms(
        "get out of the car".to_string(),
        "police".to_string(),
        format_date(),
      ),
      Message::new_ms("no!".to_string(), "thief".to_string(), format_date()),
      Message::new_ms(
        "get the werewolf".to_string(),
        "police".to_string(),
        format_date(),
      ),
      Message::new_ms(
        "wait the wha...".to_string(),
        "thief".to_string(),
        format_date(),
      ),
    ];
    for value in v {
      let (t, _) = check_ms(&value);
      assert!(t);
    }
  }
}
