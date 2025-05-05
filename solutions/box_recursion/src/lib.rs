#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    #[inline]
    fn from(role: &str) -> Self {
        match role {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            "Normal Worker" => Role::Worker,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let worker = Worker {
            role: Role::from(role),
            name: name.to_owned(),
            next: self.grade.take(),
        };

        self.grade = Some(Box::new(worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|node| {
            self.grade = node.next;
            node.name
        })
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade
            .as_ref()
            .map(|node| (node.name.clone(), node.role))
    }
}
