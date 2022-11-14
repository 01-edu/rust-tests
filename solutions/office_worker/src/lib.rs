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
        let entries = s.split(',').collect::<Vec<&str>>();
        OfficeWorker {
            name: entries.get(0).unwrap().to_string(),
            age: entries.get(1).unwrap().parse().unwrap(),
            role: WorkerRole::from(*entries.get(2).unwrap()),
        }
    }
}

impl From<&str> for WorkerRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            _ => WorkerRole::Guest,
        }
    }
}
