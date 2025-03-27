pub mod boss;
pub mod member;

use std::collections::{HashMap, HashSet};

pub use boss::*;
pub use member::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    #[inline]
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_owned(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    #[inline]
    fn calculate_power(&self) -> usize {
        self.members
            .values()
            .map(|m| match m.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            })
            .sum()
    }

    #[inline]
    fn give_cities(&mut self, to: &mut Mob) {
        to.cities.extend(self.cities.drain())
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let (winner, loser) = if self.calculate_power() > target.calculate_power() {
            (self, target)
        } else {
            (target, self)
        };

        let youngest_age = loser.members.values().map(|m| m.age).min().unwrap();

        loser.members.retain(|_, m| m.age > youngest_age);

        if loser.members.is_empty() {
            loser.give_cities(winner);
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        let clamped = value.min(target.wealth);
        self.wealth += clamped;
        target.wealth -= clamped;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], wanted_city: String) {
        if !mobs
            .iter()
            .flat_map(|m| &m.cities)
            .any(|c| *c == wanted_city)
        {
            self.cities.insert(wanted_city);
        }
    }
}
