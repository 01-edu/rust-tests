/*
## box_it

### Instructions

Create the following functions:

- `transform_and_save_on_heap`, that receives a string with a number (that can or not have a k (kilo) suffix)
  and transforms those numbers into `u32` and inserts it into a vector that must be saved in the heap using **Box**.

- `take_value_ownership`, that takes the value (unboxed value) form the box and returns it

### Notions

- https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
- https://doc.rust-lang.org/book/ch15-01-box.html

*/
fn convert(s: &str) -> u32 {
    let mut a = s.to_string();
    a.pop();
    let n = a.parse::<f32>().unwrap();
    (n * 1000.0) as u32
}

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut v: Vec<u32> = Vec::new();

    for token in s.split_whitespace() {
        if token.contains("k") {
            v.push(convert(token));
        } else {
            v.push(token.parse::<u32>().unwrap());
        }
    }
    Box::new(v)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

/*
// Example :

fn main() {
    let new_str = String::from("5.5k 8.9k 32");

    // creating a variable and we save it in the Heap
    let a_h = transform_and_save_on_heap(new_str);
    println!("Box value : {:?}", &a_h);
    println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_h)));

    let a_b_v = take_value_ownership(a_h);
    println!("value : {:?}", &a_b_v);
    println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_b_v)));
    // output :
    // | Box value : [6800, 13500]
    // | size occupied in the stack : 8 bytes
    // | value : [6800, 13500]
    // | size occupied in the stack : 24 bytes

    // whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_transform() {
        let new_str = String::from("5.5k 8.9k 32");
        let new_str_1 = String::from("6.8k 13.5k");
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992");

        let a = transform_and_save_on_heap(new_str);
        let b = transform_and_save_on_heap(new_str_1);
        let c = transform_and_save_on_heap(new_str_2);

        assert_eq!(a, Box::new(vec![5500, 8900, 32]));
        assert_eq!(b, Box::new(vec![6800, 13500]));
        assert_eq!(c, Box::new(vec![20300, 3800, 7700, 992]));
        assert_eq!(mem::size_of_val(&a), 8);
        assert_eq!(mem::size_of_val(&b), 8);
        assert_eq!(mem::size_of_val(&c), 8);
    }

    #[test]
    fn test_take_value_from_box() {
        let new_str = String::from("5.5k 8.9k 32");
        let new_str_1 = String::from("6.8k 13.5k");
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992");
        let a = take_value_ownership(transform_and_save_on_heap(new_str));
        let b = take_value_ownership(transform_and_save_on_heap(new_str_1));
        let c = take_value_ownership(transform_and_save_on_heap(new_str_2));

        assert_eq!(a, vec![5500, 8900, 32]);
        assert_eq!(b, vec![6800, 13500]);
        assert_eq!(c, vec![20300, 3800, 7700, 992]);
        assert_eq!(mem::size_of_val(&a), 24);
        assert_eq!(mem::size_of_val(&b), 24);
        assert_eq!(mem::size_of_val(&c), 24);
    }
}
