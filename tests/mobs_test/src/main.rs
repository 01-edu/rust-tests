use mobs::*;

fn main() {
    let (mafia1, mafia2) = (
        Mob {
            name: "Hairy Giants".to_string(),
            boss: boss::Boss::new("Louie HaHa", 36),
            cities: vec![("San Francisco".to_string(), 7)],
            members: vec![
                member::Member::new("Benny Eggs", member::Role::Soldier, 28),
                member::Member::new("Jhonny", member::Role::Associate, 17),
                member::Member::new("Greasy Thumb", member::Role::Soldier, 30),
                member::Member::new("No Finger", member::Role::Caporegime, 32),
            ],
            wealth: 100000,
        },
        Mob {
            name: "Red Thorns".to_string(),
            boss: boss::Boss::new("Big Tuna", 30),
            cities: vec![("San Jose".to_string(), 5)],
            members: vec![
                member::Member::new("Knuckles", member::Role::Soldier, 25),
                member::Member::new("Baldy Dom", member::Role::Caporegime, 36),
                member::Member::new("Crazy Joe", member::Role::Underboss, 23),
            ],
            wealth: 70000,
        },
    );

    println!("{:?}\n{:?}", mafia1, mafia2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_boss_and_members() {
        assert_eq!(
            boss::Boss::new("Scarface", 43),
            boss::Boss {
                name: "Scarface".to_string(),
                age: 43
            }
        );
        assert_eq!(
            member::Member::new("Crazy Joe", member::Role::Soldier, 27),
            member::Member {
                name: "Crazy Joe".to_string(),
                role: member::Role::Soldier,
                age: 27
            }
        );
        assert_eq!(
            member::Member::new("Louie HaHa", member::Role::Caporegime, 30),
            member::Member {
                name: "Louie HaHa".to_string(),
                role: member::Role::Caporegime,
                age: 30
            }
        );
    }

    fn create_mobs() -> (Mob, Mob) {
        (
            Mob {
                name: "Hairy Giants".to_string(),
                boss: boss::Boss::new("Louie HaHa", 36),
                cities: vec![("San Francisco".to_string(), 7)],
                members: vec![
                    member::Member::new("Benny Eggs", member::Role::Soldier, 28),
                    member::Member::new("Jhonny", member::Role::Associate, 17),
                    member::Member::new("Greasy Thumb", member::Role::Soldier, 30),
                    member::Member::new("No Finger", member::Role::Caporegime, 32),
                ],
                wealth: 100000,
            },
            Mob {
                name: "Red Thorns".to_string(),
                boss: boss::Boss::new("Big Tuna", 30),
                cities: vec![("San Jose".to_string(), 5)],
                members: vec![
                    member::Member::new("Knuckles", member::Role::Soldier, 25),
                    member::Member::new("Baldy Dom", member::Role::Caporegime, 36),
                    member::Member::new("Crazy Joe", member::Role::Underboss, 23),
                ],
                wealth: 70000,
            },
        )
    }

    #[test]
    fn mob_recruit() {
        let (mut a, _b) = create_mobs();
        a.recruit("Rusty", 37);

        assert_eq!(
            a.members[a.members.len() - 1],
            member::Member::new("Rusty", member::Role::Associate, 37)
        );
    }

    #[test]
    fn mob_steal() {
        let (mut a, mut b) = create_mobs();
        b.steal(&mut a, 10000);
        assert_eq!(a.wealth, 90000);
        assert_eq!(b.wealth, 80000);

        b.steal(&mut a, 100000);
        assert_eq!(a.wealth, 0);
        assert_eq!(b.wealth, 170000);
    }
    #[test]
    fn mob_attack() {
        let (mut a, mut b) = create_mobs();
        a.attack(&mut b);

        assert_eq!(a.members.len(), 3);
        assert_eq!(b.members.len(), 3);
    }

    #[test]
    fn member_get_promotion() {
        let (mut a, _) = create_mobs();
        a.members[0].get_promotion();
        assert_eq!(a.members[0].role, member::Role::Caporegime);
        a.members[0].get_promotion();
        assert_eq!(a.members[0].role, member::Role::Underboss);
    }

    #[test]
    fn same_combat_power() {
        let (mut a, mut b) = create_mobs();

        a.recruit("Stitches", 28);
        a.attack(&mut b);

        assert_eq!(a.members.len(), 4);
        assert_eq!(b.members.len(), 3);
    }

    #[test]
    fn no_members_mob() {
        let (mut a, mut b) = create_mobs();
        b.attack(&mut a);
        a.attack(&mut b);
        b.attack(&mut a);
        b.attack(&mut a);

        assert_eq!(a.members.len(), 0);
        assert_eq!(a.cities.len(), 0);
        assert_eq!(a.wealth, 0);

        assert_eq!(b.cities[0], ("San Jose".to_string(), 5));
        assert_eq!(b.cities[1], ("San Francisco".to_string(), 7));
        assert_eq!(b.wealth, 170000);
    }

    #[test]
    fn mob_conquer_city() {
        let (mut a, mut b) = create_mobs();

        b.conquer_city(vec![a.clone()], "Las Vegas".to_string(), 9);
        assert_eq!(b.cities[1], ("Las Vegas".to_string(), 9));

        a.conquer_city(vec![b.clone()], "Las Vegas".to_string(), 6);
        assert_eq!(a.cities.len(), 1);
    }
}
