#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Underboss,
    Soldier,
    Caporegime,
    Associate,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    #[inline]
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            _ => unreachable!(),
        };
    }
}
