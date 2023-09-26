/*
## invert_sentence

### Instructions

Write a function called `invert_sentence` that takes a string as input and returns the words in the string in reverse order.
In other words, the function should take a sentence as input and return a new sentence with the words reversed.

```rust
pub fn invert_string(string: &str) -> String {
    // Your code goes here
}
```

### Usage

Here is a possible runner to test your function :

```rust
use invert_sentence::invert_sentence;

fn main() {
    println!("{}", invert_sentence("Rust is Awesome"));
    println!("{}", invert_sentence("   word1     word2  "));
    println!("{}", invert_sentence("Hello, World!"));
}
```

And its output:

```console
$ cargo run | cat -e
Awesome is Rust$
word2 word1$
World! Hello,$
$
```

*/

pub fn invert_sentence(input: &str) -> String {
    // Split the input sentence into words
    let words: Vec<&str> = input.split_whitespace().collect();

    // Reverse the order of words and join them back into a single string
    let inverted_sentence = words.into_iter().rev().collect::<Vec<&str>>().join(" ");

    // Return the inverted sentence as a String
    inverted_sentence
}
