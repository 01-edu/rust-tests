#[derive(Debug, Clone)]
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
    pub fn new() -> Queue {
        Queue { node: None }
    }

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
        if self.clone().node.as_ref().is_none() {
            return None;
        }
        let mut q = Queue::new();
        let result = recursion_rm(&self.clone().node, &mut q);
        *self = q;
        self.invert_queue();
        return Some(result);
    }

    pub fn search(&self, s: &str) -> Option<(String, u32)> {
        recursion(&self.clone().node, s.to_string())
    }
}

fn recursion(node: &Link, s: String) -> Option<(String, u32)> {
    if node.as_ref().is_none() {
        return None;
    }
    let a = node.as_ref().unwrap();
    if a.name == s {
        return Some((a.name.clone(), a.discount.clone()));
    }
    return recursion(&node.as_ref().unwrap().next_person, s);
}

fn recursion_rm(node: &Link, q: &mut Queue) -> (String, u32) {
    let a = node.as_ref().unwrap();
    if !a.next_person.is_none() {
        q.add(a.name.clone(), a.discount.clone());
        return recursion_rm(&node.as_ref().unwrap().next_person, q);
    } else {
        return (a.as_ref().name.clone(), a.as_ref().discount.clone());
    }
}

fn recursion_inv(node: &Link, q: &mut Queue) {
    let a = node.as_ref();
    if !a.is_none() {
        q.add(a.unwrap().name.clone(), a.unwrap().discount.clone());
        return recursion_inv(&node.as_ref().unwrap().next_person, q);
    }
}
