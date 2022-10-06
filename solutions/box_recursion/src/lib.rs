#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_node = Box::new(Worker {
            role: role,
            name: name,
            next: self.grade.take(),
        });
        self.grade = Some(new_node);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|node| {
            self.grade = node.next;
            node.name
        })
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade
            .as_ref()
            .map(|node| (node.name.clone(), node.role.clone()))
    }
}
