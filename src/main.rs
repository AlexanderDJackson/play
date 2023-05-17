pub mod play;
use crate::play::game::*;

#[derive(Clone, Debug)]
enum Player {
    Human,
    Computer,
}

fn play(player: (Player, Player)) {
    let mut game = TicTacToe::new();
    let mut moves = game.moves().peekable();
    let mut turn = vec![(player.0, 1), (player.1, -1)].into_iter().cycle();

    println!("{game}");

    while moves.peek().is_some() {
        game = match turn.next() {
            Some((player, piece)) => {
                match player {
                    Player::Human => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    let index = input.trim().parse::<usize>().unwrap();

                    game.moves().nth(index).unwrap()
                    },
                    Player::Computer => {
                        if piece < 0 {
                            game.moves().min_by_key(|m| m.score(Depth::Unlimited)).unwrap()
                        } else {
                            game.moves().max_by_key(|m| m.score(Depth::Unlimited)).unwrap()
                        }
                    },
                }
            },
            None => break,
        };

        println!("{game}");
        moves = game.moves().peekable();
    }

    if game.score(Depth::Unlimited) == 1 {
        println!("X wins!");
    } else if game.score(Depth::Unlimited) == -1 {
        println!("O wins!");
    } else {
        println!("Draw!");
    }
}

fn main() {
    play((Player::Computer, Player::Computer));
}
