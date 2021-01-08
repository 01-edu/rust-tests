/*
## borrow box

### Instructions

You will have to create a **CRUD** functionality. Therefore creating the following functions :

- `new`, that receives two players and initializes them with a name and a score. This functions should
  return the structure wrapped in a Box.

- `read_winner`, returns a tuple with the name and the score of the player who is winning.
  In case none of the players are winning, it should return the same tuple with the string "Same score! tied" and the tied score.

  - `update_score`, that receives the name of a player.
  This function should increment the score of the player. The score should only be increased if it does not pass the `nbr_of_games`.
  Example: if the nbr_of_games is 3 then the game should be best out of three. So if one play as 2 wins then
  he is the winner and the function should not increase the score anymore for either players.

- `delete`, that takes the ownership of the boxed game and returning a string : "Game deleted: id -> 0".

### Notions

- https://doc.rust-lang.org/book/ch15-01-box.html

*/

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_of_games: u16
}

impl Game {
    // create the box
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<Game> {
        Box::new(Game { id: i, p1: (pl1, 0), p2: (pl2, 0), nbr_of_games: n })
    }

    // read from the box using the reference `&`
    // return only the player that as the bigger score
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            self.p1.clone()
        } else if self.p1.1 < self.p2.1 {
            self.p2.clone()
        } else {
            (String::from("Same score! tied"), self.p2.1)
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        let total = self.p1.1 + self.p2.1;
        if self.p1.0 == user_name && total != self.nbr_of_games {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name && total != self.nbr_of_games {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        String::from(format!("game deleted: id -> {:?}", self.id))
    }
}

/*
fn main() {
    let mut game = Game::create_game(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 0)

    game.update_score(String::from("Joao"));
    game.update_score(String::from("Joao"));
    game.update_score(String::from("Susana"));
    game.update_score(String::from("Susana"));
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 2)

    game.update_score(String::from("Joao"));
    // this one will not count because it already 5 games played, the nbr_of_games
    game.update_score(String::from("Susana"));
    
    println!("{:?}", game.read_winner());
    // output : ("Joao", 3)

    game.delete();
    println!("{:?}", game.delete());
    // output : "game deleted: id -> 0"

    // game.read_winner();
    // this will give error
    // because the game was dropped, no longer exists on the heap
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn create_games() -> Vec<Box<Game>> {
        vec![
            Game::new(0, String::from("player1"), String::from("player2"), 1),
            Game::new(1, String::from("Alice"), String::from("Mark"), 3),
            Game::new(2, String::from("Jack"), String::from("Miller"), 5)
        ]
    }

    #[test]
    fn test_create() {
        let games = create_games();
        assert_eq!(*games[0], Game {id: 0, p1: (String::from("player1"), 0), p2: (String::from("player2"), 0), nbr_of_games: 1});
        assert_eq!(*games[1], Game {id: 1, p1: (String::from("Alice"), 0), p2: (String::from("Mark"), 0), nbr_of_games: 3});
        assert_eq!(*games[2], Game {id: 2, p1: (String::from("Jack"), 0), p2: (String::from("Miller"), 0), nbr_of_games: 5});
    }

    #[test]
    fn test_update_and_read() {
        let mut games = create_games();
        games[0].update_score(String::from("player1"));
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[0].update_score(String::from("player2"));
        // this will stay the same because the nbr_of_games is 1 so if one
        // of the players wins just once it will no longer increment the score
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Miller"));
        games[2].update_score(String::from("Miller"));
        // tie between players
        assert_eq!(games[2].read_winner(), (String::from("Same score! tied"), 2));

        games[2].update_score(String::from("Jack"));
        assert_eq!(games[2].read_winner(), (String::from("Jack"), 3));
    }

    #[test]
    fn test_delete() {
        let game = Game::new(0, String::from("Alice"), String::from("Mark"), 3);
        let game1 = Game::new(23, String::from("Jack"), String::from("Miller"), 1);

        assert_eq!(game.delete(), String::from("game deleted: id -> 0"));
        assert_eq!(game1.delete(), String::from("game deleted: id -> 23"));
    }

    // #[test]
    // #[should_panic]
    // fn test_delete_ownership() {
    //     let game = new(0, String::from("Alice"), String::from("Mark"), 3);
    //     {
    //         let a = &game;
    //         // error! cant destroy boxed while the inner value is borrowed later in scope
    //         delete(game);
    //         read_winner(&a);
    //     }
    // }
}
