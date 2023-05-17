pub mod play;
use crate::play::game::*;

fn main() {
    let game = TicTacToe::from("XXOOXX--O".to_string());

    println!("Game: {}", game);

    println!("Score: {}", game.score(None));
}
