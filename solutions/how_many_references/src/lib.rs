/*
## how many references

### Instructions

Create the following functions :

- `add_element` that adds an element to the value in the `Node`

- `how_many_references` that returns how many times the value is referenced in the code

- `rm_all_ref` that receives a `Rc<String>` and removes all elements from the vector that
  are equal to that value, this should only happen if the two Rcs point to the same allocation

### Notions

- https://doc.rust-lang.org/book/ch15-04-rc.html
- https://doc.rust-lang.org/std/rc/struct.Rc.html

*/

pub use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }

    pub fn add_element(&mut self, v: Rc<String>) {
        self.ref_list.push(v);
    }

    pub fn rm_all_ref(&mut self, v: Rc<String>) {
        self.ref_list.retain(|x| is_same_allocate(x, &v));
    }
}

fn is_same_allocate(x: &Rc<String>, v: &Rc<String>) -> bool {
    if is_eq(x, v) {
        return v != x;
    }
    return true;
}

pub fn how_many_references(value: &Rc<String>) -> usize {
    Rc::strong_count(value)
}

fn is_eq(value: &Rc<String>, cmp: &Rc<String>) -> bool {
    Rc::ptr_eq(value, cmp)
}

/*
fn main() {
    let a = Rc::new(String::from("a"));
    let b = Rc::new(String::from("b"));
    let c = Rc::new(String::from("c"));

    let a1 = Rc::new(String::from("a"));

    let mut new_node = Node::new(vec![a.clone()]);
    new_node.add_element(b.clone());
    new_node.add_element(a.clone());
    new_node.add_element(c.clone());
    new_node.add_element(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
    /*
    output:
        a: 4
        b: 2
        c: 2
    */
    new_node.rm_all_ref(a1.clone());
    new_node.rm_all_ref(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
    /*
    output:
        a: 1
        b: 2
        c: 2
    */
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_ele() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));

        let mut new_node = Node::new(vec![a.clone()]);
        new_node.add_element(a.clone());
        new_node.add_element(b.clone());
        new_node.add_element(c.clone());

        assert_eq!(new_node.value, vec![a.clone(), a, b, c]);
    }
    #[test]
    fn test_how_many_references() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let d = Rc::new(String::from("d"));
        let mut new_node = Node::new(vec![]);
        new_node.add_element(b.clone());
        new_node.add_element(a.clone());
        new_node.add_element(c.clone());
        new_node.add_element(a.clone());

        assert_eq!(how_many_references(&d), 1);
        assert_eq!(how_many_references(&a), 3);
        assert_eq!(how_many_references(&b), 2);
        assert_eq!(how_many_references(&c), 2);
    }

    #[test]
    fn test_rm_all_ref() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let d = Rc::new(String::from("d"));

        let a1 = Rc::new(String::from("a"));
        let b1 = Rc::new(String::from("b"));
        let c1 = Rc::new(String::from("c"));
        let d1 = Rc::new(String::from("d"));
        let mut new_node = Node::new(vec![
            d.clone(),
            d.clone(),
            b.clone(),
            a.clone(),
            c.clone(),
            a.clone(),
            d.clone(),
        ]);

        new_node.rm_all_ref(a1.clone());
        assert_eq!(how_many_references(&a), 3);
        new_node.rm_all_ref(a.clone());
        assert_eq!(how_many_references(&a), 1);

        new_node.rm_all_ref(b1.clone());
        assert_eq!(how_many_references(&b), 2);
        new_node.rm_all_ref(b.clone());
        assert_eq!(how_many_references(&b), 1);

        new_node.rm_all_ref(c1.clone());
        assert_eq!(how_many_references(&c), 2);
        new_node.rm_all_ref(c.clone());
        assert_eq!(how_many_references(&c), 1);

        new_node.rm_all_ref(d1.clone());
        assert_eq!(how_many_references(&d), 4);
        new_node.rm_all_ref(d.clone());
        assert_eq!(how_many_references(&d), 1);
    }
}
