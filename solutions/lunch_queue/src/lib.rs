#[derive(Debug, Clone, Default)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub discount: u32,
    pub next_person: Link,
}

impl Queue {
    #[inline]
    pub fn new() -> Queue {
        Default::default()
    }

    #[inline]
    pub fn add(&mut self, name: String, discount: u32) {
        let new_node = Box::new(Person {
            name,
            discount,
            next_person: self.node.take(),
        });
        self.node = Some(new_node);
    }

    pub fn invert_queue(&mut self) {
        let mut q = Queue::new();
        recursion_inv(&self.clone().node, &mut q);
        *self = q;
    }

    pub fn rm(&mut self) -> Option<(String, u32)> {
        let mut q = Queue::new();
        let result = recursion_rm(&self.node, &mut q);
        *self = q;
        self.invert_queue();
        return Some(result);
    }

    pub fn search(&self, s: &str) -> Option<(&String, &u32)> {
        recursion(&self.node, s.to_string())
    }
}

fn recursion(node: &Link, s: String) -> Option<(&String, &u32)> {
    let Some(node) = node.as_ref() else {
        return None;
    };

    if node.name == s {
        return Some((&node.name, &node.discount));
    }
    return recursion(&node.next_person, s);
}

fn recursion_rm(node: &Link, q: &mut Queue) -> (String, u32) {
    let Some(node) = node.as_ref() else {
        unreachable!();
    };

    if node.next_person.is_some() {
        q.add(node.name.clone(), node.discount);
        return recursion_rm(&node.next_person, q);
    } else {
        return (node.name.clone(), node.discount);
    }
}

fn recursion_inv(node: &Link, q: &mut Queue) {
    let Some(node) = node.as_ref() else {
        return;
    };

    q.add(node.name.clone(), node.discount.clone());
    return recursion_inv(&node.next_person, q);
}
