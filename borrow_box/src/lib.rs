/*
## borrow box

### Instructions

You will have to create a **CRUD** functionality. Therefore creating the following functions :

- `create_game`, that receives two players and initializes them with a name and a score. This functions should
  return the structure wrapped in a Box.

- `read_winner`, that receives a game and returns a tuple with the name and the score of the player who is winning.
  In case none of the players are winning, it should return the same tuple with the string "Same score! tied" and the tied score.

- `update_score`, that receives a copy of the owned value (Game) in the box and the name of a player.
  This function should increment the score of the player and return the structure wrapped in a box. The
  score should only be increased if it does not pass the `nbr_of_games`.
  Example: if the nbr_of_games is 3 then the game should be best out of three. So if one play as 2 wins then
  he is the winner and the function should not increase the score anymore for either players.

- `delete`, that takes the ownership of the boxed game and returning a string : "Game deleted: id -> 0".

### Notions

- https://doc.rust-lang.org/book/ch15-01-box.html

*/

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    p1: (String, u16),
    p2: (String, u16),
    nbr_of_games: u16
}

// create the box
fn create_game(i: u32, pl1: String, pl2: String, n: u16) -> Box<Game> {
    Box::new(Game { id: i, p1: (pl1, 0), p2: (pl2, 0), nbr_of_games: n })
}

// read from the box using the reference `&`
// return only the player that as the bigger score
fn read_winner(box_ref: &Game) -> (String, u16) {
    if box_ref.p1.1 > box_ref.p2.1 {
        box_ref.p1.clone()
    } else if box_ref.p1.1 < box_ref.p2.1 {
        box_ref.p2.clone()
    } else {
        (String::from("Same score! tied"), box_ref.p2.1)
    }
}

// update_score the box by using `*` taking the owner "value" (a copy of the owner value)
fn update_score(mut owned_value: Game, user_name: String) -> Box<Game> {
    let total = owned_value.p1.1 + owned_value.p2.1;
    if owned_value.p1.0 == user_name && total != owned_value.nbr_of_games {
        owned_value.p1.1 += 1;
    } else if owned_value.p2.0 == user_name && total != owned_value.nbr_of_games {
        owned_value.p2.1 += 1;
    }
    Box::new(owned_value)
}

// delete value, this will just transferrer the ownership to the function
// this way the box will be deleted when the scope ends "the box value should no longer live"
 fn delete(b: Box<Game>) -> String {
     String::from(format!("game deleted: id -> {:?}", b.id))
 }

/*
fn main() {
    let mut game = create_game(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", read_winner(&game));
    // output : ("Same score! tied", 0)

    game = update_score(*game.clone(), String::from("Joao"));
    game = update_score(*game.clone(), String::from("Joao"));
    game = update_score(*game.clone(), String::from("Susana"));
    game = update_score(*game.clone(), String::from("Susana"));
    println!("{:?}", read_winner(&game));
    // output : ("Same score! tied", 2)

    game = update_score(*game.clone(), String::from("Joao"));
    // this one will not count because it already 5 games played, the nbr_of_games
    game = update_score(*game.clone(), String::from("Susana"));
    
    println!("{:?}", read_winner(&game));
    // output : ("Joao", 3)

    println!("{:?}", delete(game));
    // output : "game deleted: id -> 0"

    // read_winner(&game);
    // this will give error
    // because the delete function already took the ownership of the value `game`
}
*/


#[cfg(test)]
mod tests {
    use super::*;

    fn create_games() -> Vec<Box<Game>> {
        vec![
            create_game(0, String::from("player1"), String::from("player2"), 1),
            create_game(1, String::from("Alice"), String::from("Mark"), 3),
            create_game(2, String::from("Jack"), String::from("Miller"), 5)
        ]
    }

    fn test_struct(game: &Game, results: (u32, &str, &str, u16)) {
        assert_eq!(game.id, results.0);
        assert_eq!(game.p1, (results.1.to_string(), 0));
        assert_eq!(game.p2, (results.2.to_string(), 0));
        assert_eq!(game.nbr_of_games, results.3);
    } 

    #[test]
    fn test_create() {
        let games = create_games();
        test_struct(&games[0], (0, "player1", "player2", 1));
        test_struct(&games[1], (1, "Alice", "Mark", 3));
        test_struct(&games[2], (2, "Jack", "Miller", 5));
    }

    #[test]
    fn test_update_and_read() {
        let mut games = create_games();
        games[0] = update_score(*games[0].clone(), String::from("player1"));
        assert_eq!(read_winner(&games[0]), (String::from("player1"), 1));

        games[0] = update_score(*games[0].clone(), String::from("player2"));
        // this will stay the same because the nbr_of_games is 1 so if one
        // of the players wins just once it will no longer increment the score
        assert_eq!(read_winner(&games[0]), (String::from("player1"), 1));

        games[2] = update_score(*games[2].clone(), String::from("Jack"));
        games[2] = update_score(*games[2].clone(), String::from("Jack"));
        games[2] = update_score(*games[2].clone(), String::from("Miller"));
        games[2] = update_score(*games[2].clone(), String::from("Miller"));
        // tie between players
        assert_eq!(read_winner(&games[2]), (String::from("Same score! tied"), 2));

        games[2] = update_score(*games[2].clone(), String::from("Jack"));
        assert_eq!(read_winner(&games[2]), (String::from("Jack"), 3));
    }

    #[test]
    fn test_delete() {
        let game = create_game(0, String::from("Alice"), String::from("Mark"), 3);
        let game1 = create_game(23, String::from("Jack"), String::from("Miller"), 1);

        assert_eq!(delete(game), String::from("game deleted: id -> 0"));
        assert_eq!(delete(game1), String::from("game deleted: id -> 23"));
    }

    // #[test]
    // #[should_panic]
    // fn test_delete_ownership() {
    //     let game = create_game(0, String::from("Alice"), String::from("Mark"), 3);
    //     {
    //         let a = &game;
    //         // error! cant destroy boxed while the inner value is borrowed later in scope
    //         delete(game);
    //         read_winner(&a);
    //     }
    // }
}
