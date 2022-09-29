#[allow(dead_code)]
pub struct Message {
    pub content: String,
    pub user: String,
}

#[allow(dead_code)]
impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }
    // allows the student to use options
    #[allow(dead_code)]
    pub fn send_ms(&self) -> Option<&str> {
        match (&self.content[..], "stupid") {
            (x, p) if x.contains(p) || x == "" => None,
            _ => Some(&self.content),
        }
    }
}

// the student can catch the None and return it as an error
#[allow(dead_code)]
pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(e) => (true, e),
        None => (false, "ERROR: illegal"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_ms() {
        let v = vec![
            Message::new("".to_string(), "toby".to_string()),
            Message::new("stupid".to_string(), "jack".to_string()),
            Message::new("you are stupid".to_string(), "jacob".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(!t);
        }
    }

    #[test]
    fn test_ok_ms() {
        let v = vec![
            Message::new("get out of the car".to_string(), "police".to_string()),
            Message::new("no!".to_string(), "thief".to_string()),
            Message::new("get the werewolf".to_string(), "police".to_string()),
            Message::new("wait the wha...".to_string(), "thief".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(t);
        }
    }
}
