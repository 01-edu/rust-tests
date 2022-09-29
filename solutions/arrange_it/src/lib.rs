/*
## arrange_it

### Instructions

Create a function called `arrange_phrase` that takes a string literal as a phrase and returns it organized
Each word will have a number that indicates the position of that word

### Example

```rust
fn main() {
    println!("{:?}", initials("is2 Thi1s T4est 3a"));
    // output: This is a Test
}
```

> This exercise will test the **heap allocation** of your function!
> So try your best to allocate the minimum data on the heap!

### Notions

- https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html
- https://doc.rust-lang.org/std/primitive.str.html#method.split

*/

pub fn arrange_phrase(phrase: &str) -> String {
    let nbrs: Vec<&str> = phrase.matches(char::is_numeric).collect();
    let a = &phrase.replace(char::is_numeric, "");
    let mut m: Vec<&str> = a.split_whitespace().collect();

    for (i, ele) in nbrs.iter().enumerate() {
        let strs: Vec<&str> = a.split_whitespace().collect();
        m[ele.parse::<usize>().unwrap() - 1] = strs[i];
    }
    m.join(" ")
}

// // example of function that works but does not pass the heap test
// pub fn arrange_phrase(phrase: &str) -> String {
// 	let mut vec = vec![0; 1024];
// 	vec.push(213);
// 	let words_nbr = phrase.matches(" ").count() + 1;
// 	let mut result_vec: Vec<String> = vec!["".to_string(); words_nbr];
// 	for word in phrase.split_whitespace().into_iter() {
// 		for i in 1..words_nbr + 1 {
// 			if word.contains(&i.to_string()) {
// 				result_vec[i - 1] = word.split(&i.to_string()).collect::<String>();
// 			}
// 		}
// 	}
// 	result_vec.join(" ")
// }
