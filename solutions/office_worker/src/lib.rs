#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let mut entries = s.split(',');

        let v = Self {
            name: entries.next().unwrap().to_owned(),
            age: entries.next().unwrap().parse().unwrap(),
            role: WorkerRole::from(entries.next().unwrap()),
        };

        assert!(entries.next().is_none());

        v
    }
}

impl From<&str> for WorkerRole {
    #[inline]
    fn from(s: &str) -> Self {
        match s {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => unreachable!(),
        }
    }
}
