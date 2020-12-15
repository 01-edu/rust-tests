pub mod boss;
pub mod member;

use boss::Boss;
use member::{new, Member, Role};

#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, member_name: &str, member_age: u8) {
        self.members
            .push(new(member_name, Role::Associate, member_age));
    }

    pub fn attack(&mut self, target: &mut Mob) {
        if calculate_power(self) > calculate_power(target) {
            target.members.pop();
        } else {
            self.members.pop();
        }

        if self.members.len() == 0 {
            switch_cities(target, self);
            target.wealth += self.wealth;
            self.cities = vec![];
            self.wealth = 0;
        } else if target.members.len() == 0 {
            switch_cities(self, target);
            self.wealth += target.wealth;
            target.cities = vec![];
            target.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u32) {
        self.wealth += value;
        target.wealth -= value;
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, wanted_city: String, value: u8) {
        let result = mobs
            .into_iter()
            .filter(|ele| ele.cities.iter().any(|(city, _)| city == &wanted_city))
            .collect::<Vec<Mob>>();

        if result.is_empty() {
            self.cities.push((wanted_city, value));
        }
    }
}

fn calculate_power(mob: &Mob) -> usize {
    let mut result = 0;
    for member in &mob.members {
        match member.role {
            Role::Underboss => result += 4,
            Role::Caporegime => result += 3,
            Role::Soldier => result += 2,
            Role::Associate => result += 1,
        }
    }
    result
}

fn switch_cities(winner: &mut Mob, loser: &Mob) {
    for city in &loser.cities {
        winner.cities.push(city.clone());
    }
}
