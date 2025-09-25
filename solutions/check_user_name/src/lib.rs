pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    pub access_level: AccessLevel,
    pub name: String,
}

impl User {
    pub const fn new(user: String, access_level: AccessLevel) -> Self {
        Self {
            access_level,
            name: user,
        }
    }

    pub const fn send_name(&self) -> Option<&str> {
        match self.access_level {
            AccessLevel::Guest => None,
            _ => Some(self.name.as_str()),
        }
    }
}

pub const fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        Some(name) => (true, name),
        None => (false, "ERROR: User is guest"),
    }
}
