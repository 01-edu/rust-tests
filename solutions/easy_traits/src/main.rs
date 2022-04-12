// Your task is to implement the trait
// `AppendStr' for the type `String',
// `AppendVec' for a vector of strings.

// The trait AppendStr has only one function,
// which appends a string to any object
// implementing this trait.
// AppendVec does the same but to a vector

trait AppendStr {
    fn append_str(self) -> Self;
}

trait AppendVec {
    fn append_vec(&mut self) -> Self;
}

impl AppendStr for String {
    //Add your code here
    fn append_str(self) -> Self {
        let hello = String::from("Bar");
        return hello;
    }
}

impl AppendVec for Vec<String> {
    fn append_vec(&mut self) -> Self {
        self.push(String::from("Bar"));
        self.to_vec()
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_str();
    println!("s: {}", s);

    let mut l = vec![String::from("-w"), String::from("60"), String::from("arg")];
    let l = l.append_vec();

    println!("vector: {:?}", l);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_str(), String::from("FooBar"));
    }

    #[test]
    fn is_BarBar() {
        assert_eq!(
            String::from("").append_str().append_str(),
            String::from("BarBar")
        );
    }
}

#[test]
fn is_vec_pop_eq_bar() {
    let mut foo = vec![String::from("Foo")].append_vec();
    assert_eq!(foo.pop().unwrap(), String::from("Bar"));
    assert_eq!(foo.pop().unwrap(), String::from("Foo"));
}
