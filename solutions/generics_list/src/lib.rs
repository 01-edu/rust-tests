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
    #[inline]
    pub fn new() -> Self {
        Self {
            head: Default::default(),
        }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = Node {
            next: Default::default(),
            value,
        };

        match self.head.take() {
            None => self.head = Some(new_node),
            Some(current_head) => {
                new_node.next = Some(Box::new(current_head));
                self.head = Some(new_node);
            }
        }
    }

    #[inline]
    pub fn pop(&mut self) {
        self.head
            .take()
            .map(|head| self.head = head.next.map(|h| *h));
    }

    #[inline]
    pub fn len(&self) -> usize {
        std::iter::successors(self.head.as_ref(), |node| node.next.as_deref()).count()
    }
}
