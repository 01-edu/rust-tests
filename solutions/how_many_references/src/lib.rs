use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    #[inline]
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self { ref_list }
    }

    #[inline]
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    #[inline]
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|e| !Rc::ptr_eq(&element, e));
    }
}

#[inline]
pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
