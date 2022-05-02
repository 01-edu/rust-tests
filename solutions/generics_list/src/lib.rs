/*
## generics_list

### Instructions

Create a linked list of generic values with the following methods.

- `new` which returns a new empty list.

- `push` which adds at the begining of the list a new element.

- `pop` which deletes the last added element to the list.

- `len` which returns size of the list.



### Expected Functions

```rust
#[derive(Clone)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
    }

    pub fn push(&mut self, value: T) {
    }

    pub fn pop(&mut self) {
    }

    pub fn len(&self) -> usize {
    }
}
```

*/

#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = Node { next: None, value };

        match self.head {
            None => self.head = Some(new_node),
            Some(_) => {
                let current_head = self.head.take().unwrap();
                new_node.next = Some(Box::new(current_head));
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop(&mut self) {
        self.head
            .take()
            .map(|head| self.head = head.next.map(|node| *node));
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            count = count + 1;
            current_node = node.next.as_ref().map(|node| &**node)
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_list_test() {
        let mut new_list_str = List::new();
        new_list_str.push("String Test");

        let mut new_list_num = List::new();
        new_list_num.push(5);

        assert_eq!(new_list_str.head.unwrap().value, "String Test");
        assert_eq!(new_list_num.head.unwrap().value, 5);
    }

    #[test]
    fn big_list_test() {
        let mut new_list_nbr = List::new();

        for i in 0..10 {
            new_list_nbr.push(i);
        }

        let mut aux = new_list_nbr.head.unwrap();
        for i in (1..10).collect::<Vec<i32>>().iter().rev() {
            assert_eq!(aux.value, *i);
            aux = *aux.next.unwrap();
        }
    }

    #[test]
    fn remove_test() {
        let mut new_list_nbr = List::new();

        for i in 0..10 {
            new_list_nbr.push(i);
        }

        assert_eq!(new_list_nbr.len(), 10);

        for _ in 0..5 {
            new_list_nbr.pop();
        }

        assert_eq!(new_list_nbr.len(), 5);

        let mut aux = new_list_nbr.clone().head.unwrap();
        for i in (1..5).collect::<Vec<i32>>().iter().rev() {
            assert_eq!(aux.value, *i);
            aux = *aux.next.unwrap();
        }

        for _ in 0..5 {
            assert_eq!(new_list_nbr.head.as_ref().is_none(), false);
            new_list_nbr.pop();
        }

        assert_eq!(new_list_nbr.head.as_ref().is_none(), true);
    }

    #[test]
    fn len_test() {
        let mut new_list_nbr = List::new();

        assert_eq!(new_list_nbr.len(), 0);

        for i in 0..10 {
            new_list_nbr.push(i);
            assert_eq!(new_list_nbr.len(), i + 1);
        }
    }
}
