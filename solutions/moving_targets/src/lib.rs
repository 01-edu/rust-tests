#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}

#[derive(Default)]
pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: Target,
    next: Link,
}

impl Field {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn push(&mut self, target: Target) {
        let new_node = Box::new(Node {
            elem: target,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Target> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    #[inline]
    pub fn peek(&self) -> Option<&Target> {
        self.head.as_ref().map(|node| &node.elem)
    }

    #[inline]
    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}
