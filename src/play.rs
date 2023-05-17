pub mod game {
    use std::convert::{From, Into};
    use std::fmt::Display;

    pub enum Depth {
        Unlimited,
        Depth(usize),
    }
    
    pub trait Game: Clone + Copy + Display + Eq + Sized + Into<Self> + From<String> {
        fn new() -> Self;
        fn moves(&self) -> Box<dyn Iterator<Item = Self>>;
        fn score(&self, depth: Depth) -> isize;
    }

    /// A game of tic-tac-toe.
    /// The game state is represented as a 32-bit unsigned integer.
    /// The first bit signifies the next player to move, 1 = X, 0 = O.
    /// The next 9 bits represent which squares are occupied by X.
    /// The next 9 bits represent which squares are occupied by O.
    #[derive(Debug)]
    pub struct TicTacToe {
        state: u32,
    }

    impl TicTacToe {
        fn won(&self) -> isize {
            let x = ((self.state >> 22) ^ 0b1000000000) & 0b111111111;
            let o = ((self.state >> 13) ^ 0b1000000000) & 0b111111111;

            // check for X horizontal wins
            if x & 0b111000000 == 0b111000000 ||
            x & 0b000111000 == 0b000111000 ||
            x & 0b000000111 == 0b000000111 ||

            // check for X vertical wins
            x & 0b100100100 == 0b100100100 ||
            x & 0b010010010 == 0b010010010 ||
            x & 0b001001001 == 0b001001001 ||

            // check for X diagonal wins
            x & 0b100010001 == 0b100010001 ||
            x & 0b001010100 == 0b001010100 {
                return 1;
            }

            // check for O horizontal wins
            if o & 0b111000000 == 0b111000000 ||
            o & 0b000111000 == 0b000111000 ||
            o & 0b000000111 == 0b000000111 ||

            // check for O vertical wins
            o & 0b100100100 == 0b100100100 ||
            o & 0b010010010 == 0b010010010 ||
            o & 0b001001001 == 0b001001001 ||

            // check for O diagonal wins
            o & 0b100010001 == 0b100010001 ||
            o & 0b001010100 == 0b001010100 {
                return -1;
            }

            return 0;
        }
    }

    impl Game for TicTacToe {
        /// Create a new game of tic-tac-toe.
        /// The first player to move is X.
        /// No squares are occupied.
        fn new() -> Self {
            TicTacToe {
                state: 0b1_000000000_000000000_0000000000000,
            }
        }

        /// Return an iterator over all possible moves.
        /// A move is represented as a new game state.
        /// The iterator is empty if there are no moves for the current player.
        fn moves(&self) -> Box<dyn Iterator<Item = TicTacToe>> {
            let mut moves = Vec::new();

            if self.won() != 0 {
                return Box::new(moves.into_iter());
            }

            let occupied = ((self.state << 1) | (self.state << 10)) & 0b11111111100000000000000000000000;
            let lsb = (self.state >> 31) & 0b1;

            for i in 0..9 {
                if occupied & (1 << (31 - i)) == 0 {
                    moves.push(TicTacToe {
                        state: if lsb == 0 {
                            (self.state | (1 << (21 - i))) ^ (1 << 31)
                        } else {
                            (self.state | (1 << (30 - i))) ^ (1 << 31)
                        },
                    });
                }
            }

            Box::new(moves.into_iter())
        }

        /// Return the score of the current game state.
        /// A score of 1 means X wins, -1 means O wins, 0 means a draw.
        fn score(&self, depth: Depth) -> isize {
            let won = self.won();
            if won != 0 {
                return won;
            } else {
                match depth {
                    Depth::Depth(_) => todo!(),
                    Depth::Unlimited => {
                        if self.moves().count() == 0 {
                            return won
                        } else {
                            if (self.state >> 31) & 0b1 == 1 {
                                return self.moves().map(|m| m.score(Depth::Unlimited)).max().unwrap();
                            } else {
                                return self.moves().map(|m| m.score(Depth::Unlimited)).min().unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    impl From<String> for TicTacToe {
        fn from(s: String) -> Self {
            let mut state = 0;

            if s.len() != 9 {
                panic!("Invalid game string length");
            }

            let num_x = s.chars().filter(|c| *c == 'X').count() as isize;
            let num_o = s.chars().filter(|c| *c == 'O').count() as isize;

            if (num_x - num_o).abs() > 1 {
                panic!("Invalid game string");
            }

            // an even number of Xs and Os means it's X's turn
            if (num_x + num_o) % 2 == 0 {
                state |= 1 << 31;
            }

            for (i, c) in s.chars().enumerate() {
                match c {
                    'X' => state |= 1 << (30 - i),
                    'O' => state |= 1 << (21 - i),
                    '-' | ' ' | '_' => (),
                    _ => panic!("Invalid character in game string"),
                }
            }

            TicTacToe { state }
        }
    }

    impl Into<String> for TicTacToe {
        fn into(self) -> String {
            let mut s = String::new();

            for i in 0..9 {
                if self.state & (1 << (30 - i)) != 0 {
                    s.push('X');
                } else if self.state & (1 << (21 - i)) != 0 {
                    s.push('O');
                } else {
                    s.push('-');
                }
            }

            s
        }
    }

    impl Copy for TicTacToe {}

    impl Clone for TicTacToe {
        fn clone(&self) -> Self {
            TicTacToe { state: self.state }
        }
    }

    impl Eq for TicTacToe {}

    impl PartialEq for TicTacToe {
        fn eq(&self, other: &Self) -> bool {
            self.state == other.state
        }
    }

    impl Display for TicTacToe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "\n")?;
            for (i, c) in Into::<String>::into(*self).chars().enumerate() {
                let c = match c {
                    'X' => 'X',
                    'O' => 'O',
                    _ => ' ',
                };
                match i {
                    3 | 6 => write!(f, "\n═══╬═══╬═══\n {c} ║")?,
                    2 | 5 | 8 => write!(f, " {c} ")?,
                    0 | 1 | 4 | 7 => write!(f, " {c} ║")?,
                    _ => (),
                }
            }

            Ok(())
        }
    }
}
