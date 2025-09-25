use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Blog {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_article(&self, body: String) -> (usize, Article<'_>) {
        let new_article = Article::new(self.new_id(), body, self);
        self.states.borrow_mut().push(false);
        (new_article.id, new_article)
    }

    #[inline]
    pub fn new_id(&self) -> usize {
        self.states.borrow().len() // const when borrow is stabilised
    }

    #[inline]
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{} is already dropped", id)
        }
        self.states.borrow_mut()[id] = true;
        self.drops.update(|x| x + 1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    id: usize,
    body: String,
    parent: &'a Blog,
}

impl<'a> Article<'a> {
    pub const fn new(id: usize, body: String, parent: &'a Blog) -> Self {
        Self { id, body, parent }
    }

    #[inline]
    pub fn discard(self) {}
}

impl Drop for Article<'_> {
    fn drop(&mut self) {
        self.parent.add_drop(self.id);
    }
}
