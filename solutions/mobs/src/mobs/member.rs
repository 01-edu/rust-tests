#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Soldier,
    Caporegime,
    Associate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

impl Member {
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Caporegime => self.role = Role::Underboss,
            _ => {}
        }
    }

    pub fn new(name: &str, role: Role, age: u8) -> Self {
        Self {
            name: name.to_string(),
            role,
            age,
        }
    }
}
