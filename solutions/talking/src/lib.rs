/*
## talking

### Instructions

Build the function `talking` that will allow you to talk with your computer.

His answers will be created by you following the rules below.

- He answers "There is no need to yell, calm down!" if you yell at him, for example "LEAVE ME ALONE!"
(it is consider yelling when the sentence is all written in capital letters).
- He answers "Sure" if you ask him something without yelling, for example "Is everything ok with you?"
- He answers "Quiet, I am thinking!" if you yell a question at him. "HOW ARE YOU?"
- He says "Just say something!" if you address him without actually saying anything.
- He answers "Interesting" to anything else.

### Notions

- https://doc.rust-lang.org/book/ch18-00-patterns.html

### Expected functions

```rust
fn talking(text: &str) -> &str {}
```

### Usage

Here is a program to test your function.

```rust
fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
"There is no need to yell, calm down!"
"Sure."
"Quiet, I am thinking!"
"Interesting"
"Just say something!"
student@ubuntu:~/[[ROOT]]/test$
```
*/

pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_yelling = trimmed[..trimmed.len() - 1].chars().all(is_uppercase)
        && trimmed.chars().any(char::is_alphabetic);
    let is_question = match trimmed.chars().last() {
        Some('?') => true,
        _ => false,
    };

    if is_question && is_yelling {
        "Quiet, I am thinking!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else {
        "Interesting"
    }
}

pub fn is_uppercase(c: char) -> bool {
    !c.is_alphabetic() || c.is_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_yell() {
        assert_eq!(
            talking("JUST DO IT!"),
            "There is no need to yell, calm down!"
        );
        assert_eq!(
            talking("1, 2, 3 GO!"),
            "There is no need to yell, calm down!"
        );
        assert_eq!(
            talking("I LOVE YELLING"),
            "There is no need to yell, calm down!"
        );
        assert_eq!(
            talking("WJDAGSAF ASVF EVA VA"),
            "There is no need to yell, calm down!"
        );
    }

    #[test]
    fn test_question() {
        assert_eq!(talking("Hello how are you?"), "Sure.");
        assert_eq!(talking("Are you going to be OK?"), "Sure.");
        assert_eq!(talking("7?"), "Sure.");
        assert_eq!(talking("Like 15?"), "Sure.");
    }

    #[test]
    fn test_question_yelling() {
        assert_eq!(talking("WHAT'S GOING ON?"), "Quiet, I am thinking!");
        assert_eq!(talking("ARE YOU FINISHED?"), "Quiet, I am thinking!");
        assert_eq!(talking("WHAT DID I DO?"), "Quiet, I am thinking!");
        assert_eq!(talking("ARE YOU COMING?"), "Quiet, I am thinking!");
    }

    #[test]
    fn test_interesting() {
        assert_eq!(talking("something"), "Interesting");
        assert_eq!(talking("Wow that's good!"), "Interesting");
        assert_eq!(talking("Run far"), "Interesting");
        assert_eq!(talking("1 2 3 go!"), "Interesting");
        assert_eq!(talking("This is not ? a question."), "Interesting");
    }

    #[test]
    fn test_empty() {
        assert_eq!(talking(""), "Just say something!");
        assert_eq!(talking("										"), "Just say something!");
        assert_eq!(talking("          "), "Just say something!");
    }
}
